use ringbuf::HeapRb;

pub enum BufferState {
    ReadWrite,
    WriteRead
}
use BufferState::{ReadWrite, WriteRead};

pub struct DoubleBuffer {
    buffer_1: HeapRb<f32>,
    buffer_2: HeapRb<f32>,
    state: BufferState
}

impl DoubleBuffer {
    pub fn with_capacity(cap: usize) -> DoubleBuffer {
        DoubleBuffer {
            buffer_1: HeapRb::<f32>::new(cap),
            buffer_2: HeapRb::<f32>::new(cap),
            state: BufferState::ReadWrite
        }
    }
    pub fn get_read_buffer(&self) -> &HeapRb<f32> {
        match self.state {
            ReadWrite => &self.buffer_1,
            WriteRead => &self.buffer_2,
        }
    }
    pub fn get_write_buffer(&self) -> &HeapRb<f32> {
        match self.state {
            ReadWrite => &self.buffer_1,
            WriteRead => &self.buffer_2,
        }
    }
    pub fn swap_states(&mut self) {
        match self.state {
            ReadWrite => self.state = WriteRead,
            WriteRead => self.state = ReadWrite,
        }
    }
}