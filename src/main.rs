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
#[derive(Parser)]
#[clap(version = "1.0", author = "D3faIt")]
struct Cli {
    /// One or multiple audio files or one or multiple directories containing audio files
    #[clap(name = "INPUT", default_value = "./sounds/minecraft/villagers")]
    input: Vec<PathBuf>,

    /// Play sounds in random order
    #[clap(short, long)]
    random: bool
}


fn key_up() {

}

fn key_down(event: InputEvent, player_mutex: Arc<Mutex<SoundPlayer>>, random: bool) {
    if let Ok(player) = player_mutex.lock() {
        if random {
            player.play_random();
        }else{
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
}

fn handle_event(event : InputEvent, player : Arc<Mutex<SoundPlayer>>, random: bool) {
    match event {
        InputEvent::Mouse(key) => {
            match key.status {
                Status::Pressed => key_down(InputEvent::Mouse(key), player, random),
                Status::Released => key_up()
            }
        },
        InputEvent::Keyboard(key) => {
            match key.status {
                Status::Pressed => key_down(InputEvent::Keyboard(key), player, random),
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
    let player = Arc::new(Mutex::new(SoundPlayer::new(paths).unwrap()));
    let callback: SourceCallback = Box::new(move |event| handle_event(event, Arc::clone(&player), cli.random));

    // Start the event loop
    if let Err(e) = source.eventloop(callback) {
        eprintln!("Failed to start event loop: {:?}", e);
    }
}