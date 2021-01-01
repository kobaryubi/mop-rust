use super::super::common::common;

pub fn hoge(bar: u32) {
    common::foo(bar);
}

pub struct AudioCapturer {
    host: cpal::Host,
    // device: cpal::Device,
    // stream: cpal::Stream,
}

impl AudioCapturer {
    // TODO: Result
    pub fn new() -> Self {
        AudioCapturer {
            host: cpal::default_host(),
        }
    }

    fn capture(&self) {}
}
