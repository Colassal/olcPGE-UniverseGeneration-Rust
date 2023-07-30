extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;
//use std::time::{SystemTime, UNIX_EPOCH};
//use chrono::Duration as ChronoDuration;

const G_STAR_COLOURS: [u32; 8] = [
  0xFFFFFFFF, 0xFFD9FFFF, 0xFFA3FFFF, 0xFFFFC8C8,
  0xFFFFCB9D, 0xFF9F9FFF, 0xFF415EFF, 0xFF28199D
];

struct StarSystem{
  x: u32,
  y: u32,
  n_lehmer: u32,
  star_exists: bool,
  star_diameter: f64,
  star_color: olc::Pixel,
}

impl StarSystem{
  fn new(x: u32, y: u32) -> Self{
    Self{
      x: x,
      y: y,
      n_lehmer: (x & 0xFFFF) << 16 | (y & 0xFFFF),
      star_exists: false,
      star_diameter: 0.0,
      star_color: olc::WHITE,
    }
  }
}

trait Lehmer32 {
  fn lehmer_32(&mut self) -> u32;
  fn rnd_int(&mut self, max: i32, min: i32) -> i32;
  fn rnd_double(&mut self, max: f64, min: f64) -> f64;
}

struct ExampleProgram {
    debug_bool: bool,
    n_lehmer: u32,
}

impl Lehmer32 for ExampleProgram {
    fn lehmer_32(&mut self) -> u32 {
      self.n_lehmer += 0xe120fc15;
      let mut tmp: u64;
      tmp = self.n_lehmer as u64 * 0x4a39b70d;
      let mut m1: u32 = ((tmp >> 32) ^ tmp) as u32;
      tmp = m1 as u64 * 0x12fad5c9;
      let mut m2: u32 = ((tmp >> 32) & tmp) as u32;
      return m2;
    }
    fn rnd_int(&mut self, max: i32, min: i32) -> i32 {
     return (self.lehmer_32() as i32 % (max - min)) + min;   
    }
    fn rnd_double(&mut self, max: f64, min: f64) -> f64 {
        return (self.lehmer_32() as f64 / (0x7FFFFFFF as f64)) * (max - min) + min;
    }
}

impl Lehmer32 for StarSystem {
  fn lehmer_32(&mut self) -> u32 {
    self.n_lehmer += 0xe120fc15;
    let mut tmp: u64;
    tmp = self.n_lehmer as u64 * 0x4a39b70d;
    let mut m1: u32 = ((tmp >> 32) ^ tmp) as u32;
    tmp = m1 as u64 * 0x12fad5c9;
    let mut m2: u32 = ((tmp >> 32) & tmp) as u32;
    return m2;
  }
  fn rnd_int(&mut self, max: i32, min: i32) -> i32 {
      return (self.lehmer_32() as i32 % (max - min)) + min;
  }
  fn rnd_double(&mut self, max: f64, min: f64) -> f64 {
      return (self.lehmer_32() as f64 / (0x7FFFFFFF as f64)) * (max - min) + min;
  }
}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {      // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {      // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

    // let test_star_system: StarSystem = StarSystem::new(0, 0); // This is just a test to make sure we can create a star system inside the loop
    if olc::get_key(olc::Key::H).pressed{
        if self.debug_bool{
            self.debug_bool = false;
        } else {
            self.debug_bool = true;
        }
    }   

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> { // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }
}

fn main() {
  let mut example = ExampleProgram {
    debug_bool: false,
    n_lehmer: 0,
  };

  let screen_vars = [500, 480, 2, 2];
  olc::start("olcGalaxy", &mut example, screen_vars[0], screen_vars[1], screen_vars[2], screen_vars[3]).unwrap(); // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square, and starts the main game loop.
}