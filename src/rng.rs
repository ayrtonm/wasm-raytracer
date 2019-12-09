extern crate rand;

use std::cell::RefCell;
use rand::Rng;
use rand::os::OsRng;

thread_local!(pub static GENERATOR: RefCell<rand::OsRng> = RefCell::new(OsRng::new().unwrap()));

pub fn randf64() -> f64 {
    let ret: f64 = GENERATOR.with(|rng| {
      rng.borrow_mut().gen()
    });
    ret
}

pub fn randu8() -> u8 {
    let ret: u8 = GENERATOR.with(|rng| {
      rng.borrow_mut().gen()
    });
    ret
}

pub fn randbool() -> bool {
    let ret: bool = GENERATOR.with(|rng| {
      rng.borrow_mut().gen()
    });
    ret
}
