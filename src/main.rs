use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

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
    x: f32,
    y: f32,
}

impl Game {
    fn new(_ctx: &mut Context) -> Game {
        Game { x: 0.0, y: 0.0 }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, 10.0, 20.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        graphics::draw(ctx, &rect, ([0.0, 0.0],))?;

        graphics::present(ctx)?;

        Ok(())
    }
}
