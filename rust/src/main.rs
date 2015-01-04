extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate num;

use std::cell::RefCell;
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use event::{ Events, WindowSettings };
use image::GenericImage;
use input::{ Button, MouseButton };
use std::iter::{Cycle, CloneIteratorExt, Range, Peekable};
use std::fmt::Show;
use std::iter::FromIterator;
use num::complex::Complex;

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    let (width, height) = (300, 200);
    let (ymin, ymax) = (-1.0, 1.0); // 1.0 - (-1.0) = 2.0
    let (xmin, xmax) = (-2.0, 1.0); // -2.0 - 1.0 = 3.0

    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Mandelbrot Set".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );

    let mut image = image::ImageBuffer::new(width, height);
    let mut texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);

    for x in range(0, width) {
        for y in range(0, height) {
            let imaginary = ((y as f64 / height as f64) * (ymin - ymax)) - ymin;
            let real = ((x as f64 / width as f64) * (xmin - xmax)) - xmin;

            let point = Complex::new(real, imaginary);
            let mut z = Complex::new(0.0, 0.0);
            let mut e = 0;

            for i in range(0u8, 255u8) {
                z = z*z + point;

                if z.norm_sqr() > 4.0 {
                    e = i;
                    break;
                }
            }

            println!("{} -> {} ", point, e);
            image.put_pixel(x, y, image::Rgba([e, e, e, 255]));
            texture.update(&image);
        }
        println!("");
    }

    for e in Events::new(&window) {
        use event::{ RenderEvent };
        e.render(|args| {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                //graphics::clear([1.0, ..4], gl);
                graphics::image(&texture, &c, gl);
            });
        });
    }
}
