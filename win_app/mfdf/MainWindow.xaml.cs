
using Microsoft.UI.Xaml;
using System;
using System.Runtime.InteropServices;
using System.Text;
using Windows.Storage.Pickers;
using Windows.Storage;
using Microsoft.UI.Windowing;
using Microsoft.UI;
using Windows.Graphics;

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

        private IntPtr hWnd;

        public MainWindow()
        {
            this.InitializeComponent();
            Title = "Media File Date Fixer (mfdf)";

            // Resize window
            hWnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            WindowId windowId = Win32Interop.GetWindowIdFromWindow(hWnd);
            AppWindow appWindow = AppWindow.GetFromWindowId(windowId);
            appWindow.Resize(new SizeInt32(800, 600));
        }

        private static string MakeReport(string path)
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

            var folderPicker = new FolderPicker
            {
                SuggestedStartLocation = PickerLocationId.Desktop
            };
            // Associate the folder picker with the current window
            WinRT.Interop.InitializeWithWindow.Initialize(folderPicker, hWnd);

            StorageFolder folder = await folderPicker.PickSingleFolderAsync();
            if (folder != null)
            {
                string folderPath = folder.Path;
                // Pass the chosen folder path to the Rust DLL
                string result = MakeReport(folderPath);

                // Hide the intro text grid row
                MainGrid.RowDefinitions[0].Height = new GridLength(0);

                // Show the results in the output TextBlock control
                MainGrid.RowDefinitions[2].Height = new GridLength(350);

                if (String.IsNullOrEmpty(result))
                {
                    ResultText.Text = "Error: could not examine the chosen directory path";
                }
                else
                {
                    ResultText.Text = result.Replace("\\\\", "\\"); 
                }
                    
                ResultText.Visibility = Visibility.Visible;
            }
        }

        private static string Utf8PtrToString(IntPtr ptr)
        {
            int length = 0;
            while (Marshal.ReadByte(ptr, length) != 0)
                length++;

            byte[] buffer = new byte[length];
            Marshal.Copy(ptr, buffer, 0, length);
            return Encoding.UTF8.GetString(buffer);
        }

        private static void FreeReport(IntPtr ptr)
        {
            free_string(ptr);
        }
    }
}
