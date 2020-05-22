use yape::{YapeCallback, YapeEngineApi, YapeEngine, YapeResult, Pixel};
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

struct RandomPixel {
    rng: ThreadRng
}

impl YapeCallback for RandomPixel {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {
        engine.clear(&yape::WHITE);
        for x in 0..engine.get_screen_width() {
            for y in 0..engine.get_screen_height() {
                let pixel = Pixel::from_rgb(self.rng.gen_range(0, 16777215));
                engine.draw_pixel(x, y, &pixel);
            }
        }
        Ok(true)
    }
}

fn main() -> YapeResult<()>  {
    let mut example = RandomPixel { rng: thread_rng() };
    let mut engine = YapeEngine::construct("Random", 160, 120, 4, 4)?;
    engine.start(&mut example)
}
