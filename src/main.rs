#[macro_use]
extern crate serde_derive;

use {
    digitalocean::{api::Domain, request::Executable, DigitalOcean},
    std::error::Error,
};

#[derive(Deserialize, Debug)]
struct Ipify {
    ip: String,
}

fn main() -> Result<(), Box<Error>> {
    let mut response = reqwest::get("https://api.ipify.org?format=json")?;
    let ipify: Ipify = response.json()?;
    let myip = ipify.ip;
    println!("MY IP: {:?}", myip);

    let api_key = std::env::var("DIGITALOCEAN_TOKEN")?;
    let client = DigitalOcean::new(api_key)?;

    let records = Domain::get("goddard.id.au").records().execute(&client)?;
    let record = records
        .iter()
        .find(|&r| r.name() == "acmelabs")
        .ok_or("acmelabs not found")?;
    let (id, existing_ip) = (record.id().to_owned(), record.data().to_owned());
    println!("RECORD: {} -- {}", id, existing_ip);

    if myip == existing_ip {
        println!("acmelabs.goddard.id.au was up to date: {}", existing_ip);
    } else {
        Domain::get("goddard.id.au")
            .records()
            .update(id)
            .data(myip.to_owned())
            .execute(&client)?;
        println!(
            "acmelabs.goddard.id.au was updated from {} to {}",
            existing_ip, myip
        )
    }

    Ok(())
}
