use std::error::Error;
use std::fs;
use std::fs::{File, read_dir};
use std::io::{BufReader};
use std::path::Path;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rand::seq::SliceRandom;
use rodio::source::Buffered;

pub struct SoundPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    audio_sources: Vec<Buffered<Decoder<BufReader<fs::File>>>>,
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
                    if path.is_file() {
                        Self::process_file(&path, &mut sources)?;
                    }
                }
            } else if path.is_file() {
                Self::process_file(&path, &mut sources)?;
            }
        }

        Ok(Self {
            _stream: stream,
            handle,
            audio_sources: sources,
        })
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        if let Some(source) = self.audio_sources.choose(&mut rand::thread_rng()) {
            self.handle.play_raw(source.clone().convert_samples())?;
        }

        Ok(())
    }

    fn process_file(path: &Path, sources: &mut Vec<Buffered<Decoder<BufReader<File>>>>) -> Result<(), Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        if let Ok(decoder) = Decoder::new(file) {
            if decoder.total_duration().unwrap_or_default() <= std::time::Duration::from_secs(5) {
                sources.push(decoder.buffered());
            }
        }
        Ok(())
    }
}
