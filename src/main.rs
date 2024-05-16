use tiny_mystery_game::names::get_first_name;
use tiny_mystery_game::rng::{calculate_hash, coin_flip};
use tiny_mystery_game::villagers::Gender;


fn main() {
    let stringy_seed = "some_seedz".to_string();

    let hash = calculate_hash(&stringy_seed);

    let gender = match coin_flip(hash) {
        true => Gender::Male,
        false => Gender::Female,
    };

    println!("Got a name: {}", get_first_name(hash, gender));
}
