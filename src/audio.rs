use std::fs::File;
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use std::time::Duration;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn capture_audio_samples(path: PathBuf, timeout: Duration) {
    let file = File::create(path).expect("Failed to create file");
    let mut buf = BufWriter::new(file);
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("Failed to get default input device");
    let config = input_device.default_input_config().expect("Failed to get default config");
    let stream = input_device
        .build_input_stream(
            &StreamConfig::from(config),
            create_callback(&mut buf),
            |err| {
                eprintln!("An error occurred: {}", err);
            },
            None,
            )
        .expect("Failed to build input stream");
    stream.play().expect("Failed to play stream");
    std::thread::sleep(timeout);
    stream.pause().expect("Failed to pause stream");
    buf.flush();
    std::thread::sleep(std::time::Duration::from_millis(500));
}

fn create_callback<'a, B>(
    buf: &'a mut B,
) -> impl FnMut(&[f32], &cpal::InputCallbackInfo)
       + 'a
where
    B: std::io::Write,
{
    move |data: &[f32], _: &cpal::InputCallbackInfo| {
        for &sample in data {
            let scaled_sample = (sample * std::i16::MAX as f32) as u16;
            buf.write(&scaled_sample.to_le_bytes())
                .expect("Failed to write sample");
        }
    }
}
