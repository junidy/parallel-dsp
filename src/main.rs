mod io;
mod dsp;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use cpal::traits::DeviceTrait;
use dubble::DoubleBuffered;

const BUFFER_SIZE: cpal::FrameCount = 1024;

fn main() {
    // Initalize our I/O to default settings.
    let host = io::init_host();
    let (input_device, mut input_stream_config) = io::init_input_device(&host);
    let (output_device, mut output_stream_config) = io::init_output_device(&host);
    println!("Outputting to {}", output_device.name().unwrap());
    // output_stream_config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE / 2);
    // buffer_size = output_stream_config.buffer_size

    // This double buffer is the lock-free connection between
    // the terminal node in the audio graph, and the output audio thread
    let mut buffer = Arc::new(Mutex::new(DoubleBuffered::new(vec![0.; BUFFER_SIZE as usize])));
    let manager_handle = dsp::init_thread_manager(buffer.clone());
    let output_stream = io::init_output_stream(buffer, manager_handle, &output_device, &output_stream_config);
    thread::park();
}