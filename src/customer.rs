use serde::{Deserialize, Serialize};

/// Contact information of custommer
#[derive(Deserialize, Serialize)]
pub struct CustomerData {
    /// the id present in the database of the thirdparty
    pub id: u32,
    /// Full name of the customer
    pub name: String,
    /// Phone number
    pub phone: String,
    /// Email address
    pub email: String,
    /// Street number and Street name
    pub address: String,
    /// Postal code
    pub zip: String,
    /// Town
    pub town: String,
}
