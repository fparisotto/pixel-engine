// use pixel_engine::{PixelEngine, YapeCallback, YapeError};
// use rand::{thread_rng, Rng};

// struct RandomPixel {}

// impl YapeCallback for RandomPixel {
//     fn on_user_create(&mut self, _engine: &mut PixelEngine) -> bool {
//         true
//     }

//     fn on_user_update(&mut self, engine: &mut PixelEngine, _time_elapsed: u32) -> bool {
//         let mut rng = thread_rng();
//         for x in 0..engine.get_screen_width() {
//             for y in 0..engine.get_screen_height() {
//                 let r: u8 = rng.gen_range(0, 255);
//                 let g: u8 = rng.gen_range(0, 255);
//                 let b: u8 = rng.gen_range(0, 255);
//                 engine.drawn_rgb(x, y, r, g, b);
//             }
//         }
//         true
//     }
// }

// fn main() -> Result<(), PixelEngineError>  {
//     let mut example = RandomPixel {};
//     let mut engine = PixelEngine::new("Example:", 160, 120, 4, 4)?;
//     engine.start(&mut example)
// }

fn main() {
    
}