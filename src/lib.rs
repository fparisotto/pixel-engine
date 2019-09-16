extern crate sdl2;

use std::boxed::Box;
use std::time::Duration;
use std::error::Error;
use std::fmt;

use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::*;

#[derive(Debug)]
pub struct PixelEngineError {
    kind: String,
    message: String,
}

impl PixelEngineError {
    fn new(kind: &str, message: &str) -> PixelEngineError {
        PixelEngineError {
            kind: kind.to_string(),
            message: message.to_string()
        }
    }

    fn default(message: &str) -> PixelEngineError {
        PixelEngineError::new("default", message)
    }
}

impl fmt::Display for PixelEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"PixelEngineError[kind={}, message={}]", self.kind, self.message)
    }
}

impl Error for PixelEngineError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<String> for PixelEngineError {
    fn from(err: String) -> Self {
        PixelEngineError::default(&err)
    }
}

impl From<sdl2::video::WindowBuildError> for PixelEngineError {
    fn from(err: sdl2::video::WindowBuildError) -> Self {
        let message = format!("{}", err);
        PixelEngineError::new("WindowBuildError", &message[..])
    }
}

impl From<sdl2::IntegerOrSdlError> for PixelEngineError {
    fn from(err: sdl2::IntegerOrSdlError) -> Self {
        let message = format!("{}", err);
        PixelEngineError::new("WindowBuildError", &message[..])
    }
}

type PixelEngineResult<T> = Result<T, PixelEngineError>;

pub struct PixelEngine {
    sdl_context: Box<Sdl>,
    event_pump: Box<EventPump>,
    canvas: Box<Canvas<Window>>,
    pixel_w: u8,
    pixel_h: u8,
}

impl PixelEngine {

    pub fn new(title: &str, screen_w: u32, screen_h: u32, pixel_w: u8, pixel_h: u8) -> PixelEngineResult<Box<PixelEngine>> {
        let real_window_h = screen_h * pixel_h as u32;
        let real_window_w = screen_w * pixel_w as u32;
        let sdl_context = Box::new(sdl2::init()?);
        let video_subsystem = Box::new(sdl_context.video()?);
        let event_pump = Box::new(sdl_context.event_pump()?);
        let window = video_subsystem
            .window(title, real_window_w, real_window_h)
            .position_centered().opengl().build()?;
        let canvas = Box::new(window.into_canvas().build()?);
        let engine = PixelEngine { sdl_context, event_pump, canvas, pixel_w, pixel_h };
        return Ok(Box::new(engine));
    }

    pub fn get_screen_width(&self) -> i32 {
        let (w, _h) = self.canvas.window().size();
        return w as i32 / self.pixel_w as i32;
    }

    pub fn get_screen_height(&self) -> i32 {
        let (_w, h) = self.canvas.window().size();
        return h as i32 / self.pixel_h as i32;
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title).unwrap();
    }

    pub fn start(&mut self, callback: &mut dyn PixelEngineCallback) -> PixelEngineResult<()> {
        let mut timer = self.sdl_context.timer()?;
        let mut running = true;
        let mut ticks = timer.ticks();
        while running {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    _ => {}
                }
            }

            ticks = timer.ticks() - ticks;
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            callback.on_user_update(self, ticks);
            self.canvas.present();
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    pub fn drawn_rgb(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let real_x = x * self.pixel_w as i32;
        let real_y = y * self.pixel_h as i32;
        let rect = Rect::new(real_x as i32, real_y as i32, self.pixel_w as u32, self.pixel_h as u32);
        self.canvas.fill_rect(rect).unwrap();
    }
}

pub trait PixelEngineCallback {
    fn on_user_create(&mut self, engine: &mut PixelEngine) -> bool;
    fn on_user_update(&mut self, engine: &mut PixelEngine, time_elapsed: u32) -> bool;
}

