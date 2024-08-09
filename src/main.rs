use std::net::{TcpListener, TcpStream};
use bottymc::ThreadPool;

mod network;
use network::client;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")
        .unwrap();
    // TODO: Set num_threads dynamically based on config/system
    let thread_pool = ThreadPool::new(None);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let client = client::spawn(stream);
}
