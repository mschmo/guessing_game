extern crate guessing_game;

use guessing_game::get_random_number;


#[test]
fn test_get_random_number() {
    let random_number = test_get_random_number(1, 100);
    assert!(random_number >= 1 && random_number <= 100);
}
