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
    @State private var showProgress = false
    
    var body: some View {
        ZStack {
            
            // Blue gradient background layer
            LinearGradient(
                gradient: Gradient(colors: [
                    Color(red: 0.85, green: 0.95, blue: 0.98),
                    Color(red: 0.96, green: 0.99, blue: 1)
                ]),
                startPoint: .top,
                endPoint: .bottom
            )

            // Foreground stack content on top
            VStack(alignment: .center) {
                
                if showProgress {
                    ProgressView("working...")
                }
                   
                Button("Choose Directory…") {
                    chooseDirectory()
                }
                .buttonStyle(.borderedProminent)
                .padding(20)
                .font(.system(size: 16))
                
                if !introHidden {
                    let introText = """
                        mfdf can recover:  
                        • the *Created* & *Modified* dates for most image files
                        • the *Created* dates for most video files
                        It works best for media files copied directly from a camera.
                        More details at the [app website](https://mfdf.osola.org.uk)
                        """
                    
                    let attributed = try! AttributedString(
                        markdown: introText,
                        options: .init(
                            allowsExtendedAttributes: true,
                            interpretedSyntax: .inlineOnlyPreservingWhitespace
                        )
                    )
                    Text(attributed)
                        .padding(25)
                        .font(.system(size: 16, weight: Font.Weight.medium))
                        .background(Color(red: 0.72, green: 0.91, blue: 0.97))
                        .cornerRadius(20)
                }

                if !report.isEmpty {
                    ScrollView {
                        Text(report)
                            .padding(25)
                            .font(.system(size: 14, weight: Font.Weight.medium))
                            .textSelection(.enabled)
                            .background(Color(red: 0.72, green: 0.91, blue: 0.97))
                    }
                  .background(Color(red: 0.72, green: 0.91, blue: 0.97))
                  .cornerRadius(20)
                }
                Spacer()
            }
            .padding(25)
        }
        .frame(minWidth: 450, minHeight: 300)
    }

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
            showProgress = true
            let reportText = callRustReport(for: url.path)
            DispatchQueue.main.async {
                self.report = reportText
                self.introHidden = true
                showProgress = false
            }
        }
    }

    // -------------------------------------------------
    //  Rust FFI helpers
    // -------------------------------------------------
    private func callRustReport(for path: String) -> String {
        
        let dylib = "libmfdf.dylib"
        
        let ffiHandle: UnsafeMutableRawPointer? = {
            guard let h = dlopen(dylib, RTLD_NOW) else {
                fatalError("Could not open mfdf dylib")
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
