fn main() {
    // Only run on Windows targets
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
