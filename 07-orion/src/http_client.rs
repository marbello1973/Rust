use reqwest::blocking::Client;
use anyhow::{Result, anyhow};

pub fn send_request(url: &str, method: &str, body: Option<&str>) -> Result<String> {
    let client = Client::new();

    let response = match method {
        "GET" => client.get(url).send()?,
        "POST" => client.post(url).body(body.unwrap_or("").to_string()).send()?,
        "PUT" => client.put(url).body(body.unwrap_or("").to_string()).send()?,
        "DELETE" => client.delete(url).send()?,
        _ => return Err(anyhow!("Método no soportado".to_string())),
    };
    
    Ok(response.text()?)
}
