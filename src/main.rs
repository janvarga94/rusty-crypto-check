use ::error_chain;
use std::fs::File;
use std::io::prelude::*;

error_chain::error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        println!("bye bye");
        std::process::exit(0);
    })
    .unwrap();

    loop {
        get_print_data().await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
}

async fn get_print_data() -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?start=1&limit=10&convert=USD")
        .header("X-CMC_PRO_API_KEY", "")
        .header("Accept", "application/json")
        .send()
        .await?;

    let text_body = res.text().await?;
    let mut file = File::create("result.json")?;
    file.write_all(text_body.as_bytes())?;

    let v: serde_json::Result<serde_json::Value> = serde_json::from_str(&text_body);

    match v {
        Ok(json) => {
            println!("=====RESULT=====");

            let coins = &json["data"].as_array().unwrap();
            let allowed = ["Bitcoin", "Ethereum", "XRP"];
            for one_coin in coins.iter() {
                let name = &one_coin["name"].as_str().unwrap();
                if allowed.contains(name) {
                    let change = one_coin["quote"]["USD"]["percent_change_24h"]
                        .as_f64()
                        .unwrap();
                    println!(
                        "{} $ {:.2}, 24h {:.1}%",
                        name,
                        one_coin["quote"]["USD"]["price"].as_f64().unwrap(),
                        change
                    );
                    if change.abs() >= 15.0 {
                        sent_notification();
                    }
                }
            }
        }
        Err(_) => println!("not ok"),
    }

    Ok(())
}

fn sent_notification() {
    notify_rust::Notification::new()
        .summary("BIG CHANGE")
        .body("gogogogogogogoggoogoggogogo")
        .icon("firefox")
        .show()
        .unwrap();
}
