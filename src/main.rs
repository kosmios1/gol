use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, render::Canvas};

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

// const GRID_WIDTH: i64 = 30;
// const GRID_HEIGHT: i64 = 30;

const GAP: u32 = 10;
const WINDOW_WIDTH: u32 = 960;
const ASPECT_RATIO: f32 = 9.0 / 16.0;

// Oscillator
// grid[idx2dto1d(10, 10)] = CELL::Alive;
// grid[idx2dto1d(11, 10)] = CELL::Alive;
// grid[idx2dto1d(12, 10)] = CELL::Alive;
// grid[idx2dto1d(11, 11)] = CELL::Alive;
// grid[idx2dto1d(12, 11)] = CELL::Alive;
// grid[idx2dto1d(13, 11)] = CELL::Alive;

// Glider
// grid[idx2dto1d(1, 1)] = CELL::Alive;
// grid[idx2dto1d(2, 2)] = CELL::Alive;
// grid[idx2dto1d(2, 3)] = CELL::Alive;
// grid[idx2dto1d(1, 3)] = CELL::Alive;
// grid[idx2dto1d(0, 3)] = CELL::Alive;

// Penta-decathlon
// grid[idx2dto1d(15, 0)] = CELL::Alive;
// grid[idx2dto1d(15, 1)] = CELL::Alive;
// grid[idx2dto1d(17, 0)] = CELL::Alive;
// grid[idx2dto1d(16, 1)] = CELL::Alive;
// grid[idx2dto1d(16, 2)] = CELL::Alive;

// grid[idx2dto1d(22, 2)] = CELL::Alive;
// grid[idx2dto1d(22, 3)] = CELL::Alive;
// grid[idx2dto1d(22, 4)] = CELL::Alive;
// grid[idx2dto1d(23, 4)] = CELL::Alive;
// grid[idx2dto1d(24, 3)] = CELL::Alive;

// grid[idx2dto1d(7, 15)] = CELL::Alive;
// grid[idx2dto1d(8, 15)] = CELL::Alive;
// grid[idx2dto1d(9, 15)] = CELL::Alive;
// grid[idx2dto1d(9, 16)] = CELL::Alive;
// grid[idx2dto1d(8, 17)] = CELL::Alive;

#[derive(Clone, PartialEq)]
enum CellType {
	Alive,
	Dead,
}

struct Cell {
	pos: (i64, i64),
	cell_type: CellType,
}

impl Cell {
	fn new(pos: (i64, i64), cell_type: CellType) -> Cell {
		Cell { pos, cell_type }
	}
}

struct Grid {
	grid: HashMap<(i64, i64), Cell>,
}

impl Grid {
	pub fn new() -> Grid {
		Grid {
			grid: HashMap::new(),
		}
	}

	pub fn add_cell(&mut self, pos: (i64, i64), typ: CellType) {
		self.grid.insert(pos, Cell::new(pos, typ));
	}

	fn get_neigh_count(&self, x: i64, y: i64) -> i64 {
		static OFFSETS: [(i64, i64); 8] =
			[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

		let mut alive_neigh = 0;
		for p in &OFFSETS {
			let nx = match p.0.checked_add(x) {
				Some(px) => px,
				None => 0,
			};
			let ny = match p.1.checked_add(y) {
				Some(py) => py,
				None => 0,
			};

			if self.grid.contains_key(&(nx, ny)) {
				alive_neigh += 1;
			}
		}
		return alive_neigh;
	}
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window(
			"game_of_life",
			WINDOW_WIDTH,
			(WINDOW_WIDTH as f32 * ASPECT_RATIO) as u32,
		)
		.position_centered()
		.borderless()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut grid = Grid::new();
	grid.add_cell((1, 1), CellType::Alive);
	grid.add_cell((2, 2), CellType::Alive);
	grid.add_cell((2, 3), CellType::Alive);
	grid.add_cell((1, 3), CellType::Alive);
	grid.add_cell((0, 3), CellType::Alive);

	'running: loop {
		canvas.set_draw_color(Color::RGB(100, 100, 100));
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}

		game_cyle(&mut grid, &mut canvas);

		canvas.present();
		sleep(Duration::from_millis(500));
	}
}
const QUAD_WIDTH: u32 = 20;
const QUAD_HEIGHT: u32 = 20;

fn game_cyle(grid: &mut Grid, canvas: &mut Canvas<sdl2::video::Window>) {
	let mut new_grid = Grid::new();

	for (k, v) in &grid.grid {
		let neigh = grid.get_neigh_count(k.0, k.1);
		match v.cell_type {
			CellType::Alive => {
				if neigh < 2 {
					new_grid.add_cell(*k, CellType::Dead);
				} else if neigh == 2 || neigh == 3 {
					new_grid.add_cell(*k, CellType::Alive);
				} else if neigh >= 3 {
					new_grid.add_cell(*k, CellType::Dead);
				}
			}
			CellType::Dead => {
				if neigh == 3 {
					new_grid.add_cell(*k, CellType::Alive);
				}
			}
		}

		match new_grid.grid.get(&k) {
			Some(c) => {
				if c.cell_type == CellType::Alive {
					canvas.set_draw_color(Color::RGB(255, 255, 255));
				} else {
					canvas.set_draw_color(Color::RGB(0, 0, 0));
				}
			}
			None => {}
		}

		let rectangle = sdl2::rect::Rect::new(
			k.0 as i32 * (1 + QUAD_WIDTH as i32) + GAP as i32,
			k.1 as i32 * (1 + QUAD_HEIGHT as i32) + GAP as i32,
			QUAD_WIDTH,
			QUAD_HEIGHT,
		);
		let _ = canvas.fill_rect(rectangle);
	}

	*grid = new_grid;
}
