use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::Response;

// Структура для ответа от API
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

// Структура для создания нового поста
#[derive(Debug, Serialize)]
struct NewPost {
    title: String,
    body: String,
    userId: u32,
}

pub async fn parseUrl() -> Result<String, Box<dyn Error>> {
    let posts = reqwest::Client::new().get("https://next.dnd.su/spells/10419-acid-splash/").send().await?;
    let content = posts.text().await?;
    println!("{}",content);
    return Ok(content)
}