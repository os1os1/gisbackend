use std::env;
use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json::Value;
use geojson::{GeoJson, Geometry};

#[derive(Deserialize, Serialize, Debug)]
struct MapMemo {
    id: String,
    title: String,
    location: Value, // GeoJSON形式を保持
}

#[get("/api/places")]
async fn get_places() -> impl Responder {

    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY not set");


    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("apikey", api_key)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .query(&[("select", "id,title,location")])
        .send()
        .await;

    match res {
        Ok(resp) => {
            let data: Vec<MapMemo> = match resp.json().await {
                Ok(val) => val,
                Err(e) => {
                    return HttpResponse::InternalServerError().body(format!("Parse error: {}", e));
                }
            };

            // GeoJSON の内容をログ出力する例
            for memo in &data {
                match GeoJson::from_json_value(memo.location.clone()) {
                    Ok(geo) => println!("GeoJSON: {:?}", geo),
                    Err(e) => println!("Failed to parse GeoJSON: {}", e),
                }
            }

            HttpResponse::Ok().json(data)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Request error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");
    HttpServer::new(|| App::new().service(get_places))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}