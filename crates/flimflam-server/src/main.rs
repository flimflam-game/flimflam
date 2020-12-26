use flimflam_model::Event;
use parking_lot::Mutex;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let mut stream = BufReader::new(stream?);
        let clients = Arc::clone(&clients);

        thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let event = jsonl::read(&mut stream)?;
                dbg!(&event);

                match event {
                    Event::JoinGame(client) => clients.lock().push(client),
                    Event::PlayerMoved(_) => {}
                }

                dbg!(&clients);
            }
        });
    }

    Ok(())
}
