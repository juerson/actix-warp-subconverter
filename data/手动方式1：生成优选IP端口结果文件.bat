chcp 936
cls
@echo off & setlocal enabledelayedexpansion
goto start

:start
if not exist "warp.exe" echo ȱ��warp.exe����&pause&exit
if not exist "ips-v4.txt" echo ȱ��IPV4���� ips-v4.txt&pause&exit
if not exist "ips-v6.txt" echo ȱ��IPV6���� ips-v6.txt&pause&exit
goto main

:main
title CF WARP ��ѡ
set /a menu=1
echo 1. WARP-V4��ѡ&echo 2. WARP-V6��ѡ&echo 0. �˳�&echo.
set /p menu=��ѡ��˵�(Ĭ��%menu%):
if %menu%==0 exit
if %menu%==1 title WARP-V4��ѡ&set filename=ips-v4.txt&goto getv4
if %menu%==2 title WARP-V6��ѡ&set filename=ips-v6.txt&goto getv6
cls
goto main

:getv4
for /f "delims=" %%i in (%filename%) do (
set !random!_%%i=randomsort
)
for /f "tokens=2,3,4 delims=_.=" %%i in ('set ^| findstr =randomsort ^| sort /m 10240') do (
call :randomcidrv4
if not defined %%i.%%j.%%k.!cidr! set %%i.%%j.%%k.!cidr!=anycastip&set /a n+=1
if !n! EQU 300 goto getip
)
goto getv4

:randomcidrv4
set /a cidr=%random%%%256
goto :eof

:getv6
for /f "delims=" %%i in (%filename%) do (
set !random!_%%i=randomsort
)
for /f "tokens=2,3,4 delims=_:=" %%i in ('set ^| findstr =randomsort ^| sort /m 10240') do (
call :randomcidrv6
if not defined [%%i:%%j:%%k::!cidr!] set [%%i:%%j:%%k::!cidr!]=anycastip&set /a n+=1
if !n! EQU 300 goto getip
)
goto getv6

:randomcidrv6
set str=0123456789abcdef
set /a r=%random%%%16
set cidr=!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!:!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!:!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!:!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
set /a r=%random%%%16
set cidr=!cidr!!str:~%r%,1!
goto :eof

:getip
del ip.txt > nul 2>&1
for /f "tokens=1 delims==" %%i in ('set ^| findstr =randomsort') do (
set %%i=
)
for /f "tokens=1 delims==" %%i in ('set ^| findstr =anycastip') do (
echo %%i>>ip.txt
)
for /f "tokens=1 delims==" %%i in ('set ^| findstr =anycastip') do (
set %%i=
)
warp
del ip.txt > nul 2>&1
echo �밴������رմ��ڼ���
pause>nul
exit