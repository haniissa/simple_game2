use std::time::{Duration, Instant};

struct GameState {
    player_pos: (f32, f32),
    player_vel: (f32, f32),
}
impl GameState {
    fn handle_input(&mut self) {
        //velocity
        // read input state
    }
    fn update(&mut self, dt: f32) {
        self.player_pos.0 += self.player_pos.0 * dt;
        self.player_vel.1 += self.player_vel.1 * dt;
    }

    fn render(&self, alpha: f32) {
        // render
    }
}
fn main() {
    let mut state = GameState {
        player_pos: (0.0, 0.0),
        player_vel: (120.0, 80.0),
    };

    let fixed_dt = Duration::from_secs_f32(1.0 / 60.0);
    let mut previous_time = Instant::now();
    let mut lag = Duration::ZERO;

    //main loop for game
    loop {
        let current_time = Instant::now();
        let elapsed = current_time - previous_time;
        previous_time = current_time;
        lag += elapsed;

        state.handle_input();

        while lag >= fixed_dt {
            state.update(fixed_dt.as_secs_f32());
            lag -= fixed_dt;
        }
        let alpha = lag.as_secs_f32() / fixed_dt.as_secs_f32();
        state.render(alpha);

        //
        break;
    }
}
