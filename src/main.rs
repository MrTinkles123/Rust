use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Gaming,
    End,
}
struct State {
    mode: GameMode,
}

impl State {
    
    fn new() -> Self {
        State { 
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //todo!()
        self.mode = GameMode::End;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        //todo!()
        ctx.cls();
        ctx.print_centered( 5,"You are dead!");
        ctx.print_centered(8, "(p) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Gaming;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        //  todo!()
        ctx.cls();
        ctx.print_centered( 5,"Welcome to Flappy Dragon");
        ctx.print_centered(8, "(p) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Gaming => self.dead(ctx),
            GameMode::End => self.play(ctx),
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("flappy dragon")
        .build()?;
    main_loop(context, State::new())
}