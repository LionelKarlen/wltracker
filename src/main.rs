use std::{thread, time::Duration};

use config::Config;
use files::write_to_file;
use format::format_seasonal;
use parse::APIResult;
use ureq::Response;

mod config;
mod files;
mod format;
mod parse;

fn main() {
    let config = Config::from_file("./config.toml");

    let url = format!(
        "https://api.henrikdev.xyz/valorant/v3/mmr/{}/{}/{}/{}",
        config.user.region.unwrap_or("eu".to_string()),
        config.user.platform.unwrap_or("pc".to_string()),
        config.user.username,
        config.user.tag
    );

    loop {
        let res = make_request(&url, &config.auth.key);

        // FIXME: don't panic on errors, just keep old value

        match res {
            Ok(r) => {
                // FIXME: don't panic on errors, just keep old value
                let res =
                    APIResult::from_raw_string(&r.into_string().expect("failed to parse string"));
                // FIXME: don't panic on errors, just keep old value
                let current_season = res
                    .data
                    .seasonal
                    .iter()
                    .last()
                    .expect("player has not played this season");

                let format_string = format_seasonal(&config.display.template, current_season);

                if let Err(e) = write_to_file(&config.display.file_path, format_string) {
                    // FIXME: don't panic on errors, just keep old value
                    panic!("Error in writing to file: {e}");
                }
            }
            Err(e) => {
                // FIXME: don't panic on errors, just keep old value
                // especially include potential fixes according to the status code
                panic!("Error in api response: {e}");
            }
        }
        let interval = &config.internal.interval.unwrap_or(120);
        thread::sleep(Duration::from_secs(*interval));
    }
}

fn make_request(url: &str, token: &str) -> Result<Response, ureq::Error> {
    return ureq::get(url).set("Authorization", token).call();
}
