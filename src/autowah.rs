use std::cell::Cell;
use std::f32::consts::E;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Autowah {
  // Sin and Tan Constants
  sin_const3: f32,
  sin_const5: f32,
  tan_const3: f32,
  tan_const5: f32,
  // Level Detector parameters
  alpha_a: f32,
  alpha_r: f32,
  beta_a: f32,
  beta_r: f32,
  buffer_l_0: Cell<f32>,
  buffer_l_1: Cell<f32>,
  // Lowpass filter parameters
  buffer_lp: Cell<f32>,
  gain_lp: f32,
  // State Variable Filter parameters
  min_freq: f32,
  freq_bandwidth: f32,
  q: f32,
  fs: f32,
  center_freq: Cell<f32>,
  y_highpass: Cell<f32>,
  y_bandpass: Cell<f32>,
  y_lowpass: Cell<f32>,
  // Mixer parameters
  alpha_mix: f32,
  beta_mix: f32
}

impl Autowah {
  pub fn default() -> Autowah {
    Autowah::new(40e-3, 2e-3, 20.0, 3000.0, 1.0 / 5.0, 1.0)
  }

  pub fn new(tau_a: f32, tau_r: f32, min_f: f32, max_f: f32, q: f32, alpha_mix: f32) -> Autowah {
    let fs = 44_100.0 / 1.0;
    let alpha_a = E.powf(-1.0 / (tau_a * fs));
    let alpha_r = E.powf(-1.0 / (tau_r * fs));

    Autowah {
      sin_const3: -1.0 / 6.0,
      sin_const5: 1.0 / 120.0,
      tan_const3: 1.0 / 3.0,
      tan_const5: 1.0 / 3.0,
      // Level Detector parameters
      alpha_a: alpha_a,
      alpha_r: alpha_r,
      beta_a: 1.0 - alpha_a,
      beta_r: 1.0 - alpha_r,
      buffer_l_0: Cell::new(0.0),
      buffer_l_1: Cell::new(0.0),
      // Lowpass filter parameters
      buffer_lp: Cell::new(0.0),
      gain_lp: (0.5 * q).sqrt(),
      // State Variable Filter parameters
      min_freq: PI * min_f / fs,
      freq_bandwidth: PI * (2.0 * max_f - min_f) / fs,
      q: q,
      fs: fs,
      center_freq: Cell::new(0.0),
      y_highpass: Cell::new(0.0),
      y_bandpass: Cell::new(0.0),
      y_lowpass: Cell::new(0.0),
      // Mixer parameters
      alpha_mix: alpha_mix,
      beta_mix: 1.0 - alpha_mix
    }
  }

  fn y_filter(&self) -> f32 {
    self.y_bandpass.get()
  }

  pub fn run(&self, x: f32) -> f32 {
    let x_l = if x < 0.0 {-x} else {x};
    let y_l: f32 = self.level_detector(x_l);
    self.center_freq.set(y_l * self.freq_bandwidth + self.min_freq);
    let x_f: f32 = self.low_pass_filter(x);
    let y_f: f32 = self.state_variable_filter(x_f);
    let y: f32 = self.mixer(x, y_f);
    y
  }

  fn level_detector(&self, x: f32) -> f32 {
    let y1: f32 = self.alpha_r * self.buffer_l_1.get() + self.beta_r * x;
    self.buffer_l_1.set(if x > y1 {x} else {y1});
    self.buffer_l_0.set(self.alpha_a * self.buffer_l_0.get() + self.beta_a * self.buffer_l_1.get());
    self.buffer_l_0.get()
  }

  fn low_pass_filter(&self, x: f32) -> f32 {
    let k: f32 = self.tan(self.center_freq.get());
    let b0: f32 = k / (k + 1.0);
    let a1: f32 = 2.0 * (b0 - 0.5);
    let buffer_lp_val = self.buffer_lp.get();
    let xh: f32 = x - a1 * buffer_lp_val;
    let y: f32 = b0 * (xh + buffer_lp_val);
    self.buffer_lp.set(xh);
    self.gain_lp * y
  }

  fn tan(&self, x: f32) -> f32 {
    x * (1.0 + self.tan_const3 * x * x)
  }

  fn state_variable_filter(&self, x: f32) -> f32 {
    let f: f32 = 2.0 * self.sin(self.center_freq.get());
    self.y_highpass.set(x - self.y_lowpass.get() - self.q * self.y_bandpass.get());

    let y_band_value = self.y_bandpass.get();
    self.y_bandpass.set(y_band_value + f * self.y_highpass.get());

    let y_low_value = self.y_lowpass.get();
    self.y_lowpass.set(y_low_value + f * self.y_bandpass.get());

    self.y_filter()
  }

  fn sin(&self, x: f32) -> f32 {
    x * (1.0 + self.sin_const3 * x * x)
  }

  fn mixer(&self, x: f32, y: f32) -> f32 {
    self.alpha_mix * y + self.beta_mix * x
  }

}
