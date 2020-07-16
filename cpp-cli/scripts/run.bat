
#!/bin/sh
@echo off

SETLOCAL EnableDelayedExpansion

call .\scripts\build.bat  ||  (echo "build err" && exit /b !ERRORLEVEL!)
echo "build over"

.\build\build32\Release\_t_name_t_.exe || (echo "run err" && exit /b !ERRORLEVEL!)
echo "run over"