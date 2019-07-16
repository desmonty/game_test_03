extern crate cgmath;
extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;


const FPS: f64 = 60.0;

const LEFT: keyboard::KeyCode = keyboard::KeyCode::Q;
const RIGHT: keyboard::KeyCode = keyboard::KeyCode::D;
const UP: keyboard::KeyCode = keyboard::KeyCode::Z;
const DOWN: keyboard::KeyCode = keyboard::KeyCode::S;


struct State {
	dt: std::time::Duration,
	text: graphics::Text,
	font: ggez::graphics::Font,
	time_since_last_update: f64,
	position_x: f32,
	position_y: f32,
}

impl State {
	fn new(ctx: &mut Context) -> GameResult<State> {
		let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
		let text = graphics::Text::new(("O", font, 48.0));
		let time_since_last_update = 0.0f64;

		let s = State {
			dt: std::time::Duration::new(0, 0),
			text,
			font,
			time_since_last_update,
			position_x: 0.5,
			position_y: -0.5,
		};

		Ok(s)
	}
}

impl ggez::event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.dt = timer::delta(ctx);
		let dt_millisec: f64 = self.dt.subsec_nanos() as f64 / 1_000_000.0f64;
		self.time_since_last_update += dt_millisec;

		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult {

		if self.time_since_last_update > 1000.0f64 / FPS {
			graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

			let dest_point = cgmath::Point2::new(self.position_x, self.position_y);
			graphics::draw(ctx, &self.text, (dest_point,))?;
			graphics::present(ctx)?;

			self.time_since_last_update = 0.0f64;
		}

		Ok(())
	}
	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		keycode: keyboard::KeyCode,
		_keymods: keyboard::KeyMods,
		_repeat: bool
	) {
		match keycode {
			LEFT => {
				self.position_x -= 5.0;
				println!("Left key pressed");
			},
			RIGHT => {
				self.position_x += 5.0;
				println!("Right key pressed");
			},
			UP => {
				self.position_y -= 5.0;
				println!("Up key pressed");
			},
			DOWN => {
				self.position_y += 5.0;
				println!("Down key pressed");
			},
			_ => (),
		}	
	}
}


fn main() {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		path
	} else {
		path::PathBuf::from("./resources")
	};

	let cb = ggez::ContextBuilder::new("hello_ggez", "me")
		.add_resource_path(resource_dir);

	let (ctx, event_loop) = &mut cb.build().unwrap();


	let state = &mut State::new(ctx).unwrap();

	event::run(ctx, event_loop, state).unwrap();
}
