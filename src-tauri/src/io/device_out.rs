use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, panic, thread};
use std::time::Duration;
use cpal::{BufferSize, BuildStreamError, Data, Device, FrameCount, Host, OutputStreamTimestamp, Stream, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

use crate::utils::double_buffer::DoubleBuffer;
type SampleCount = usize;
use super::{get_stream_config, get_buffer_size_in_samples};

pub fn init_host() -> Host {
    cpal::default_host()
}


pub fn init_output_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_output_device().expect("no output device available");
    let supported_config = device.default_output_config().unwrap();
    let config = get_stream_config(supported_config);
    (device, config)
}

// pub fn match_input_output_configs(input_device: &Device, output_device: &Device) -> () {
//     let supported_input_configs = input_device.supported_input_configs().unwrap();
//     let supported_output_configs = output_device.supported_output_configs().unwrap();
//     for supported_config_range in supported_input_configs {
//         println!("{:?}", supported_config_range);
//     }
// }


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
