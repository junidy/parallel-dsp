pub mod midi;
pub mod file_in;
pub mod device_in;
pub mod device_out;

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, panic, thread};
use std::time::Duration;
use cpal::{BufferSize, BuildStreamError, Data, Device, FrameCount, Host, OutputStreamTimestamp, Stream, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use crate::utils::double_buffer::DoubleBuffer;
type SampleCount = usize;

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

pub fn get_buffer_size_in_samples(stream_config: &StreamConfig) -> usize {
    if let BufferSize::Fixed(frame_count) = stream_config.buffer_size {
        println!("{}", frame_count as u32 * stream_config.channels as u32);
    }
    match stream_config.buffer_size {
        BufferSize::Fixed(frame_count) => (frame_count * stream_config.channels as u32).try_into().unwrap(),
        BufferSize::Default => panic!("Could not get buffer size")
    }
}