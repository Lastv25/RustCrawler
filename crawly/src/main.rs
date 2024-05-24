use reqwest::Client;
use scraper::{Html, Selector};
mod check_robots;
use check_robots::is_path_allowed_for_website;
mod inputs;
use inputs::starting_website_input;
use inputs::search_query_input;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let (path, base) = starting_website_input().unwrap();
    // println!("Path: {}", path);
    // println!("Base: {}", base);

    // To use afterwards
    // let is_allowed = is_path_allowed_for_website(base,path).await?;
    // println!("Is path Allowed: {}", is_allowed);
    
    // let search_query = search_query_input()?;

    let client = Client::new(); 
    let query = "rust%2Bprogramming%2Blanguage";
    let url = format!("https://www.google.com/search?q={}" , query); 
    let res = client.get(&url).send().await?;
    let html = res.text().await?;
    println!("{}", html);
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.g").unwrap();

    for element in fragment.select(&selector) {
        let title_selector = Selector::parse("h3").unwrap();
        let title_element = element.select(&title_selector).next().unwrap();
        let title = title_element.text().collect::<Vec<_>>().join("");
 
        let link_selector = Selector::parse(".yuRUbf > a").unwrap();
        let link_element = element.select(&link_selector).next().unwrap();
        let link = link_element.value().attr("href").unwrap();
 
        let snippet_selector = Selector::parse(".VwiC3b").unwrap();
        let snippet_element = element.select(&snippet_selector).next().unwrap();
        let snippet = snippet_element.text().collect::<Vec<_>>().join("");
 
        println!("Title: {}", title);
        println!("Link: {}", link);
        println!("Snippet: {}", snippet);
    }

    Ok(())
}