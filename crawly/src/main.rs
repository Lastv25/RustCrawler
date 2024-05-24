use reqwest::Client;
// use scraper::{Html, Selector};
mod check_robots;
use check_robots::is_path_allowed_for_website;
mod inputs;
use inputs::starting_website_input;
use inputs::search_query_input;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (path, base) = starting_website_input().unwrap();
    println!("Path: {}", path);
    println!("Base: {}", base);

    let is_allowed = is_path_allowed_for_website(base,path).await?;
    println!("Is path Allowed: {}", is_allowed);
    
    let search_query = search_query_input()?;

    Ok(())
}