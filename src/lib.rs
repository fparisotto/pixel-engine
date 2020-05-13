// impl PixelEngineError {
//     fn new(kind: &str, message: &str) -> PixelEngineError {
//         PixelEngineError {
//             kind: kind.to_string(),
//             message: message.to_string()
//         }
//     }

//     fn default(message: &str) -> PixelEngineError {
//         PixelEngineError::new("default", message)
//     }
// }

// impl fmt::Display for PixelEngineError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"PixelEngineError[kind={}, message={}]", self.kind, self.message)
//     }
// }

// impl Error for PixelEngineError {
//     fn description(&self) -> &str {
//         &self.message
//     }
// }

// impl From<String> for PixelEngineError {
//     fn from(err: String) -> Self {
//         PixelEngineError::default(&err)
//     }
// }

// impl From<sdl2::video::WindowBuildError> for PixelEngineError {
//     fn from(err: sdl2::video::WindowBuildError) -> Self {
//         let message = format!("{}", err);
//         PixelEngineError::new("WindowBuildError", &message[..])
//     }
// }

// impl From<sdl2::IntegerOrSdlError> for PixelEngineError {
//     fn from(err: sdl2::IntegerOrSdlError) -> Self {
//         let message = format!("{}", err);
//         PixelEngineError::new("WindowBuildError", &message[..])
//     }
// }

// type PixelEngineResult<T> = Result<T, PixelEngineError>;

// pub struct PixelEngine {
//     sdl_context: Box<Sdl>,
//     event_pump: Box<EventPump>,
//     canvas: Box<Canvas<Window>>,
//     pixel_w: u8,
//     pixel_h: u8,
// }

// impl PixelEngine {

//     pub fn new(title: &str, screen_w: u32, screen_h: u32, pixel_w: u8, pixel_h: u8) -> PixelEngineResult<Box<PixelEngine>> {
//         let real_window_h = screen_h * pixel_h as u32;
//         let real_window_w = screen_w * pixel_w as u32;
//         let sdl_context = Box::new(sdl2::init()?);
//         let video_subsystem = Box::new(sdl_context.video()?);
//         let event_pump = Box::new(sdl_context.event_pump()?);
//         let window = video_subsystem
//             .window(title, real_window_w, real_window_h)
//             .position_centered().opengl().build()?;
//         let canvas = Box::new(window.into_canvas().build()?);
//         let engine = PixelEngine { sdl_context, event_pump, canvas, pixel_w, pixel_h };
//         return Ok(Box::new(engine));
//     }

//     pub fn get_screen_width(&self) -> i32 {
//         let (w, _h) = self.canvas.window().size();
//         return w as i32 / self.pixel_w as i32;
//     }

//     pub fn get_screen_height(&self) -> i32 {
//         let (_w, h) = self.canvas.window().size();
//         return h as i32 / self.pixel_h as i32;
//     }

//     pub fn set_window_title(&mut self, title: &str) {
//         self.canvas.window_mut().set_title(title).unwrap();
//     }

//     pub fn start(&mut self, callback: &mut dyn PixelEngineCallback) -> PixelEngineResult<()> {
//         let mut timer = self.sdl_context.timer()?;
//         let mut running = true;
//         let mut ticks = timer.ticks();
//         while running {
//             for event in self.event_pump.poll_iter() {
//                 match event {
//                     Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
//                         running = false;
//                     },
//                     _ => {}
//                 }
//             }

//             ticks = timer.ticks() - ticks;
//             self.canvas.set_draw_color(Color::RGB(0, 0, 0));
//             self.canvas.clear();
//             callback.on_user_update(self, ticks);
//             self.canvas.present();
//             std::thread::sleep(Duration::from_millis(100));
//         }
//         Ok(())
//     }

//     pub fn drawn_rgb(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
//         self.canvas.set_draw_color(Color::RGB(r, g, b));
//         let real_x = x * self.pixel_w as i32;
//         let real_y = y * self.pixel_h as i32;
//         let rect = Rect::new(real_x as i32, real_y as i32, self.pixel_w as u32, self.pixel_h as u32);
//         self.canvas.fill_rect(rect).unwrap();
//     }
// }

#[derive(Debug)]
pub struct YapeError {
    kind: String,
    message: String,
}

type YapeResult<T> = Result<T, YapeError>;

pub struct ButtonState {
    pressed: bool,
    released: bool,
    held: bool,
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

enum PixelMode {
    Normal,
    Mask,
    Alpha,
}

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

const BLANK: Pixel = Pixel { red: 0, green: 0, blue: 0, alpha: 0 };
const BLACK: Pixel = Pixel::rgb(0, 0, 0);
const BLUE: Pixel = Pixel::rgb(0, 0, 255);
const CYAN: Pixel = Pixel::rgb(0, 255, 255);
const DARK_BLUE: Pixel = Pixel::rgb(0, 0, 128);
const DARK_CYAN: Pixel = Pixel::rgb(0, 128, 128);
const DARK_GREEN: Pixel = Pixel::rgb(0, 128, 0);
const DARK_GREY: Pixel = Pixel::rgb(128, 128, 128);
const DARK_MAGENTA: Pixel = Pixel::rgb(128, 0, 128);
const DARK_RED: Pixel = Pixel::rgb(128, 0, 0);
const DARK_YELLOW: Pixel = Pixel::rgb(128, 128, 0);
const GREEN: Pixel = Pixel::rgb(0, 255, 0);
const GREY: Pixel = Pixel::rgb(192, 192, 192);
const MAGENTA: Pixel = Pixel::rgb(255, 0, 255);
const RED: Pixel = Pixel::rgb(255, 0, 0);
const VERY_DARK_BLUE: Pixel = Pixel::rgb(0, 0, 64);
const VERY_DARK_CYAN: Pixel = Pixel::rgb(0, 64, 64);
const VERY_DARK_GREEN: Pixel = Pixel::rgb(0, 64, 0);
const VERY_DARK_GREY: Pixel = Pixel::rgb(64, 64, 64);
const VERY_DARK_MAGENTA: Pixel = Pixel::rgb(64, 0, 64);
const VERY_DARK_RED: Pixel = Pixel::rgb(64, 0, 0);
const VERY_DARK_YELLOW: Pixel = Pixel::rgb(64, 64, 0);
const WHITE: Pixel = Pixel::rgb(255, 255, 255);
const YELLOW: Pixel = Pixel::rgb(255, 255, 0);

impl Pixel {
    
    const fn rbga(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel { red, green, blue, alpha }
    }

    const fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
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

struct YapeEngine {}

impl YapeEngine {
    fn create(screen_width: i32, height: i32, pixel_w: i32, pixel_h: i32) -> YapeEngine {
        unimplemented!()
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
