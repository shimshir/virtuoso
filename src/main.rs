mod autowah;
mod io;

use hound;
use std::time::Duration;

use autowah::Autowah;

fn main() {
  duplex_stream(Duration::from_secs(10));
  //rw_file("resources/classical.wav");
}

fn duplex_stream(duration: Duration) {
  let autowah = Autowah::default();
  io::run(move |s| 2.0 * autowah.run(s), duration).unwrap();
}

fn rw_file(path: &str) {
  let autowah = Autowah::default();

  let scale_factor: f32 = 32768.0;
  let mut reader = hound::WavReader::open(path).unwrap();
  let mut writer = hound::WavWriter::create("wah.wav", reader.spec()).unwrap();

  reader
    .samples::<i16>()
    .map(|sample| sample.unwrap() as f32 / scale_factor)
    .map(|x| autowah.run(x))
    .map(|w| (w * scale_factor) as i16)
    .for_each(|dn| writer.write_sample(dn).unwrap());

  writer.finalize().unwrap();
}
