use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseDto<T>{
    pub success: bool,
    pub code: u32,
    pub json: Option<T>,
    pub message: Option<String>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HashValueDto{
    pub hash_value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenDto{
    pub token: String,
}