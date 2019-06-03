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
  buffer_l: [f32; 2],
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
  beta_mix: f32,
}

impl Autowah {
  pub fn default() -> Autowah {
    Autowah::new(40e-3, 2e-3, 20.0, 3000.0, 1.0 / 5.0, 1.0)
  }

  pub fn new(tau_a: f32, tau_r: f32, min_f: f32, max_f: f32, q: f32, alpha_mix: f32) -> Autowah {
    let fs = 44.1e3;
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
      buffer_l: [0.0, 0.0],
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
      beta_mix: 1.0 - alpha_mix,
    }
  }

  fn y_filter(&self) -> f32 {
    self.y_bandpass.get()
  }
}
