:: Debug build for 64 bit Windows
cargo build --target=x86_64-pc-windows-msvc

:: Copy the DLL to the windows application
SET PROJECT_DIR=C:\Users\Bob\rust\media-file-date-fixer
SET DLL="%PROJECT_DIR%\target\x86_64-pc-windows-msvc\debug\mfdf_ffi.dll"
SET DESTINATION="%PROJECT_DIR%\win_app\mfdf"
copy /y %DLL% %DESTINATION%
