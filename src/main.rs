use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, render::Canvas};
use std::thread::sleep;
use std::time::Duration;
use std::usize;

const GRID_WIDTH: i64 = 100;
const GRID_HEIGHT: i64 = 25;

const GAP: u32 = 10;
const WINDOW_WIDTH: u32 = 960;
const ASPECT_RATIO: f32 = 9.0 / 16.0;

#[derive(Clone, PartialEq)]
enum CELL {
	Alive,
	Dead,
}

fn idx2dto1d(x: i64, y: i64) -> usize {
	return (x + GRID_WIDTH * y) as usize;
}

fn get_neigh_count(grid: &Vec<CELL>, x: i64, y: i64) -> i64 {
	static OFFSETS: [(i64, i64); 8] =
		[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	let mut alive_neigh = 0;
	for p in &OFFSETS {
		let mut nx = p.0 + x;
		let mut ny = p.1 + y;

		if nx < 0 || nx > GRID_WIDTH {
			nx = nx.rem_euclid(GRID_WIDTH);
		}

		if ny < 0 || ny > GRID_HEIGHT {
			ny = ny.rem_euclid(GRID_HEIGHT);
		}

		if idx2dto1d(nx, ny) >= 2500 {
			continue;
		}

		match grid[idx2dto1d(nx, ny) as usize] {
			CELL::Dead => alive_neigh += 0,
			CELL::Alive => alive_neigh += 1,
		}
	}
	return alive_neigh;
}

fn game_cyle(grid: &Vec<CELL>, canvas: &mut Canvas<sdl2::video::Window>) -> Vec<CELL> {
	let quad_width: u32 = (WINDOW_WIDTH / GRID_WIDTH as u32) as u32;
	let quad_height: u32 =
		((WINDOW_WIDTH as f32 * ASPECT_RATIO) as u32 / GRID_HEIGHT as u32) as u32;

	let mut new_grid: Vec<CELL> = vec![CELL::Dead; (GRID_WIDTH * GRID_HEIGHT) as usize];

	for y in 0..GRID_HEIGHT {
		for x in 0..GRID_WIDTH {
			let neigh = get_neigh_count(&grid, x, y);
			match grid[idx2dto1d(x, y)] {
				CELL::Alive => {
					if neigh < 2 {
						new_grid[idx2dto1d(x, y)] = CELL::Dead;
					} else if neigh == 2 || neigh == 3 {
						new_grid[idx2dto1d(x, y)] = CELL::Alive;
					} else if neigh >= 3 {
						new_grid[idx2dto1d(x, y)] = CELL::Dead;
					}
				}
				CELL::Dead => {
					if neigh == 3 {
						new_grid[idx2dto1d(x, y)] = CELL::Alive;
					}
				}
			}

			if new_grid[idx2dto1d(x, y)] == CELL::Alive {
				canvas.set_draw_color(Color::RGB(255, 255, 255));
			} else {
				canvas.set_draw_color(Color::RGB(0, 0, 0));
			}
			let rectangle = sdl2::rect::Rect::new(
				((x as i32) * quad_width as i32) + (GAP as i32 * 3) as i32,
				((y as i32) * quad_height as i32) + (GAP as i32 * 3) as i32,
				quad_width,
				quad_height,
			);
			let _ = canvas.fill_rect(rectangle);
		}
	}
	return new_grid;
}

fn main() {
	let mut grid: Vec<CELL> = vec![CELL::Dead; (GRID_WIDTH * GRID_HEIGHT) as usize];

	// Oscillator
	grid[idx2dto1d(10, 10)] = CELL::Alive;
	grid[idx2dto1d(11, 10)] = CELL::Alive;
	grid[idx2dto1d(12, 10)] = CELL::Alive;
	grid[idx2dto1d(11, 11)] = CELL::Alive;
	grid[idx2dto1d(12, 11)] = CELL::Alive;
	grid[idx2dto1d(13, 11)] = CELL::Alive;

	// Glider
	grid[idx2dto1d(1, 1)] = CELL::Alive;
	grid[idx2dto1d(2, 2)] = CELL::Alive;
	grid[idx2dto1d(2, 3)] = CELL::Alive;
	grid[idx2dto1d(1, 3)] = CELL::Alive;
	grid[idx2dto1d(0, 3)] = CELL::Alive;

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
	'running: loop {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
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

		grid = game_cyle(&grid, &mut canvas);

		canvas.present();
		sleep(Duration::from_millis(500));
	}
}
