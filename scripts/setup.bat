@echo off

set exe=pvm
@rem where to place the executable?
copy /y %~dp0%exe%.exe %SystemRoot%

if not defined PYTHON_VENV_PATH (
set PYTHON_VENV_PATH="%USERPROFILE%\.local\venv"
setx PYTHON_VENV_PATH "%USERPROFILE%\.local\venv"
)

if not exist %PYTHON_VENV_PATH% md %PYTHON_VENV_PATH%

cls

echo Done.
echo Use %exe% to
echo   create a new env: %exe% add foo
echo   list envs: %exe% ls
echo   activate an env: %exe% use foo
echo   remove an env: %exe% rm foo

pause
