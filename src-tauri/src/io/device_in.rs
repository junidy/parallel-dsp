use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, panic, thread};
use std::time::Duration;
use cpal::{BufferSize, BuildStreamError, Data, Device, FrameCount, Host, OutputStreamTimestamp, Stream, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use crate::utils::double_buffer::DoubleBuffer;
type SampleCount = usize;

use super::{get_stream_config, get_buffer_size_in_samples};

pub fn init_input_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_input_device().expect("no input device available");
    let supported_config = device.default_input_config().unwrap();
    let config = get_stream_config(supported_config);
    (device, config)
}


pub fn init_input_stream(mut ringbuf: ringbuf::HeapProducer<f32>, device: &Device, config: &StreamConfig) -> Stream {
    let data_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        // Fill ring buffer
        ringbuf.push_slice(data);
    };
    let error_callback = move |error: StreamError| {
        ()
    };
    device.build_input_stream(config.into(), data_callback, error_callback, None).unwrap()
}