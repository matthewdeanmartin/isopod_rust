use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};

// Define the structure for the TOML data
#[derive(Debug, Deserialize)]
struct GameData {
    game: GameText,
    locations: HashMap<String, LocationData>,
    items: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct GameText {
    welcome_text: String,
    win_text: String,
}

#[derive(Debug, Deserialize)]
struct LocationData {
    description: String,
    #[serde(flatten)]
    exits: HashMap<String, String>,
}

// Function to load game data from the TOML file
fn load_game_data() -> GameData {
    let toml_str = fs::read_to_string("data.toml").expect("Could not read data.toml file");
    toml::from_str(&toml_str).expect("Could not parse data.toml file")
}

// Helper function to convert a String to a &'static str
fn to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// Function to get locations from data.toml
pub fn get_locations() -> HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>
{
    let game_data = load_game_data();

    // Convert locations to the desired output format
    game_data
        .locations
        .into_iter()
        .map(|(loc_name, loc_data)| {
            let exits: HashMap<&'static str, &'static str> = loc_data
                .exits
                .into_iter()
                .map(|(direction, destination)| {
                    (to_static_str(direction), to_static_str(destination))
                })
                .collect();
            (
                to_static_str(loc_name),
                (to_static_str(loc_data.description), exits),
            )
        })
        .collect()
}

// Function to get items from data.toml
pub fn get_items() -> HashMap<&'static str, &'static str> {
    let game_data = load_game_data();

    // Convert items to the desired output format
    game_data
        .items
        .into_iter()
        .map(|(location, item)| (to_static_str(location), to_static_str(item)))
        .collect()
}

// Game state structure
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    pub current_location: String,
    pub inventory: HashSet<String>,
    pub found_items: HashSet<String>,
}

// Look around the current location
pub fn look_around(
    game_state: &mut GameState,
    locations: &HashMap<&str, (&str, HashMap<&str, &str>)>,
) {
    if let Some(location_data) = locations.get(game_state.current_location.as_str()) {
        println!("{}", location_data.0);

        let items = load_game_data().items;
        if let Some(item) = items.get(&game_state.current_location) {
            if !game_state.found_items.contains(item) {
                println!("You found: {}", item);
                game_state.inventory.insert(item.to_string());
                game_state.found_items.insert(item.to_string());
            }
        }

        let exits: Vec<_> = location_data.1.keys().cloned().collect(); // Access exits using .1
        println!("You can go: {}", exits.join(", "));
    }
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
pub fn display_inventory(inventory: &HashSet<String>) {
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
    locations: &HashMap<&str, (&str, HashMap<&str, &str>)>,
) {
    if let Some(location_data) = locations.get(game_state.current_location.as_str()) {
        if let Some(new_location) = location_data.1.get(direction) {
            // Access exits using .1
            game_state.current_location = new_location.to_string();
            println!("You move {} to the {}.", direction, new_location);
            look_around(game_state, locations);
        } else {
            println!("You can't go {} from here.", direction);
        }
    }
}

// Main game function that contains the game loop
pub fn start_game() {
    let game_data = load_game_data(); // Load game data including welcome and win text
    let locations = get_locations();
    let items = get_items();

    let mut game_state = GameState {
        current_location: "Garden".to_string(),
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };

    // Display welcome text at the start of the game
    println!("{}", game_data.game.welcome_text);
    display_help(); // Display help commands at the start

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
            "look" => look_around(&mut game_state, &locations),
            "inventory" => display_inventory(&game_state.inventory),
            "quit" => {
                println!("Goodbye, little isopod! 🐾");
                break;
            }
            _ if command.starts_with("go ") => {
                let direction = command[3..].trim();
                if ["north", "south", "east", "west"].contains(&direction) {
                    move_to(&mut game_state, direction, &locations);
                } else {
                    println!("Invalid direction. Try north, south, east, or west.");
                }
            }
            _ => println!("Unknown command. Type 'help' for a list of commands."),
        }

        // Check for win condition
        if game_state.inventory.len() == items.len() {
            println!("{}", game_data.game.win_text);
            break;
        }
    }
}
