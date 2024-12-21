use dotenv::dotenv;
use parse::APIResult;
use ureq::Response;

mod parse;

fn main() {
    dotenv().ok();
    let token = std::env::var("API_KEY").expect("API_KEY must be set.");
    let username = std::env::var("USERNAME").expect("USERNAME must be set.");
    let tag = std::env::var("TAG").expect("TAG must be set.");
    let region = std::env::var("REGION").expect("REGION must be set.");
    let url = format!(
        "https://api.henrikdev.xyz/valorant/v3/mmr/{}/pc/{}/{}",
        region, username, tag
    );
    let res = make_request(&url, &token);

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
            println!(
                "{}/{}",
                current_season.wins,
                current_season.games - current_season.wins
            );
        }
        Err(e) => {
            panic!("we goofed {e}");
        }
    }
}

fn make_request(url: &str, token: &str) -> Result<Response, ureq::Error> {
    return ureq::get(url).set("Authorization", token).call();
}
