use bollard::Docker;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const DOCKER_REPOSITORY: &str = "amazon/dynamodb-local";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TableConfig {
    pub table_name: String,
}

pub fn verify(config: Value) {
    let config = TableConfig::deserialize(config).expect("incorrect config format");
    println!("pole::verify: config: {:?}", config)
}

pub fn deploy(config: Value) {
    println!("PLACEHOLDER FUNCTION:\npole::deploy: config: {}", config)
}

pub fn pull_image(connection: &Docker, repository: Option<&str>, tag: Option<&str>) {
    let tag = tag.unwrap_or("latest");
    let repository = repository.unwrap_or(DOCKER_REPOSITORY);
}

pub fn deploy_container() {}
