use pixel_engine::{PixelEngine, PixelEngineCallback, PixelEngineError};
use rand;

struct Example {}

impl PixelEngineCallback for Example {
    fn on_user_create(&mut self, _engine: &mut PixelEngine) -> bool {
        true
    }

    fn on_user_update(&mut self, engine: &mut PixelEngine, _time_elapsed: u32) -> bool {
        for x in 0..engine.get_screen_width() {
            for y in 0..engine.get_screen_height() {
                let r: u8 = rand::random();
                let g: u8 = rand::random();
                let b: u8 = rand::random();
                engine.drawn_rgb(x, y, r, g, b);
            }
        }
        true
    }

}

fn main() -> Result<(), PixelEngineError>  {
    let mut example = Example {};
    let mut engine = PixelEngine::new("Example:", 160, 120, 4, 4)?;
    engine.start(&mut example)
}
