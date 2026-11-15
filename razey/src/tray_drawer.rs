use image::{Rgba, RgbaImage};

/// 배터리 잔량과 충전 상태에 따른 시스템 트레이 아이콘을 동적으로 생성합니다.
pub fn draw_vertical_icon(percentage: u8, is_charging: bool) -> tray_icon::Icon {
    let width = 64;
    let height = 64;
    let mut img = RgbaImage::new(width, height);
    
    // 배터리 몸체 외곽선 (회색)
    for x in 20..=44 {
        for y in 14..=54 {
            if x == 20 || x == 44 || y == 14 || y == 54 {
                img.put_pixel(x, y, Rgba([160, 160, 160, 255]));
            }
        }
    }
    
    // 배터리 상단 캡 (회색)
    for x in 28..=36 {
        for y in 8..=13 {
            img.put_pixel(x, y, Rgba([160, 160, 160, 255]));
        }
    }

    // 배터리 내부 잔량 채우기
    if percentage > 0 {
        let mut fill_color = if percentage <= 20 {
            [255, 67, 67, 255] // 20% 이하: 빨간색
        } else {
            [255, 255, 255, 255] // 일반 잔량: 흰색
        };
        
        if is_charging {
            fill_color = [34, 197, 94, 255]; // 충전 중: 초록색
        }

        let padding = 6;
        let max_h = (54 - 14) - (padding * 2);
        let current_h = std::cmp::max(2, (max_h as f32 * (percentage as f32 / 100.0)) as u32);
        let start_y = 54 - padding - current_h;

        for x in (20 + padding)..=(44 - padding) {
            for y in start_y..=(54 - padding) {
                img.put_pixel(x, y, Rgba(fill_color));
            }
        }
    }

    // 트레이 아이콘 객체로 변환 반환
    tray_icon::Icon::from_rgba(img.into_raw(), width, height).unwrap()
}
