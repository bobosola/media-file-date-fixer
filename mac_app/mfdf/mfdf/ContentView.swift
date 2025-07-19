//
//  ContentView.swift
//  mfdf
//
//  Created by Bob Osola on 19/07/2025.
//

import SwiftUI

struct ContentView: View {
    @State private var introHidden = false
    @State private var report = ""

    var body: some View {
        ZStack {
            // 1. Blue gradient background
            LinearGradient(
                gradient: Gradient(colors: [
                    Color(red: 0.42, green: 0.73, blue: 0.98), // light sky
                    Color(red: 0.10, green: 0.35, blue: 0.87)  // deep cobalt
                ]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            // 2. Foreground content on top
            VStack(alignment: .leading, spacing: 16) {
                if !introHidden {
//                    Text("Select a directory and the Rust library will generate a short report for you.")
//                        .font(.body)
//                        .foregroundStyle(.white)   // <-- legible on blue
                    
                    let text = """
                        This app can recover:  
                        • the *Created* and *Modified* dates for common image file types  
                        • the *Created* date for common video file types

                        It fixes all the supported file types in your chosen directory plus any  
                        subdirectories. All other file types are ignored.

                        NB: files downloaded from social media often have their metadata removed  
                        for privacy reasons, so dates usually cannot be recovered for such files.
                        
                        More details at the [app website](https://osola.org.uk/utils)
                        """
                    
                    let attributed = try! AttributedString(
                        markdown: text,
                        options: .init(
                            allowsExtendedAttributes: true,
                            interpretedSyntax: .inlineOnlyPreservingWhitespace,
                            failurePolicy: .returnPartiallyParsedIfPossible
                        )
                    )

                    Text(attributed)
                        .padding()
                        .font(.body)
                     //   .foregroundStyle(.white)   // <-- legible on blue
                    
                    
                }

                Button("Choose Directory…") {
                    chooseDirectory()
                }
                .buttonStyle(.borderedProminent)
                .tint(.white.opacity(0.25))        // frosted glass look

                if !report.isEmpty {
                    ScrollView {
                        Text(report)
                            .font(.system(.body, design: .monospaced))
                            .textSelection(.enabled)
                            .foregroundStyle(.white)
                            .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .scrollContentBackground(.hidden) // macOS 13+
                    .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 8))
                    .padding(.top, 8)
                }

                Spacer()
            }
            .padding()
        }
        .frame(minWidth: 450, minHeight: 300)
    }

    // …chooseDirectory() & Rust FFI unchanged…

    private func chooseDirectory() {
        let panel = NSOpenPanel()
        panel.canChooseFiles   = false
        panel.canChooseDirectories = true
        panel.allowsMultipleSelection = false

        panel.begin { response in
            guard response == .OK, let url = panel.urls.first else { return }

            // 1. Start the security-scoped access
            guard url.startAccessingSecurityScopedResource() else {
                report = "❌ Could not acquire security-scoped access"
                return
            }
            defer { url.stopAccessingSecurityScopedResource() }

            // 2. Now the Rust library may write inside that directory
            let reportText = callRustReport(for: url.path)
            DispatchQueue.main.async {
                self.report = reportText
                self.introHidden = true
            }
        }
    }

    // -------------------------------------------------
    //  Rust FFI helpers
    // -------------------------------------------------
    private func callRustReport(for path: String) -> String {
       
        let ffiHandle: UnsafeMutableRawPointer? = {
            guard let h = dlopen("libmfdf_ffi.dylib", RTLD_NOW) else {
                fatalError("Could not open libmfdf_ffi.dylib")
            }
            return h
        }()

        // 2. Get the Rust symbols
         let make_report: @convention(c) (UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? = {
            guard let sym = dlsym(ffiHandle, "make_report") else {
                fatalError("symbol make_report not found")
            }
            return unsafeBitCast(sym,
                                 to: (@convention(c) (UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?).self)
        }()

        let free_string: @convention(c) (UnsafeMutablePointer<CChar>?) -> Void = {
            guard let sym = dlsym(ffiHandle, "free_string") else {
                fatalError("symbol free_string not found")
            }
            return unsafeBitCast(sym,
                                 to: (@convention(c) (UnsafeMutablePointer<CChar>?) -> Void).self)
        }()

        // 2. Call Rust
        let reportCString = path.withCString(make_report)

        guard let reportCString else {
            return "❌ Rust returned NULL"
        }
        defer { free_string(reportCString) }

        // 3. Copy into Swift string
        return String(cString: reportCString)
    }
}
