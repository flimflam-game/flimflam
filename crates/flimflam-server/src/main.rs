use flimflam_model::{Client, Event, Player};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let ip = flimflam_utils::get_ip()?;
    let address: SocketAddr = (ip, 1234).into();

    println!("Listening on {}", address);

    let listener = TcpListener::bind(address)?;
    let players = Arc::new(RwLock::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let players = Arc::clone(&players);

        thread::spawn(move || {
            handle_connection(stream, players).unwrap_or_else(|err| eprintln!("Error: {:?}", err))
        });
    }

    Ok(())
}

fn handle_connection(
    stream: TcpStream,
    players: Arc<RwLock<HashMap<Client, Player>>>,
) -> anyhow::Result<()> {
    let mut stream = BufReader::new(stream);

    let (client, player) = match jsonl::read(&mut stream)? {
        Event::JoinGame(c, p) => (c, p),
        _ => anyhow::bail!("must send JoinGame event before others"),
    };

    {
        players.write().insert(client.clone(), player.clone());
    }

    tell_all_other_clients(
        &client,
        &Event::JoinGame(client.clone(), player),
        players.read().keys(),
    )?;

    loop {
        let event = jsonl::read(&mut stream)?;

        match &event {
            Event::PlayerUpdate(c, p) => {
                {
                    players.write().insert(c.clone(), p.clone()).unwrap();
                }

                tell_all_other_clients(&client, &event, players.read().keys())?;
            }
            Event::JoinGame(_, _) => anyhow::bail!("cannot send JoinGame event after first"),
        }
    }
}

fn tell_all_other_clients<'a>(
    client: &Client,
    event: &Event,
    clients: impl Iterator<Item = &'a Client>,
) -> anyhow::Result<()> {
    let all_other_clients = clients.filter(|c| *c != client);

    for c in all_other_clients {
        let stream = TcpStream::connect(c.address())?;
        jsonl::write(stream, event)?;
    }

    Ok(())
}
