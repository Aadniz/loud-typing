use std::error::Error;
use std::fs;
use std::fs::{File, read_dir};
use std::io::{BufReader};
use std::path::Path;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rand::seq::SliceRandom;
use rodio::source::{Buffered, SamplesConverter};

fn default_options() -> PlayerOptions {
    PlayerOptions {
        random: false,
        random_pitch: false,
        pitch_deviation: 0.2
    }
}
struct PlayerOptions {
    random: bool,
    random_pitch: bool,
    pitch_deviation: f32
}

pub struct SoundPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    audio_sources: Vec<SamplesConverter<Buffered<Decoder<BufReader<File>>>, f32>>,
    options: PlayerOptions
}

impl SoundPlayer {
    pub fn new(paths: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let (stream, handle) = OutputStream::try_default()?;

        let mut sources = Vec::new();

        for path in &paths {
            let path = Path::new(path);
            if path.is_dir() {
                for entry in read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if let Ok(a) = Self::process_file(&path) {
                        sources.push(a);
                    }
                }
            }else {
                if let Ok(a) = Self::process_file(&path) {
                    sources.push(a);
                }
            }
        }

        Ok(Self {
            _stream: stream,
            handle,
            audio_sources: sources,
            options: default_options(),
        })
    }

    pub fn random_select(mut self, state: bool) -> Self {
        self.options.random = state;
        self
    }

    pub fn random_pitch(mut self, state: bool) -> Self {
        self.options.random_pitch = state;
        self
    }

    pub fn pitch_deviation(mut self, value: f32) -> Self {
        self.options.pitch_deviation = value;
        self
    }

    pub fn play(&self, code: u32) {
        if self.audio_sources.is_empty() {
            return;
        }

        let audio = if self.options.random {
            self.audio_sources.choose(&mut rand::thread_rng()).cloned().unwrap_or_else(|| self.audio_sources[0].clone())
        } else {
            let index = (code as usize) % self.audio_sources.len();
            self.audio_sources[index].clone()
        };

        if self.options.random_pitch {
            let pitch = 1.0 - self.options.pitch_deviation + rand::random::<f32>() * self.options.pitch_deviation * 2.0;

            let source = audio.speed(pitch);
            self.handle.play_raw(source).unwrap();
        } else {
            self.handle.play_raw(audio).unwrap();
        }
    }

    fn process_file(path: &Path) -> Result<SamplesConverter<Buffered<Decoder<BufReader<File>>>, f32>, Box<dyn Error>> {
        if !path.is_file() {
            return Err(Box::from("Not a file"));
        }
        let metadata = fs::metadata(&path)?;
        let file_size_in_bytes = metadata.len();
        if file_size_in_bytes > 30 * 1024 * 1024 {
            return Err(Box::from("File is larger than 30 MB"));
        }
        let file = BufReader::new(File::open(path)?);
        let decoder = Decoder::new(file)?;

        return Ok(decoder.buffered().convert_samples());
    }
}