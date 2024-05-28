use crate::rng::position_in_range;
use crate::villagers::Gender;
use log::trace;

pub const MALE_FIRST_NAMES: [&str; 61] = [
    "Norman",
    "Leroy",
    "William",
    "John",
    "Thomas",
    "Henry",
    "Richard",
    "Edward",
    "Arthur",
    "George",
    "Charles",
    "Benjamin",
    "Robert",
    "James",
    "David",
    "Michael",
    "Andrew",
    "Peter",
    "Paul",
    "Stephen",
    "Philip",
    "Simon",
    "Alan",
    "Brian",
    "Colin",
    "Daniel",
    "Eric",
    "Francis",
    "Gavin",
    "Harry",
    "Ian",
    "Keith",
    "Liam",
    "Martin",
    "Nigel",
    "Oliver",
    "Patrick",
    "Quentin",
    "Roger",
    "Sean",
    "Trevor",
    "Vincent",
    "Walter",
    "Xavier",
    "Yannick",
    "Zachary",
    "Lars",
    "Erik",
    "Johan",
    "Magnus",
    "Anders",
    "Henrik",
    "Gustav",
    "Oskar",
    "Sven",
    "Mikkel",
    "Francesco",
    "Giovanni",
    "Matteo",
    "Marco",
    "Lorenzo",
];

pub const FEMALE_FIRST_NAMES: [&str; 65] = [
    "Elizabeth",
    "Mary",
    "Anne",
    "Jane",
    "Zoe",
    "Margaret",
    "Catherine",
    "Caroline",
    "Sarah",
    "Sara",
    "Emma",
    "Alice",
    "Eleanor",
    "Grace",
    "Isabella",
    "Victoria",
    "Charlotte",
    "Sophia",
    "Lucy",
    "Rebecca",
    "Louise",
    "Rose",
    "Hannah",
    "Emily",
    "Julia",
    "Caroline",
    "Diana",
    "Amelia",
    "Melissa",
    "Melisa",
    "Evelyn",
    "Florence",
    "Harriet",
    "Irene",
    "Katherine",
    "Laura",
    "Mabel",
    "Nora",
    "Olive",
    "Penelope",
    "Rachel",
    "Samantha",
    "Tabitha",
    "Ursula",
    "Violet",
    "Winifred",
    "Xanthe",
    "Yvette",
    "Zara",
    "Ingrid",
    "Astrid",
    "Freya",
    "Greta",
    "Sigrid",
    "Ebba",
    "Linnea",
    "Hilda",
    "Solveig",
    "Malin",
    "Holga",
    "Isabella",
    "Sofia",
    "Giulia",
    "Caterina",
    "Lucia",
];

pub const SURNAMES: [&str; 336] = [
    "Smith",
    "Jones",
    "Williams",
    "Taylor",
    "Brown",
    "Davies",
    "Evans",
    "Wilson",
    "Thomas",
    "Roberts",
    "Johnson",
    "Lewis",
    "Walker",
    "White",
    "Edwards",
    "Green",
    "Hall",
    "Harris",
    "Clarke",
    "Jackson",
    "Wood",
    "Turner",
    "Wright",
    "Scott",
    "Adams",
    "Mitchell",
    "Campbell",
    "Parker",
    "Young",
    "King",
    "Baker",
    "Hill",
    "Phillips",
    "Lee",
    "Watson",
    "Cook",
    "Murphy",
    "Bell",
    "Carter",
    "Anderson",
    "Allen",
    "Ross",
    "Howard",
    "Ward",
    "Morgan",
    "Hayes",
    "Holmes",
    "Harrison",
    "Murray",
    "Graham",
    "Sullivan",
    "Porter",
    "Fisher",
    "Butler",
    "Simmons",
    "Foster",
    "Price",
    "Gray",
    "Knight",
    "Reid",
    "Gibson",
    "West",
    "Stewart",
    "Pearson",
    "Reynolds",
    "Fox",
    "Weber",
    "McDonald",
    "Douglas",
    "Marshall",
    "Lane",
    "Harper",
    "Ryan",
    "Spencer",
    "Gordon",
    "Holcomb",
    "Vaughn",
    "Hammond",
    "Warren",
    "Barrett",
    "Hoover",
    "Hobbs",
    "Norris",
    "Wyatt",
    "Curtis",
    "Bennett",
    "Stevens",
    "Horton",
    "Carlson",
    "Watkins",
    "O'Brien",
    "Barnes",
    "Wells",
    "Chapman",
    "Hartman",
    "Fuller",
    "Shaw",
    "Stokes",
    "Lambert",
    "Vargas",
    "Potter",
    "Haynes",
    "Howell",
    "Bowen",
    "Barlow",
    "Preston",
    "Wilkinson",
    "Hodges",
    "Baldwin",
    "Grimes",
    "Valdez",
    "Pierce",
    "Hess",
    "Garza",
    "Navarro",
    "Schmidt",
    "Gutierrez",
    "Yoder",
    "Chan",
    "Mendez",
    "Roth",
    "Bauer",
    "Han",
    "Holland",
    "Becker",
    "Floyd",
    "Santos",
    "Booth",
    "Keith",
    "Cardenas",
    "Walters",
    "Hatfield",
    "Cantu",
    "Huffman",
    "Leblanc",
    "Davila",
    "Vincent",
    "Hull",
    "Archer",
    "Trujillo",
    "Lang",
    "Duke",
    "Gillespie",
    "Buck",
    "Castillo",
    "Faulkner",
    "Drake",
    "Solomon",
    "Moyer",
    "Khan",
    "Rasmussen",
    "Mcguire",
    "Alvarado",
    "Huffman",
    "Frost",
    "Mccall",
    "Levy",
    "Mcmillan",
    "Rosales",
    "Patel",
    "Rice",
    "Hood",
    "Cortez",
    "Conway",
    "Dominguez",
    "Golden",
    "Calhoun",
    "Hess",
    "Blackburn",
    "Conrad",
    "Mcneil",
    "Nicholson",
    "Wiggins",
    "Padilla",
    "Stout",
    "Vega",
    "Whitaker",
    "Baird",
    "Mcintyre",
    "Rosario",
    "Cantu",
    "Bradford",
    "Valencia",
    "Oneal",
    "Delgado",
    "Stout",
    "Nolan",
    "Park",
    "Gilmore",
    "Blair",
    "Leblanc",
    "Estes",
    "Parrish",
    "Beasley",
    "Lara",
    "Fritz",
    "Macdonald",
    "Proctor",
    "Morris",
    "Sheppard",
    "Barajas",
    "Eaton",
    "Mcpherson",
    "Carrillo",
    "Carney",
    "Beasley",
    "Faulkner",
    "Gallegos",
    "Underwood",
    "Hanna",
    "Conley",
    "Holt",
    "Ayers",
    "Whitley",
    "Nieves",
    "Montes",
    "Carson",
    "Benson",
    "Hensley",
    "Mccarthy",
    "Nguyen",
    "Whitehead",
    "Robles",
    "Pena",
    "Calderon",
    "Ayers",
    "Gaines",
    "Robb",
    "Mann",
    "Holden",
    "Nash",
    "Nolan",
    "Duke",
    "Cantu",
    "Montgomery",
    "Preston",
    "Friedman",
    "Schaefer",
    "Dunlap",
    "Dickson",
    "Giles",
    "Langley",
    "Franco",
    "Phelps",
    "Pruitt",
    "Beck",
    "Lester",
    "Mann",
    "Briggs",
    "Sexton",
    "Brennan",
    "Vincent",
    "Davies",
    "Parsons",
    "Garcia",
    "Nieves",
    "Gibbs",
    "Carrillo",
    "Goodwin",
    "Curtis",
    "Wade",
    "Hess",
    "Fletcher",
    "Mccall",
    "Mccullough",
    "Lutz",
    "Kent",
    "Fischer",
    "Cohen",
    "Morton",
    "Pollard",
    "Clayton",
    "Rosales",
    "Parks",
    "Landry",
    "Randall",
    "Skinner",
    "Mckenzie",
    "Jennings",
    "Cameron",
    "Montgomery",
    "Colon",
    "Gallegos",
    "Hutchinson",
    "Sherman",
    "Cantu",
    "Calderon",
    "Wallace",
    "Sellers",
    "Bass",
    "Norton",
    "Whitney",
    "Roy",
    "Madden",
    "van den Berg",
    "Fleming",
    "Bowers",
    "Mcmahon",
    "Church",
    "Kaufman",
    "Giles",
    "Dudley",
    "Randolph",
    "Case",
    "Cabrera",
    "Osborne",
    "Small",
    "Wiggins",
    "Rosales",
    "Hess",
    "Gallegos",
    "Bender",
    "Rosario",
    "Sexton",
    "Ballard",
    "Björnsson",
    "Pollard",
    "Carrillo",
    "Vandroogenbröck",
    "Vaughan",
    "Serrano",
    "Terrell",
    "Fuentes",
    "Wang",
    "Zimmerman",
    "Larsen",
    "Johansen",
    "Andersen",
    "Pedersen",
    "Olsen",
    "Eriksen",
    "Hansen",
    "Sørensen",
    "Nielsen",
    "Christensen",
    "von Harten",
];

pub fn get_first_name(hash: &u64, gender: &Gender) -> String {
    match gender {
        Gender::Male => {
            trace!(target: "Names: Male", "Getting male name...");
            let position = position_in_range(&(MALE_FIRST_NAMES.len() as u64), hash);
            trace!(target: "Names: Male", "Calculated Position {} from hash {}", position, hash);
            MALE_FIRST_NAMES[position as usize].to_string()
        }
        Gender::Female => {
            trace!(target: "Names: Female", "Getting female name...");
            let position = position_in_range(&(FEMALE_FIRST_NAMES.len() as u64), hash);
            trace!(target: "Names: Female", "Calculated Position {} from hash {}", position, hash);
            FEMALE_FIRST_NAMES[position as usize].to_string()
        }
    }
}

pub fn get_surname(hash: &u64) -> String {
    trace!(target: "Names: Surname", "Getting surname...");
    let position = position_in_range(&(SURNAMES.len() as u64), hash);
    trace!(target: "Names: Surname", "Calculated Position {} from hash {}", position, hash);
    SURNAMES[position as usize].to_string()
}
