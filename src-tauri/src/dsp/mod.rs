mod generators;
use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Instant, path::Path};
use cpal::StreamConfig;
use fundsp::hacker32::*;
use ringbuf::{Rb};
use crate::io::{device_out::*, get_buffer_size_in_samples};
use crate::io::file_in::*;
use crate::dsp::generators::*;
use crate::utils::double_buffer::DoubleBuffer;
use itertools::Itertools;

pub fn init_thread_manager(input_buffer: ringbuf::HeapConsumer<f32>, output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) -> JoinHandle<()> {
    thread::spawn(move || manager_loop(input_buffer, output_buffer, stream_config))
}

// fn generate_test_network() -> {

// }

fn manager_loop(input_buffer: ringbuf::HeapConsumer<f32>, output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) {
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

    // let path = Path::new("samples/BarC4.wav");
    // let mut wav_samples = get_samples_from_wav(path, &stream_config);
    // let mut sample_iter = wav_samples.iter();
    let mut mic = Microphone::new(input_buffer, 8192);
    
    loop {
        // Wait until the output buffer has unparked us
        thread::park();
        // Compute the output
        // let now = Instant::now();
        // for frame in next_buffer.chunks_mut(2) {
        //     match sample_iter.next() {
        //         Some(result) => frame[0] = *result,
        //         None => frame[0] = 0.0,
        //     }
        //     match sample_iter.next() {
        //         Some(result) => frame[1] = *result,
        //         None => frame[1] = 0.0,
        //     }
        //     // println!("{}", frame[0]);
        // }
        // output_buffer.write(&next_buffer);
        let mic_samples = mic.next_mic_samples();
        // println!("{:?}", mic_samples);
        output_buffer.write(&mic_samples);
        // println!("{} ms", now.elapsed().as_millis());
    }
}

