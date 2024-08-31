use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

// Define possible directions for navigation
pub const DIRECTIONS: &[&str] = &["north", "south", "east", "west"];

// Game state structure
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    pub current_location: &'static str,
    pub inventory: HashSet<&'static str>,
    pub found_items: HashSet<&'static str>,
}

// Locations and their descriptions
pub fn get_locations() -> HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>
{
    let mut locations = HashMap::new();

    locations.insert(
        "Garden",
        (
            "🌷 You are in a lush garden with colorful flowers.",
            HashMap::from([("north", "Pond"), ("east", "Rocky Path")]),
        ),
    );

    locations.insert(
        "Pond",
        (
            "🐟 You are at a peaceful pond with lilypads floating on the surface.",
            HashMap::from([("south", "Garden"), ("east", "Forest")]),
        ),
    );

    locations.insert(
        "Rocky Path",
        (
            "🪨 You are on a rocky path. Watch your step!",
            HashMap::from([("west", "Garden"), ("north", "Forest")]),
        ),
    );

    locations.insert(
        "Forest",
        (
            "🌲 You are in a dense forest with tall trees surrounding you.",
            HashMap::from([("west", "Pond"), ("south", "Rocky Path")]),
        ),
    );

    locations
}

// Possible items to find in the game
pub fn get_items() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("Garden", "Cookie Crumb 🍪"),
        ("Pond", "Isopod Friend 🐾"),
        ("Forest", "A Place to Hide 🛏️"),
    ])
}

// Display help message
pub fn display_help() {
    println!(
        "Commands:
- go [direction]: Move in a direction (north, south, east, west)
- look: Look around the current location
- inventory: Show your inventory
- help: Show this help message
- quit: Exit the game"
    );
}

// Display the current inventory
pub fn display_inventory(inventory: &HashSet<&str>) {
    if inventory.is_empty() {
        println!("Your inventory is empty.");
    } else {
        println!("You have: {:?}", inventory);
    }
}

// Move to a new location based on direction
pub fn move_to(
    game_state: &mut GameState,
    direction: &str,
    locations: &HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>,
) {
    if let Some((_, exits)) = locations.get(game_state.current_location) {
        if let Some(new_location) = exits.get(direction) {
            game_state.current_location = new_location;
            println!("You move {} to the {}.", direction, new_location);
            look_around(game_state, locations, &get_items());
        } else {
            println!("You can't go {} from here.", direction);
        }
    }
}

// Look around the current location
pub fn look_around(
    game_state: &mut GameState,
    locations: &HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>,
    items: &HashMap<&'static str, &'static str>,
) {
    if let Some((description, exits)) = locations.get(game_state.current_location) {
        println!("{}", description);

        if let Some(item) = items.get(game_state.current_location) {
            if !game_state.found_items.contains(item) {
                println!("You found: {}", item);
                game_state.inventory.insert(*item);
                game_state.found_items.insert(*item);
            }
        }

        println!(
            "You can go: {}",
            exits.keys().cloned().collect::<Vec<_>>().join(", ")
        );
    }
}

// Main game function that contains the game loop
pub fn start_game() {
    let locations = get_locations();
    let items = get_items();

    let mut game_state = GameState {
        current_location: "Garden",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };

    println!("Welcome to the Isopod Adventure Game! 🐞");
    display_help();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => display_help(),
            "look" => look_around(&mut game_state, &locations, &items),
            "inventory" => display_inventory(&game_state.inventory),
            "quit" => {
                println!("Goodbye, little isopod! 🐾");
                break;
            }
            _ if command.starts_with("go ") => {
                let direction = command[3..].trim();
                if DIRECTIONS.contains(&direction) {
                    move_to(&mut game_state, direction, &locations);
                } else {
                    println!("Invalid direction. Try north, south, east, or west.");
                }
            }
            _ => println!("Unknown command. Type 'help' for a list of commands."),
        }

        // Check for win condition
        if game_state.inventory.len() == 3 {
            println!("Congratulations! You've found all three things and won the game! 🎉");
            break;
        }
    }
}
