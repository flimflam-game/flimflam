use flimflam_model::Event;
use std::io::BufReader;
use std::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;
    let mut clients = Vec::new();

    for stream in listener.incoming() {
        let mut stream = BufReader::new(stream?);

        loop {
            let event = jsonl::read(&mut stream)?;
            dbg!(&event);

            match event {
                Event::JoinGame(client) => clients.push(client),
                Event::PlayerMoved(_) => {}
            }

            dbg!(&clients);
        }
    }

    Ok(())
}
