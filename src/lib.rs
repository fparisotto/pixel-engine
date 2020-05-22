use minifb;
use std::fmt;
use std::convert;
use std::time;
use std::cmp;
use std::collections::HashMap;
use minifb::Key;

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

const BUTTON_STATE_EMPTY: ButtonState = ButtonState { pressed: false, released: false, held: false };

pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,
}

impl ButtonState {
    pub fn empty() -> ButtonState {
        ButtonState { pressed: false, released: false, held: false }
    }
}

impl std::clone::Clone for ButtonState {
    fn clone(&self) -> Self {
        ButtonState { pressed: self.pressed, released: self.released, held: self.held }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum KeyboardKey {
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,
    Down, Left, Right, Up, Backspace, Enter, Escape, Space, Tab,
    Insert, Delete, Home, End, PageUp, PageDown,
    LeftShift, RightShift, LeftCtrl, RightCtrl, LeftAlt, RightAlt,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left, Middle, Right
}

pub enum PixelMode {
    Normal,
    Mask,
    Alpha,
}

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

    pub fn from_rgb(i: u32) -> Pixel {
        let red = (i / (256 ^ 2)) as u8;
        let green = ((i / 256) % 256) as u8;
        let blue = (i % 256) as u8;
        Pixel::rgb(red, green, blue)
    }

    pub fn to_rgb(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn calculate_alpha(&self, depth: &Pixel, blend_factor: f32) -> Pixel {
        let a: f32 = (self.alpha as f32 / 255.0) * blend_factor;
        let c = 1.0 - a;
        let r = (a * self.red as f32 + c * depth.red as f32) as u8;
        let g = (a * self.green as f32 + c * depth.green as f32) as u8;
        let b = (a * self.blue as f32 + c * depth.blue as f32) as u8;
        Pixel::rgb(r, g, b)
    }
}

impl std::clone::Clone for Pixel {
    fn clone(&self) -> Self {
        Pixel::rbg_a(self.red, self.green, self.blue, self.alpha)
    }
}

pub enum SpriteMode {
    Normal, Periodic
}

pub enum SpriteFlip {
    None, Horizontal, Vertical
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Pixel>,
    pub mode: SpriteMode
}

impl Sprite {

    pub fn new(width: u32, height: u32) -> Sprite {
        Sprite {
            width,
            height,
            data: vec![BLANK; (width * height) as usize],
            mode: SpriteMode::Normal
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Pixel {
        match self.mode {
            SpriteMode::Normal => {
                if x < self.width && y < self.height {
                    let index = (y * self.width + x) as usize;
                    &self.data[index]
                } else {
                    &BLANK
                }
            }
            _ => {
                let index = ((y % self.height) + (x % self.width)) as usize;
                &self.data[index]
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        let index = (y * self.width + x) as usize;
        self.data[index] = pixel.clone();
    }

    pub fn sample(&self, x: u32, y: u32) -> &Pixel {
        let sx: u32 = cmp::min(x * self.width, self.width - 1);
        let sy: u32 = cmp::min(y * self.height, self.height - 1);
        self.get_pixel(sx, sy)
    }

    pub fn clear(&mut self, pixel: &Pixel) {
        for i in self.data.iter_mut() {
            *i = pixel.clone()
        }
    }

}

pub trait YapeEngineApi {
    fn is_window_focused(&self) -> bool;
    fn get_key_state(&self, key: &KeyboardKey) -> &ButtonState;
    fn get_mouse_button_state(&self, button: &MouseButton) -> &ButtonState;
    fn get_mouse_x(&self) -> u32;
    fn get_mouse_y(&self) -> u32;
    fn get_mouse_wheel(&self) -> i32;
    fn get_screen_width(&self) -> u32;
    fn get_screen_height(&self) -> u32;
    fn get_fps(&self) -> u32;
    fn set_pixel_mode(&mut self, mode: PixelMode);
    fn get_pixel_mode(&mut self) -> &PixelMode;
    fn set_pixel_blend(&mut self, blend: f32);
    fn get_pixel_blend(&self) -> f32;
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
}

pub struct YapeEngine {
    app_name: String,
    screen_w: u32,
    screen_h: u32,
    pixel_w: u32,
    pixel_h: u32,
    pixel_mode: PixelMode,
    blend_factor: f32,
    active: bool,
    minifb_window: minifb::Window,
    is_window_active: bool,
    buffer: Sprite,
    last_fps: u32,
    mouse_pos_x: u32,
    mouse_pos_y: u32,
    mouse_wheel_delta: i32,
    mouse_button_state: HashMap<MouseButton, (ButtonState, bool)>,
    keyboard_state: HashMap<KeyboardKey, ButtonState>,
    minifb_key_mapping: HashMap<minifb::Key, KeyboardKey>,
}

impl YapeEngine {
    pub fn construct(app_name: &str, screen_w: u32, screen_h: u32, pixel_w: u32, pixel_h: u32) -> YapeResult<YapeEngine> {
        let options = minifb::WindowOptions {
            scale_mode: minifb::ScaleMode::Stretch,
            scale: minifb::Scale::X1,
            borderless: false,
            resize: false,
            title: true,
            ..minifb::WindowOptions::default()
        };
        let window = minifb::Window::new(
            app_name,
            (screen_w * pixel_w) as usize,
            (screen_h * pixel_h) as usize,
            options
        )?;

        let mouse_button_state: HashMap<MouseButton, (ButtonState, bool)> = vec![
            (MouseButton::Left, (ButtonState::empty(), false)),
            (MouseButton::Middle, (ButtonState::empty(), false)),
            (MouseButton::Right, (ButtonState::empty(), false)),
        ].into_iter().collect();

        let minifb_key_mapping: HashMap<Key, KeyboardKey> = YapeEngine::build_minifb_key_mapping();
        let mut keyboard_state: HashMap<KeyboardKey, ButtonState> = HashMap::new();
        for (_, v) in minifb_key_mapping.iter() {
            keyboard_state.insert(v.clone(), ButtonState::empty());
        }

        Ok(
            YapeEngine {
                app_name: app_name.to_string(),
                screen_w,
                screen_h,
                pixel_w,
                pixel_h,
                pixel_mode: PixelMode::Normal,
                blend_factor: 1.0,
                active: true,
                minifb_window: window,
                is_window_active: false,
                buffer: Sprite::new(screen_w, screen_h),
                last_fps: 0,
                mouse_pos_x: 0,
                mouse_pos_y: 0,
                mouse_wheel_delta: 0,
                mouse_button_state,
                keyboard_state,
                minifb_key_mapping
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

        let mut rgb_buffer: Vec<u32> = vec![0; self.buffer.data.len()];

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

            for (button, (state, old_state)) in self.mouse_button_state.iter_mut() {
                let new_state = match button {
                    MouseButton::Left => self.minifb_window.get_mouse_down(minifb::MouseButton::Left),
                    MouseButton::Middle => self.minifb_window.get_mouse_down(minifb::MouseButton::Middle),
                    MouseButton::Right => self.minifb_window.get_mouse_down(minifb::MouseButton::Right),
                };
                state.pressed = false;
                state.released = false;
                if new_state != *old_state {
                    if new_state {
                        state.pressed = !state.held;
                        state.held = true;
                    } else {
                        state.released = true;
                        state.held = false;
                    }
                    *old_state = new_state;
                }
            }

            for (minifb_key, key) in self.minifb_key_mapping.iter() {
                match self.keyboard_state.get_mut(key) {
                    Some(state) => {
                        state.held = self.minifb_window.is_key_down(minifb_key.clone());
                        state.released = self.minifb_window.is_key_released(minifb_key.clone());
                        state.pressed = self.minifb_window.is_key_pressed(minifb_key.clone(), minifb::KeyRepeat::No);
                    },
                    _ => (),
                }
            }

            if !callback.on_update(self, elapsed)? {
                self.active = false;
            }

            for (i, p) in self.buffer.data.iter().enumerate() {
                rgb_buffer[i] = p.to_rgb();
            }
            self.minifb_window.update_with_buffer(&rgb_buffer, self.screen_w as usize, self.screen_h as usize)?;

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

    fn build_minifb_key_mapping() -> HashMap<minifb::Key, KeyboardKey> {
        vec![
            (minifb::Key::Key0, KeyboardKey::Key0),
            (minifb::Key::Key1, KeyboardKey::Key1),
            (minifb::Key::Key2, KeyboardKey::Key2),
            (minifb::Key::Key3, KeyboardKey::Key3),
            (minifb::Key::Key4, KeyboardKey::Key4),
            (minifb::Key::Key5, KeyboardKey::Key5),
            (minifb::Key::Key6, KeyboardKey::Key6),
            (minifb::Key::Key7, KeyboardKey::Key7),
            (minifb::Key::Key8, KeyboardKey::Key8),
            (minifb::Key::Key9, KeyboardKey::Key9),
            (minifb::Key::A, KeyboardKey::A),
            (minifb::Key::B, KeyboardKey::B),
            (minifb::Key::C, KeyboardKey::C),
            (minifb::Key::D, KeyboardKey::D),
            (minifb::Key::E, KeyboardKey::E),
            (minifb::Key::F, KeyboardKey::F),
            (minifb::Key::G, KeyboardKey::G),
            (minifb::Key::H, KeyboardKey::H),
            (minifb::Key::I, KeyboardKey::I),
            (minifb::Key::J, KeyboardKey::J),
            (minifb::Key::K, KeyboardKey::K),
            (minifb::Key::L, KeyboardKey::L),
            (minifb::Key::M, KeyboardKey::M),
            (minifb::Key::N, KeyboardKey::N),
            (minifb::Key::O, KeyboardKey::O),
            (minifb::Key::P, KeyboardKey::P),
            (minifb::Key::Q, KeyboardKey::Q),
            (minifb::Key::R, KeyboardKey::R),
            (minifb::Key::S, KeyboardKey::S),
            (minifb::Key::T, KeyboardKey::T),
            (minifb::Key::U, KeyboardKey::U),
            (minifb::Key::V, KeyboardKey::V),
            (minifb::Key::W, KeyboardKey::W),
            (minifb::Key::X, KeyboardKey::X),
            (minifb::Key::Y, KeyboardKey::Y),
            (minifb::Key::Z, KeyboardKey::Z),
            (minifb::Key::F1, KeyboardKey::F1),
            (minifb::Key::F2, KeyboardKey::F2),
            (minifb::Key::F3, KeyboardKey::F3),
            (minifb::Key::F4, KeyboardKey::F4),
            (minifb::Key::F5, KeyboardKey::F5),
            (minifb::Key::F6, KeyboardKey::F6),
            (minifb::Key::F7, KeyboardKey::F7),
            (minifb::Key::F8, KeyboardKey::F8),
            (minifb::Key::F9, KeyboardKey::F9),
            (minifb::Key::F10, KeyboardKey::F10),
            (minifb::Key::F11, KeyboardKey::F11),
            (minifb::Key::F12, KeyboardKey::F12),
            (minifb::Key::F13, KeyboardKey::F13),
            (minifb::Key::F14, KeyboardKey::F14),
            (minifb::Key::F15, KeyboardKey::F15),
            (minifb::Key::Down, KeyboardKey::Down),
            (minifb::Key::Left, KeyboardKey::Left),
            (minifb::Key::Right, KeyboardKey::Right),
            (minifb::Key::Up, KeyboardKey::Up),
            (minifb::Key::Backspace, KeyboardKey::Backspace),
            (minifb::Key::Enter, KeyboardKey::Enter),
            (minifb::Key::Escape, KeyboardKey::Escape),
            (minifb::Key::Space, KeyboardKey::Space),
            (minifb::Key::Tab, KeyboardKey::Tab),
            (minifb::Key::Insert, KeyboardKey::Insert),
            (minifb::Key::Delete, KeyboardKey::Delete),
            (minifb::Key::Home, KeyboardKey::Home),
            (minifb::Key::End, KeyboardKey::End),
            (minifb::Key::PageUp, KeyboardKey::PageUp),
            (minifb::Key::PageDown, KeyboardKey::PageDown),
            (minifb::Key::LeftShift, KeyboardKey::LeftShift),
            (minifb::Key::RightShift, KeyboardKey::RightShift),
            (minifb::Key::LeftCtrl, KeyboardKey::LeftCtrl),
            (minifb::Key::RightCtrl, KeyboardKey::RightCtrl),
            (minifb::Key::LeftAlt, KeyboardKey::LeftAlt),
            (minifb::Key::RightAlt, KeyboardKey::RightAlt),
        ].into_iter().collect()
    }
}

impl YapeEngineApi for YapeEngine {
    fn is_window_focused(&self) -> bool {
        self.is_window_active
    }

    fn get_key_state(&self, key: &KeyboardKey) -> &ButtonState {
        &self.keyboard_state
            .get(key)
            .unwrap_or(&BUTTON_STATE_EMPTY)
    }

    fn get_mouse_button_state(&self, button: &MouseButton) -> &ButtonState {
        &self.mouse_button_state
            .get(button)
            .map(|(button, _)| button)
            .unwrap_or(&BUTTON_STATE_EMPTY)
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

    fn set_pixel_mode(&mut self, mode: PixelMode) {
        self.pixel_mode = mode;
    }

    fn get_pixel_mode(&mut self) -> &PixelMode {
        &self.pixel_mode
    }

    fn set_pixel_blend(&mut self, blend: f32) {
        self.blend_factor = blend;
    }

    fn get_pixel_blend(&self) -> f32 {
        self.blend_factor
    }

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        match self.pixel_mode {
            PixelMode::Normal => self.buffer.set_pixel(x, y, pixel),
            PixelMode::Alpha => {
                let background = self.buffer.get_pixel(x, y).clone();
                self.buffer.set_pixel(x, y, &pixel.calculate_alpha(&background, self.blend_factor));
            },
            PixelMode::Mask => if pixel.alpha == 255 {
                self.buffer.set_pixel(x, y, pixel);
            }
        }
    }

    fn draw_line(&mut self, _x1: u32, _y1: u32, _x2: u32, _y2: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_circle(&mut self, _x: u32, _y: u32, _radius: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_circle(&mut self, _x: u32, _y: u32, _radius: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_rect(&mut self, _x: u32, _y: u32, _w: u32, _h: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_rect(&mut self, _x: u32, _y: u32, _w: u32, _h: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn draw_triangle(&mut self, _x1: u32, _y1: u32, _x2: u32, _y2: u32, _x3: u32, _y3: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn fill_triangle(&mut self, _x1: u32, _y1: u32, _x2: u32, _y2: u32, _x3: u32, _y3: u32, _pixel: &Pixel) {
        unimplemented!()
    }

    fn clear(&mut self, pixel: &Pixel) {
        self.buffer.clear(pixel);
    }
}
