use async_graphql::SimpleObject;

use crate::prisma::{post, user};

// Note: ideally, I would not need this file at all. I would just use the generated
// struct definitions from the prisma client, however with async-graphql I need
// to derive some traits to make it work.

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub display_name: String,
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
pub struct Post {
    pub id: String,
    pub content: String,
    pub user: Option<Box<User>>,
    pub user_id: String,
}

impl Into<Post> for post::Data {
    fn into(self) -> Post {
        let u: Option<Box<User>> = match self.user() {
            Ok(u) => Some(Box::new(u.into())),
            _ => None,
        };

        Post {
            id: self.id,
            content: self.content,
            user_id: self.user_id,
            user: u,
        }
    }
}
