use portaudio as pa;

use std::time::Duration;
use std::time::Instant;
const SAMPLE_RATE: f64 = 44_100.0 / 1.0;
const FRAMES: u32 = 8;
const CHANNELS: i32 = 1;
const INTERLEAVED: bool = true;

pub fn run(mut effect_fn: impl FnMut(f32) -> f32 + 'static, duration: Duration) -> Result<(), pa::Error> {
  let pa = pa::PortAudio::new()?;
  let start = Instant::now();

  println!("PortAudio:");
  println!("version: {}", pa.version());
  println!("version text: {:#?}", pa.version_text());
  println!("host count: {}", pa.host_api_count()?);

  let default_host = pa.default_host_api()?;
  println!("default host: {:#?}", pa.host_api_info(default_host));

  let def_input = pa.default_input_device()?;
  let input_info = pa.device_info(def_input)?;
  println!("Default input device info: {:#?}", input_info);

  let latency = input_info.default_low_input_latency;
  let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);

  let def_output = pa.default_output_device()?;
  let output_info = pa.device_info(def_output)?;
  println!("Default output device info: {:#?}", output_info);

  let latency = output_info.default_low_output_latency;
  let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

  pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE)?;

  let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);

  println!("Starting capture, will run for {:#?}", duration);
  let mut last_sec: u64 = 0;

  let callback = move |pa::DuplexStreamCallbackArgs {
                         in_buffer,
                         out_buffer,
                         frames,
                         ..
                       }| {
    assert_eq!(frames, FRAMES as usize);

    for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
      *output_sample = effect_fn(*input_sample);
    }

    let elapsed: Duration = start.elapsed();
    let elapsed_sec = elapsed.as_secs();

    if elapsed < duration {
      if last_sec != elapsed_sec {
        println!("Elapsed: {}s", elapsed_sec);
        last_sec = elapsed_sec;
      }
      pa::Continue
    } else {
      println!(
        "Elapsed: {}s, stopped capturing",
        elapsed.as_millis() as f32 / 1000.0
      );
      pa::Complete
    }
  };

  let mut stream = pa.open_non_blocking_stream(settings, callback)?;

  stream.start()?;

  while stream.is_active()? {}

  stream.stop()?;

  Ok(())
}
