use actix_web::{web, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine};

#[derive(Deserialize, Serialize, Clone)]
struct Data {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

#[derive(Deserialize, Serialize)]
struct Order {
    location: String,
    timestamp: String,
    data: Data,
}

#[derive(Deserialize, Serialize)]
struct OrderDetail {
    location: String,
    timestamp: String,
    signature: String,
    material: i32,
    data: Data,
}

fn calculate_signature(data: &Data) -> String {
    let signature = data.a + data.b + data.c + data.d;
    general_purpose::STANDARD.encode(signature.to_string().as_bytes())
}

fn calculate_material(data: &Data) -> i32 {
    data.a * 3 + data.b * 2 + data.c * 4 + data.d * 10
}

fn calculate_order_detail(order: &Order) -> OrderDetail {
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

async fn index(order: web::Json<Order>) -> HttpResponse {
    let order_detail = calculate_order_detail(&order);
    HttpResponse::Ok().json(order_detail)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .route("/api/inventory", web::post().to(index))
    })
    .workers(16)
    .bind("127.0.0.1:8200")?
    .run()
    .await
}