// #![windows_subsystem = "windows"] // dont use console

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        println!("bye bye");
        std::process::exit(0);
    })
    .unwrap();

    loop {
        get_print_data().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
}

async fn get_print_data() {
    let client = reqwest::Client::new();
    let res = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?start=1&limit=10&convert=USD")
        .header("X-CMC_PRO_API_KEY", "45ea0bbe-b259-4a95-8e91-07b22c0d8772")
        .header("Accept", "application/json")
        .send()
        .await.unwrap();

    let text_body = res.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&text_body).unwrap();

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

fn sent_notification() {
    notify_rust::Notification::new()
        .summary("BIG CHANGE")
        .body("gogogogogogogoggoogoggogogo")
        .icon("firefox")
        .show()
        .unwrap();
}
