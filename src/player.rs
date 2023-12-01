use std::error::Error;
use std::fs;
use std::fs::{File, read_dir};
use std::io::{BufReader};
use std::path::Path;
use std::time::Duration;
use metadata::MediaFileMetadata;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rand::seq::SliceRandom;
use rodio::source::{Buffered, SamplesConverter};

pub struct SoundPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    audio_sources: Vec<SamplesConverter<Buffered<Decoder<BufReader<File>>>, f32>>,
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
        })
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        if let Some(source) = self.audio_sources.choose(&mut rand::thread_rng()) {
            self.handle.play_raw(source.clone())?;
        }

        Ok(())
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
        if let Ok(duration) = Self::duration(path) {
            if duration <= Duration::from_secs(5) {
                return Ok(decoder.buffered().convert_samples());
            }
        }

        return Err(Box::from("Nothing found"));
    }

    fn duration(path: &Path) -> Result<Duration, Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let decoder = Decoder::new(file)?;
        return if let Some(duration) = decoder.total_duration() {
            Ok(duration)
        } else if let Some(duration) = MediaFileMetadata::new(&path)?._duration {
            Ok(Duration::from_secs_f64(duration))
        } else {
            Err(Box::from("No duration found"))
        }
    }
}