mod dsp;
mod gui;
mod io;
mod utils;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use cpal::traits::{DeviceTrait, StreamTrait};
use crate::io::device_out::{self, match_input_output_configs};
use crate::utils::double_buffer::DoubleBuffer;

const BUFFER_SIZE: cpal::FrameCount = 1024;
type SampleCount = usize;

fn main() {
    // Initalize our I/O with default settings.
    let host = device_out::init_host();
    let (output_device, output_stream_config) = device_out::init_output_device(&host);
    let (input_device, input_stream_config) = device_out::init_input_device(&host);
    let buffer_size: SampleCount = device_out::get_buffer_size_in_samples(&output_stream_config);

    match_input_output_configs(&input_device, &output_device);

    // Debug info
    // assert_eq!(input_stream_config.buffer_size, output_stream_config.buffer_size);
    println!("Input from {}", input_device.name().unwrap());
    println!("Output to {}", output_device.name().unwrap());
    println!("Input config: {:?}", input_stream_config);
    println!("Output config: {:?}", output_stream_config);
    println!("Buffer size (in samples) is {:?}", buffer_size);

    let mut output_buffer = Arc::new(DoubleBuffer::<f32>::new(buffer_size));
    let manager_handle = dsp::init_thread_manager(output_buffer.clone(), output_stream_config.clone());
    let output_stream = device_out::init_output_stream(output_buffer, manager_handle, &output_device, &output_stream_config);
    output_stream.play();
    thread::park();
}