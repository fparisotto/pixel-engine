use rand;
use yape::{YapeCallback, YapeEngine, YapeEngineApi, YapeResult};

struct Life {
    cols: u32,
    rows: u32,
    grid: Vec<bool>,
}

impl Life {
    fn new(cols: u32, rows: u32) -> Life {
        let mut life = Life {
            cols,
            rows,
            grid: vec![false; (cols * rows) as usize],
        };
        for cell in life.grid.iter_mut() {
            *cell = rand::random();
        }
        life
    }

    fn simulate(&mut self) {
        unimplemented!()
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        // let mut sum: u8 = 0;
        // let _x: i32 = x as i32;
        // let _y: i32 = y as i32;
        // let _cols: i32 = self.cols as i32;
        // let _rows: i32 = self.rows as i32;
        // for i in -1..2 {
        //     let col = ((_x + i + _cols) % _cols) as usize;
        //     for j in -1..2 {
        //         let row = ((_y + j + _rows) % _rows) as usize;
        //         if *self.get_at(col, row) {
        //             sum += 1;
        //         }
        //     }
        // }
        //
        // if *self.get_at(x as usize, y as usize) {
        //     sum -= 1;
        // }
        // sum
        unimplemented!()
    }
}

impl YapeCallback for Life {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {
        engine.clear(&yape::BLACK);
        // for (j, row) in self.grid.iter_mut().enumerate() {
        //     for (i, cell) in row.iter_mut().enumerate() {
        //         if *cell {
        //             engine.draw_pixel(i as u32, j as u32, &yape::WHITE);
        //         }
        //         let neighbors = self.count_neighbors(i, j);
        //         if !*cell && neighbors == 3 {
        //             self.set_next_at(i, j, true);
        //         } else if *cell && (neighbors < 2 || neighbors > 3) {
        //             self.set_next_at(i, j, false);
        //         } else {
        //             self.set_next_at(i, j, *cell);
        //         }
        //     }
        // }
        //
        // for (j, row) in self.next_grid.iter().enumerate() {
        //     for (i, cell) in row.iter().enumerate() {
        //         self.set_at(i, j, *cell);
        //     }
        // }

        Ok(true)
    }
}

fn main() -> YapeResult<()>  {
    let mut life = Life::new(160, 120);
    let mut engine = YapeEngine::construct("Conway's Game of Life", life.cols, life.rows, 4, 4)?;
    engine.start(&mut life)
}
