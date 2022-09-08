use bollard::{image::CreateImageOptions, Docker};
use futures;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default;

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

pub async fn pull_image(docker: &Docker, repository: Option<&str>, tag: Option<&str>) {
    let tag = tag.unwrap_or("latest");
    let repository = repository.unwrap_or(DOCKER_REPOSITORY);

    println!("Pulling Image: {}:{}", repository, tag);
    for item in futures::executor::block_on_stream(docker.create_image(
        Some(CreateImageOptions {
            from_image: repository,
            tag: tag,
            ..Default::default()
        }),
        None,
        None,
    )) {
        match item {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Unable to download image {}:{}: {}", repository, tag, err);
                Err(())
            }
        }
        .unwrap();
    }
}

pub fn deploy_container() {}
