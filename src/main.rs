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

struct Game;

impl Game {
    fn new(_ctx: &mut Context) -> Game {
        Game
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)?;

        Ok(())
    }
}
