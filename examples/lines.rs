use yape::{YapeCallback, YapeEngine, YapeEngineApi, YapeResult, MouseButton};

struct Lines {
    x: u32,
    y: u32,
    clicked: bool,
    lines: Vec<(u32, u32, u32, u32)>
}

impl YapeCallback for Lines {
    fn on_create(&mut self) -> YapeResult<bool> {
        Ok(true)
    }

    fn on_update(&mut self, engine: &mut dyn YapeEngineApi, _time_elapsed: f32) -> YapeResult<bool> {

        engine.clear(&yape::WHITE);

        if engine.get_mouse_button_state(&MouseButton::Left).released {
            if self.clicked {
                let x = engine.get_mouse_x();
                let y = engine.get_mouse_y();
                self.lines.push((self.x, self.y, x, y));
                self.clicked = false;
            } else {
                self.x = engine.get_mouse_x();
                self.y = engine.get_mouse_y();
                self.clicked = true;
            }
        }

        if self.clicked {
            engine.draw_line(self.x, self.y, engine.get_mouse_x(), engine.get_mouse_y(), &yape::BLACK);
        }

        for (x1, y1, x2, y2) in self.lines.as_slice() {
            engine.draw_line(*x1, *y1, *x2, *y2, &yape::BLACK);
        }

        Ok(true)
    }
}

fn main() -> YapeResult<()>  {
    let mut example = Lines { x: 0, y: 0, clicked: false, lines: Vec::new() };
    let mut engine = YapeEngine::construct("Lines", 320, 240, 4, 4)?;
    engine.start(&mut example)
}
