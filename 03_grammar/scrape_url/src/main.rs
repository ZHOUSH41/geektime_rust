use std::{fs, env::args};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    // let url = "https://www.rust-lang.org";
    let url = &args[1];
    let output = &args[2];
    // let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(&output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
