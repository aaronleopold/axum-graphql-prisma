# Rust + Axum + Prisma

A template for creating services in Rust using Axum and Prisma. This uses the super cool [Prisma Rust Client](https://github.com/Brendonovich/prisma-client-rust), which is not stable. I just wanted to try it out, hopefully when it is more stable I will actually put this template to good use.

## Getting Started

You'll want to have `cargo-watch` installed for the best DX, however it isn't required.

```bash
cargo install cargo-watch
```

Then you can run the `cargo-watch -x run` command to watch for changes and automatically rebuild the project.

## Prisma

To set up prisma, run:

```bash
cargo prisma generate
cargo prisma db push
```

## GraphQL Playground

Go to [localhost:8080/api/graphql](http://localhost:8080/api/graphql) to see the playground. You can see the schema and the docs, but a few examples would be:

```graphql
# Create user
mutation {
  createUser(input: { displayName: "oromei" }) {
    id
  }
}

# Create post
mutation {
  createPost(
    input: {
      content: "Woah there!"
      userId: "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f"
    }
  ) {
    id
    content
    user {
      displayName
    }
  }
}

# Get all users
query {
  getUsers {
    id
    displayName
  }
}

# Get all posts
query {
  getPosts {
    id
    content
    user {
      displayName
    }
  }
}
```
