#[path = "../prisma.rs"]
mod prisma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = prisma::new_client().await?;
    unimplemented!("Add your seed logic here")
}
