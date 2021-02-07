use async_trait::async_trait;
use futures::StreamExt;
use shiplift::{Container, ContainerOptions, Docker};
use xtra::prelude::*;
use xtra::spawn::Tokio;

struct Cont {
    docker: Docker,
}

impl Cont {
    fn new(docker: Docker) -> Self {
        Self { docker }
    }
}

impl Actor for Cont {}

struct Run(String);
impl Message for Run {
    type Result = String;
}

#[async_trait]
impl Handler<Run> for Cont {
    async fn handle(&mut self, run: Run, _ctx: &mut Context<Self>) -> String {
        let result = self
            .docker
            .containers()
            .create(&ContainerOptions::builder(run.0.as_ref()).build())
            .await
            .unwrap();

        let container = Container::new(&self.docker, result.id.clone());
        container.start().await.unwrap();

        result.id
    }
}

struct Log(String);
impl Message for Log {
    type Result = ();
}

#[async_trait]
impl Handler<Log> for Cont {
    async fn handle(&mut self, log: Log, _ctx: &mut Context<Self>) {
        let containers = self.docker.containers();
        let container = containers.get(log.0);
        dbg!("got container");
        while let Some(result) = container.stats().next().await {
            match result {
                Ok(stat) => println!("{:?}", stat),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let docker = Docker::new();

    let addr = Cont::new(docker).create(None).spawn(&mut Tokio::Global);
    let id = addr.send(Run("redis".to_string())).await.unwrap();
    addr.send(Log(id)).await.unwrap();
    loop {}
}
