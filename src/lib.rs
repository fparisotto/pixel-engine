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

impl std::clone::Clone for ButtonState {
    fn clone(&self) -> Self {
        ButtonState { pressed: self.pressed, released: self.released, held: self.held }
    }
}

pub enum Key {
    None,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    K0, K1, K2, K3, K4, K5, K6, K7, K8, K9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Up, Down, Left, Right,
    Space, Tab, Shift, Ctrl, Ins, Del, Home, End, PgUp, PgDn,
    Back, Escape, Return, Enter, Pause, Scroll,
    Np0, Np1, Np2, Np3, Np4, Np5, Np6, Np7, Np8, Np9,
    NpMul, NpDiv, NpAdd, NpSub, NpDecimal, Period
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
pub const BLACK: Pixel = Pixel { red: 0, green: 0, blue: 0, alpha: 255 };
pub const BLUE: Pixel = Pixel { red: 0, green: 0, blue: 255, alpha: 255 };
pub const CYAN: Pixel = Pixel { red: 0, green: 255, blue: 255, alpha: 255 };
pub const DARK_BLUE: Pixel = Pixel { red: 0, green: 0, blue: 128, alpha: 255 };
pub const DARK_CYAN: Pixel = Pixel { red: 0, green: 128, blue: 128, alpha: 255 };
pub const DARK_GREEN: Pixel = Pixel { red: 0, green: 128, blue: 0, alpha: 255 };
pub const DARK_GREY: Pixel = Pixel { red: 128, green: 128, blue: 128, alpha: 255 };
pub const DARK_MAGENTA: Pixel = Pixel { red: 128, green: 0, blue: 128, alpha: 255 };
pub const DARK_RED: Pixel = Pixel { red: 128, green: 0, blue: 0, alpha: 255 };
pub const DARK_YELLOW: Pixel = Pixel { red: 128, green: 128, blue: 0, alpha: 255 };
pub const GREEN: Pixel = Pixel { red: 0, green: 255, blue: 0, alpha: 255 };
pub const GREY: Pixel = Pixel { red: 192, green: 192, blue: 192, alpha: 255 };
pub const MAGENTA: Pixel = Pixel { red: 255, green: 0, blue: 255, alpha: 255 };
pub const RED: Pixel = Pixel { red: 255, green: 0, blue: 0, alpha: 255 };
pub const VERY_DARK_BLUE: Pixel = Pixel { red: 0, green: 0, blue: 64, alpha: 255 };
pub const VERY_DARK_CYAN: Pixel = Pixel { red: 0, green: 64, blue: 64, alpha: 255 };
pub const VERY_DARK_GREEN: Pixel = Pixel { red: 0, green: 64, blue: 0, alpha: 255 };
pub const VERY_DARK_GREY: Pixel = Pixel { red: 64, green: 64, blue: 64, alpha: 255 };
pub const VERY_DARK_MAGENTA: Pixel = Pixel { red: 64, green: 0, blue: 64, alpha: 255 };
pub const VERY_DARK_RED: Pixel = Pixel { red: 64, green: 0, blue: 0, alpha: 255 };
pub const VERY_DARK_YELLOW: Pixel = Pixel { red: 64, green: 64, blue: 0, alpha: 255 };
pub const WHITE: Pixel = Pixel { red: 255, green: 255, blue: 255, alpha: 255 };
pub const YELLOW: Pixel = Pixel { red: 255, green: 255, blue: 0, alpha: 255 };

impl Pixel {
    pub fn rbg_a(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel { red, green, blue, alpha }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue, alpha: 255 }
    }

    pub fn to_rgb(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn from_rgb(i: u32) -> Pixel {
        let red = (i / (256 ^ 2)) as u8;
        let green = ((i / 256) % 256) as u8;
        let blue = (i % 256) as u8;
        Pixel::rgb(red, green, blue)
    }
}

pub trait YapeEngineApi {
    fn is_window_focused(&self) -> bool;
    fn get_key_state(&self, key: &Key) -> &ButtonState;
    fn get_mouse_button_state(&self, i: u8) -> &ButtonState;
    fn get_mouse_x(&self) -> u32;
    fn get_mouse_y(&self) -> u32;
    fn get_mouse_wheel(&self) -> i32;
    fn get_screen_width(&self) -> u32;
    fn get_screen_height(&self) -> u32;
    fn get_fps(&self) -> u32;
    fn set_screen_size(&mut self, w: u32, h: u32);
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
    active: bool,
    minifb_window: minifb::Window,
    is_window_active: bool,
    buffer: Vec<u32>,
    last_fps: u32,
    mouse_pos_x: u32,
    mouse_pos_y: u32,
    mouse_wheel_delta: i32,
    mouse_button_state: Vec<ButtonState>,
    mouse_button_old_state: Vec<bool>,
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
        let window = minifb::Window::new(app_name, (screen_w * pixel_w) as usize, (screen_h * pixel_h) as usize, options)?;
        let buffer: Vec<u32> = vec![0; (screen_w * screen_h) as usize];
        Ok(
            YapeEngine {
                app_name: app_name.to_string(),
                screen_w,
                screen_h,
                pixel_w,
                pixel_h,
                active: true,
                minifb_window: window,
                is_window_active: false,
                buffer,
                last_fps: 0,
                mouse_pos_x: 0,
                mouse_pos_y: 0,
                mouse_wheel_delta: 0,
                mouse_button_state: vec![ButtonState { held: false, pressed: false, released: false }; 3],
                mouse_button_old_state: vec![false; 3],

            }
        )
    }

    pub fn start(&mut self, callback: &mut dyn YapeCallback) -> YapeResult<()> {
        if !callback.on_create()? {
            self.active = false;
        }

        self.minifb_window.limit_update_rate(None);

        let mut last_frame = time::Instant::now();
        let mut frame_counter: u32 = 0;
        let mut frame_timer: f32 = 0.0;

        while self.active && self.minifb_window.is_open() {
            let current_frame = time::Instant::now();
            let elapsed = (current_frame - last_frame).as_secs_f32();
            last_frame = current_frame;

            self.is_window_active = self.minifb_window.is_active();

            if let Some((screen_x, screen_y)) = self.minifb_window.get_mouse_pos(minifb::MouseMode::Discard) {
                self.mouse_pos_x = screen_x as u32 / self.pixel_w;
                self.mouse_pos_y = screen_y as u32 / self.pixel_h;
            }
            if let Some((_wheel_x, wheel_y)) = self.minifb_window.get_scroll_wheel() {
                self.mouse_wheel_delta = wheel_y as i32;
            }

            let mouse_button_new_state = vec![
                self.minifb_window.get_mouse_down(minifb::MouseButton::Left),
                self.minifb_window.get_mouse_down(minifb::MouseButton::Middle),
                self.minifb_window.get_mouse_down(minifb::MouseButton::Right),
            ];

            for i in 0..mouse_button_new_state.len() {
                self.mouse_button_state[i].pressed = false;
                self.mouse_button_state[i].released = false;
                if mouse_button_new_state[i] != self.mouse_button_old_state[i] {
                    if mouse_button_new_state[i] {
                        self.mouse_button_state[i].pressed = !self.mouse_button_state[i].held;
                        self.mouse_button_state[i].held = true;
                    } else {
                        self.mouse_button_state[i].released = true;
                        self.mouse_button_state[i].held = false;
                    }
                    self.mouse_button_old_state[i] = mouse_button_new_state[i];
                }
            }

            if !callback.on_update(self, elapsed)? {
                self.active = false;
            }

            self.minifb_window.update_with_buffer(&self.buffer, self.screen_w as usize, self.screen_h as usize)?;

            self.mouse_wheel_delta = 0;

            frame_counter += 1;
            frame_timer += elapsed;
            if frame_timer >= 1.0 {
                self.last_fps = frame_counter;
                frame_timer -= 1.0;
                frame_counter = 0;
                let new_title = format!("{} - {} fps", self.app_name, self.last_fps);
                self.minifb_window.set_title(new_title.as_str());
            }
        }
        Ok(())
    }
}

impl YapeEngineApi for YapeEngine {
    fn is_window_focused(&self) -> bool {
        self.is_window_active
    }

    fn get_key_state(&self, key: &Key) -> &ButtonState {
        unimplemented!()
    }

    fn get_mouse_button_state(&self, i: u8) -> &ButtonState {
        let index = i as usize;
        &self.mouse_button_state[index]
    }

    fn get_mouse_x(&self) -> u32 {
        self.mouse_pos_x
    }

    fn get_mouse_y(&self) -> u32 {
        self.mouse_pos_y
    }

    fn get_mouse_wheel(&self) -> i32 {
        self.mouse_wheel_delta
    }

    fn get_screen_width(&self) -> u32 {
        self.screen_w
    }

    fn get_screen_height(&self) -> u32 {
        self.screen_h
    }

    fn get_fps(&self) -> u32 {
        self.last_fps
    }

    fn set_screen_size(&mut self, w: u32, h: u32) {
        unimplemented!()
    }

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        let index = (y * self.screen_w + x) as usize;
        self.buffer[index] = pixel.to_rgb();
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
