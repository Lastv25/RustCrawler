
use reqwest::Client;

pub async fn is_path_allowed_for_website(base: String, path: String) -> Result<bool, Box<dyn std::error::Error>>{
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
                if path.contains(&path_robot) || path_robot == path {
                    return Ok(true);
                }
            } else if line.starts_with("Disallow:") {
                let path_robot = line.trim_start_matches("Disallow:").trim().to_string();
                if path.contains(&path_robot) || path_robot == path  {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}