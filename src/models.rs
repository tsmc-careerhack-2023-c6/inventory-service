use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Data {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderPayload {
    pub location: String,
    pub timestamp: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    pub location: String,
    pub timestamp: String,
    pub signature: String,
    pub material: i32,
    pub data: Data,
}