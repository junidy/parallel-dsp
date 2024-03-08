pub mod double_buffer {
    use std::cell::UnsafeCell;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread::JoinHandle;

    pub struct DoubleBuffer<T: Copy> {
        buffers: [UnsafeCell<Vec<T>>; 2],
        write_index: AtomicBool,
        read_index: AtomicUsize,
        buffer_size: usize,
    }

    // SAFETY: The API must ensure that there are no concurrent writes and reads to the same buffer.
    unsafe impl<T: Copy + Sync> Sync for DoubleBuffer<T> {}

    impl<T: Copy + Default> DoubleBuffer<T> {
        pub fn new(size: usize) -> Self {
            DoubleBuffer {
                buffers: [
                    UnsafeCell::new(vec![T::default(); size]),
                    UnsafeCell::new(vec![T::default(); size]),
                ],
                write_index: AtomicBool::new(false),
                read_index: AtomicUsize::new(0),
                buffer_size: size,
            }
        }

        pub fn write(&self, data: &[T]) {
            let index = self.write_index.load(Ordering::Relaxed) as usize;
            // SAFETY: This is safe based on the external guarantee that no other thread is concurrently writing to or reading from this buffer.
            unsafe {
                (*self.buffers[index].get()).copy_from_slice(data);
            }
            // Switch the buffer after writing.
            self.swap(); // Assuming switch logic is safe and correct
        }

        pub fn read(&self, output: &mut [T], manager_handle: JoinHandle<()>) {
            let index = !self.write_index.load(Ordering::Acquire) as usize;

            let mut output_index = 0;
            while output_index < output.len() {
                // println!("{} < {}", output_index, output.len());
                let read_ind = self.read_index.load(Ordering::Relaxed);
                let buffer_left = self.buffer_size - self.read_index.load(Ordering::Relaxed);
                let output_left = output.len() - output_index;
                // println!("{} {}", buffer_left, output_left);
                let num_to_write = if buffer_left < output_left {
                    buffer_left
                } else {
                    output_left
                };
                unsafe {
                    output[output_index..output_index + num_to_write].copy_from_slice(
                        &(*self.buffers[index].get())[read_ind..read_ind + num_to_write],
                    );
                }
                output_index += num_to_write;
                self.read_index
                    .store(read_ind + num_to_write, Ordering::Release);
            }
            println!("wow");

            // SAFETY: This is safe based on the external guarantee that no other thread is concurrently writing to or reading from this buffer.
            // unsafe {
            //     output.copy_from_slice(&(*self.buffers[index].get())[..output.len()]);
            // }
        }

        pub fn swap(&self) {
            let current_index = self.write_index.load(Ordering::Relaxed);
            self.write_index.store(!current_index, Ordering::Release);
        }
    }
}

