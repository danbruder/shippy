//use model::{Initialize, RUNTIME};

//RUNTIME.send(Initialize).await.unwrap();

// let filter = async_graphql_warp::graphql(schema()).and_then(
//     |(schema, request): (Schema, async_graphql::Request)| async move {
//         // Execute query
//         let resp = schema.execute(request).await;

//         // Return result
//         Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
//     },
// );
// warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
