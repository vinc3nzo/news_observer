use std::error::Error;
use serde::Deserialize;

const NEWS_API_TOP_HEADLINES: &str = "https://newsapi.org/v2/top-headlines";
const US: &str = "us";

#[derive(Deserialize, Debug)]
pub struct Articles { // verbose, but necessary for serde_json to work
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub description: Option<String>
}

pub fn get_articles(api_key: &str) -> Result<Articles, Box<dyn Error>> {
    let url = format!("{}?country={}&apiKey={}", NEWS_API_TOP_HEADLINES, US, api_key);
    let response = ureq::get(&url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}