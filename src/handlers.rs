use actix_web::{web, HttpResponse, post};
use base64::{engine::general_purpose, Engine};

use crate::models::*;

fn calculate_signature(data: &Data) -> String {
    let signature = data.a + data.b + data.c + data.d;
    general_purpose::STANDARD.encode(signature.to_string().as_bytes())
}

fn calculate_material(data: &Data) -> i32 {
    data.a * 3 + data.b * 2 + data.c * 4 + data.d * 10
}

fn calculate_order_detail(order: &OrderPayload) -> OrderDetail {
    let signature = calculate_signature(&order.data);
    let material = calculate_material(&order.data);
    OrderDetail {
        location: order.location.clone(),
        timestamp: order.timestamp.clone(),
        signature,
        material,
        data: order.data.clone(),
    }
}

#[post("/inventory")]
async fn index(order: web::Json<OrderPayload>) -> HttpResponse {
    let order_detail = calculate_order_detail(&order);
    HttpResponse::Ok().json(order_detail)
}