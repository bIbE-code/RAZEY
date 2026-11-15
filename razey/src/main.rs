#![windows_subsystem = "windows"]

mod autostart;
mod hid_monitor;
mod tray_drawer;

use notify_rust::Notification;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::TrayIconBuilder;

const MOUSE_NAME: &str = "Razer Viper V3 Pro SE";
const THRESHOLD_LOW: u8 = 15;
const THRESHOLD_CRITICAL: u8 = 5;
const THRESHOLD_FULL: u8 = 100;

#[derive(Debug, Clone)]
pub struct BatteryStatus {
    pub level: u8,
    pub is_charging: bool,
    pub connected: bool,
}

fn main() {
    if !autostart::is_autostart_enabled() {
        autostart::set_autostart(true);
    }

    let event_loop = EventLoopBuilder::new().build();
    let tray_menu = Menu::new();

    let autostart_title = if autostart::is_autostart_enabled() {
        "✓ 시작 시 자동 실행 (켜짐)"
    } else {
        "   시작 시 자동 실행 (꺼짐)"
    };
    let autostart_item = MenuItem::new(autostart_title, true, None);
    let quit_item = MenuItem::new("종료 (Quit)", true, None);

    let _ = tray_menu.append(&autostart_item);
    let _ = tray_menu.append(&PredefinedMenuItem::separator());
    let _ = tray_menu.append(&quit_item);

    let tray_icon = match TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip(MOUSE_NAME)
        .with_icon(tray_drawer::draw_vertical_icon(0, false))
        .build()
    {
        Ok(icon) => icon,
        Err(e) => {
            eprintln!("시스템 트레이 초기화 실패: {:?}", e);
            return;
        }
    };

    let (tx, rx) = mpsc::channel::<BatteryStatus>();

    thread::spawn(move || {
        let mut notified_low = false;
        let mut notified_critical = false;
        let mut notified_full = false;

        loop {
            if let Some((level, is_charging)) = hid_monitor::get_battery_percentage() {
                let status = BatteryStatus {
                    level,
                    is_charging,
                    connected: true,
                };
                let _ = tx.send(status);

                if level >= THRESHOLD_FULL && is_charging {
                    if !notified_full {
                        let _ = Notification::new()
                            .summary(&format!("{} 완충", MOUSE_NAME))
                            .body("배터리가 100% 충전되었습니다. 케이블을 분리하셔도 좋습니다.")
                            .timeout(7000)
                            .show();
                        notified_full = true;
                    }
                } else if level < THRESHOLD_FULL - 5 {
                    notified_full = false;
                }

                if level <= THRESHOLD_CRITICAL && !is_charging {
                    if !notified_critical {
                        let _ = Notification::new()
                            .summary(&format!("⚠️ {} 배터리 경고", MOUSE_NAME))
                            .body(&format!("배터리가 {}%입니다! 즉시 충전이 필요합니다.", level))
                            .timeout(15000)
                            .show();
                        notified_critical = true;
                    }
                } else if level <= THRESHOLD_LOW && !is_charging {
                    if !notified_low {
                        let _ = Notification::new()
                            .summary(&format!("{} 배터리 부족", MOUSE_NAME))
                            .body(&format!("현재 잔량이 {}%입니다.", level))
                            .timeout(10000)
                            .show();
                        notified_low = true;
                    }
                }

                if level > THRESHOLD_LOW + 5 || is_charging {
                    notified_low = false;
                    notified_critical = false;
                }
            } else {
                let _ = tx.send(BatteryStatus {
                    level: 0,
                    is_charging: false,
                    connected: false,
                });
            }

            thread::sleep(Duration::from_secs(30));
        }
    });

    let menu_channel = MenuEvent::receiver();
    let tray_icon_ref = tray_icon;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(100));

        while let Ok(status) = rx.try_recv() {
            if status.connected {
                let _ = tray_icon_ref.set_tooltip(Some(format!("{} : {}%", MOUSE_NAME, status.level)));
                let _ = tray_icon_ref.set_icon(Some(tray_drawer::draw_vertical_icon(status.level, status.is_charging)));
            } else {
                let _ = tray_icon_ref.set_tooltip(Some(format!("{} : 연결 끊김", MOUSE_NAME)));
                let _ = tray_icon_ref.set_icon(Some(tray_drawer::draw_vertical_icon(0, false)));
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == autostart_item.id() {
                let current_state = autostart::is_autostart_enabled();
                let new_state = !current_state;
                if autostart::set_autostart(new_state) {
                    let title = if new_state {
                        "✓ 시작 시 자동 실행 (켜짐)"
                    } else {
                        "   시작 시 자동 실행 (꺼짐)"
                    };
                    autostart_item.set_text(title);
                    let _ = Notification::new()
                        .summary(&format!("{} 설정", MOUSE_NAME))
                        .body(&format!("시작 시 자동 실행이 {}로 설정되었습니다.", if new_state { "켜짐" } else { "꺼짐" }))
                        .timeout(5000)
                        .show();
                }
            } else if event.id == quit_item.id() {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::NewEvents(StartCause::Init) => {}
            _ => {}
        }
    });
}
