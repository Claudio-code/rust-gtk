use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "word".into()).await?;
    let result = client.get("hello").await?;
    println!("teste {:?}", result);
    Ok(())
}
