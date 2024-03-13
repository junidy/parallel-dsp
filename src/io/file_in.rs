use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use cpal::StreamConfig;
use hound::WavReader;
use super::device_out;

// Given a Path to a .wav file on disk, returns a hound::WavReader for that 
pub fn get_file_bufreader(file_path: &Path, config: &StreamConfig) -> WavReader<BufReader<File>> {
    // let wav_reader = WavReader::open(file_path).expect("could not open audio file");
    // let wav_spec = wav_reader.spec();
    // let mut frame_vec = Vec::new();
    // for (sample, i) in wav_reader.samples().c
    // let audio_buffer = fon::Audio::with_audio(wav_spec.sample_rate, );
    WavReader::open(file_path).expect("could not open audio file")
}