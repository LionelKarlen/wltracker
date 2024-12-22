use crate::parse::LiteralBody;

// FIXME: refactor
pub fn format_literals(format_string: &str, literal_data: &LiteralBody) -> String {
    let v = format_string;
    let v = &v.replace("{w}", &literal_data.w.to_string());
    let v = &v.replace("{l}", &literal_data.l.to_string());
    let v = &v.replace("{season}", &literal_data.season);
    let v = &v.replace("{change}", &literal_data.change.to_string());
    let v = &v.replace("{rr}", &literal_data.rr.to_string());
    let v = &v.replace("{rank}", &literal_data.rank);
    return v.to_string();
}
