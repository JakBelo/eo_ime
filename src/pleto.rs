use tray_icon::{Icon, TrayIcon, TrayIconBuilder, menu::{ContextMenu, Menu, MenuItem}};

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
        .unwrap()
        .into_rgba8();

    let (l, a) = bildo.dimensions();

    Icon::from_rgba(
        bildo.into_raw(),
        l,
        a,
    )
    .unwrap()
}

pub fn krei_menuon() -> Box<dyn ContextMenu> {
    let menuo = Menu::new();

    let eliri = MenuItem::new("退出", true, None);

    menuo.append(&eliri).unwrap();

    Box::new(menuo)
}
