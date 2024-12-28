use core::panic;
use std::{
    fmt::format,
    fs::{read_to_string, OpenOptions},
};

use serde::{de::Error, Deserialize};

use crate::{config::Config, files::write_to_file};

pub struct LiteralBody {
    pub w: u32,
    pub l: u32,
    pub change: i32,
    pub season: String,
    pub rr: u32,
    pub rank: String,
}

#[derive(Deserialize)]
struct APIResult {
    data: APIBody,
}

#[derive(Deserialize)]
struct APIBody {
    seasonal: Vec<Seasonal>,
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    rr: u32,
    last_change: i32,
    tier: Tier,
}

#[derive(Deserialize)]
struct Tier {
    name: String,
}

#[derive(Deserialize)]
struct Seasonal {
    season: Season,
    wins: u32,
    games: u32,
}

#[derive(Deserialize)]
struct Season {
    short: String,
}

impl APIResult {
    fn from_raw_string(raw_string: &str) -> Result<Self, String> {
        let res = serde_json::from_str(raw_string);
        match res {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                return Err(format!("[API] Failed to parse json string: {e}"));
            }
        }
    }
}

impl LiteralBody {
    pub fn new(raw_string: &str, config: &Config, setup: Option<bool>) -> Result<Self, String> {
        let res = APIResult::from_raw_string(raw_string)?.data;
        let season = res
            .seasonal
            .iter()
            .last()
            .expect("[API] Failed to find latest season. Have you played yet?");
        let session = config.internal.session.unwrap_or(false);
        let mut w: u32;
        let mut l: u32;
        if session {
            w = season.wins;
            l = season.games - season.wins;
            let filename = "./.session.tmp";
            if setup.unwrap_or(false) {
                let sessionwl = format!("{}:{}", w, l);
                if let Err(e) = write_to_file(filename, sessionwl) {
                    panic!("[SETUP] Failed to write setup data: {e}");
                }
            } else {
                let s = read_to_string(filename).expect("[SETUP] Failed to read setup data");
                let s = s.split_once(":").unwrap();
                let s: (u32, u32) = (s.0.parse().unwrap(), s.1.parse().unwrap());
                w -= s.0;
                l -= s.1;
            }
        } else {
            w = season.wins;
            l = season.games - season.wins;
        }
        return Ok(LiteralBody {
            w,
            l,
            change: res.current.last_change,
            season: season.season.short.clone(),
            rr: res.current.rr,
            rank: res.current.tier.name,
        });
    }
}
