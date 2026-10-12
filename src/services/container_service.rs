use bollard::{Docker, plugin::ContainerSummary, query_parameters::ListContainersOptions};
use std::{collections::HashMap, string};


pub struct ContainerService {
    docker : Docker
}

impl ContainerService {

    pub fn new(docker : Docker) -> Self {
        Self { docker }
    }

    pub async fn is_docker_running (&self) -> bool {
        self.docker.ping().await.is_ok()  
    }


    pub async fn list_containers_by_status(&self, status : String) -> Result<Vec<ContainerSummary>, bollard::errors::Error>{
        let mut options = ListContainersOptions::default();
        let mut filters = HashMap::new();
        if status != "" {
            filters.insert("status".to_string(), vec![status]);
        }
        options.filters = Some(filters);

        self.docker.list_containers(Some(options)).await
    }

    pub async fn list_all_containers(&self) -> Result<Vec<ContainerSummary>, bollard::errors::Error>{
        let options = ListContainersOptions::default();
        self.docker.list_containers(Some(options)).await
    }
}