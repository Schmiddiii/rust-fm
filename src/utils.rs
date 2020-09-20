use std::process::Command;
use std::str;

pub fn get_mime_type(path: &str) -> Result<mime::Mime, String> {
    let out = Command::new("file")
        .args(&["--mime-type", "-b", path])
        .output();
    if out.is_err() {
        let error = out.err().unwrap().to_string();
        return Err(String::from(error));
    }
    let out = out.unwrap();
    let output_arr = out.stdout.into_boxed_slice();
    let mime_str = str::from_utf8(&output_arr).unwrap_or("text/plain");

    return Ok(mime_str[0..mime_str.len() - 2]
        .parse()
        .unwrap_or(mime::TEXT_PLAIN));
}
