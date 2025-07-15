use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use crate::vault::Vault;
use crate::snapshot_chain::PegSnapshot;
use serde_json::json;


/// Configures routes for Actix app
pub fn config(cfg: &mut web::ServiceConfig, vault: Arc<Vault>) {
    cfg
        .app_data(web::Data::new(vault))
        .route("/zor/latest", web::get().to(get_latest_snapshot))
        .route("/zor/verify", web::get().to(verify_snapshot_chain))
        .route("/zor/sync/{hash}", web::get().to(get_snapshot_by_hash));
}

/// GET /zor/latest
async fn get_latest_snapshot(vault: web::Data<Arc<Vault>>) -> impl Responder {
    match vault.latest() {
        Some(snapshot) => HttpResponse::Ok().json(snapshot),
        None => HttpResponse::NotFound().body("No snapshot available"),
    }
}

/// GET /zor/verify
async fn verify_snapshot_chain(vault: web::Data<Arc<Vault>>) -> impl Responder {
    let is_valid = vault.verify_integrity();
    HttpResponse::Ok().json(serde_json::json!({ "valid": is_valid }))
}

/// GET /zor/sync/{hash}
async fn get_snapshot_by_hash(path: web::Path<String>, vault: web::Data<Arc<Vault>>) -> impl Responder {
    let prefix = path.into_inner();
    let all_json = vault.export_json();
    let snapshots: Vec<PegSnapshot> = serde_json::from_str(&all_json).unwrap_or_default();

    if let Some(matched) = snapshots.into_iter().find(|snap| snap.snapshot_hash.starts_with(&prefix)) {
        HttpResponse::Ok().json(matched)
    } else {
        HttpResponse::NotFound().body("Snapshot not found")
    }
}
