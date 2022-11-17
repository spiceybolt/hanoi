use glutin_window::GlutinWindow;
use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{
    input::{Button, Key},
    EventLoop, PressEvent, RenderEvent, WindowSettings,
};

mod column;

use column::{GameState, Plank, Selection};
pub const PLANK_NUMBER: f64 = 5.0;
pub const PLANK_WIDTH: f64 = 50.0;
pub const PLANK_LENGTH: f64 = 50.0;
pub const COLUMN_LENGTH: f64 = PLANK_LENGTH * (PLANK_NUMBER + 1.0);
pub const COLUMN_WIDTH: f64 = PLANK_WIDTH * (PLANK_NUMBER + 2.0);
fn main() {
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("hanoi", (1000.0, 1000.0))
        .exit_on_esc(true)
        .graphics_api(opengl);

    let mut window: GlutinWindow = window_settings.build().expect("could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let pos = [10.0, 10.0];

    let mut moving_plank: Option<Plank> = None;

    let mut game_state = GameState::new(pos);

    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::A => {
                    game_state.sel_c = Selection::Left;

                    if let Some(plk) = moving_plank.as_mut() {
                        plk.rect[0] = pos[0] + (COLUMN_LENGTH - PLANK_LENGTH * plk.size) / 2.0;
                    }
                }
                Key::S => {
                    game_state.sel_c = Selection::Centre;

                    if let Some(plk) = moving_plank.as_mut() {
                        plk.rect[0] = pos[0]
                            + COLUMN_LENGTH
                            + (COLUMN_LENGTH - PLANK_LENGTH * plk.size) / 2.0;
                    }
                }
                Key::D => {
                    game_state.sel_c = Selection::Right;

                    if let Some(plk) = moving_plank.as_mut() {
                        plk.rect[0] = pos[0]
                            + 2.0 * COLUMN_LENGTH
                            + (COLUMN_LENGTH - PLANK_LENGTH * plk.size) / 2.0;
                    }
                }
                Key::W => {
                    moving_plank = match moving_plank {
                        //a plank is already up
                        Some(plk) => match game_state.sel_c {
                            Selection::Left
                                if game_state.left_c.planks[game_state.left_c.planks.len() - 1]
                                    .size
                                    > plk.size =>
                            {
                                game_state.left_c.insert_top(plk);
                                None
                            }
                            Selection::Centre
                                if game_state.centre_c.planks
                                    [game_state.centre_c.planks.len() - 1]
                                    .size
                                    > plk.size =>
                            {
                                game_state.centre_c.insert_top(plk);
                                None
                            }
                            Selection::Right
                                if game_state.right_c.planks
                                    [game_state.right_c.planks.len() - 1]
                                    .size
                                    > plk.size =>
                            {
                                game_state.right_c.insert_top(plk);
                                None
                            }
                            _ => Some(plk),
                        },
                        //a plank hasnt been selected
                        None => match game_state.sel_c {
                            Selection::Left => {
                                if game_state.left_c.planks.len() > 1 {
                                    Some(game_state.left_c.remove_top())
                                } else {
                                    None
                                }
                            }
                            Selection::Centre => {
                                if game_state.centre_c.planks.len() > 1 {
                                    Some(game_state.centre_c.remove_top())
                                } else {
                                    None
                                }
                            }
                            Selection::Right => {
                                if game_state.right_c.planks.len() > 1 {
                                    Some(game_state.right_c.remove_top())
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        },
                    };

                    //println!("ter:\n\n{:?}\n\n{:?}\n\n{:?}\n\n",game_state.left_c.planks,game_state.centre_c.planks,game_state.right_c.planks);
                }
                _ => {}
            }
        }

        if let Some(args) = event.render_args() {
            //drawing stuff here
            gl.draw(args.viewport(), |c: Context, g: &mut GlGraphics| {
                graphics::clear([1.0, 1.0, 1.0, 1.0], g);

                game_state.draw(&c, g);
                if let Some(plk) = moving_plank.as_mut() {
                    plk.draw(&c, g);
                }
            });
        }
    }
}
