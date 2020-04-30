set windbg="C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Windows Kits\Debugging Tools for Windows (X64)\WinDbg (X64).lnk"
set cwd=%cd%
set current_crate=%cwd%
set exe_path=%current_crate%\target\debug\dns-demo.exe
set current_src=%current_crate%\src
set symbol=%current_crate%\target\debug\deps

%windbg% -srcpath %current_src% -y %symbol% %exe_path% 