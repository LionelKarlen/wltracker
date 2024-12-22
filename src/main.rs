use std::{env::args, thread, time::Duration};

use config::Config;
use files::write_to_file;
use format::format_literals;
use parse::LiteralBody;
use ureq::Response;

mod config;
mod files;
mod format;
mod parse;

fn main() {
    let mut config_path = "./config.toml";
    let args: Vec<String> = args().collect();
    if let Some(s) = args.get(1) {
        config_path = s;
    }
    let config = Config::from_file(config_path);

    let url = format!(
        "https://api.henrikdev.xyz/valorant/v3/mmr/{}/{}/{}/{}",
        config.user.region.clone().unwrap_or("eu".to_string()),
        config.user.platform.clone().unwrap_or("pc".to_string()),
        config.user.username,
        config.user.tag
    );

    loop {
        if let Err(e) = tick(&config, &url) {
            eprintln!("[TICK] Failed to update value: {e}");
        }
        let interval = &config.internal.interval.unwrap_or(120);
        thread::sleep(Duration::from_secs(*interval));
    }
}

fn tick(config: &Config, url: &str) -> Result<(), String> {
    let res = make_request(&url, &config.auth.key);

    match res {
        Ok(r) => {
            let raw_string = r.into_string();
            match raw_string {
                Err(e) => return Err(format!("[API] Failed to parse api response to string: {e}")),
                _ => (),
            }
            let data = LiteralBody::new(&raw_string.unwrap())?;

            let format_string = format_literals(&config.display.template, &data);

            if let Err(e) = write_to_file(&config.display.file_path, format_string) {
                return Err(format!("[FILE] Error in writing to file: {e}",));
            }
        }
        Err(e) => {
            // TODO: include potential fixes according to the status code
            return Err(format!("[API] Error in api response: {e}"));
        }
    }
    return Ok(());
}

fn make_request(url: &str, token: &str) -> Result<Response, ureq::Error> {
    return ureq::get(url).set("Authorization", token).call();
}
