use tokio;
use select::node::Node;
use select::predicate::Name;
use tokio::task::Id;
use std::fs;
use select::document::Document;
use select::predicate::{Attr, Class, Predicate};

#[tokio::main]

async fn main() {
    let webpage = match fetch_website().await {
        Ok(content) =>  content,
        Err(e) => ("Error found").to_string(),
    };
    let web = &webpage[0..webpage.len()];
    let document = Document::from(web);
    for node in document.find(Class("wb-break-all").descendant(Name("a"))){
        println!("#Only Projects : {}", node.text());

    }
}

async fn fetch_website()->Result<String , reqwest::Error>{
    let url = "https://github.com/akhil888binoy?tab=repositories";
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}