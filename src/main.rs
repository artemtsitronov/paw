use std::{env, error, fs::File, io}; // env for latah

fn main() -> Result<(), Box<dyn error::Error>> {
    // Cute kitty image hehe
    let url = "https://img.magnific.com/free-photo/little-cat-sitting-grass_1150-17019.jpg?semt=ais_hybrid&w=740&q=80";
    let name = "cute_kitty.jpg";

    let mut response = ureq::get(url).call()?;

    let mut file = File::create(name)?;
    let bytes = io::copy(&mut response.body_mut().as_reader(), &mut file);

    println!("saved {url} as {name}, bytes: {:?}", bytes);

    Ok(())
}
