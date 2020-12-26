use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics;
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};
use ultraviolet::Vec2;

fn main() -> ggez::GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("flimflam", "The Razzaghipours")
        .window_setup(WindowSetup::default().title("Flimflam"))
        .build()
        .unwrap();

    let mut game = Game::new(&mut ctx);

    event::run(&mut ctx, &mut event_loop, &mut game)?;

    Ok(())
}

struct Game {
    pos: Vec2,
}

impl Game {
    fn new(_ctx: &mut Context) -> Game {
        Game { pos: Vec2::zero() }
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

        self.pos += movement;

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
