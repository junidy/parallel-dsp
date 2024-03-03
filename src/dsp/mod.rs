use std::thread;

type BufferTx = ringbuf::Producer<f32, std::sync::Arc<ringbuf::SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>;
fn init_thread_manager(output_buffer: BufferTx) -> {
    thread::spawn()
}

fn manager_loop() {

    output_buffer.push_slice();
}