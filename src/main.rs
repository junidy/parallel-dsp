mod io;
mod dsp;
mod utils;
use ringbuf::HeapRb;

const BUFFER_SIZE: cpal::FrameCount = 1024;

fn main() {
    let host = io::init_host();
    let (input_device, mut input_stream_config) = io::init_input_device(&host);
    let (output_device, mut output_stream_config) = io::init_output_device(&host);
    output_stream_config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE);

    // This ring buffer is the lock-free connection between
    // the terminal node in the audio graph, and the output audio thread
    let buffer = DoubleBuffer::new(BUFFER_SIZE);
    // let buffer = HeapRb::<f32>::new(BUFFER_SIZE);
    let (mut buffer_tx, mut buffer_rx) = buffer.split();
    let output_stream = io::init_output_stream(buffer_rx, &output_device, &output_stream_config);
    let manager = dsp::init_thread_manager(buffer_tx);
}