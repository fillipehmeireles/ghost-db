mod shellcore;
#[tokio::main]
async fn main() {
    shellcore::welcome();
    shellcore::run().await;
}
