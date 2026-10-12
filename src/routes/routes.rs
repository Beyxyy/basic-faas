use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use crate::{model, services::{ContainerService}};
use serde_derive::Deserialize;

#[get("/healthcheck")]
async fn healthcheck(container_service: web::Data<ContainerService>) -> impl Responder {
    match container_service.is_docker_running().await {
        true => HttpResponse::Ok().body("OK"),
        false => HttpResponse::ServiceUnavailable().body("Docker not running"),
    }
}

#[get("/")]
async  fn home() -> impl Responder {
    HttpResponse::Ok().body("welcome home")
}


#[derive(Deserialize)]
struct StatusParam {
    status : Option<String>
}

#[get("/containers")]
async fn list_running_containers(container_service: web::Data<ContainerService>, status: web::Query<StatusParam>) -> impl Responder {
    let result = if let Some(status) = &status.status {
        container_service.list_containers_by_status(status.clone()).await
    } else {
        container_service.list_all_containers().await
    };
    match result {
        Ok(containers) => HttpResponse::Ok().json(containers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}