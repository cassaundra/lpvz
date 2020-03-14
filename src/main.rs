use launchpad::*;

use std::ops::Neg;
use std::thread;
use std::time::Duration;

mod util;
use util::*;

const TARGET_FPS: f32 = 30.0;

pub type LaunchpadBuffer = [[Color; 8]; 8];

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut launchpad = MidiLaunchpadMk2::autodetect()?;

    launchpad.light_all(0)?;

    let mut color_selector = ColorSelector::new();
    color_selector.render(&mut launchpad)?;

    loop {
        for event in launchpad.poll() {
            color_selector.handle_event(&event);
        }

        color_selector.render(&mut launchpad)?;

        std::thread::sleep(Duration::from_millis(1));
    }

    std::thread::sleep(Duration::from_millis(3000));

    // do_intro(&mut launchpad);

    Ok(())
}

pub struct Color {
    hue: f32,
    saturation: f32,
    value: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            hue: 0.,
            saturation: 1.,
            value: 1.,
        }
    }
}

impl Color {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Color {
            hue,
            saturation,
            value,
        }
    }
}

pub struct ColorSelector {
    sat: u8,
    val: u8,
}

impl ColorSelector {
    pub fn new() -> Self {
        ColorSelector {
            sat: 7,
            val: 7,
        }
    }

    pub fn render(&self, launchpad: &mut impl LaunchpadMk2) -> launchpad::Result<()> {
        for (x, y, hue) in Self::hue_ring() {
            let c = RGBColor::from_hsv(hue, self.sat(), self.val());
            launchpad.light_single_rgb(pad_position(x as u8, y as u8), c.0, c.1, c.2)?;

            launchpad.light_single_rgb(button_position(self.sat, true), 20, 20, 20)?;
            launchpad.light_single_rgb(button_position(self.val, false), 20, 20, 20)?;
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Mk2Event) {
        use Location::*;
        use Mk2Event::*;

        match event {
            Mk2Event::Press(Location::Button(i, is_top)) => {
                if *is_top {
                    self.sat = *i;
                } else {
                    self.val = *i;
                }
            }
            _ => {}
        }
    }

    fn sat(&self) -> f32 {
        (self.sat as f32 / 7.).powf(0.5)
    }

    fn val(&self) -> f32 {
        self.val as f32 / 7.
    }

    fn hue_ring() -> Vec<(u8, u8, f32)> {
        ring(6)
            .into_iter()
            .enumerate()
            .map(|(i, (x, y))| (x + 1, ((y as i8).neg() + 6) as u8, i as f32 / 20.))
            .collect()
    }
}

fn do_intro(lp: &mut impl LaunchpadMk2) -> launchpad::Result<()> {
    const DURATION: f32 = 3.;
    const ANGLE: f32 = std::f32::consts::PI / 4.; // = 30 deg
    const WAVE_WIDTH: f32 = 1.0;
    const CELL_SIZE: u8 = 2;

    const GRID_SIZE: u8 = CELL_SIZE / 8;
    const FRAME_DELAY: f32 = 1. / (DURATION * TARGET_FPS);

    lp.light_all(0)?;

    let mut t = 0.;

    while t < DURATION {
        let r = t / DURATION * 2f32.sqrt() * 8.;

        for pad_y in 0..8 {
            for pad_x in 0..8 {
                let x = pad_x / CELL_SIZE;
                let y = pad_y / CELL_SIZE;

                let x = x as f32 + CELL_SIZE as f32 / 2.;
                let y = y as f32 + CELL_SIZE as f32 / 2.;

                // line: y = -cot(theta) * x + r * csc(theta)

                let line_value = -1. * ANGLE.tan().recip() * x + r * ANGLE.sin().recip();

                // distance = |cot(theta)x_0 + y_0 - r * csc(theta)| / |csc(theta)|
                let t = (ANGLE.tan().recip() * x + y - r * ANGLE.sin().recip()).abs()
                    / ANGLE.sin().recip().abs();

                // let a = -1. * (2. * t / DURATION).powi(2) + 1.;
                // let a = a.max(0.);
                let a = (1. * t).cosh().recip().powf(2.);
                let a = (a * 63.) as u8;

                lp.light_single_rgb(pad_position(pad_x, pad_y), a, a, a)?;
            }
        }

        t += 1. / TARGET_FPS;
        thread::sleep(Duration::from_secs_f32(TARGET_FPS.recip()));
    }

    lp.light_all(0)?;

    Ok(())
}
