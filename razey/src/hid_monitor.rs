use hidapi::HidApi;
use std::thread;
use std::time::Duration;

const RAZER_VID: u16 = 0x1532;
const MOUSE_PID_WIRELESS: u16 = 0x00df;
const MOUSE_PID_WIRED: u16 = 0x00de;

// 레이저 HID 체크섬 계산
fn calculate_razer_checksum(data: &[u8; 91]) -> u8 {
    let mut checksum = 0;
    for i in 2..89 {
        checksum ^= data[i];
    }
    checksum
}

/// Razer 마우스의 배터리 퍼센트와 충전 상태를 조회합니다.
/// 반환값: Option<(배터리 잔량 %, 충전 중 여부)>
pub fn get_battery_percentage() -> Option<(u8, bool)> {
    let api = HidApi::new().ok()?;
    let pids = [MOUSE_PID_WIRELESS, MOUSE_PID_WIRED];

    for &pid in &pids {
        for device_info in api.device_list() {
            if device_info.vendor_id() == RAZER_VID
                && device_info.product_id() == pid
                && device_info.interface_number() == 0
            {
                if let Ok(dev) = device_info.open_device(&api) {
                    let mut query = [0u8; 91];
                    query[2] = 0x00;
                    query[6] = 0x02;
                    query[7] = 0x07;
                    query[8] = 0x80;
                    query[89] = calculate_razer_checksum(&query);

                    if dev.send_feature_report(&query).is_ok() {
                        thread::sleep(Duration::from_millis(100));
                        let mut response = [0u8; 91];
                        response[0] = 0; // Report ID
                        if dev.get_feature_report(&mut response).is_ok() {
                            if response[1] == 0x02 {
                                let raw_value = response[10];
                                let actual_percentage = ((raw_value as u32 * 100) / 255) as u8;
                                let is_charging = pid == MOUSE_PID_WIRED;
                                return Some((std::cmp::min(100, actual_percentage), is_charging));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
