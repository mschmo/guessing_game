extern crate rand;

use self::rand::Rng;


pub fn get_random_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max + 1)
}
