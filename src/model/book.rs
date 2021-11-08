use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub ids: Vec<i64>,
    pub title: String,
    pub titles: Vec<String>,
    pub price: f64,
    pub prices: Vec<f64>,
    pub hot: bool,
    pub hots: Vec<bool>,
    pub author: Author,
    pub authors: Vec<Author>,
    pub weights: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub age: i32,
    pub male: bool,
}
