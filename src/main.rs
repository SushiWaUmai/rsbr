mod config;
mod hexcolor;
mod icons;

use crate::config::{read_config, RsbrConfig};
use crate::icons::{get_battery_icon, get_brightness_icon};
use anyhow::Result;
use battery::Battery;
use brightness::Brightness;
use chrono::{DateTime, Local};
use futures_util::stream::StreamExt;
use getopts::Options;
use std::env;
use tokio::time::{sleep, Duration};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt;

fn usage(progname: &str, opts: getopts::Options) {
    let brief = format!("Usage: {progname} [options]");
    let usage = opts.usage(&brief);
    eprint!("{usage}");

}

fn get_datetime() -> DateTime<Local> {
    Local::now()
}

async fn get_brightness() -> Result<u32, anyhow::Error> {
    match brightness::brightness_devices().next().await {
        Some(x) => match x {
            Ok(x) => Ok(x.get().await? + 1),
            Err(x) => Err(anyhow::anyhow!(x)),
        },
        None => Err(anyhow::anyhow!("No brightness devices found!")),
    }
}

fn get_battery(batteries: &Vec<Battery>) -> Result<(f32, bool), anyhow::Error> {
    if batteries.len() == 0 {
        return Err(anyhow::anyhow!("No Battery found!"));
    }

    let sum: f32 = batteries
        .iter()
        .map(|x| x.state_of_charge().get::<battery::units::ratio::percent>())
        .sum();

    let battery_charging = batteries
        .iter()
        .map(|x| x.state())
        .any(|x| x == battery::State::Charging);

    Ok((sum / batteries.len() as f32, battery_charging))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let default_config_path = "~/.config/rsbr/rsbrrc";

    let args: Vec<String> = env::args().collect();
    let progname = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print help and exit");
    opts.optopt("c", "config", "toml config file", default_config_path);

    let matches = match opts.parse(&args[1..]) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{x}");
            usage(&progname, opts);
            return Err(anyhow::anyhow!(x));
        }
    };

    if matches.opt_present("h") {
        usage(&progname, opts);
        return Ok(());
    }

    let config_path = match matches.opt_str("c") {
        Some(x) => x,
        None => String::from(default_config_path),
    };

    let config = match read_config(&config_path) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Error while loading config file: {x}");
            RsbrConfig::default()
        }
    };

    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let root_window = screen.root;

    let manager = battery::Manager::new()?;

    loop {
        let batteries = manager.batteries()?;
        let batteries: Vec<Battery> = batteries.filter_map(Result::ok).collect();

        let datetime = get_datetime().format(&config.datetime.format).to_string();

        let battery = match get_battery(&batteries) {
            Ok((battery_charge, battery_status)) => format!("{} {}", get_battery_icon(battery_charge, battery_status), battery_charge),
            Err(x) => {
                eprintln!("{x}");
                "No Battery".to_string()
            }
        };

        let brightness = match get_brightness().await {
            Ok(x) => format!("{} {}", get_brightness_icon(x), x),
            Err(x) => {
                eprintln!("{x}");
                "No Brightness Device".to_string()
            }
        };

        let root_name = &config
            .format
            .replace(
                "{battery}",
                &format!(
                    "^c{}^^b{}^ {}% ",
                    &config.theme.get_color(&config.battery.fgcolor),
                    &config.theme.get_color(&config.battery.bgcolor),
                    battery.as_str()
                ),
            )
            .replace(
                "{brightness}",
                &format!(
                    "^c{}^^b{}^ {}% ",
                    &config.theme.get_color(&config.brightness.fgcolor),
                    &config.theme.get_color(&config.brightness.bgcolor),
                    brightness.as_str()
                ),
            )
            .replace(
                "{datetime}",
                &format!(
                    "^c{}^^b{}^ {} ",
                    &config.theme.get_color(&config.datetime.fgcolor),
                    &config.theme.get_color(&config.datetime.bgcolor),
                    datetime,
                ),
            );

        conn.change_property8(
            PropMode::REPLACE,
            root_window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            root_name.as_bytes(),
        )?
        .check()?;

        sleep(Duration::from_secs(1)).await;
    }
}
