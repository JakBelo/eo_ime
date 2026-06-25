fn main() {
    let versio = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let nomo = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".to_string());

    if let Ok(cela_operaciumo) = std::env::var("CARGO_CFG_TARGET_OS") {
        if cela_operaciumo == "windows" {
            let _ = winresource::WindowsResource::new()
                .set_icon("aseto/eo.ico")
                .set("FileVersion", &versio)
                .set("ProductVersion", &versio)
                .set("ProductName", &nomo)
                .set("FileDescription", "Esperanta enigmetodo.")
                .set("CompanyName", "Jak Belo")
                .set("LegalCopyright", &format!("kopirajto © 2026 {}. Ĉiuj rajtoj rezervitaj.", &nomo))
                .set("InternalName", &nomo)
                .set("OriginalFilename", &format!("{}.exe", &nomo))
                .compile();
        }
    }
}
