// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;
mod io;
mod utils;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use cpal::traits::{DeviceTrait, StreamTrait};
use crate::{io::device_out, utils::double_buffer::DoubleBuffer};
use crate::io::device_in::{self};
use crate::utils::tauri;

type SampleCount = usize;

fn main() {

    // Initalize our I/O with default settings.
    let host = device_out::init_host();
    let (output_device, output_stream_config) = device_out::init_output_device(&host);
    let (input_device, input_stream_config) = device_in::init_input_device(&host);
    let buffer_size: SampleCount = io::get_buffer_size_in_samples(&output_stream_config);

    // Debug info
    println!("Input from {}", input_device.name().unwrap());
    println!("Output to {}", output_device.name().unwrap());
    println!("Input config: {:?}", input_stream_config);
    println!("Output config: {:?}", output_stream_config);
    println!("Buffer size (in samples) is {:?}", buffer_size);

    let mut input_buffer = ringbuf::HeapRb::<f32>::new(16384);
    let (mut rb_prod, mut rb_cons) = input_buffer.split();
    let mut output_buffer = Arc::new(DoubleBuffer::<f32>::new(buffer_size));
    let manager_handle = dsp::init_thread_manager(rb_cons, output_buffer.clone(), output_stream_config.clone());
    let input_stream = device_in::init_input_stream(rb_prod, &input_device, &input_stream_config);
    let output_stream = device_out::init_output_stream(output_buffer, manager_handle, &output_device, &output_stream_config);
    input_stream.play().unwrap();
    output_stream.play();

    tauri::init_tauri_gui();

    thread::park();
}
