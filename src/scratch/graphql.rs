// containers
//use crate::model::{GetContainers, RemoveContainer, RunContainer, RUNTIME};
use async_graphql::{Schema as GqlSchema, *};

custom_derive! {
    #[derive(NewtypeFrom, NewtypeDeref, NewtypeDerefMut)]
    pub struct Container(shiplift::rep::Container);
}

pub type Schema = GqlSchema<Query, EmptyMutation, EmptySubscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}

// GRAPHQL
pub struct Query;

#[Object]
impl Query {
    async fn containers(&self) -> Vec<Container> {
        //RUNTIME.send(GetContainers).await.unwrap()
        vec![]
    }
}

// Mutation
// pub struct Mutation;

// #[Object]
// impl Mutation {
//     async fn create_container(
//         &self,
//         image: String,
//         expose_src: u32,
//         expose_host: u32,
//     ) -> Container {
//         // let container = RUNTIME
//         //     .send(RunContainer {
//         //         image,
//         //         expose_src,
//         //         expose_host,
//         //     })
//         //     .await
//         //     .unwrap();
//         // container
//     }

//     async fn remove_container(&self, id: String) -> bool {
//         RUNTIME.send(RemoveContainer { id }).await.unwrap();
//         true
//     }
// }
