use crate::parse::Seasonal;

pub fn format_seasonal(format_string: &str, season: &Seasonal) -> String {
    let v = format_string;
    let v = &v.replace("{w}", &season.wins.to_string());
    let v = &v.replace("{l}", &(&season.games - &season.wins).to_string());
    let v = &v.replace("{season}", &season.season.short);
    return v.to_string();
}
