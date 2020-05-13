use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use yape::{KeyboardKey, MouseButton, Pixel, YapeCallback, YapeEngine, YapeEngineApi, YapeResult};

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
        if engine.get_mouse_button_state(&MouseButton::Left).held {
            let x = engine.get_mouse_x();
            let y = engine.get_mouse_y();
            let pixel = Pixel::from_rgb(self.rng.gen_range(0, 16777215));
            engine.draw_pixel(x, y, &pixel);
        }

        if engine.get_key_state(&KeyboardKey::Space).pressed {
            println!("Space bar pressed")
        }

        Ok(true)
    }
}

fn main() -> YapeResult<()>  {
    let mut example = MousePixel { rng: thread_rng() };
    let mut engine = YapeEngine::construct("Random", 320, 240, 4, 4)?;
    engine.start(&mut example)
}
