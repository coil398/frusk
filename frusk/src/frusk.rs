use futures::executor;
use std::io::ErrorKind;

pub struct Frusk {
    host: String,
    port: u16,
}

impl Frusk {
    pub fn new(host: String, port: u16) -> Frusk {
        Frusk { host, port }
    }

    async fn async_run(&self) {
        let pool = executor::ThreadPool::new().unwrap();

        let listener = frusk_core::create_listener(&self.host, self.port);

        loop {
            match listener.accept() {
                Ok((socket, _)) => {
                    pool.spawn_ok(frusk_core::handle_connection(socket));
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    panic!("Unexpected error: {}", e);
                }
            }
        }
    }

    pub fn run(&self) {
        executor::block_on(self.async_run());
    }
}
