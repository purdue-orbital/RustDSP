use crate::elements::element::Element;
use crate::elements::parts::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalGenerator {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
}

impl Element for SignalGenerator {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create wave generator
        let arr = wave_generator_complex_time_banked(builder,self.sample_rate,self.frequency,self.sps);

        // set output as the out of the wave generator
        samples.set_complex_f32(arr)
    }

    fn run(&mut self, samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        true
    }
}

impl SignalGenerator {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalGenerator {
        SignalGenerator {
            sps,
            sample_rate,
            frequency
        }
    }
}