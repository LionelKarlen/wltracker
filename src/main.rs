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
    let res = make_request(&url, &config.auth.key);

    // FIXME: don't panic on errors, just keep old value

    match res {
        Ok(r) => {
            let res = APIResult::from_raw_string(&r.into_string().expect("failed to parse string"));
            let current_season = res
                .data
                .seasonal
                .iter()
                .last()
                .expect("player has not played this season");

            let format_string = format_seasonal(&config.display.template, current_season);

            if let Err(e) = write_to_file(&config.display.file_path, format_string) {
                panic!("Error in writing to file: {e}");
            }
        }
        Err(e) => {
            panic!("Error in api response: {e}");
        }
    }
}

fn make_request(url: &str, token: &str) -> Result<Response, ureq::Error> {
    return ureq::get(url).set("Authorization", token).call();
}
