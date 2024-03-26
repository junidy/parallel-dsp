    use std::sync::atomic::{AtomicBool, Ordering};
    use std::cell::UnsafeCell;

    pub struct DoubleBuffer<T: Copy> {
        buffers: [UnsafeCell<Vec<T>>; 2],
        write_index: AtomicBool,
    }

    // SAFETY: The API must ensure that there are no concurrent writes and reads to the same buffer.
    unsafe impl<T: Copy + Sync> Sync for DoubleBuffer<T> {}

    impl<T: Copy + Default> DoubleBuffer<T> {
        pub fn new(size: usize) -> Self {
            DoubleBuffer {
                buffers: [UnsafeCell::new(vec![T::default(); size]), UnsafeCell::new(vec![T::default(); size])],
                write_index: AtomicBool::new(false),
            }
        }

        pub fn write(&self, data: &[T]) {
            let index = self.write_index.load(Ordering::Relaxed) as usize;
            // SAFETY: This is safe based on the external guarantee that no other thread is concurrently writing to or reading from this buffer.
            unsafe {
                (*self.buffers[index].get()).copy_from_slice(data);
            }
            // Switch the buffer after writing.
            self.swap();  // Assuming switch logic is safe and correct
        }

        pub fn read(&self, output: &mut [T]) {
            let index = !self.write_index.load(Ordering::Acquire) as usize;
            // SAFETY: This is safe based on the external guarantee that no other thread is concurrently writing to or reading from this buffer.
            unsafe {
                output.copy_from_slice(&(*self.buffers[index].get()));
            }
        }

        pub fn swap(&self) {
            let current_index = self.write_index.load(Ordering::Relaxed);
            self.write_index.store(!current_index, Ordering::Release);
        }
    }