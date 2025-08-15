// cq5c_design_a_decent.rs

// Import necessary libraries
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate tokio;
extern crate tokio_core;

use serde::{Serialize, Deserialize};
use serde_json::json;
use rand::Rng;
use tokio::prelude::*;
use tokio_core::reactor::Core;

// Define the struct for game metadata
#[derive(Serialize, Deserialize)]
struct Game {
    id: String,
    name: String,
    description: String,
    players: Vec<String>,
}

// Define the struct for player metadata
#[derive(Serialize, Deserialize)]
struct Player {
    id: String,
    username: String,
    score: u32,
}

// Define the struct for game state
#[derive(Serialize, Deserialize)]
struct GameState {
    games: Vec<Game>,
    players: Vec<Player>,
}

// Define the enum for event types
#[derive(Serialize, Deserialize)]
enum EventType {
    GameCreated,
    GameUpdated,
    PlayerJoined,
    PlayerLeft,
    ScoreUpdated,
}

// Define the struct for events
#[derive(Serialize, Deserialize)]
struct Event {
    id: String,
    event_type: EventType,
    data: serde_json::Value,
}

// Initialize the game state
let mut game_state = GameState {
    games: vec![],
    players: vec![],
};

// Initialize the event queue
let mut event_queue: Vec<Event> = vec![];

// Function to create a new game
fn create_game(name: String, description: String) -> Game {
    let game_id = format!("game_{}", rand::random::<u32>());
    Game {
        id: game_id,
        name,
        description,
        players: vec![],
    }
}

// Function to join a game
fn join_game(game_id: String, player_id: String) {
    // Find the game and add the player
    for game in &mut game_state.games {
        if game.id == game_id {
            game.players.push(player_id);
            break;
        }
    }
}

// Function to update a player's score
fn update_score(player_id: String, score: u32) {
    // Find the player and update their score
    for player in &mut game_state.players {
        if player.id == player_id {
            player.score = score;
            break;
        }
    }
}

// Function to process events
fn process_events() {
    while let Some(event) = event_queue.pop() {
        match event.event_type {
            EventType::GameCreated => {
                // Add the game to the game state
                game_state.games.push(event.data);
            }
            EventType::PlayerJoined => {
                // Join the game
                join_game(event.data["game_id"].as_str().unwrap().to_string(), event.data["player_id"].as_str().unwrap().to_string());
            }
            EventType::ScoreUpdated => {
                // Update the player's score
                update_score(event.data["player_id"].as_str().unwrap().to_string(), event.data["score"].as_u64().unwrap() as u32);
            }
            _ => {}
        }
    }
}

// Function to run the game loop
fn run_game_loop() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Run the event processing function
    handle.spawn(process_events());

    // Run the game loop
    core.run(tokio::timer::Interval::new_interval(std::time::Duration::from_millis(100)));
}

fn main() {
    // Create a new game
    let game = create_game("My Game".to_string(), "A game about something".to_string());
    game_state.games.push(game);

    // Add an event to the queue
    event_queue.push(Event {
        id: "event_1".to_string(),
        event_type: EventType::GameCreated,
        data: json!({
            "game_id": game.id,
        }),
    });

    // Run the game loop
    run_game_loop();
}