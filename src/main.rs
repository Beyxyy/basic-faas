use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use bollard::Docker;
mod routes;
mod model;
mod services;
use services::{ContainerCreatorService, ContainerViewerService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults()
        .expect("Failed to connect to Docker");

    let docker = Arc::new(docker);
    
    // Crée le service une seule fois
    let container_viewer_service = web::Data::new(
        ContainerViewerService::new((*docker).clone())
    );

    let container_creator_service = web::Data::new(
        ContainerCreatorService::new((*docker).clone())
    );
    
    HttpServer::new(move|| {
        App::new()
            .app_data(container_viewer_service.clone())
            .app_data(container_creator_service.clone())
            .service(routes::healthcheck)
            .service(routes::home)
            .service(routes::list_running_containers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
