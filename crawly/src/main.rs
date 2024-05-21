use reqwest::Client;
use scraper::{Html, Selector};
use std::io;
use url::Url;

fn starting_website_input() -> Result<(String, String), Box<dyn std::error::Error>>{
    // Enter Text to search for
    println!("Enter starting website to search from:"); 
    let mut website_name = String::new();
    io::stdin().read_line(&mut website_name).expect("Failed to read website");
    let website_name = website_name.trim();

    // Parse input into a URL
    let url = Url::parse(website_name).unwrap();

    let path = url.path().to_owned();
    let base = format!("{}://{}", url.scheme(), url.host().unwrap());

    return Ok((path, base));
}

fn search_query_input() -> Result<String, Box<dyn std::error::Error>>{
    // Enter Text to search for
    println!("Enter query to search from:"); 
    let mut search_query = String::new();
    io::stdin().read_line(&mut search_query).expect("Failed to read search_query");
    let search_query = search_query.trim().to_string();
    return Ok(search_query);
}

async fn is_path_allowed_for_website(base: String, path: String) -> Result<bool, Box<dyn std::error::Error>>{
    // Search robots file
    let client = Client::new();
    let robots_url = format!("{}/{}", base,"robots.txt");

    let response = client.get(&robots_url).send().await?;
    let html_body = response.text().await?;
    
    // search if path to look for is allowed or not
    let mut current_agent = "".to_string();

    for line in html_body.lines() {
        if line.starts_with("User-agent:") {
            current_agent = line.trim_start_matches("User-agent:").trim().to_string();
        } else if !current_agent.is_empty() && current_agent == "*"{
            if line.starts_with("Allow:") {
                let path_robot = line.trim_start_matches("Allow:").trim().to_string();
                if path_robot == path {
                    return Ok(true);
                }
            } else if line.starts_with("Disallow:") {
                let path_robot = line.trim_start_matches("Disallow:").trim().to_string();
                if path_robot == path {
                    return Ok(false);
                }
            }
        }
    }
    return Ok(true);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (path, base) = starting_website_input().unwrap();
    println!("Path: {}", path);
    println!("Base: {}", base);

    let is_allowed = is_path_allowed_for_website(base,path).await?;
    println!("Is path Allowed: {}", is_allowed);
    // let search_query = search_query_input()?;

    Ok(())
}