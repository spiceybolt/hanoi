use graphics::Context;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{WindowSettings, EventLoop, RenderEvent};
use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings,Events};

mod game_screen;
pub use game_screen::GameScreen;

pub const BLOCKS: i32 = 5;
fn main() {
    println!("Hello, world!");

    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("hanoi", (500,500))
        .exit_on_esc(true)
        .graphics_api(opengl);

    let mut window: GlutinWindow = window_settings.build()
        .expect("could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let Game = GameScreen::new(BLOCKS);
    Game.read();
    while let Some(event) = events.next(&mut window){
        //pass events into  controllers here
        if let Some(args) = event.render_args() {
            //drawing stuff here
            gl.draw(args.viewport(), |c: Context, g: &mut GlGraphics| {
                use graphics::clear;

                clear([1.0,1.0,1.0,1.0],g);
            });
        }
    }
}
