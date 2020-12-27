use flimflam_model::{Client, Event, Update};
use parking_lot::Mutex;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let clients = Arc::clone(&clients);

        thread::spawn(move || {
            handle_connection(stream, clients).unwrap_or_else(|err| eprintln!("Error: {:?}", err))
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, clients: Arc<Mutex<Vec<Client>>>) -> anyhow::Result<()> {
    let mut stream = BufReader::new(stream);

    let client = match jsonl::read(&mut stream)? {
        Event::JoinGame(c) => c,
        _ => anyhow::bail!("must send JoinGame event before others"),
    };

    {
        clients.lock().push(client.clone());
    }

    loop {
        let event = jsonl::read(&mut stream)?;

        match event {
            Event::PlayerMoved(pos) => {
                let clients = clients.lock();
                let all_other_clients = clients.iter().filter(|c| **c != client);

                for c in all_other_clients {
                    let stream = TcpStream::connect(c.address())?;
                    jsonl::write(stream, &Update::PlayerMoved(pos))?;
                }
            }

            Event::JoinGame(_) => anyhow::bail!("cannot send JoinGame event after first"),
        }
    }
}
