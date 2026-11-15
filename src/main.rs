//! Paw is a simple URL fetcher, written by Artem Tsitronov.
//! Made for Hack Club (https://hackclub.com)

use percent_encoding::percent_decode_str;
use std::{
    error, fmt,
    fs::{self, File},
    io, process,
};
mod theme;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use theme::{Theme, get_theme};
use url::Url;

fn print_if(condition: bool, text: impl fmt::Display) {
    if condition {
        println!("{text}");
    }
}

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
        format!("{v:.1} {}", UNITS[i])
    }
}

fn get_fname(raw: &str) -> Result<String, Box<dyn error::Error>> {
    let parsed = Url::parse(raw)?;

    let name = parsed
        .path_segments()
        .and_then(|mut segs| segs.next_back())
        .filter(|s| !s.is_empty())
        .map(|s| percent_decode_str(s).decode_utf8_lossy().into_owned())
        .unwrap_or_else(|| "paw_download".to_string());

    Ok(name)
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    url: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let url = args.url;
    let name = args.output.unwrap_or(get_fname(&url)?);

    let Theme {
        green: _,
        red: _,
        yellow,
        blue,
        mauve,
        dim,
        reset,
        frame: _,
        chip_ok,
        chip_err,
        chip_dim,
    } = get_theme();

    print_if(
        !args.quiet,
        format!(
            "{dim}~{reset} Started downloading {blue}{} {dim}~{reset}",
            url
        ),
    );

    let mut response = match ureq::get(&url).call() {
        Ok(r) => r,
        Err(ureq::Error::StatusCode(code)) => {
            eprintln!("{chip_err} ERROR {reset} - server replied {code}");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{chip_err} ERROR {reset} - {e}");
            process::exit(1);
        }
    };
    let size = response.body().content_length();
    let mime = response.body().mime_type();

    print_if(
        !args.quiet,
        format!(
            "{chip_ok} {} {reset} - {chip_dim} {} {reset} {dim} {} {reset}",
            response.status(),
            size.map(hooman).unwrap_or("UNKNOWN".into()),
            mime.unwrap_or("UNKNOWN")
        ),
    );

    let pb = if args.quiet {
        ProgressBar::hidden()
    } else {
        match size {
            Some(t) => ProgressBar::new(t),
            None => ProgressBar::new_spinner(),
        }
    };
    pb.set_style(ProgressStyle::with_template("{msg}  {bar:24.183/240}  {percent:>3}%  {bytes}/{total_bytes}  {bytes_per_sec}  eta {eta}")?.progress_chars("━╸━"));
    pb.set_message(name.clone());

    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&name)
    {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            eprintln!("{chip_err} ERROR {reset} - {name} already exists");
            process::exit(1);
        }
        Err(e) => return Err(e.into()),
    };
    let bytes = io::copy(
        &mut pb.wrap_read(response.body_mut().as_reader()),
        &mut file,
    )?;

    pb.finish_and_clear();
    print_if(
        !args.quiet,
        format!(
            "\n{mauve}\x1b[1mYES!{reset} Saved {blue}{url}{reset} as {yellow}{name}{reset}, ({dim}{}{reset})",
            bytes
        ),
    );

    Ok(())
}
