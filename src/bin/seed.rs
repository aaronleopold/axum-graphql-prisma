#[path = "../prisma.rs"]
mod prisma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = prisma::new_client().await.expect("Failed to create client");

    let user = client
        .user()
        .create(String::from("Bob"), vec![])
        .exec()
        .await?;

    println!("Created user: {:?}", user);

    let post = client
        .post()
        .create(
            String::from("Hello world!"),
            prisma::user::id::equals(user.id.clone()),
            vec![],
        )
        .exec()
        .await?;

    println!("Created post: {:?}", post);

    Ok(())
}
