use rand;
use yape::{YapeCallback, YapeEngine, YapeEngineApi, YapeResult};

struct Life {
    grid: Vec<u8>,
    output: Vec<u8>,
}

impl Life {
    fn new(cols: u32, rows: u32) -> Life {
        let mut life = Life {
            grid: vec![0; (cols * rows) as usize],
            output: vec![0; (cols * rows) as usize],
        };
        for cell in life.grid.iter_mut() {
            *cell = if rand::random::<bool>() { 1 } else { 0 };
        }
        life
    }
}

impl YapeCallback for Life {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {
        engine.clear(&yape::BLACK);

        let cols = engine.get_screen_width() as i32;
        let rows = engine.get_screen_height() as i32;

        for row in 0..rows {
            for col in 0..cols {
                let i = (row * cols + col) as usize;
                let state = self.grid[i] == 1;

                if state {
                    engine.draw_pixel(col as u32, row as u32, &yape::WHITE);
                }

                let mut n_sum: i32 = - (self.grid[i] as i32);
                for n_row in -1..2 {
                    for n_col in -1..2 {
                        let _c = (col + n_col + cols) % cols;
                        let _r = (row + n_row + rows) % rows;
                        let n = (_r * cols as i32 + _c) as usize;
                        n_sum += self.grid[n] as i32;
                    }
                }

                if !state && n_sum == 3 {
                    self.output[i] = 1;
                } else if state && (n_sum < 2 || n_sum > 3) {
                    self.output[i] = 0;
                } else {
                    self.output[i] = self.grid[i];
                }
            }
        }

        self.grid.copy_from_slice(self.output.as_slice());

        Ok(true)
    }
}

fn main() -> YapeResult<()> {
    let cols = 140;
    let rows = 100;
    let res = 5;
    let mut life = Life::new(cols, rows);
    let mut engine = YapeEngine::construct("Conway's Game of Life", cols, rows, res, res)?;
    engine.start(&mut life)
}
