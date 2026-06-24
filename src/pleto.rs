use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder, menu::{ContextMenu, Menu, MenuEvent, MenuItem},
};
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;

pub fn krei_pleton() -> TrayIcon {
    let ikono = akiri_ikonon();

    let menuo = krei_menuon();

    TrayIconBuilder::new()
        .with_tooltip("EO IME")
        .with_icon(ikono)
        .with_menu(menuo)
        .with_menu_on_left_click(false)
        .build()
        .expect("Kreado de la pleto malsukcesis.")
}

fn akiri_ikonon() -> Icon {
    let png_datumoj = include_bytes!("../aseto/eo.png");

    let bildo = image::load_from_memory(png_datumoj)
        .expect("Ŝargi PNG‑on malsukcesis.")
        .into_rgba8();

    let (l, a) = bildo.dimensions();

    Icon::from_rgba(
        bildo.into_raw(),
        l,
        a,
    )
    .expect("Kreado de la pleto‑ikono malsukcesis: la RGBA‑datumoj eble havas malĝustan grandecon aŭ nevalidan pikselo‑formaton.")
}

fn krei_menuon() -> Box<dyn ContextMenu> {
    let menuo = Menu::new();

    let eliri = MenuItem::new("退出", true, None);

    menuo.append(&eliri).expect("Aldono de la menuero malsukcesis: la menu-tenilo eble estas nevalida aŭ la objekto jam estas liberigita.");

    Box::new(menuo)
}

pub fn pritrakti_tray_eventojn() {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        if event.id.0 == "1001" {
            unsafe {
                PostQuitMessage(0);
            }
        }
    }
}
