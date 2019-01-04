use {
    digitalocean::{api::Domain, request::Executable, DigitalOcean},
    serde_derive::Deserialize,
    std::error::Error,
};

#[derive(Deserialize, Debug)]
struct Ipify {
    address: String,
}

fn update_record(client: &DigitalOcean, kind: &str, myip: &str) -> Result<(), Box<Error>> {
    let records = Domain::get("goddard.id.au").records().execute(client)?;
    let record = records
        .iter()
        .find(|&r| r.name() == "acmelabs" && r.kind() == kind)
        .ok_or("acmelabs not found")?;
    let (id, existing_ip) = (record.id().to_owned(), record.data().to_owned());
    // println!("RECORD: {} -- {}", id, existing_ip);

    if myip == existing_ip {
        println!("acmelabs.goddard.id.au was up to date: {}", existing_ip);
    } else {
        Domain::get("goddard.id.au")
            .records()
            .update(id)
            .data(myip.to_owned())
            .execute(client)?;
        println!(
            "acmelabs.goddard.id.au was updated from {} to {}",
            existing_ip, myip
        )
    }
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let api_key = std::env::var("DIGITALOCEAN_TOKEN").map_err(|_| "env var DIGITALOCEAN_TOKEN not found")?;
    let client = DigitalOcean::new(api_key)?;

    let mut response = reqwest::get("http://v4.ipv6-test.com/api/myip.php?json")?;
    let ipify: Ipify = response.json()?;
    let myipv4 = ipify.address;
    println!("MY IPV4: {:?}", myipv4);
    update_record(&client, "A", &myipv4)?;

    let mut response = reqwest::get("http://v6.ipv6-test.com/api/myip.php?json")?;
    let ipify: Ipify = response.json()?;
    let myipv6 = ipify.address;
    println!("MY IPV6: {:?}", myipv6);
    update_record(&client, "AAAA", &myipv6)?;

    Ok(())
}
