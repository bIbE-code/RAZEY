@echo off
chcp 65001 > nul
echo =========================================
echo   Razey 클린업 및 설정 원복 스크립트
echo =========================================
echo.

:: 1. 실행 중인 프로세스 종료
echo [1/2] 실행 중인 razey.exe 프로세스 종료 중...
taskkill /f /im razey.exe > nul 2>&1
if %errorlevel% equ 0 (
    echo ✔️ razey.exe 프로세스가 성공적으로 중단되었습니다.
) else (
    echo ℹ️ 실행 중인 razey.exe 프로세스가 없거나 이미 종료되었습니다.
)

echo.
:: 2. 레지스트리 자동 실행 등록 해제
echo [2/2] 레지스트리 시작 프로그램 등록 해제 중...
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "razey" /f > nul 2>&1
if %errorlevel% equ 0 (
    echo ✔️ 레지스트리에서 자동 실행 설정이 제거되었습니다.
) else (
    echo ℹ️ 등록된 레지스트리 키가 존재하지 않습니다.
)

echo.
echo =========================================
echo   모든 정리가 완료되었습니다.
echo =========================================
pause