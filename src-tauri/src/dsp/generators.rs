pub struct Microphone {
    input_buffer: ringbuf::HeapConsumer<f32>,
    buffer_size: usize,
}

impl Microphone {
    // Consume buffer_size samples from the microphone in ring buffer, resampling as required
    pub fn new(mut input_buffer: ringbuf::HeapConsumer<f32>, buffer_size: usize) -> Microphone {
        Microphone {
            input_buffer,
            buffer_size
        }
    }

    pub fn next_mic_samples(&mut self) -> Vec<f32> {
        let mut sample_vec = vec![0.0; self.buffer_size];
        self.input_buffer.pop_slice(&mut sample_vec);
        sample_vec
    }
}