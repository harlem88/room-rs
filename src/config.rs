#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("localhost")]
    mqtt_host: &'static str,
    #[default("room-1")]
    room_name: &'static str,
    #[default("home")]
    topic_root: &'static str,
}
