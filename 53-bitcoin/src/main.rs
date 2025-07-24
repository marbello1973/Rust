use serde::{Deserialize, Serialize};


fn main() {
    let mut coin = String::new();
    println!("Ingrese el nombre de la moneda: ");
    let _ = std::io::stdin()
        .read_line( &mut coin)
        .expect("Error al leer la moneda");

    let resl: Result<String, ureq::Error> = get_precio(&coin);
    
    match resl {
        Ok(precio) => println!("El precio del {} es: {}", coin, precio),
        Err(e) => println!("error: {}", e),        
    }    
}

fn get_precio(coin: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false", 
        coin
    ))
        .call()?
        .into_string()?;

    let coin_data: CoinData = serde_json::from_str(&body).unwrap();

    println!("{:?}", coin_data);   

    Ok(coin_data.market_data.current_price.usd.to_string())
    
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String, 
    market_data: MarketData

}
#[derive(Serialize, Deserialize, Debug)]
struct MarketData{
    current_price: Prices,
    total_volume: Prices,
    market_cap: Prices,
    high_24h: Prices,
    low_24h: Prices,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    /*market_cap: f64,
    total_volume: f64,
    high_24h: f64,
    low_24h: f64,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    market_cap_change_24h: f64,
    market_cap_change_percentage_24h: f64,
    circulating_supply: f64,
    total_supply: f64,
    ath: f64,
    ath_change_percentage: f64,
    ath_date: String,
    roi: Option<String>,
    last_updated: String,*/
}
#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f64,
    eur: f64,
    btc: f64,
}
