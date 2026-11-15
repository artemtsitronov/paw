use percent_encoding::percent_decode_str;
use std::{env, error, fs::File, io};
use url::Url;

fn get_fname(raw: &str) -> Result<String, Box<dyn error::Error>> {
    let parsed = Url::parse(raw)?;

    let name = parsed
        .path_segments()
        .and_then(|segs| segs.last())
        .filter(|s| !s.is_empty())
        .map(|s| percent_decode_str(s).decode_utf8_lossy().into_owned())
        .unwrap_or_else(|| "paw_download".to_string());

    Ok(name)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let url = env::args().nth(1).expect("OH NO! Usage: paw <url>");
    let name = get_fname(&url)?;

    let mut response = ureq::get(&url).call()?;

    let mut file = File::create(&name)?;
    let bytes = io::copy(&mut response.body_mut().as_reader(), &mut file);

    println!("saved {url} as {name}, bytes: {:?}", bytes);

    Ok(())
}
