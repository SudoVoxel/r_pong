extern crate rand;
use macroquad::{
    miniquad::conf::{Icon, Platform},
    prelude::*,
};
use rand::seq::SliceRandom;

const BALL_SPEED: u16 = 1;
const PLAYER_SPEED: u16 = 5;

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
            (true, true) => {}
            (false, false) => {}
        }
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let scr_width: f32 = screen_width();
    let scr_height: f32 = screen_height();
    println!("{}, {}", scr_width, scr_height);
    let mut ball: Ball = Ball {
        rect: Rect {
            x: (scr_width / 2.0) - 10.0,
            y: (scr_height / 2.0) - 10.0,
            w: 20.0,
            h: 20.0,
        },
        moving: true,
    };
    let mut player_1: Player = Player {
        // Refined & perfected values delivered by my anus™
        rect: Rect {
            x: 20.0,
            y: scr_height / 2.0,
            w: 20.0,
            h: 80.0,
        },
        score: 0,
    };
    let mut player_2: Player = Player {
        rect: Rect {
            x: scr_width - 40.0,
            y: scr_height / 2.0,
            w: 20.0,
            h: 80.0,
        },
        score: 0,
    };
    let possible_directions: [(i32, i32); 4] = [(-5, 5), (-5, -5), (5, -5), (5, 5)]; // this code is duplicated twice, but im too lazy to put it into a function.

    let starting_direction: (i32, i32) = match possible_directions.choose(&mut rand::thread_rng()) {
        Some(dir) => *dir,
        None => panic!("Could not select a starting direction"),
    };

    let mut direction = vec2(starting_direction.0 as f32, starting_direction.1 as f32);
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
            player_1.rect.x.clone(),
            player_1.rect.y.clone(),
            player_1.rect.w.clone(),
            player_1.rect.h.clone(),
            WHITE,
        );
        draw_rectangle(
            player_2.rect.x.clone(),
            player_2.rect.y.clone(),
            player_2.rect.w.clone(),
            player_2.rect.h.clone(),
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
                        ball.rect.x = scr_width / 2.0 - 10.0;
                        ball.rect.y = scr_height / 2.0 - 10.0;
                        ball.moving = !ball.moving;
                        let possible_directions: [(i32, i32); 4] =
                            [(-5, 5), (-5, -5), (5, -5), (5, 5)];

                        let starting_direction: (i32, i32) =
                            match possible_directions.choose(&mut rand::thread_rng()) {
                                Some(dir) => *dir,
                                None => panic!("Could not select a starting direction"),
                            };
                        direction = vec2(starting_direction.0 as f32, starting_direction.1 as f32)
                    }
                    2 => {
                        player_2.score += 1;
                        ball.rect.x = scr_width / 2.0 - 10.0;
                        ball.rect.y = scr_height / 2.0 - 10.0;
                        ball.moving = !ball.moving;
                        let possible_directions: [(i32, i32); 4] =
                            [(-5, 5), (-5, -5), (5, -5), (5, 5)];

                        let starting_direction: (i32, i32) =
                            match possible_directions.choose(&mut rand::thread_rng()) {
                                Some(dir) => *dir,
                                None => panic!("Could not select a starting direction"),
                            };
                        direction = vec2(starting_direction.0 as f32, starting_direction.1 as f32)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if is_key_pressed(KeyCode::Space) {
            ball.moving = !ball.moving;
        }
        if ball.moving {
            ball.rect.x += direction.x * BALL_SPEED as f32;
            ball.rect.y += direction.y * BALL_SPEED as f32;
        }

        //putting a score counter on the screen
        if ball.moving {
            let text = format!("LEFT: {} RIGHT: {}", player_1.score, player_2.score);
            let font_size = 30.;
            let text_size = measure_text(&text, None, font_size as _, 1.0);

            draw_text(
                &text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                WHITE,
            );
        } else {
            let text = "Press SPACE to start";
            let font_size = 30.;
            let text_size = measure_text(&text, None, font_size as _, 1.0);
            draw_text(
                &text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                WHITE,
            );
        }

        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "r_pong".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: Some(Icon::miniquad_logo()),
        platform: Platform::default(),
    }
}
