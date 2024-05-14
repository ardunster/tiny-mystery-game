use tiny_mystery_game::names::get_first_name;
use tiny_mystery_game::villagers::Gender;

fn main() {
    println!("Got a name: {}", get_first_name(12345, Gender::Female));
}
