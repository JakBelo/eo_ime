fn main() {
    if let Ok(cela_operaciumo) = std::env::var("CARGO_CFG_TARGET_OS") {
        if cela_operaciumo == "windows" {
            let _ = winresource::WindowsResource::new()
                .set_icon("aseto/eo.ico")
                .compile();
        }
    }
}
