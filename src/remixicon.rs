use std::collections::HashMap;
use std::sync::RwLock;
use iced::Font;
use iced::widget::{ text, Text};
// use iced::Theme;
use regex::Regex;

// use crate::Message;



lazy_static::lazy_static! {
    static ref ICON_MAP: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);
}

pub fn remix_init() {
    let input = include_bytes!("../assets/fonts/remixicon.css");
    let input_str = std::str::from_utf8(input).unwrap();

    let icon_map = parse_icon_map(input_str);
    let mut icon_map_lock = ICON_MAP.write().unwrap(); // write lock
    *icon_map_lock = Some(icon_map);
}

fn parse_icon_map(input: &str) -> HashMap<String, String> {
    let re = Regex::new(r#"\.(ri-.*-.*):before \{ content: "\\(\w+)"; \}"#).unwrap();
    let mut icon_map = HashMap::new();

    for cap in re.captures_iter(input) {
        let name = format!("{}", &cap[1]);
        let value = &cap[2];
        icon_map.insert(name, value.to_string());
    }

    icon_map
}

pub fn get_icon_map() -> HashMap<String, String> {
    let icon_map_lock = ICON_MAP.read().unwrap(); // read lock
    icon_map_lock.as_ref().unwrap().clone()
}

pub fn ri_icon(icon: &str) -> String {
    let icon_map = get_icon_map();
    let hex = icon_map.get(icon).map(|v| v.as_str());

    // Use unwrap_or to return a space character if hex is None
    hex_to_unicode_char(hex).unwrap_or(' ').to_string()
}


fn hex_to_unicode_char(hex_str: Option<&str>) -> Option<char> {
    hex_str.and_then(|hex| {
        // Convert hex string to u32
        u32::from_str_radix(hex, 16)
            .ok()
            .and_then(char::from_u32)
    })
}

// pub fn ri_icon_text(icon: &str) -> Text<crate::Message, Theme> {
//     Text::new(ri_icon(icon)) // Assuming ri_icon returns a String or &str
// }

pub fn remix_icon<>(label: &str) -> Text<'_> {
    text(ri_icon(label)).font(Font::with_name("remixicon"))
}