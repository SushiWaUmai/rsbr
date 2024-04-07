mod config;
mod hexcolor;
mod icons;
mod property;

use crate::config::{read_config, RsbrConfig};
use anyhow::Result;
use getopts::Options;
use property::audio::AudioProperty;
use property::battery::BatteryProperty;
use property::brightness::BrightnessProperty;
use property::datetime::DatetimeProperty;
use property::network::NetworkProperty;
use property::{ShowBar, ShowBars};
use std::env;
use std::path::PathBuf;
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut default_config_path = match dirs::home_dir() {
        Some(x) => x,
        None => {
            eprintln!("Could not find home directory!");
            return Err(anyhow::anyhow!("Could not find home directory!"));
        }
    };

    default_config_path.push("./.config/rsbr/rsbrrc");

    let args: Vec<String> = env::args().collect();
    let progname = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print help and exit");
    opts.optopt(
        "c",
        "config",
        "toml config file",
        default_config_path.to_str().unwrap_or(""),
    );

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
        Some(x) => PathBuf::from(x),
        None => default_config_path,
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

    let properties: Vec<Box<dyn ShowBar>> = vec![
        Box::new(BatteryProperty),
        Box::new(BrightnessProperty),
        Box::new(DatetimeProperty),
        Box::new(AudioProperty),
        Box::new(NetworkProperty),
    ];
    let attributes = ShowBars::new(properties);

    loop {
        let root_name = attributes.process(config.clone()).await;

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
