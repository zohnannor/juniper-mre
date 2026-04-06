use std::{collections::HashMap, sync::Arc};

use axum::{
    Extension, Router,
    routing::{get, post},
};
use juniper::{EmptyMutation, EmptySubscription, RootNode, graphql_object};
use juniper_axum::{graphiql, graphql, playground};
use tokio::net::TcpListener;

#[derive(Clone, Copy, Debug)]
struct Query;

#[graphql_object]
impl Query {
    const fn hello() -> &'static str {
        "world"
    }
}

type Schema = RootNode<Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());

    let query = "{hello(unknown: true)}";
    let res = juniper::execute(query, None, &schema, &HashMap::default(), &())
        .await
        .unwrap();

    dbg!(res);

    let app = Router::new()
        .route("/graphql", post(graphql::<Arc<Schema>>))
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)));

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    eprintln!(
        r"listening on http://{addr}/
try querying this:
```
{query}
```"
    );
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("failed to run `axum::serve`: {e}"));
}
