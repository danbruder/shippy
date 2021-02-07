// containers
use crate::model::{GetContainers, RunContainer, RUNTIME};
use async_graphql::{Schema as GqlSchema, *};

pub type Schema = GqlSchema<Query, Mutation, EmptySubscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription)
}

#[derive(SimpleObject, Clone, Default)]
pub struct Container {
    id: String,
    name: String,
    image: String,
}

impl Container {
    pub fn new(id: String, name: String, image: String) -> Self {
        Self { id, name, image }
    }
}

// GRAPHQL
pub struct Query;

#[Object]
impl Query {
    async fn containers(&self) -> Vec<Container> {
        RUNTIME.send(GetContainers).await.unwrap()
    }
}

// Mutation
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_container(
        &self,
        image: String,
        expose_src: u32,
        expose_host: u32,
    ) -> Container {
        let container = RUNTIME
            .send(RunContainer {
                image,
                expose_src,
                expose_host,
            })
            .await
            .unwrap();
        container
    }
}
