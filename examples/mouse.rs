use pixel_engine::{YapeCallback, YapeEngineApi, YapeEngine, YapeResult, Pixel};
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

struct MousePixel {
    rng: ThreadRng
}

impl YapeCallback for MousePixel {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {
        if engine.get_mouse_wheel() != 0 {
            println!("wheel={}", engine.get_mouse_wheel());
        }
        let l = engine.get_mouse_button_state(0);
        let x = engine.get_mouse_x();
        let y = engine.get_mouse_y();
        let pixel = Pixel::from_rgb(self.rng.gen_range(0, 16777215));
        if l.held {
            engine.draw_pixel(x, y, &pixel);
        }
        Ok(true)
    }
}

fn main() -> YapeResult<()>  {
    let mut example = MousePixel { rng: thread_rng() };
    let mut engine = YapeEngine::construct("Random", 160, 120, 4, 4)?;
    engine.start(&mut example)
}
