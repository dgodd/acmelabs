#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Ipify {
    ip: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut response = reqwest::get("https://api.ipify.org?format=json")?;
    let ipify: Ipify = response.json()?;
    println!("{:?}", ipify);
    Ok(())
}
