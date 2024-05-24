
use std::io;
use url::Url;

pub fn starting_website_input() -> Result<(String, String), Box<dyn std::error::Error>>{
    // Enter Text to search for
    println!("Enter starting website to search from:"); 
    let mut website_name = String::new();
    io::stdin().read_line(&mut website_name).expect("Failed to read website");
    let website_name = website_name.trim();

    // Parse input into a URL
    let url = Url::parse(website_name).unwrap();

    let path = url.path().to_owned();
    let base = format!("{}://{}", url.scheme(), url.host().unwrap());

    Ok((path, base))
}

pub fn search_query_input() -> Result<String, Box<dyn std::error::Error>>{
    // Enter Text to search for
    println!("Enter query to search from:"); 
    let mut search_query = String::new();
    io::stdin().read_line(&mut search_query).expect("Failed to read search_query");
    let search_query = search_query.trim().to_string();
    Ok(search_query)
}
