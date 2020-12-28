use flimflam_model::{Client, Event, Player};
use flume::{Receiver, Sender};
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::input::keyboard;
use ggez::{graphics, timer};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{env, iter, thread};
use ultraviolet::Vec2;

const SPEED: f32 = 200.0;

fn main() -> anyhow::Result<()> {
    let mut server_connection = {
        let server_address = if let Some(addr) = env::args().nth(1) {
            addr
        } else {
            anyhow::bail!("expected server address")
        };

        TcpStream::connect(server_address)?
    };

    let address = (
        flimflam_utils::get_ip()?,
        portpicker::pick_unused_port().unwrap(),
    )
        .into();

    let client = Client::new(address);

    jsonl::write(
        &mut server_connection,
        &Event::JoinGame(
            client.clone(),
            Player {
                position: Vec2::zero(),
            },
        ),
    )?;

    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        listen_for_events(tx, address).unwrap_or_else(|err| eprintln!("Error: {:?}", err))
    });

    let mut game = Game::new(client, server_connection, rx);

    let (mut ctx, mut event_loop) = ContextBuilder::new("flimflam", "The Razzaghipours")
        .window_setup(WindowSetup::default().title("Flimflam"))
        .build()
        .unwrap();

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}

struct Game {
    client: Client,
    player: Player,
    other_players: HashMap<Client, Player>,
    server_connection: BufReader<TcpStream>,
    events_rx: Receiver<Event>,
}

impl Game {
    fn new(client: Client, server_connection: TcpStream, events_rx: Receiver<Event>) -> Self {
        Self {
            client,
            player: Player {
                position: Vec2::zero(),
            },
            other_players: HashMap::new(),
            server_connection: BufReader::new(server_connection),
            events_rx,
        }
    }

    fn process_outstanding_updates(&mut self) {
        for event in self.events_rx.try_iter() {
            let (client, player) = match event {
                Event::PlayerUpdate(c, p) => (c, p),
                Event::JoinGame(c, p) => (c, p),
            };

            self.other_players.insert(client, player);
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_outstanding_updates();

        let keys = keyboard::pressed_keys(ctx);

        let mut movement = Vec2::zero();

        if keys.contains(&KeyCode::S) {
            movement.y += 1.0;
        }

        if keys.contains(&KeyCode::W) {
            movement.y -= 1.0;
        }

        if keys.contains(&KeyCode::D) {
            movement.x += 1.0;
        }

        if keys.contains(&KeyCode::A) {
            movement.x -= 1.0;
        }

        if movement != Vec2::zero() {
            movement.normalize();
        }

        let diff = movement * SPEED * timer::delta(ctx).as_secs_f32();

        if diff != Vec2::zero() {
            self.player.position += diff;

            jsonl::write(
                self.server_connection.get_mut(),
                &Event::PlayerUpdate(self.client.clone(), self.player.clone()),
            )
            .unwrap_or_else(|err| eprintln!("Error: {:?}", anyhow::Error::new(err)));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for player in self.other_players.values().chain(iter::once(&self.player)) {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(player.position.x, player.position.y, 10.0, 20.0),
                graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            )?;

            graphics::draw(ctx, &rect, ([0.0, 0.0],))?;
        }

        graphics::present(ctx)?;

        Ok(())
    }
}

fn listen_for_events(tx: Sender<Event>, address: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = BufReader::new(stream?);
        let event = jsonl::read(stream)?;
        tx.send(event)?;
    }

    Ok(())
}
