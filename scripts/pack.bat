set rvv=rvv.zip
if exist %rvv% del %rvv%
.\scripts\7z.exe a %rvv% .\scripts\setup.bat .\target\release\rvv.exe
