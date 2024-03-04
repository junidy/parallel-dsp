#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, thread};
use std::time::Duration;

use cpal::{BuildStreamError, Data, Device, Host, OutputStreamTimestamp, Stream, StreamConfig, StreamError, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use dubble::DoubleBuffered;
use ringbuf::Rb;

use crate::utils::double_buffer::DoubleBuffer;

pub fn init_host() -> Host {
    cpal::default_host()
}

pub fn init_input_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_input_device().expect("no input device available");
    let config = device.default_input_config().unwrap().into();
    (device, config)
}

pub fn init_output_device(host: &Host) -> (Device, StreamConfig) {
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().unwrap().into();
    (device, config)
}

// type BufferRx = ringbuf::Consumer<f32, std::sync::Arc<ringbuf::SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>;
pub fn init_output_stream(buffer: Arc<Mutex<DoubleBuffered<Vec<f32>>>>, manager_handle: JoinHandle<()>, device: &Device, config: &StreamConfig) -> Stream {
    let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // Signal to thread manager to begin computing the next buffer
        manager_handle.thread().unpark();
        data[..].copy_from_slice(buffer.try_lock().unwrap().read());
        // let (b1, b2) = buffer.as_slices();
        // data[..b1.len()].copy_from_slice(b1);
        // data[b1.len()..].copy_from_slice(b2);
    };
    let error_callback = move |error: StreamError| {
        ()
    };
    device.build_output_stream(config.into(), data_callback, error_callback, None).unwrap()
}

pub fn initialize_audio_io() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    // eprintln!("{:?}", <SupportedStreamConfig as Into<StreamConfig>>::into(config.clone()));

    // let mut tick: f32 = 0.;
    // let mut power: f32 = 0.;

    let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // for frame in data.chunks_mut(2) {
        //     if let [l, r] = frame {
        //         let two: f32 = 2.;
        //         // *l = f32::sin(tick / (50. * two.powf(power / 12.))) * 0.1;
        //         *r = f32::sin(tick / 50.) * 0.1;
        //         *r = f32::cos(tick / 50.) * 0.1;
        //         tick += 1.;
        //         if tick as usize % 10000 == 0 {
        //             power -= 1.0;
        //         }
        //     }
        // }
        // ()
    };
    let error_callback = move |error: StreamError| {
        ()
    };

    // let stream = device.build_output_stream(&config.into(), data_callback, error_callback, None).unwrap();
    // stream.play().unwrap();

    // thread::sleep(Duration::from_millis(5100));
    // println!("{}", tick);
}
