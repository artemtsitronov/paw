use percent_encoding::percent_decode_str;
use std::{env, error, fs::File, io, process};
mod theme;
use indicatif::{ProgressBar, ProgressStyle};
use theme::{Theme, get_theme};
use url::Url;

fn hooman(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let (mut v, mut i) = (bytes as f64, 0);
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{bytes} B")
    } else {
        format!("{v:.1}, {}", UNITS[i])
    }
}

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

    let Theme {
        green,
        red,
        yellow,
        blue,
        mauve,
        dim,
        reset,
    } = get_theme();

    println!(
        "{dim}~{reset} Started downloading {blue}{} {dim}~{reset}",
        &url
    );

    let mut response = match ureq::get(&url).call() {
        Ok(r) => r,
        Err(ureq::Error::StatusCode(code)) => {
            eprintln!("{red}✗ error:{reset} server replied {code}");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{red}✗ error:{reset} {e}");
            process::exit(1);
        }
    };
    let size = response.body().content_length();
    let mime = response.body().mime_type();

    println!(
        "{green}{}{reset} - {blue}{}{reset} {}",
        response.status(),
        size.map(hooman).unwrap_or("UNKNOWN".into()),
        mime.unwrap_or("UNKNOWN")
    );

    let pb = match size {
        Some(t) => ProgressBar::new(t),
        None => ProgressBar::new_spinner(),
    };
    pb.set_style(ProgressStyle::with_template("{msg}  {bar:24.183/240}  {percent:>3}%  {bytes}/{total_bytes}  {bytes_per_sec}  eta {eta}")?.progress_chars("━╸━"));
    pb.set_message(name.clone());

    let mut file = File::create(&name)?;
    let bytes = io::copy(
        &mut pb.wrap_read(response.body_mut().as_reader()),
        &mut file,
    )?;

    pb.finish_and_clear();
    println!(
        "{mauve}YES!{reset} Saved {blue}{url}{reset} as {green}{name}{reset}, ({dim}{}{reset})",
        bytes
    );

    Ok(())
}
