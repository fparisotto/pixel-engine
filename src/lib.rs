#[derive(Debug)]
pub struct YapeError {
    pub kind: String,
    pub message: String,
}

type YapeResult<T> = Result<T, YapeError>;

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
    
    pub fn rbga(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel { red, green, blue, alpha }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue, alpha: 0, }
    }
}

pub trait YapeEngineApi {
    fn is_window_focused(&self) -> bool;
    fn get_key_state(&self, key: &Key) -> ButtonState;
    fn get_mouse_button_state(&self, i: u8) -> ButtonState;
    fn get_mouse_x(&self) -> i32;
    fn get_mouse_y(&self) -> i32;
    fn get_mouse_wheel(&self) -> i32;

    fn get_screen_width(&self) -> i32;
    fn get_screen_height(&self) -> i32;
    fn set_screen_size(&self, w: i32, h: i32);
    fn get_fps(&self) -> u32;

    fn set_pixel_mode(&self, mode: &PixelMode);
    fn get_pixel_mode(&self) -> PixelMode;

    fn draw_pixel(&self, x: i32, y: i32, pixel: &Pixel);
    fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, pixel: &Pixel);
    fn draw_circle(&self, x: i32, y: i32, radius: i32, pixel: &Pixel);
    fn fill_circle(&self, x: i32, y: i32, radius: i32, pixel: &Pixel);
    fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, pixel: &Pixel);
    fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32, pixel: &Pixel);
    fn draw_triangle(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, pixel: &Pixel);
    fn fill_triangle(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, pixel: &Pixel);
    fn clear(&self, pixel: &Pixel);
}

pub trait YapeCallback {
    fn on_create(&mut self, engine: &mut dyn YapeEngineApi) -> YapeResult<()>;
    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, time_elapsed: u32) -> YapeResult<()>;
    fn on_terminate(&mut self, engine: &mut dyn YapeEngineApi, time_elapsed: u32) -> YapeResult<()>;
}

pub struct YapeEngine {}

impl YapeEngine {
    pub fn construct(screen_width: i32, height: i32, pixel_w: i32, pixel_h: i32) -> YapeEngine {
        todo!()
    }

    pub fn start(&mut self, callback: &mut dyn YapeCallback) -> YapeResult<()> {
        todo!()
    }
}

impl YapeEngineApi for YapeEngine {
    
    fn is_window_focused(&self) -> bool {
        todo!()
    }

    fn get_key_state(&self, _: &Key) -> ButtonState {
        todo!()
    }
    
    fn get_mouse_button_state(&self, _: u8) -> ButtonState {
        todo!()
    }
    
    fn get_mouse_x(&self) -> i32 {
        todo!()
    }
    
    fn get_mouse_y(&self) -> i32 {
        todo!()
    }
    
    fn get_mouse_wheel(&self) -> i32 {
        todo!()
    }
    
    fn get_screen_width(&self) -> i32 {
        todo!()
    }
    
    fn get_screen_height(&self) -> i32 {
        todo!()
    }
    
    fn set_screen_size(&self, _: i32, _: i32) {
        todo!()
    }
    
    fn get_fps(&self) -> u32 {
        todo!()
    }
    
    fn set_pixel_mode(&self, _: &PixelMode) {
        todo!()
    }
    
    fn get_pixel_mode(&self) -> PixelMode {
        todo!()
    }
    
    fn draw_pixel(&self, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn draw_line(&self, _: i32, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn draw_circle(&self, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn fill_circle(&self, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn draw_rect(&self, _: i32, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn fill_rect(&self, _: i32, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn draw_triangle(&self, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn fill_triangle(&self, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: &Pixel) {
        todo!()
    }
    
    fn clear(&self, _: &Pixel) {
        todo!()
    }

}
