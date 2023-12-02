mod player;

use std::env;
use std::sync::{Arc, Mutex};
use crate::player::SoundPlayer;
use espanso_detect;
use espanso_detect::event::{InputEvent, Status};
use espanso_detect::{Source, SourceCallback};


fn key_up() {

}

fn key_down(_key: InputEvent, player_mutex: Arc<Mutex<SoundPlayer>>) {
    if let Ok(player) = player_mutex.lock() {
        player.play();
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

    let args: Vec<String> = env::args().skip(1).collect();
    let paths = if args.is_empty() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let default_path = current_dir.join("sounds").display().to_string();
        vec![default_path]
    } else {
        args
    };

    // Get device events
    let mut source : Box<dyn Source> = espanso_detect::get_source(Default::default()).unwrap();

    // Initialize the source
    if let Err(e) = source.initialize() {
        eprintln!("Failed to initialize source: {:?}", e);
    }

    // Define the callback function
    let player = Arc::new(Mutex::new(SoundPlayer::new(paths).unwrap()));
    let callback: SourceCallback = Box::new(move |event| handle_event(event, Arc::clone(&player)));

    // Start the event loop
    if let Err(e) = source.eventloop(callback) {
        eprintln!("Failed to start event loop: {:?}", e);
    }
}