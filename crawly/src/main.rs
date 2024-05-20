use reqwest::Client;
use scraper::{Html, Selector};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Enter Text to search for
    println!("Enter text to search for:"); 
    let mut search_query = String::new();
    io::stdin().read_line(&mut search_query).expect("Failed to read search_query");
    let search_query = search_query.trim().to_string();

    // Run query on google
    let client = Client::new();
    let search_url = format!("https://www.google.com/search?q={}", search_query.replace(" ", "+"));

    let response = client.get(&search_url).send().await?;
    let html_body = response.text().await?;
    println!("{}", html_body);

   
    Ok(())
}