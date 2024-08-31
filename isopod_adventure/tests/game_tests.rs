use isopod_adventure::*;
use std::collections::HashSet;

#[test]
fn test_move_to_valid_direction() {
    let mut game_state = GameState {
        current_location: "Garden",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };
    let locations = get_locations();

    move_to(&mut game_state, "north", &locations);
    assert_eq!(game_state.current_location, "Pond");
}

#[test]
fn test_move_to_invalid_direction() {
    let mut game_state = GameState {
        current_location: "Garden",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };
    let locations = get_locations();

    move_to(&mut game_state, "west", &locations); // No west exit from Garden
    assert_eq!(game_state.current_location, "Garden");
}

#[test]
fn test_look_around_finds_item() {
    let mut game_state = GameState {
        current_location: "Garden",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };
    let locations = get_locations();
    let items = get_items();

    look_around(&mut game_state, &locations, &items);
    assert!(game_state.inventory.contains("Cookie Crumb 🍪"));
}

#[test]
fn test_look_around_no_item() {
    let mut game_state = GameState {
        current_location: "Rocky Path",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };
    let locations = get_locations();
    let items = get_items();

    look_around(&mut game_state, &locations, &items);
    assert!(game_state.inventory.is_empty());
}

#[test]
fn test_win_condition() {
    let mut game_state = GameState {
        current_location: "Garden",
        inventory: HashSet::new(),
        found_items: HashSet::new(),
    };

    game_state.inventory.insert("Cookie Crumb 🍪");
    game_state.inventory.insert("Isopod Friend 🐾");
    game_state.inventory.insert("A Place to Hide 🛏️");

    assert_eq!(game_state.inventory.len(), 3); // Check win condition
}
