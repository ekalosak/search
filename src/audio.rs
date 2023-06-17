use std::sync::{Arc, Mutex};
use std::time::Duration;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn capture_audio_samples(timeout: Duration) -> Vec<u16> {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("Failed to get default input device");
    let config = input_device.default_input_config().expect("Failed to get default config");
    // let mut samples = Vec::new();
    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_closure = Arc::clone(&samples);
    let stream = input_device
        .build_input_stream(
            &StreamConfig::from(config),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut samples = samples_closure.lock().unwrap();
                for &sample in data {
                    let scaled_sample = (sample * std::i16::MAX as f32) as u16;
                    samples.push(scaled_sample);
                }
            },
            |err| {
                eprintln!("An error occurred: {}", err);
            },
            Some(timeout),
        )
        .expect("Failed to build input stream");
    stream.play().expect("Failed to play stream");
    stream.pause().expect("Failed to pause stream");
    let out = Arc::try_unwrap(samples).unwrap().into_inner().unwrap();
    println!("Captured {} samples", out.len());
    out
}
