set pvm=pvm.zip
if exist %pvm% del %pvm%
.\scripts\7z.exe a %pvm% .\scripts\setup.bat .\target\release\pvm.exe
