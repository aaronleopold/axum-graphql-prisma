use async_graphql::{Context, Object, Result};

use crate::{graphql::types::Post, prisma::PrismaClient};

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        // FIXME: not working, figure that out. I'm sure I am doing something wrong
        // either here or in graphql/types.rs
        Ok(db
            .post()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}
