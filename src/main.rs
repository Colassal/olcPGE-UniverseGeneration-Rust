extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Duration as ChronoDuration;
//use rand::prelude::*;
//use rand_chacha::ChaCha8Rng;

//fn lehmer_32(nlehmer: u32) -> u32 {
//  let mut new_lehmer = nlehmer;
//  new_lehmer += 0xe120fc15;
//  let mut tmp: u64;
//  tmp = new_lehmer as u64 * 0x4a39b70d;
//  let mut m1: u32 = ((tmp >> 32) ^ tmp) as u32;
//  tmp = m1 as u64 * 0x12fad5c9;
//  let mut m2: u32 = ((tmp >> 32) & tmp) as u32;
//  return m2;
//}

trait Lehmer32 {
  fn lehmer_32(&mut self) -> u32;
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
}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {      // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {      // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

    //olc::clear(olc::BLACK); // Clears the screen and sets the blanking color to black
    //olc::draw_string(40, 40, "Hello, World!", olc::WHITE)?; // Prints the string starting at the position (40, 40) and using white colour.

    if olc::get_key(olc::Key::SPACE).released {
        olc::clear(olc::BLACK);  

        let tp1 = SystemTime::now();

        for x in 0..olc::screen_width(){
            for y in 0..olc::screen_height(){
                let mut b_is_star: bool = false;
                let n_seed: u32 = (y << 16) as u32 | x as u32;
                /*
                olc::c_srand(n_seed);
                b_is_star = olc::c_rand() % 256 < 128;
                */

                //let mut rng = ChaCha8Rng::seed_from_u64(n_seed.into());
                //b_is_star = rng.gen_range(0..256) < 32;
                
                self.n_lehmer = n_seed;
                b_is_star = self.lehmer_32() % 256 < 32;


                olc::draw(x, y, if b_is_star {olc::WHITE} else {olc::BLACK});
            }
        }


        let tp2 = SystemTime::now();
        let std_tp1 = tp1.duration_since(UNIX_EPOCH).unwrap();
        let std_tp2 = tp2.duration_since(UNIX_EPOCH).unwrap();

        let chrono_tp1 = ChronoDuration::from_std(std_tp1).unwrap();
        let chrono_tp2 = ChronoDuration::from_std(std_tp2).unwrap();

        let elapsed_time = chrono_tp2 - chrono_tp1;
        olc::fill_rect(0, 0, olc::screen_width(), 18, olc::Pixel { r: 0, g: 0, b: 100, a: 255 });
        olc::draw_string(5, 5, &format!("{:?}", elapsed_time), olc::RED)?;
        
    }

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