use bollard::{Docker, plugin::ContainerSummary, query_parameters::ListContainersOptions};
use std::{collections::HashMap};


pub struct ContainerCreatorService {
    docker : Docker
}

impl ContainerCreatorService {

    pub fn new(docker : Docker) -> Self {
        Self { docker }
    }
}