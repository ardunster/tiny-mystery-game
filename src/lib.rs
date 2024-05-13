mod names;
mod profession;

pub fn get_male_name() -> String {
    names::MALE_FIRST_NAMES[0].to_string()
}
