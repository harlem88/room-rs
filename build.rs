#[path = "src/config.rs"]
mod config;

fn main() {
    // Check if the `cfg.toml` file exists and has been filled out.
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` as a template.");
    }

    let app_config = config::CONFIG;

    if app_config.wifi_ssid == "" {
        panic!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    }

    embuild::espidf::sysenv::output();
}
