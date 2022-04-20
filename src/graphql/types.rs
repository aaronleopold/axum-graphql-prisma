use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::prisma::{post, user, PrismaClient};

// Note: ideally, I would not need this file at all. I would just use the generated
// struct definitions from the prisma client, however with async-graphql I need
// to derive some traits to make it work.

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub display_name: String,
}

#[ComplexObject]
impl User {
    pub async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .post()
            .find_many(vec![post::user_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

impl Into<User> for user::Data {
    fn into(self) -> User {
        User {
            id: self.id,
            display_name: self.display_name,
        }
    }
}

impl Into<User> for &user::Data {
    fn into(self) -> User {
        User {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub user_id: String,
}

#[ComplexObject]
impl Post {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(self.user_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
}

impl Into<Post> for post::Data {
    fn into(self) -> Post {
        Post {
            id: self.id,
            content: self.content,
            user_id: self.user_id,
        }
    }
}
