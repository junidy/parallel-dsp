mod io;
mod dsp;
mod utils;
use std::{sync::{Arc, Mutex}, thread, time::Duration};

// use utils::double_buffer::{self, DoubleBuffer};
use dubble::DoubleBuffered;

const BUFFER_SIZE: cpal::FrameCount = 1024;

fn main() {
    let host = io::init_host();
    let (input_device, mut input_stream_config) = io::init_input_device(&host);
    let (output_device, mut output_stream_config) = io::init_output_device(&host);
    output_stream_config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE / 2);

    // This ring buffer is the lock-free connection between
    // the terminal node in the audio graph, and the output audio thread
    let mut buffer = Arc::new(Mutex::new(DoubleBuffered::new(vec![0.; BUFFER_SIZE as usize])));
    // static BUFFER: Arc<&mut DoubleBuffer> = Arc::new(&mut DoubleBuffer::with_capacity(BUFFER_SIZE as usize));
    // let buffer = HeapRb::<f32>::new(BUFFER_SIZE);    
    // let (mut buffer_tx, mut buffer_rx) = buffer.split();
    let manager_handle = dsp::init_thread_manager(buffer.clone());
    let output_stream = io::init_output_stream(buffer, manager_handle, &output_device, &output_stream_config);
    thread::sleep(Duration::from_millis(5000));
}