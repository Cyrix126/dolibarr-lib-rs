use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Document can be an order or an invoice. They share the same attributes.
pub struct Document {
    /// id of document in the database
    pub id: u32,
    #[serde(rename(deserialize = "ref"))]
    /// reference publicly available
    pub reference: String,
    #[serde(rename(deserialize = "total_ht"))]
    /// total price HT
    pub price: f32,
    /// the lines composing the document
    pub lines: Vec<Line>,
}

#[derive(Deserialize, Serialize)]
/// Line of a document.
pub struct Line {
    /// id of line in dolibarr database
    pub id: u32,
    /// qty of the product in this line
    pub qty: u32,
    /// id of product
    pub fk_product: u32,
}
