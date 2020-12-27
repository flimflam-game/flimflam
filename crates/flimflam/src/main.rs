use flimflam_model::{Client, Event, Update};
use flume::{Receiver, Sender};
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::input::keyboard;
use ggez::{graphics, timer};
use ggez::{Context, ContextBuilder, GameResult};
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use ultraviolet::Vec2;

const SPEED: f32 = 100.0;

fn main() -> anyhow::Result<()> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("flimflam", "The Razzaghipours")
        .window_setup(WindowSetup::default().title("Flimflam"))
        .build()
        .unwrap();

    let mut server_connection = TcpStream::connect("127.0.0.1:1234")?;

    let address = ([127, 0, 0, 1], portpicker::pick_unused_port().unwrap()).into();

    jsonl::write(
        &mut server_connection,
        &Event::JoinGame(Client::new(address)),
    )?;

    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        listen_for_updates(tx, address).unwrap_or_else(|err| eprintln!("Error: {:?}", err))
    });

    let mut game = Game::new(server_connection, rx);

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}

struct Game {
    pos: Vec2,
    server_connection: BufReader<TcpStream>,
    updates_rx: Receiver<Update>,
}

impl Game {
    fn new(server_connection: TcpStream, updates_rx: Receiver<Update>) -> Self {
        Self {
            pos: Vec2::zero(),
            server_connection: BufReader::new(server_connection),
            updates_rx,
        }
    }

    fn process_outstanding_updates(&mut self) {
        for update in self.updates_rx.try_iter() {
            match update {
                Update::PlayerMoved(pos) => self.pos = pos,
            }
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
            self.pos += diff;

            jsonl::write(
                self.server_connection.get_mut(),
                &Event::PlayerMoved(self.pos),
            )
            .unwrap_or_else(|err| eprintln!("Error: {:?}", anyhow::Error::new(err)));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.pos.x, self.pos.y, 10.0, 20.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        graphics::draw(ctx, &rect, ([0.0, 0.0],))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn listen_for_updates(tx: Sender<Update>, address: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = BufReader::new(stream?);
        let update = jsonl::read(stream)?;
        tx.send(update)?;
    }

    Ok(())
}
