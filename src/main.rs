use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, render::Canvas};

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

const GAP: u32 = 1;
const OUTER_GAP: u32 = 10;
const WINDOW_WIDTH: u32 = 1280;
const ASPECT_RATIO: f32 = 9.0 / 16.0;

const QUAD_SIZE: u32 = 14;

#[derive(Clone, PartialEq)]
enum CellType {
	Alive,
	Dead,
}

struct Grid {
	grid: HashMap<(i64, i64), CellType>,
}

impl Grid {
	pub fn new() -> Grid {
		Grid {
			grid: HashMap::new(),
		}
	}

	pub fn add_cell(&mut self, pos: (i64, i64), typ: CellType) {
		self.grid.insert(pos, typ);
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

	fn add_penta_decathlon(&mut self, x: i64, y: i64) {
		self.add_cell((x + 15, y + 0), CellType::Alive);
		self.add_cell((x + 15, y + 1), CellType::Alive);
		self.add_cell((x + 17, y + 0), CellType::Alive);
		self.add_cell((x + 16, y + 1), CellType::Alive);
		self.add_cell((x + 16, y + 2), CellType::Alive);

		self.add_cell((x + 22, y + 2), CellType::Alive);
		self.add_cell((x + 22, y + 3), CellType::Alive);
		self.add_cell((x + 22, y + 4), CellType::Alive);
		self.add_cell((x + 23, y + 4), CellType::Alive);
		self.add_cell((x + 24, y + 3), CellType::Alive);

		self.add_cell((x + 7, y + 15), CellType::Alive);
		self.add_cell((x + 8, y + 15), CellType::Alive);
		self.add_cell((x + 9, y + 15), CellType::Alive);
		self.add_cell((x + 9, y + 16), CellType::Alive);
		self.add_cell((x + 8, y + 17), CellType::Alive);
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
	grid.add_penta_decathlon(10, 10);

	let mut halt = true;
	'running: loop {
		canvas.set_draw_color(Color::RGB(150, 150, 150));
		canvas.clear();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				Event::MouseMotion {
					mousestate,
					x,
					y,
					..
				} => {
					if mousestate.left() {
						let vpx = (x - OUTER_GAP as i32) / (QUAD_SIZE + GAP) as i32;
						let vpy = (y - OUTER_GAP as i32) / (QUAD_SIZE + GAP) as i32;
						grid.add_cell((vpx as i64, vpy as i64), CellType::Alive);
					} else if mousestate.right() {
						let vpx = (x - OUTER_GAP as i32) / (QUAD_SIZE + GAP) as i32;
						let vpy = (y - OUTER_GAP as i32) / (QUAD_SIZE + GAP) as i32;
						grid.add_cell((vpx as i64, vpy as i64), CellType::Dead);
					}
				}
				Event::KeyDown { keycode, .. } => {
					if keycode == Some(Keycode::P) {
						halt = !halt;
					}
				}
				_ => {}
			}
		}

		let draw_surf = canvas.viewport();
		let vpwidth = (draw_surf.width() - OUTER_GAP) / (QUAD_SIZE + GAP);
		let vpheight = (draw_surf.height() - OUTER_GAP) / (QUAD_SIZE + GAP);

		game_cyle(&mut grid, &mut canvas, vpwidth, vpheight, 0, 0, halt);
		canvas.present();

		sleep(Duration::from_millis(50));
	}
}

fn game_cyle(
	grid: &mut Grid, canvas: &mut Canvas<sdl2::video::Window>, vpwidth: u32, vpheight: u32,
	off_x: i64, off_y: i64, halt: bool,
) {
	let mut new_grid = Grid::new();
	for vpy in 0..vpheight {
		for vpx in 0..vpwidth {
			let x: i64 = vpx as i64 + off_x;
			let y: i64 = vpy as i64 + off_y;

			let neigh = grid.get_neigh_count(x, y);
			let v = grid.grid.get(&(x, y)).unwrap_or(&CellType::Dead);
			if !halt {
				match v {
					CellType::Alive => {
						if neigh == 2 || neigh == 3 {
							new_grid.add_cell((x, y), CellType::Alive);
						}
					}
					CellType::Dead => {
						if neigh == 3 {
							new_grid.add_cell((x, y), CellType::Alive);
						}
					}
				}
			}
			if *v == CellType::Alive {
				canvas.set_draw_color(Color::RGB(255, 255, 255));
			} else {
				canvas.set_draw_color(Color::RGB(0, 0, 0));
			}

			let rectangle = sdl2::rect::Rect::new(
				vpx as i32 * (GAP + QUAD_SIZE) as i32 + OUTER_GAP as i32,
				vpy as i32 * (GAP + QUAD_SIZE) as i32 + OUTER_GAP as i32,
				QUAD_SIZE,
				QUAD_SIZE,
			);
			let _ = canvas.fill_rect(rectangle);
		}
	}
	if !halt {
		grid.grid = new_grid.grid;
	}
}
