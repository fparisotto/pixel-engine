use pixel_engine::{YapeCallback, YapeEngineApi, YapeEngine, YapeResult, Pixel};
use rand::{thread_rng, Rng};

struct RandomPixel {}

impl YapeCallback for RandomPixel {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {
        engine.clear(&pixel_engine::WHITE);
        let mut rng = thread_rng();
        for x in 0..engine.get_screen_width() {
            for y in 0..engine.get_screen_height() {
                let r: u8 = rng.gen_range(0, 255);
                let g: u8 = rng.gen_range(0, 255);
                let b: u8 = rng.gen_range(0, 255);
                // let pixel = Pixel::rgb(r,g,b);
                // engine.draw_pixel(x, y, &pixel);
                engine.draw_pixel(x, y, &pixel_engine::BLUE);
            }
        }
        Ok(false)
    }
}

fn main() -> YapeResult<()>  {
    let mut example = RandomPixel {};
    let mut engine = YapeEngine::construct("Random", 5, 5, 2, 2)?;
    engine.start(&mut example)
}
