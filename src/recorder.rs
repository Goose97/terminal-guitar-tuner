use anyhow::{anyhow, Ok, Result};
use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, SampleFormat, SampleRate, Stream, SupportedStreamConfig};
use std::sync::{Arc, Mutex};

pub struct Recorder {
    sample_rate: usize,
    samples: Arc<Mutex<Vec<f64>>>,
    // Maximum size of the samples vec
    buffer_size: usize,
    stream: Option<Stream>,
}

impl Recorder {
    pub fn new(sample_rate: usize, buffer_size: usize) -> Self {
        Self {
            sample_rate,
            samples: Arc::new(Mutex::new(Vec::with_capacity(buffer_size))),
            stream: None,
            buffer_size,
        }
    }

    // Start recording audio and pulse code modulating. Each sample is a number in the range of
    // -1.0..1.0
    // This function will fail if the recording device doesn't support the provided sample rate
    pub fn record(&mut self) -> Result<()> {
        #[cfg(any(
            not(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            )),
            not(feature = "jack")
        ))]
        let host = cpal::default_host();

        // Set up the input device and stream with the default input config.
        let device = host
            .default_input_device()
            .ok_or(anyhow!("Can't find default input device"))?;

        let config =
            get_device_input_config(&device, SampleRate(self.sample_rate.try_into().unwrap()));

        let samples_clone = self.samples.clone();
        let buffer_size = self.buffer_size;

        let err_fn = move |err| {
            eprintln!("An error occurred on stream: {}", err);
        };

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                let mut buffer = samples_clone.lock().unwrap();

                for sample in data.iter() {
                    buffer.push(*sample as f64);
                }

                if buffer.len() > buffer_size {
                    resize_buffer(&mut buffer, buffer_size);
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    // Invoke callback on collected samples. Only use the last `limit` samples
    pub fn with_samples<'a>(&'a self, mut callback: impl FnMut(Vec<f64>) -> ()) {
        let samples = self.samples.lock().unwrap();
        let mut clone = samples.clone();
        drop(samples);
        clone.resize(self.buffer_size, 0.0);
        callback(clone);
    }
}

fn get_device_input_config(device: &Device, sample_rate: SampleRate) -> SupportedStreamConfig {
    let configs = device.supported_input_configs().unwrap();

    let config = configs
        .into_iter()
        .find(|config| {
            config.channels() == 1
                && config.max_sample_rate() >= sample_rate
                && config.min_sample_rate() <= sample_rate
                && config.sample_format() == SampleFormat::F32
        })
        .unwrap();

    config.with_sample_rate(sample_rate)
}

// Take N elements from tail. Avoid allocation by copying
fn resize_buffer(buffer: &mut Vec<f64>, amount: usize) {
    let buffer_len = buffer.len();
    buffer.copy_within((buffer_len - amount)..buffer_len, 0);
}
