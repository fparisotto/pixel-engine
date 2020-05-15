use minifb;
use std::fmt;
use std::convert;
use std::time;

#[derive(Debug)]
pub struct YapeError {
    pub kind: String,
    pub message: String,
}

impl YapeError {
    fn new(kind: &str, message: &str) -> YapeError {
        YapeError { kind: kind.to_string(), message: message.to_string() }
    }
}

impl fmt::Display for YapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "YapeError[kind={}, message={}]", self.kind, self.message)
    }
}

impl convert::From<minifb::Error> for YapeError {
    fn from(error: minifb::Error) -> Self {
        match error {
            minifb::Error::MenusNotSupported => YapeError::new("MenusNotSupported", ""),
            minifb::Error::MenuExists(message) => YapeError::new("MenuExists", message.as_str()),
            minifb::Error::WindowCreate(message) => YapeError::new("WindowCreate", message.as_str()),
            minifb::Error::UpdateFailed(message) => YapeError::new("UpdateFailed", message.as_str()),
        }
    }
}

pub type YapeResult<T> = Result<T, YapeError>;

pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,
}

pub enum Key {
    NONE,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 
    K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, 
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, 
    Up, Down, Left, Right,
    Space, Tab, Shift, Ctrl, Ins, Del, Home, End, PageUp, PageDown,
    Back, Escape, Return, Enter, Pause, Scroll,
}

// pub enum PixelMode {
//     Normal,
//     Mask,
//     Alpha,
// }

pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

pub const BLANK: Pixel = Pixel { red: 0, green: 0, blue: 0, alpha: 0 };
pub const BLACK: Pixel = Pixel { red: 0, green: 0, blue: 0, alpha: 0 };
pub const BLUE: Pixel = Pixel { red: 0, green: 0, blue: 255, alpha: 0 };
pub const CYAN: Pixel = Pixel { red: 0, green: 255, blue: 255, alpha: 0 };
pub const DARK_BLUE: Pixel = Pixel { red: 0, green: 0, blue: 128, alpha: 0 };
pub const DARK_CYAN: Pixel = Pixel { red: 0, green: 128, blue: 128, alpha: 0 };
pub const DARK_GREEN: Pixel = Pixel { red: 0, green: 128, blue: 0, alpha: 0 };
pub const DARK_GREY: Pixel = Pixel { red: 128, green: 128, blue: 128, alpha: 0 };
pub const DARK_MAGENTA: Pixel = Pixel { red: 128, green: 0, blue: 128, alpha: 0 };
pub const DARK_RED: Pixel = Pixel { red: 128, green: 0, blue: 0, alpha: 0 };
pub const DARK_YELLOW: Pixel = Pixel { red: 128, green: 128, blue: 0, alpha: 0 };
pub const GREEN: Pixel = Pixel { red: 0, green: 255, blue: 0, alpha: 0 };
pub const GREY: Pixel = Pixel { red: 192, green: 192, blue: 192, alpha: 0 };
pub const MAGENTA: Pixel = Pixel { red: 255, green: 0, blue: 255, alpha: 0 };
pub const RED: Pixel = Pixel { red: 255, green: 0, blue: 0, alpha: 0 };
pub const VERY_DARK_BLUE: Pixel = Pixel { red: 0, green: 0, blue: 64, alpha: 0 };
pub const VERY_DARK_CYAN: Pixel = Pixel { red: 0, green: 64, blue: 64, alpha: 0 };
pub const VERY_DARK_GREEN: Pixel = Pixel { red: 0, green: 64, blue: 0, alpha: 0 };
pub const VERY_DARK_GREY: Pixel = Pixel { red: 64, green: 64, blue: 64, alpha: 0 };
pub const VERY_DARK_MAGENTA: Pixel = Pixel { red: 64, green: 0, blue: 64, alpha: 0 };
pub const VERY_DARK_RED: Pixel = Pixel { red: 64, green: 0, blue: 0, alpha: 0 };
pub const VERY_DARK_YELLOW: Pixel = Pixel { red: 64, green: 64, blue: 0, alpha: 0 };
pub const WHITE: Pixel = Pixel { red: 255, green: 255, blue: 255, alpha: 0 };
pub const YELLOW: Pixel = Pixel { red: 255, green: 255, blue: 0, alpha: 0 };

impl Pixel {
    pub fn rbg_a(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel { red, green, blue, alpha }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue, alpha: 0, }
    }

    pub fn to_rgb(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 16) | (g << 8) | b
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pixel[red={}, green={}, blue={}, alpha={}]", self.red, self.green, self.blue, self.alpha)
    }
}

pub trait YapeEngineApi {
    fn is_window_focused(&mut self) -> bool;
    fn get_key_state(&mut self, key: &Key) -> ButtonState;
    fn get_mouse_button_state(&mut self, i: u8) -> ButtonState;
    fn get_mouse_x(&mut self) -> u32;
    fn get_mouse_y(&mut self) -> u32;
    fn get_mouse_wheel(&mut self) -> u32;
    fn get_screen_width(&mut self) -> u32;
    fn get_screen_height(&mut self) -> u32;
    fn set_screen_size(&mut self, w: u32, h: u32);
    fn get_fps(&mut self) -> u32;
    // fn set_pixel_mode(&mut self, mode: &PixelMode);
    // fn get_pixel_mode(&mut self) -> PixelMode;
    fn draw_pixel(&mut self, x: u32, y: u32, pixel: &Pixel);
    fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, pixel: &Pixel);
    fn draw_circle(&mut self, x: u32, y: u32, radius: u32, pixel: &Pixel);
    fn fill_circle(&mut self, x: u32, y: u32, radius: u32, pixel: &Pixel);
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, pixel: &Pixel);
    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, pixel: &Pixel);
    fn draw_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, pixel: &Pixel);
    fn fill_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, pixel: &Pixel);
    fn clear(&mut self, pixel: &Pixel);
}

pub trait YapeCallback {
    fn on_create(&mut self) -> YapeResult<bool>;
    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, time_elapsed: f32) -> YapeResult<bool>;
    // fn on_terminate(&mut self, engine: &mut dyn YapeEngineApi, time_elapsed: u32) -> YapeResult<()>;
}

pub struct YapeEngine {
    app_name: String,
    screen_w: u32,
    screen_h: u32,
    pixel_w: u32,
    pixel_h: u32,
    scaled_screen_w: usize,
    scaled_screen_h: usize,
    active: bool,
    minifb_window: minifb::Window,
    buffer: Vec<u32>,
}

impl YapeEngine {

    pub fn construct(app_name: &str, screen_w: u32, screen_h: u32, pixel_w: u32, pixel_h: u32) -> YapeResult<YapeEngine> {
        let options = minifb::WindowOptions {
            scale_mode: minifb::ScaleMode::Stretch,
            scale: minifb::Scale::X1,
            borderless: false,
            resize: true,
            title: true,
            ..minifb::WindowOptions::default()
        };
        let scaled_screen_w = (screen_w * pixel_w) as usize;
        let scaled_screen_h = (screen_h * pixel_h) as usize;
        let buffer: Vec<u32> = vec![0; scaled_screen_w * scaled_screen_h];
        let window = minifb::Window::new(app_name, scaled_screen_w, scaled_screen_h, options)?;
        Ok(
            YapeEngine {
                app_name: app_name.to_string(),
                screen_w,
                screen_h,
                pixel_w,
                pixel_h,
                scaled_screen_w,
                scaled_screen_h,
                active: true,
                minifb_window: window,
                buffer
            }
        )
    }

    pub fn start(&mut self, callback: &mut dyn YapeCallback) -> YapeResult<()> {
        if !callback.on_create()? {
            self.active = false;
        }

        self.minifb_window.limit_update_rate(None);

        let mut last_frame = time::Instant::now();

        while self.active && self.minifb_window.is_open() && !self.minifb_window.is_key_down(minifb::Key::Escape) {
            let current_frame = time::Instant::now();
            let elapsed = current_frame - last_frame;
            last_frame = current_frame;

            if !callback.on_update(self, elapsed.as_secs_f32())? {
                self.active = false;
            }

            self.minifb_window.update_with_buffer(&self.buffer, self.scaled_screen_w / 2, self.scaled_screen_h / 2)?;
        }

        println!("screen_w={}", self.screen_w);
        println!("screen_h={}", self.screen_h);
        println!("pixel_w={}", self.pixel_w);
        println!("pixel_h={}", self.pixel_h);
        println!("scaled_screen_w={}", self.scaled_screen_w);
        println!("scaled_screen_h={}", self.scaled_screen_h);
        println!("buffer_len={}", self.buffer.len());
        println!();

        Ok(())
    }
}

impl YapeEngineApi for YapeEngine {
    fn is_window_focused(&mut self) -> bool {
        unimplemented!()
    }

    fn get_key_state(&mut self, key: &Key) -> ButtonState {
        unimplemented!()
    }

    fn get_mouse_button_state(&mut self, i: u8) -> ButtonState {
        unimplemented!()
    }

    fn get_mouse_x(&mut self) -> u32 {
        unimplemented!()
    }

    fn get_mouse_y(&mut self) -> u32 {
        unimplemented!()
    }

    fn get_mouse_wheel(&mut self) -> u32 {
        unimplemented!()
    }

    fn get_screen_width(&mut self) -> u32 {
        self.screen_w
    }

    fn get_screen_height(&mut self) -> u32 {
        self.screen_h
    }

    fn set_screen_size(&mut self, w: u32, h: u32) {
        unimplemented!()
    }

    fn get_fps(&mut self) -> u32 {
        unimplemented!()
    }

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        println!("draw_pixel(x={}, y={}, pixel={}", x, y, pixel);
        let x_start = x * self.pixel_w;
        let x_end = x_start + self.pixel_w;
        let y_start = y * self.pixel_h;
        let y_end = y_start + self.pixel_h;
        for _x in x_start..x_end {
            for _y in y_start..y_end {
                let index = if x == 0 { y } else if y == 0 { x } else { _x * _y } as usize;
                self.buffer[index] = pixel.to_rgb();
                println!("_x={},_y={},buffer[{}]={}", _x, _y, index, pixel.to_rgb());
            }
        }
    }

    fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_circle(&mut self, x: u32, y: u32, radius: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_circle(&mut self, x: u32, y: u32, radius: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, pixel: &Pixel) {
        unimplemented!()
    }

    fn clear(&mut self, pixel: &Pixel) {
        for i in self.buffer.iter_mut() {
            *i = pixel.to_rgb();
        }
    }
}
