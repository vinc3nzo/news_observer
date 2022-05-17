use std::error::Error;
use serde::Serialize;
use serde::Deserialize;

const NEWS_API_TOP_HEADLINES: &str = "https://newsapi.org/v2/top-headlines";
const NEWS_API_EVERYTHING: &str = "https://newsapi.org/v2/everything";

#[derive(Clone, Serialize, Deserialize)]
pub enum Country {
    US,
    RU
}

pub fn resolve_country(c: &Country) -> &str {
    match c {
        Country::US => "us",
        Country::RU => "ru",
    }
}

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

pub fn get_articles_in_country(api_key: &str, c: &Country) -> Result<Articles, Box<dyn Error>> {
    let url = format!("{}?country={}&apiKey={}",
                            NEWS_API_TOP_HEADLINES,
                            resolve_country(c),
                            api_key);
    let response = ureq::get(&url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

pub fn get_articles_by_query(api_key: &str, query: &str) -> Result<Articles, Box<dyn Error>> {
    let url = format!("{}?q=\"{}\"&sortBy=popularity&apiKey={}",
                            NEWS_API_EVERYTHING,
                            query,
                            api_key);
    let response = ureq::get(&url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

pub fn get_articles_by_query_in_country(api_key: &str, query: &str, c: &Country) -> Result<Articles, Box<dyn Error>> {
    let url = format!("{}?q=\"{}\"&country={}&sortBy=popularity&apiKey={}",
                      NEWS_API_EVERYTHING,
                      query,
                      resolve_country(c),
                      api_key);
    let response = ureq::get(&url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}