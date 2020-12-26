use flimflam_model::{Client, Event};
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::input::keyboard;
use ggez::{graphics, timer};
use ggez::{Context, ContextBuilder, GameResult};
use std::net::TcpStream;
use ultraviolet::Vec2;

const SPEED: f32 = 100.0;

fn main() -> anyhow::Result<()> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("flimflam", "The Razzaghipours")
        .window_setup(WindowSetup::default().title("Flimflam"))
        .build()
        .unwrap();

    let mut server_connection = TcpStream::connect("127.0.0.1:1234")?;
    jsonl::write(&mut server_connection, &Event::JoinGame(Client::new()))?;
    let mut game = Game::new(server_connection);

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}

struct Game {
    pos: Vec2,
    server_connection: TcpStream,
}

impl Game {
    fn new(server_connection: TcpStream) -> Self {
        Self {
            pos: Vec2::zero(),
            server_connection,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
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

            jsonl::write(&mut self.server_connection, &Event::PlayerMoved(self.pos))
                .unwrap_or_else(|err| eprintln!("Error: {:?}", anyhow::Error::new(err)));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
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
