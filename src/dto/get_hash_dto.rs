use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HashValueDto{
    pub hash_value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHashDto{
    pub success: bool,
    pub code: u32,
    pub json: Option<HashValueDto>,
    pub message: Option<String>
}