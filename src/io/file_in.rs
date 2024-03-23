use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use super::device_out;
use itertools::Itertools;

// Given a &Path to a .wav file on disk and a &StreamConfig indicating desired bitrate, sample rate, etc.,
// returns a Vec<f32> containing the samples of 
pub fn get_samples_from_wav(file_path: &Path, config: &cpal::StreamConfig) -> Vec<f32> {
    // hound's WavReader decodes the given .wav file into raw samples that can be fed into the rest of our DSP stuff
    // Relevant docs: https://docs.rs/hound/3.5.1/hound/struct.WavReader.html
    let mut wav_reader = hound::WavReader::open(file_path).expect("could not open audio file");
    let wav_spec = wav_reader.spec();

    let source_audio = fon::Audio::with_frames(wav_spec.sample_rate, reader_to_frames(&mut wav_reader));

    // This line takes egregiously long for some reason
    let mut target_audio = fon::Audio::<fon::chan::Ch32, 2>::with_audio(config.sample_rate.0, &source_audio);

    target_audio.as_f32_slice().to_vec()
}

fn reader_to_frames(wav_reader: &mut hound::WavReader<BufReader<File>>) -> Vec<fon::Frame<fon::chan::Ch32, 2>> {
    use fon::{Frame, chan::Ch32};
    use hound::SampleFormat::{Float, Int};
    let source_channels = wav_reader.spec().channels;
    let sample_format = wav_reader.spec().sample_format;

    let mut frame_vec: Vec<Frame<Ch32, 2>> = Vec::new();
    if sample_format == Float {
        for mut frame_chunk in wav_reader.samples::<f32>().chunks(source_channels.into()).into_iter() {
            if source_channels == 1 {
                let sample = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32); 
                frame_vec.push(Frame::<Ch32, 2>::new(sample, sample));
            } else if source_channels == 2 {
                let left = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32);
                let right = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32);
                frame_vec.push(Frame::<Ch32, 2>::new(left, right));
            }
        }
    } else if sample_format == Int {
        for mut frame_chunk in wav_reader.samples::<i32>().chunks(source_channels.into()).into_iter() {
            if source_channels == 1 {
                let sample = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32 / i16::MAX as f32); 
                frame_vec.push(Frame::<Ch32, 2>::new(sample, sample));
            } else if source_channels == 2 {
                let left = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32 / i16::MAX as f32);
                let right = Ch32::new(frame_chunk.next().unwrap().unwrap() as f32 / i16::MAX as f32);
                frame_vec.push(Frame::<Ch32, 2>::new(left, right));
            }
        }
    } 
    frame_vec
}