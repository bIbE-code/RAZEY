# 동작하지 않는다면 관리자 권한으로 실행하면 됩니다.

$appName = "razey"
$exeName = "razey.exe"
$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
$regKeyName = "razey"

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "   Razey 클린업 및 설정 원복 스크립트" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

# 1. 실행 중인 프로세스 강제 종료
Write-Host "`n[1/2] 실행 중인 $exeName 프로세스 확인 및 종료 중..." -ForegroundColor DarkYellow
$process = Get-Process -Name $appName -ErrorAction SilentlyContinue

if ($process) {
    Stop-Process -Name $appName -Force -ErrorAction SilentlyContinue
    Write-Host "✔️ $exeName 프로세스가 성공적으로 중단되었습니다." -ForegroundColor Green
} else {
    Write-Host "ℹ️ 실행 중인 $exeName 프로세스가 없습니다." -ForegroundColor Gray
}

# 2. 레지스트리 자동 실행 등록 해제
Write-Host "`n[2/2] 레지스트리 시작 프로그램 등록 해제 중..." -ForegroundColor DarkYellow
try {
    if (Test-Path "$regPath\$regKeyName") {
        Remove-ItemProperty -Path $regPath -Name $regKeyName -ErrorAction Stop
        Write-Host "✔️ 레지스트리($regPath)에서 자동 실행 설정이 제거되었습니다." -ForegroundColor Green
    } else {
        Write-Host "ℹ️ 등록된 레지스트리 키가 존재하지 않습니다." -ForegroundColor Gray
    }
} catch {
    Write-Host "❌ 레지스트리 삭제 중 오류가 발생했습니다: $_" -ForegroundColor Red
}

Write-Host "`n=========================================" -ForegroundColor Cyan
Write-Host "    모든 정리가 완료되었습니다." -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Pause
