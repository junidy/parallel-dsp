mod oscillators;
use crate::{io::get_buffer_size_in_samples, utils::double_buffer::DoubleBuffer};
use cpal::StreamConfig;
use fundsp::hacker32::*;
use hound;
use ringbuf::Rb;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

const SINE_WAVE: bool = false;

pub fn init_thread_manager(
    output_buffer: Arc<DoubleBuffer<f32>>,
    stream_config: StreamConfig,
) -> JoinHandle<()> {
    thread::spawn(move || manager_loop(output_buffer, stream_config))
}

fn manager_loop(output_buffer: Arc<DoubleBuffer<f32>>, stream_config: StreamConfig) {
    let mut next_buffer = vec![0.0; get_buffer_size_in_samples(&stream_config)];

    // let mut network = Net32::new(0, 2);
    // let sine_id = network.push(Box::new(sin_hz(440)));
    // network.pipe_output(sine_id);

    if SINE_WAVE {
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

    let mut reader = hound::WavReader::open("audio_files/guitar.wav").unwrap();

    let mut samples: Vec<f32> = Vec::new();
    for sample in reader.samples::<i16>() {
        samples.push(sample.unwrap() as f32 / 32768.0);  
    }

    let mut samples_iter = samples.iter().cycle();
    

    let sample_ind = 0; 
    loop {
        thread::park();
        for frame in next_buffer.chunks_mut(2) {
            let sample = samples_iter.next().unwrap();
            frame[0] = *sample;
            frame[1] = *sample;
        }
            output_buffer.write(&next_buffer);
    }
    
}

