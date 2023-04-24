use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    prisma::PrismaClient,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the PrismaClient to the context
pub async fn build_schema() -> AppSchema {
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to create Prisma client");

    // NOTE: I just quickly added these in, but see https://prisma.brendonovich.dev/extra/migrations
    // for more information on how to handle migrations.

    #[cfg(debug_assertions)]
    client
        ._db_push()
        .accept_data_loss()
        // .force_reset()
        .await
        .expect("Failed to push database schema");

    #[cfg(not(debug_assertions))]
    client
        ._migrate_deploy()
        .await
        .expect("Failed to deploy database schema");

    // For more information about schema data, see: https://async-graphql.github.io/async-graphql/en/context.html#schema-data
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(client)
        .finish()
}
