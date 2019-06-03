use hound;

mod autowah;
use autowah::Autowah;

fn main() {
  let scale_factor: f32 = 32768.0;
  let mut autowah = Autowah::default();
  let mut reader = hound::WavReader::open("resources/classical_mono.wav").unwrap();
  let mut writer = hound::WavWriter::create("resources/classical_wah.wav", reader.spec()).unwrap();

  reader
    .samples::<i16>()
    .map(|sample| sample.unwrap() as f32 / scale_factor)
    .map(|x| autowah.run(x))
    .map(|w| (w * scale_factor) as i16)
    .for_each(|dn| writer.write_sample(dn).unwrap());

  writer.finalize().unwrap();

  println!("{:#?} ns", autowah.avg_time())
}
