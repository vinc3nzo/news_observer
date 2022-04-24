use reqwest;

pub struct Article {
    title: String,
    link: reqwest::Url,
    description: String,
}
