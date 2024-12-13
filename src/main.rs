use raylib::prelude::*;

const GRID_START_X: i32 = 10;
const GRID_START_Y: i32 = 10;
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: i32 = 30;
const BORDER_SIZE: i32 = 2;

struct TetrisGrid {
    grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
}

impl TetrisGrid {
    fn new() -> Self {
        TetrisGrid {
            grid: [[false; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = if self.grid[y][x] {
                    Color::BLACK
                } else {
                    Color::LIGHTGRAY
                };
                let x_pos = (x as i32 * CELL_SIZE) + GRID_START_X;
                let y_pos = (y as i32 * CELL_SIZE) + GRID_START_Y;
                
                // Draw cell background
                d.draw_rectangle(x_pos, y_pos, CELL_SIZE, CELL_SIZE, Color::WHITE);
                
                // Draw cell border
                d.draw_rectangle_lines(x_pos, y_pos, CELL_SIZE, CELL_SIZE, Color::GRAY);
                
                // Draw cell content
                d.draw_rectangle(x_pos + BORDER_SIZE, y_pos + BORDER_SIZE, CELL_SIZE - 2 * BORDER_SIZE, CELL_SIZE - 2 * BORDER_SIZE, color);
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 900)
        .title("raylib-tetris")
        .build();

    let tetris_grid = TetrisGrid::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);
        d.draw_text("Tetris", 12, 12, 20, Color::BLACK);
        tetris_grid.draw(&mut d);
    }
}