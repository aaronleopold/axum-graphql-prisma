use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    prisma,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the PrismaClient to the context
pub async fn build_schema() -> AppSchema {
    let db = prisma::new_client().await.unwrap();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
