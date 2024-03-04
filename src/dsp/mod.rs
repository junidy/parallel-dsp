mod oscillators;
use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use dubble::DoubleBuffered;
use fundsp::hacker32::*;
use ringbuf::{Rb};

// type BufferTx = ringbuf::Producer<f32, std::sync::Arc<ringbuf::SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>;
pub fn init_thread_manager(output_buffer: Arc<Mutex<DoubleBuffered<Vec<f32>>>>) -> JoinHandle<()> {
    thread::spawn(move || manager_loop(output_buffer))
}

fn manager_loop(output_buffer: Arc<Mutex<DoubleBuffered<Vec<f32>>>>) {
    // TODO: Unhardcode buffer size (that's what the 1024 is)
    let mut next_buffer = [0.; 1024];

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
        // println!("Hello");
        for (frame_count, frame) in next_buffer.chunks_mut(2).enumerate() {
            (frame[0], frame[1]) = net.get_stereo();
            // println!("{:?}", frame);
        }
        let mut lock = output_buffer.lock().unwrap();
        for (index, output_frame) in lock.write().iter_mut().enumerate() {
            *output_frame = *next_buffer.get(index).unwrap();
        }
        lock.update();
        // println!("{:?}", lock.read().get(0));

    }
}