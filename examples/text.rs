extern mod nalgebra;
extern mod kiss3d;

use nalgebra::na::{Vec2, Vec3};
use nalgebra::na;
use kiss3d::window::Window;
use kiss3d::text::Font;

fn main() {
    Window::spawn("Kiss3d: empty window", proc(window) {
        let bigfont   = Font::new(&Path::new("media/font/Inconsolata.otf"), 120);
        let smallfont = Font::new(&Path::new("media/font/Inconsolata.otf"), 60);

        window.render_loop(|w| {
            w.draw_text("Hello birds!", &na::zero(), &bigfont, &Vec3::new(0.0, 1.0, 1.0));

            let ascii = &" !\"#$%&'`()*+,-_./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^abcdefghijklmnopqrstuvwxyz{|}~";

            w.draw_text(ascii, &Vec2::new(0.0, 120.0), &smallfont, &Vec3::new(1.0, 1.0, 0.0))
        })
    })
}
