use async_trait::async_trait;
use futures::StreamExt;
use lazy_static::lazy_static;
use shiplift::{
    Container as SLContainer, ContainerListOptions, ContainerOptions, Docker, PullOptions,
};
use std::collections::HashMap;
use xtra::prelude::*;
use xtra::spawn::Tokio;

use crate::graphql::Container;

lazy_static! {
    pub static ref RUNTIME: Address<Model> = {
        let docker = Docker::new();
        Model::new(docker).create(None).spawn(&mut Tokio::Global)
    };
}

pub struct Model {
    docker: Docker,
    containers: HashMap<String, Container>,
}

impl Model {
    fn new(docker: Docker) -> Self {
        Self {
            docker,
            containers: Default::default(),
        }
    }
}

impl Actor for Model {}

// Run containers
pub struct RunContainer {
    pub image: String,
    pub expose_src: u32,
    pub expose_host: u32,
}
impl Message for RunContainer {
    type Result = Container;
}
#[async_trait]
impl Handler<RunContainer> for Model {
    async fn handle(&mut self, input: RunContainer, _ctx: &mut Context<Self>) -> Container {
        let image = input.image;
        let mut stream = self
            .docker
            .images()
            .pull(&PullOptions::builder().image(&image).build());

        while let Some(_) = stream.next().await {}

        println!("Done pulling");

        let options = ContainerOptions::builder(&image)
            .expose(input.expose_src, "tcp", input.expose_host)
            .build();
        let result = self.docker.containers().create(&options).await.unwrap();

        let id = result.id;
        let c = SLContainer::new(&self.docker, id.clone());
        c.start().await.unwrap();
        let info = c.inspect().await.unwrap();

        let c = Container::new(id.clone(), info.name, info.image);
        self.containers.insert(id.clone(), c.clone());

        c
    }
}

// Get containers
pub struct GetContainers;
impl Message for GetContainers {
    type Result = Vec<Container>;
}

#[async_trait]
impl Handler<GetContainers> for Model {
    async fn handle(&mut self, _input: GetContainers, _ctx: &mut Context<Self>) -> Vec<Container> {
        self.containers.values().cloned().collect()
    }
}

// INIT
pub struct Initialize;
impl Message for Initialize {
    type Result = ();
}

#[async_trait]
impl Handler<Initialize> for Model {
    async fn handle(&mut self, _input: Initialize, _ctx: &mut Context<Self>) {
        let result = self
            .docker
            .containers()
            .list(&ContainerListOptions::default())
            .await
            .unwrap();

        for item in result {
            let name = item.names.into_iter().take(1).last().unwrap_or("".into());
            let c = Container::new(item.id.clone(), name, item.image);
            self.containers.insert(item.id.clone(), c.clone());
        }
    }
}

// Remove container
pub struct RemoveContainer {
    pub id: String,
}
impl Message for RemoveContainer {
    type Result = ();
}

#[async_trait]
impl Handler<RemoveContainer> for Model {
    async fn handle(&mut self, input: RemoveContainer, _ctx: &mut Context<Self>) {
        let id = input.id;
        let c = SLContainer::new(&self.docker, id.clone());
        c.stop(None).await.unwrap();
        c.wait().await.unwrap();
        c.delete().await.unwrap();
    }
}
