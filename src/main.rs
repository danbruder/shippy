use shiplift::{Container, ContainerOptions, Docker};
use std::env;

#[tokio::main]
async fn main() {
    let docker = Docker::new();
    let image = env::args()
        .nth(1)
        .expect("You need to specify an image name");

    let result = docker
        .containers()
        .create(&ContainerOptions::builder(image.as_ref()).build())
        .await
        .unwrap();

    let container = Container::new(&docker, result.id);

    container.start().await.unwrap();
}
