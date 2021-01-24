use frusk_core;
use futures::executor::block_on;

async fn async_main() {
    frusk_core::run().await;
}

fn main() {
    block_on(async_main());
}
