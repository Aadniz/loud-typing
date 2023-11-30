mod player;

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::env;
use std::time::Duration;
use crate::player::SoundPlayer;


fn key_up(_key: Keycode) {

}

fn key_down(key: Keycode, player: &SoundPlayer) -> Result<(), Box<dyn Error>> {
    println!("{}", key);

    player.play()?;

    Ok(())
}

fn main() {
    let device_state = DeviceState::new();
    let mut key_states: HashMap<Keycode, bool> = HashMap::new();

    let args: Vec<String> = env::args().skip(1).collect();
    let paths = if args.is_empty() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let default_path = current_dir.join("sounds").display().to_string();
        vec![default_path]
    } else {
        args
    };

    let player = SoundPlayer::new(paths).unwrap();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for key down events
        for key in &keys {
            if key_states.get(key).is_none() {
                key_down(*key, &player).map_err(|err|
                    eprintln!("ERROR: {}", err)
                ).unwrap();
                key_states.insert(*key, true);
            }
        }

        // Check for key up events
        let keys_up: Vec<Keycode> = key_states.keys().cloned().collect();
        for key in keys_up {
            if !keys.contains(&key) {
                key_up(key);
                key_states.remove(&key);
            }
        }

        // Check for key up events
        thread::sleep(Duration::from_millis(3));
    }
}