extern crate rand;
use macroquad::{
    miniquad::conf::{Icon, Platform},
    prelude::*,
};
use rand::seq::SliceRandom;

const BALL_SPEED: u16 = 1;
const PLAYER_SPEED: u16 = 5;
const POSSIBLE_DIRECTIONS: [(i32, i32); 4] = [(-5, 5), (-5, -5), (5, -5), (5, 5)];
const FONT_SIZE: f32 = 30.0;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

struct Player {
    rect: Rect,
    score: u16,
}

struct Ball {
    rect: Rect,
    moving: bool,
}
impl Ball {
    fn draw(&self, color: Color) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color)
    }
    fn check_collisions(&mut self, p_1: Rect, p_2: Rect) -> (bool, bool, u8) {
        // returns an extra u8 to indicate which side (if any) scored. 0: Nobody scored. 1: Player_1 scored, 2: player_2 scored.

        let mut scoring_player: u8 = 0;
        let mut player_collision: bool = false;
        let mut wall_collision: bool = false;
        if self.rect.intersect(p_1).is_some() || self.rect.intersect(p_2).is_some() {
            player_collision = true;
        }
        match (self.rect.x as i32, self.rect.y as i32) {
            // oooh bad code!
            (_, -5..=0) => {
                // Collision with top wall
                wall_collision = true;
            }
            (_, 600..=605) => {
                // collision with bottom wall
                wall_collision = true;
            }
            (800..=820, _) => {
                wall_collision = true;
                scoring_player = 1;
            }
            (-5..=0, _) => {
                wall_collision = true;
                scoring_player = 2;
            }
            _ => {}
        }

        return (player_collision, wall_collision, scoring_player);
    }
}

impl Player {
    fn handle_character_movement(&mut self, up: bool, down: bool, speed: u16) {
        match (up, down) {
            (true, false) => {
                if self.rect.center().y > 0.0 {
                    self.rect.y -= speed as f32
                }
            }
            (false, true) => {
                if self.rect.center().y < screen_height() {
                    self.rect.y += speed as f32
                }
            }
            (true, true) | (false, false) => {}
            
        }
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut ball: Ball = Ball {
        rect: Rect {
            x: (WIN_WIDTH / 2.0) - 10.0,
            y: (WIN_HEIGHT / 2.0) - 10.0,
            w: 20.0,
            h: 20.0,
        },
        moving: false,
    };
    let mut player_1: Player = Player {
        // Refined & perfected values delivered by my anus™
        rect: Rect {
            x: 20.0,
            y: WIN_HEIGHT / 2.0,
            w: 20.0,
            h: 80.0,
        },
        score: 0,
    };
    let mut player_2: Player = Player {
        rect: Rect {
            x: WIN_WIDTH - 40.0,
            y: WIN_WIDTH / 2.0,
            w: 20.0,
            h: 80.0,
        },
        score: 0,
    };
    

    let starting_direction: (i32, i32) = get_random_direction(POSSIBLE_DIRECTIONS);

    let mut direction: Vec2 = vec2(starting_direction.0 as f32, starting_direction.1 as f32);
    loop {
        clear_background(BLACK);

        player_1.handle_character_movement(
            is_key_down(KeyCode::W),
            is_key_down(KeyCode::S),
            PLAYER_SPEED,
        );
        player_2.handle_character_movement(
            is_key_down(KeyCode::Up),
            is_key_down(KeyCode::Down),
            PLAYER_SPEED,
        );
        draw_rectangle(
            player_1.rect.x,
            player_1.rect.y,
            player_1.rect.w,
            player_1.rect.h,
            WHITE,
        );
        draw_rectangle(
            player_2.rect.x,
            player_2.rect.y,
            player_2.rect.w,
            player_2.rect.h,
            WHITE,
        );

        ball.draw(WHITE);
        let ball_collisions: (bool, bool, u8) = ball.check_collisions(player_1.rect, player_2.rect);
        match ball_collisions {
            (true, false, _) => {
                direction.x = -direction.x;
            }
            (false, true, _) => {
                direction.y = -direction.y;

                match ball_collisions.2 {
                    1 => {
                        player_1.score += 1;
                        let buffer: (i32, i32) = reset_game(&mut ball);
                        direction = vec2(buffer.0 as f32, buffer.1 as f32);
                    }
                    2 => {
                        player_2.score += 1;
                        let buffer: (i32, i32) = reset_game(&mut ball);
                        direction = vec2(buffer.0 as f32, buffer.1 as f32);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if is_key_pressed(KeyCode::Space) {
            ball.moving = !ball.moving;
        }

        match ball.moving {
            true => {
                ball.rect.x += direction.x * BALL_SPEED as f32;
                ball.rect.y += direction.y * BALL_SPEED as f32;

                let text = format!("LEFT: {} RIGHT: {}", player_1.score, player_2.score).to_owned();

                let text_size = measure_text(&text, None, FONT_SIZE as _, 1.0);

                draw_text(
                    &text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    FONT_SIZE,
                    WHITE,
                );
            }
            false => {
                let text = "Press SPACE to start";

                let text_size = measure_text(&text, None, FONT_SIZE as _, 1.0);
                draw_text(
                    &text,
                    WIN_WIDTH / 2.0 - text_size.width / 2.0,
                    WIN_HEIGHT / 2.0 - text_size.height / 2.0,
                    FONT_SIZE,
                    WHITE,
                );
            }
        };

        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "r_pong".to_string(),
        window_width: WIN_WIDTH as i32,
        window_height: WIN_HEIGHT as i32,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: Some(Icon::miniquad_logo()),
        platform: Platform::default(),
    }
}
fn reset_game(ball: &mut Ball) -> (i32, i32) {
    ball.rect.x = WIN_WIDTH / 2.0 - 10.0;
    ball.rect.y = WIN_HEIGHT / 2.0 - 10.0;
    ball.moving = !ball.moving;

    get_random_direction(POSSIBLE_DIRECTIONS)
}

fn get_random_direction(direction_array: [(i32, i32); 4]) -> (i32, i32) {
    match direction_array.choose(&mut rand::thread_rng()) {
        Some(dir) => *dir,
        None => panic!("Could not select a starting direction"),
    }
}
