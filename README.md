# Rust + Axum + Prisma

A template for creating services in Rust using Axum and Prisma. This uses the super cool [Prisma Rust Client](https://github.com/Brendonovich/prisma-client-rust).

## Getting Started

You'll want to have `cargo-watch` installed for the best DX, however it isn't required.

```bash
cargo install cargo-watch
```

Then you can run the `cargo-watch -x run` command to watch for changes and automatically rebuild the project.

## Prisma

To set up prisma, run:

```bash
cargo prisma generate # outputs client to src/prisma.rs
cargo prisma db push # outputs sqlite db to prisma/dev.db (specified in schema.prisma)
```

For more in-depth information about the prisma client, see the [Prisma Client Rust Docs](https://github.com/Brendonovich/prisma-client-rust/tree/main/docs).

## Run the Server

To run the server, run:

```bash
cargo run # or cargo-watch -x run
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
