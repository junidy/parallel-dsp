mod oscillators;
use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use cpal::StreamConfig;
use fundsp::hacker32::*;
use ringbuf::{Rb};
use crate::{io::get_buffer_size_in_samples, utils::double_buffer::DoubleBuffer};

pub fn init_thread_manager(output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) -> JoinHandle<()> {
    thread::spawn(move || manager_loop(output_buffer, stream_config))
}

fn manager_loop(output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) {
    let mut next_buffer = vec![0.0; get_buffer_size_in_samples(&stream_config)];

    // let mut network = Net32::new(0, 2);
    // let sine_id = network.push(Box::new(sin_hz(440)));
    // network.pipe_output(sine_id);

    let mut net = Net32::new(0, 1);
    // Add nodes, obtaining their IDs.
    let dc_id = net.push(Box::new(dc(420.0)));
    let sine_id = net.push(Box::new(sine()));
    // Connect nodes.
    net.pipe(dc_id, sine_id);
    net.pipe_output(sine_id);

    loop {
        // Wait until the output buffer has unparked us
        thread::park();
        // Compute the output
        for frame in next_buffer.chunks_mut(2) {
            (frame[0], frame[1]) = net.get_stereo();
            // println!("{}", frame[0]);
        }
        output_buffer.write(&next_buffer);
    }
}