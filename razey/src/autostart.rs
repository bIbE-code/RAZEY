use winreg::enums::*;
use winreg::RegKey;

const APP_NAME: &str = "RazerBatteryMonitor";

/// 현재 실행 중인 실행 파일의 절대 경로를 반환합니다.
pub fn get_executable_path() -> String {
    std::env::current_exe()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Windows 레지스트리를 확인하여 시작프로그램 등록 여부를 반환합니다.
pub fn is_autostart_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        if let Ok(_val) = key.get_value::<String, _>(APP_NAME) {
            return true;
        }
    }
    false
}

/// Windows 레지스트리를 수정하여 시작프로그램 등록 및 해제를 수행합니다.
pub fn set_autostart(enable: bool) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        if enable {
            let exe_path = get_executable_path();
            let cmd = format!("\"{}\"", exe_path);
            let _ = key.set_value(APP_NAME, &cmd);
        } else {
            let _ = key.delete_value(APP_NAME);
        }
        return true;
    }
    false
}
