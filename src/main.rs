mod io;
mod dsp;
mod utils;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use cpal::traits::DeviceTrait;
use crate::{io::get_buffer_size_in_samples, utils::double_buffer::DoubleBuffer};

const BUFFER_SIZE: cpal::FrameCount = 1024;
type SampleCount = usize;

fn main() {
    // Initalize our I/O with default settings.
    let host = io::init_host();
    let (input_device, input_stream_config) = io::init_input_device(&host);
    let (output_device, output_stream_config) = io::init_output_device(&host);
    let buffer_size: SampleCount = get_buffer_size_in_samples(&output_stream_config);

    // Debug info
    // assert_eq!(input_stream_config.buffer_size, output_stream_config.buffer_size);
    println!("Input from {}", input_device.name().unwrap());
    println!("Output to {}", output_device.name().unwrap());
    println!("Buffer size (in samples) is {:?}", buffer_size);

    let mut output_buffer = Arc::new(DoubleBuffer::<f32>::new(buffer_size));
    let manager_handle = dsp::init_thread_manager(output_buffer.clone(), output_stream_config.clone());
    let output_stream = io::init_output_stream(output_buffer, manager_handle, &output_device, &output_stream_config);
    thread::park();
}