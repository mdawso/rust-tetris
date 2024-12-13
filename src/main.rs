use rand::Rng;
use raylib::prelude::*;

const GRID_START_X: i32 = 10;
const GRID_START_Y: i32 = 10;
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: i32 = 30;
const BORDER_SIZE: i32 = 2;

const TETRIS_PIECES: [[[bool; 4]; 4]; 7] = [
    // I piece
    [
        [false, false, false, false],
        [true,  true,  true,  true ],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // O piece
    [
        [true,  true, false, false],
        [true,  true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // T piece
    [
        [false, true,  false, false],
        [true,  true,  true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // S piece
    [
        [false, true,  true, false],
        [true,  true,  false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // Z piece
    [
        [true,  true,  false, false],
        [false, true,  true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // J piece
    [
        [true,  false, false, false],
        [true,  true,  true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // L piece
    [
        [false, false, true, false],
        [true,  true,  true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
];



struct TetrisGrid {
    grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
}

impl TetrisGrid {
    fn new() -> Self {
        TetrisGrid {
            grid: [[false; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    // for debug
    fn random_fill(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
            self.grid[y][x] = rng.gen_bool(0.5);
            }
        }
    }

    fn add_piece(&mut self, piece: [[bool; 4]; 4], x: usize, y: usize) {
        for (i, row) in piece.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell {
                    let grid_x = x + j;
                    let grid_y = y + i;
                    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                        self.grid[grid_y][grid_x] = true;
                    }
                }
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = if self.grid[y][x] {
                    Color::BLACK
                } else {
                    Color::RED
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
    
    rl.set_target_fps(60);

    let mut tetris_grid = TetrisGrid::new();
    tetris_grid.add_piece(TETRIS_PIECES[rand::thread_rng().gen_range(0..TETRIS_PIECES.len())], 1, 1);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);
        d.draw_text("Tetris", 12, 12, 20, Color::BLACK);
        tetris_grid.draw(&mut d);
    }
}