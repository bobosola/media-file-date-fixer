// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Data;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.WindowsRuntime;
using System.Text;
using Windows.Foundation;
using Windows.Foundation.Collections;
using Windows.Storage.Pickers;
using Windows.Storage;
using Microsoft.UI.Windowing;
using Microsoft.UI;
using Windows.Graphics;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace mfdf
{
    /// <summary>
    /// An empty window that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class MainWindow : Window
    {

        [DllImport("mfdf_ffi.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr make_report(IntPtr path);

        [DllImport("mfdf_ffi.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern void free_string(IntPtr ptr);

        public MainWindow()
        {
            this.InitializeComponent();
            var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            var windowId = Win32Interop.GetWindowIdFromWindow(hwnd);
            AppWindow appWindow = AppWindow.GetFromWindowId(windowId);
            // Set the desired size using Resize()
            appWindow.Resize(new SizeInt32(800, 600)); 
        }

        private string MakeReport(string path)
        {
            byte[] utf8Bytes = Encoding.UTF8.GetBytes(path);
            IntPtr utf8Ptr = Marshal.AllocHGlobal(utf8Bytes.Length + 1);
            try
            {
                Marshal.Copy(utf8Bytes, 0, utf8Ptr, utf8Bytes.Length);
                Marshal.WriteByte(utf8Ptr, utf8Bytes.Length, 0);

                IntPtr ptr = make_report(utf8Ptr);
                string result = Utf8PtrToString(ptr);
                FreeReport(ptr);
                return result;
            }
            finally
            {
                Marshal.FreeHGlobal(utf8Ptr);
            }
        }

        private async void OnPickFolderClicked(object sender, RoutedEventArgs e)
        {

            var folderPicker = new FolderPicker();
            folderPicker.SuggestedStartLocation = PickerLocationId.Desktop;

            // Initialize with current window handle (WinUI-specific)
            var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            WinRT.Interop.InitializeWithWindow.Initialize(folderPicker, hwnd);

            StorageFolder folder = await folderPicker.PickSingleFolderAsync();

            if (folder != null)
            {
                string folderPath = folder.Path;
                string result = MakeReport(folderPath);
                ResultText.Text = result;
            }
        }

        private string Utf8PtrToString(IntPtr ptr)
        {
            int length = 0;
            while (Marshal.ReadByte(ptr, length) != 0)
                length++;

            byte[] buffer = new byte[length];
            Marshal.Copy(ptr, buffer, 0, length);
            return Encoding.UTF8.GetString(buffer);
        }

        private void FreeReport(IntPtr ptr)
        {
            free_string(ptr);
        }

    }
}
