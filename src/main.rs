use bracket_lib::prelude::*;

const PLAYER_SCREEN_X: i32 = 4;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 30.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let glyph = to_cp437('|');
        let screen_x = self.x - player_x;

        let top_gap = self.gap_y - (self.size / 2);
        for i in 0..top_gap {
            ctx.set(screen_x, i, RED, NAVY, glyph);
        }

        let bottom_gap = self.gap_y + (self.size / 2);
        for j in bottom_gap..SCREEN_HEIGHT {
            ctx.set(screen_x, j, RED, NAVY, glyph)
        }
    }

    fn hit_player(&mut self, player: &Player) -> bool {
        let player_above_gap = player.y < self.gap_y - (self.size / 2);
        let player_below_gap = player.y > self.gap_y + (self.size / 2);
        self.x - player.x == PLAYER_SCREEN_X && (player_above_gap || player_below_gap)
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(PLAYER_SCREEN_X, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(4, 16),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0
        }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(8, "Flappy Dragon");
        ctx.print_centered(16, "(P) Play");
        ctx.print_centered(20, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

         if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_player(&self.player) {
            self.mode = GameMode::End;
            return;
        }

        if self.player.x >= self.obstacle.x {
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
            self.score += 1;
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }

        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score: {}", self.score));
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(8, "Game Over!");
        ctx.print_centered(10, format!("You earned {} points", self.score));
        ctx.print_centered(16, "(P) Play Again");
        ctx.print_centered(20, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(4, 16);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.end(ctx),
        }
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(ctx, State::new())
}
