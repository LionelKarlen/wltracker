use std::fs::write;

pub fn write_to_file(filepath: &str, content: String) -> Result<(), std::io::Error> {
    return write(filepath, content);
}
