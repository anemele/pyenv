@echo off

@rem where to place the executable?
copy /y %~dp0pvm.exe %SystemRoot%

if not defined PYTHON_VENV_PATH (
set PYTHON_VENV_PATH="%APPDATA%\Python\venv"
setx PYTHON_VENV_PATH "%APPDATA%\Python\venv"
)

if not exist %PYTHON_VENV_PATH% md %PYTHON_VENV_PATH%

where /q python
if %errorlevel% == 0 set python=python && goto :boot
where /q py
if %errorlevel% == 0 set python=py && goto :boot
echo NO python.exe found in PATH
exit /b

:boot
set base=%PYTHON_VENV_PATH%\base

if not exist %base% %python% -m venv %base%
call %base%\Scripts\activate.bat

if not exist %base%\Scripts\pip* python -m ensurepip
python -m pip install pip --upgrade -q

pip install virtualenv -q

if %errorlevel% neq 0 echo Failed. && exit /b
echo Done.
echo Use pvm to
echo   create a new env: pvm add foo
echo   list envs: pvm ls
echo   activate an env: pvm use foo
echo   remove an env: pvm rm foo
