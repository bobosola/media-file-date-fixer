:: Release build for 64 bit Windows
SET ARCH=x86_64-pc-windows-msvc
cargo build --target=%ARCH% --release
:: Copy the DLL to the windows application
SET PROJECT_DIR=C:\Users\Bob\rust\media-file-date-fixer
SET DLL_TO_COPY="%PROJECT_DIR%\target\x86_64-pc-windows-msvc\release\mfdf_ffi.dll"
SET DESTINATION="%PROJECT_DIR%\win_app\mfdf\mfdf_ffi.dll"
copy /y %DLL_TO_COPY% %DESTINATION%