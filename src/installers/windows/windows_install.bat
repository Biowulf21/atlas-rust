
@echo off
setlocal

set exeFilePath="C:\Path\to\your\file.exe"
set targetDirectory="C:\Path\to\your\target\directory"
set shortcutName="ShortcutName.lnk"
set desktopDirectory="%USERPROFILE%\Desktop"

rem Check if the .exe file exists
if not exist %exeFilePath% (
  echo .exe file does not exist.
  exit /b 1
)

rem Check if the target directory exists
if not exist %targetDirectory% (
  echo Target directory does not exist.
  exit /b 1
)

rem Move the .exe file to the target directory
move /y %exeFilePath% %targetDirectory%
if %errorlevel% neq 0 (
  echo Failed to move .exe file.
  exit /b 1
)

rem Create a desktop shortcut
set shortcutTarget="%targetDirectory%\file.exe"
set shortcutFilePath="%desktopDirectory%\%shortcutName%"
echo Set oWS = WScript.CreateObject("WScript.Shell") > CreateShortcut.vbs
echo sLinkFile = %shortcutFilePath% >> CreateShortcut.vbs
echo Set oLink = oWS.CreateShortcut(sLinkFile) >> CreateShortcut.vbs
echo oLink.TargetPath = %shortcutTarget% >> CreateShortcut.vbs
echo oLink.Save >> CreateShortcut.vbs
cscript /nologo CreateShortcut.vbs
del CreateShortcut.vbs
if %errorlevel% neq 0 (
  echo Failed to create desktop shortcut.
  exit /b 1
)

echo .exe file moved to target directory and desktop shortcut created successfully.
start "" "<path_to_your_rust_binary>\my_app.exe"
exit /b 0
