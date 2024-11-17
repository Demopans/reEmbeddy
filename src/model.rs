/*
All compile time definitions here
*/
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub host: [u8;4],
    pub port: u16
}
pub struct SiteField {
    api: &'static str
}
impl SiteField {
    pub fn api(&self) -> &str {
        self.api
    }
}

pub const DISCORD: &str = "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)";
pub const CONFIG: &str = "config.json";
pub type Table = std::collections::HashMap<String, String>; // encodes url params correctly


