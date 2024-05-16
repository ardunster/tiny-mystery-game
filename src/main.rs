use tiny_mystery_game::names::{get_first_name, get_surname};
use tiny_mystery_game::rng::{calculate_hash, coin_flip};
use tiny_mystery_game::villagers::Gender;


fn main() {
    let stringy_seed = "some_seedz";

    for position in 0..3 {
        let seed_with_pos = format!("{stringy_seed}{}", position.to_string());

        println!("seed with position: {}", seed_with_pos);
        let hash = calculate_hash(&seed_with_pos);

        let gender = match coin_flip(hash) {
            true => Gender::Male,
            false => Gender::Female,
        };

        println!("Got a name: {} {}", get_first_name(hash, gender), get_surname(hash));
    }
}
