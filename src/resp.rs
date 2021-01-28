use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sku {
  pub part_number: String,
  pub name: String,
  pub barcode: String,
  pub price: i32,
  pub cost: i32,
  pub quantity: i32,
  pub available_quantity: i32,
}
