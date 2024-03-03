use ringbuf::HeapRb;

enum BufferState {
    ReadWrite,
    WriteRead
}

struct DoubleBuffer {
    buffer_1: HeapRb<f32>,
    buffer_2: HeapRb<f32>,
    state: BufferState
}

impl DoubleBuffer {
    fn with_capacity(cap: usize) {
        DoubleBuffer {
            buffer_1: HeapRb::<f32>::new(cap),
            buffer_2: HeapRb::<f32>::new(cap),
            state: BufferState::ReadWrite
        }
    }
    fn get_read_buffer(&self) -> &HeapRb<f32> {
        match self.state {
            ReadWrite => &self.buffer_1,
            WriteRead => &self.buffer_2,
        }
    }
    fn get_write_buffer(&self) -> &HeapRb<f32> {
        match self.state {
            ReadWrite => &self.buffer_1,
            WriteRead => &self.buffer_2,
        }
    }
    fn swap_states(&self) {
        match self.state {
            ReadWrite => *self.state = WriteRead,
            WriteRead => *self.state = ReadWrite,
        }
    }
}