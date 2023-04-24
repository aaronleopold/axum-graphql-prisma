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

impl From<user::Data> for User {
    fn from(data: user::Data) -> User {
        User {
            id: data.id,
            display_name: data.display_name,
        }
    }
}

impl From<&user::Data> for User {
    fn from(data: &user::Data) -> User {
        User {
            id: data.id.clone(),
            display_name: data.display_name.clone(),
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

impl From<post::Data> for Post {
    fn from(data: post::Data) -> Post {
        Post {
            id: data.id,
            content: data.content,
            user_id: data.user_id,
        }
    }
}
