mod player;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use clap;
use clap::Parser;
use espanso_detect;
use espanso_detect::event::{InputEvent, MouseButton, Status};
use espanso_detect::{Source, SourceCallback};

use crate::player::SoundPlayer;

/// Loud typing
///
/// Trigger noises when you type
#[derive(Parser)]
#[clap(version = "1.0", author, about, long_about = None)]
struct Cli {
    /// One or multiple audio files or one or multiple directories containing audio files
    #[clap(name = "INPUT", default_value = "./sounds/minecraft/villagers")]
    input: Vec<PathBuf>,

    /// Play sounds in random order
    #[clap(short, long, default_value = "false")]
    random: bool,

    /// Play sounds with a random pitch
    #[clap(short, long, default_value = "false")]
    pitch: bool,

    /// Set the amount of pitch deviation from 0 - 0.99
    #[clap(short = 'd', long, default_value = "0.2", value_parser = validate_pitch_deviation)]
    pitch_deviation: f32
}

fn validate_pitch_deviation(val: &str) -> Result<f32, String> {
    let pitch_deviation: f32 = val.parse().map_err(|_| "Pitch deviation must be a number")?;
    if pitch_deviation < 0.0 || pitch_deviation > 0.99 {
        Err("Pitch deviation must be between 0 and 0.99".to_string())
    } else {
        Ok(pitch_deviation)
    }
}


fn key_up() {

}

fn key_down(event: InputEvent, player_mutex: Arc<Mutex<SoundPlayer>>) {
    if let Ok(player) = player_mutex.lock() {
        let code = match event {
            InputEvent::Mouse(key) => {
                match key.button {
                    MouseButton::Left => 0,
                    MouseButton::Right => 1,
                    MouseButton::Middle => 2,
                    MouseButton::Button1 => 3,
                    MouseButton::Button2 => 4,
                    MouseButton::Button3 => 5,
                    MouseButton::Button4 => 6,
                    MouseButton::Button5 => 7
                }
            },
            InputEvent::Keyboard(key) => {
                key.code
            },
            _ => 0
        };
        player.play(code);
    }
}

fn handle_event(event : InputEvent, player : Arc<Mutex<SoundPlayer>>) {
    match event {
        InputEvent::Mouse(key) => {
            match key.status {
                Status::Pressed => key_down(InputEvent::Mouse(key), player),
                Status::Released => key_up()
            }
        },
        InputEvent::Keyboard(key) => {
            match key.status {
                Status::Pressed => key_down(InputEvent::Keyboard(key), player),
                Status::Released => key_up()
            }
        },
        _ => ()
    }
}

fn main() {
    let cli = Cli::parse();

    // Get the path options
    let paths: Vec<String> = cli.input.iter().map(|path| path.display().to_string()).collect();

    // Get device events
    let mut source : Box<dyn Source> = espanso_detect::get_source(Default::default()).unwrap();

    // Initialize the source
    if let Err(e) = source.initialize() {
        eprintln!("Failed to initialize source: {:?}", e);
    }

    // Define the callback function
    let player = SoundPlayer::new(paths).unwrap()
        .random_select(cli.random)
        .random_pitch(cli.pitch)
        .pitch_deviation(cli.pitch_deviation);
    let player_mutex = Arc::new(Mutex::new(player));
    let callback: SourceCallback = Box::new(move |event| handle_event(event, Arc::clone(&player_mutex)));

    // Start the event loop
    if let Err(e) = source.eventloop(callback) {
        eprintln!("Failed to start event loop: {:?}", e);
    }
}