#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Ipify {
    ip: String,
}

#[derive(Deserialize, Debug)]
struct DnsRecord {
    id: i32,
    // type: String,
    name: String,
    data: String,
}

#[derive(Deserialize, Debug)]
struct ListDnsRecords {
    domain_records: Vec<DnsRecord>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let mut response = client.get("https://api.ipify.org?format=json").send()?;
    let ipify: Ipify = response.json()?;
    println!("{:?}", ipify);

    let mut response = client
        .get(&format!(
            "https://api.digitalocean.com/v2/domains/{}/records",
            "goddard.id.au"
        ))
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("DIGITALOCEAN_TOKEN")?),
        )
        .send()?;
    let records: ListDnsRecords = response.json()?;
    let record = records
        .domain_records
        .iter()
        .find(|&r| r.name == "acmelabs")
        .ok_or("acmelabs not found")?;
    println!("{:?}", record);

    if record.data == ipify.ip {
        println!("acmelabs.goddard.id.au was up to date: {}", record.data)
    } else {
        let mut map = std::collections::HashMap::new();
        map.insert("data", ipify.ip.to_owned());

        let mut response = client
            .put(&format!(
                "https://api.digitalocean.com/v2/domains/{}/records/{}",
                "goddard.id.au", record.id
            ))
            .json(&map)
            .header(
                "Authorization",
                format!("Bearer {}", std::env::var("DIGITALOCEAN_TOKEN")?),
            )
            .send()?;
        println!(
            "acmelabs.goddard.id.au was updated from {} to {}",
            record.data, ipify.ip
        )
    }

    Ok(())
}
