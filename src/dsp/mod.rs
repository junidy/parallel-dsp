mod oscillators;
use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Instant, path::Path};
use cpal::StreamConfig;
use fundsp::hacker32::*;
use ringbuf::{Rb};
use crate::io::device_out::*;
use crate::io::file_in::*;
use crate::utils::double_buffer::DoubleBuffer;
use itertools::Itertools;

pub fn init_thread_manager(output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) -> JoinHandle<()> {
    thread::spawn(move || manager_loop(output_buffer, stream_config))
}

fn manager_loop(output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) {
    let mut next_buffer = vec![0.0; get_buffer_size_in_samples(&stream_config)];

    // let mut network = Net32::new(0, 2);
    // let sine_id = network.push(Box::new(sin_hz(440)));
    // network.pipe_output(sine_id);

    // let mut net = Net32::new(0, 1);
    // // Add nodes, obtaining their IDs.
    // let dc_id = net.push(Box::new(dc(420.0)));
    // let sine_id = net.push(Box::new(sine()));
    // // Connect nodes.
    // net.pipe(dc_id, sine_id);
    // net.pipe_output(sine_id);

    let path = Path::new("samples/BarC4.wav");
    // let path = Path::new("BarC4.wav");
    // println!("{:?}", std::env::current_dir().unwrap());
    let mut wav_reader = get_file_bufreader(path, &stream_config);
    println!("{:?}", wav_reader.spec());
    // println!("{:?}", wav_reader.spec());
    let mut iterator = wav_reader.samples::<i16>();
    println!("{:?}", iterator.next().unwrap());
    
    loop {
        // Wait until the output buffer has unparked us
        thread::park();
        // Compute the output
        // let now = Instant::now();
        for frame in next_buffer.chunks_mut(2) {
            // (frame[0], frame[1]) = net.get_stereo();
            match iterator.next() {
                Some(result) => frame[0] = result.expect("s") as f32 / i16::MAX as f32,
                // Some(result) => frame[0] = result.expect("s"),
                None => frame[0] = 0.0,
            }
            match iterator.next() {
                Some(result) => frame[1] = result.expect("s") as f32 / i16::MAX as f32,
                // Some(result) => frame[1] = result.expect("s"),
                None => frame[1] = 0.0,
            }
            // println!("{}", frame[0]);
        }
        output_buffer.write(&next_buffer);
        // println!("{} ms", now.elapsed().as_millis());
    }
}

