use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, panic, thread};
use std::time::Duration;
use cpal::{BufferSize, BuildStreamError, Data, Device, FrameCount, Host, OutputStreamTimestamp, Stream, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

use crate::utils::double_buffer::DoubleBuffer;
type SampleCount = usize;

pub fn init_host() -> Host {
    cpal::default_host()
}

pub fn init_input_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_input_device().expect("no input device available");
    let supported_config = device.default_input_config().unwrap();
    let config = get_stream_config(supported_config);
    (device, config)
}

pub fn init_output_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_output_device().expect("no output device available");
    let supported_config = device.default_output_config().unwrap();
    let config = get_stream_config(supported_config);
    (device, config)
}

pub fn get_stream_config(supported_config: SupportedStreamConfig) -> StreamConfig {
    StreamConfig {
        channels: 2,
        sample_rate: supported_config.sample_rate(),
        buffer_size: match supported_config.buffer_size() {
            SupportedBufferSize::Range {min, max} => BufferSize::Fixed(*max),
            SupportedBufferSize::Unknown => BufferSize::Fixed(1024)
        }
    }
}

pub fn init_output_stream(buffer: Arc<DoubleBuffer<f32>>, manager_handle: JoinHandle<()>, device: &Device, config: &StreamConfig) -> Stream {
    let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // buffer.swap();
        // Signal to thread manager to begin computing the next buffer
        manager_handle.thread().unpark();
        buffer.read(data);
        // println!("{:?}", data);
    };
    let error_callback = move |error: StreamError| {
        ()
    };
    device.build_output_stream(config.into(), data_callback, error_callback, None).unwrap()
}

pub fn get_buffer_size_in_samples(stream_config: &StreamConfig) -> usize {
    match stream_config.buffer_size {
        BufferSize::Fixed(frame_count) => (frame_count * stream_config.channels as u32).try_into().unwrap(),
        BufferSize::Default => panic!("Could not get buffer size")
    }
}