use hound;
use std::iter::Map;
use hound::WavSamples;

mod autowah;
use autowah::Autowah;

fn main() {
  let autowah = autowah::Autowah::default();
  println!("{:#?}", autowah);

}

fn load_wav() -> Vec<f32> {
  let mut reader = hound::WavReader::open("resources/classical.wav").unwrap();
  reader
    .samples::<i16>()
    .map(|sample| sample.unwrap() as f32 / 32768.0)
    .collect()
}
