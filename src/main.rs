use launchpad::{LaunchpadMk2, Event, Location, RGBColor};
use std::time::Duration;
use std::thread;
use std::io::{Write, stdin, Read, BufRead, stdout};
use std::str::FromStr;

const RACK_SIZE: usize = 16;

fn main() {
	let launchpad = LaunchpadMk2::autodetect();

	let mut reader = stdin();
	let mut writer = stdout();

	let mut prompter = Prompter {
		reader: &mut reader.lock(),
		writer: &mut writer.lock(),
	};

	let evt = FuckEffect::prompt_init(&mut prompter);
	println!("{},{}", evt.center.0, evt.center.1);
}

pub struct Prompter<'a> {
	reader: &'a mut dyn BufRead,
	writer: &'a mut dyn Write,
}

impl<'a> Prompter<'a> {
	pub fn get_value<T: FromStr>(&mut self, prompt: &str, type_hint: &str) -> std::io::Result<T> {
		self.get_value_validated(prompt, type_hint, |_| true)
	}

	pub fn get_value_validated<T, F>(&mut self, prompt: &str, type_hint: &str, validator: F) -> std::io::Result<T>
		where T: FromStr, F: Fn(&T) -> bool {
		loop {
			write!(self.writer, "{} ({}): ", prompt, type_hint)?;
			self.writer.flush();

			let mut buffer = String::new();
			self.reader.read_line(&mut buffer);

			let parsed = buffer.trim().parse::<T>();

			if let Ok(value) = parsed {
				if validator(&value) {
					return Ok(value);
				}
			}
		}
	}
}

pub struct LaunchpadController {

}

pub trait Effect {
	fn prompt_init(prompter: &mut Prompter) -> Self;
	fn play(&self, controller: &mut LaunchpadController);
	fn stop(&self, controller: &mut LaunchpadController);
}

struct FuckEffect {
	center: (u8, u8),
}

impl Effect for FuckEffect {
	fn prompt_init(prompter: &mut Prompter) -> Self {
		let bounds_validator = |v: &u8| v < &8;
		let x = prompter.get_value_validated("center.x", "0-7", bounds_validator).unwrap();
		let y = prompter.get_value_validated("center.y", "0-7", bounds_validator).unwrap();

		Self {
			center: (x, y),
		}
	}

	fn play(&self, controller: &mut LaunchpadController) {

	}

	fn stop(&self, controller: &mut LaunchpadController) {

	}
}