static BATTERY_ICONS: [&str; 10] = ["󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];

static CHARGING_ICONS: [&str; 10] = ["󰢜", "󰂆", "󰂇", "󰂈", "󰢝", "󰂉", "󰢞", "󰂊", "󰂋", "󰂅"];

const NO_WIFI_ICON: &str = "󰖪";
const WIFI_ICON: &str = "󰖩";

static VOLUME_ICONS: [&str; 3] = ["󰕿", "󰖀", "󰕾"];

static BRIGHTNESS_ICONS: [&str; 3] = ["󰃞", "󰃟", "󰃠"];

const VOLUME_MUTED_ICON: &str = "󰸈";
const KEYBOARD_ICON: &str = "󰥻";
const CLOCK_ICON: &str = "󰥔";

pub fn get_battery_icon(battery_percent: f32, is_charging: bool) -> &'static str {
    let index = if battery_percent >= 100.0 {
        9
    } else {
        (battery_percent / 10.0) as usize
    };
    if is_charging {
        CHARGING_ICONS[index]
    } else {
        BATTERY_ICONS[index]
    }
}

pub fn get_brightness_icon(brightness_percent: u32) -> &'static str {
    let index = match brightness_percent {
        0..=33 => 0,
        34..=66 => 1,
        _ => 2,
    };
    BRIGHTNESS_ICONS[index]
}
