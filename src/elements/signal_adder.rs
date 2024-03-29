use crate::elements::element::Element;
use crate::elements::prefabs::wave_generators::wave_generator_complex_time_banked;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct SignalAdder {
    sps: usize,
    sample_rate: f32,
    frequency: f32,
}

impl Element for SignalAdder {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        // create wave generator
        let arr = wave_generator_complex_time_banked(builder, self.sample_rate, self.frequency, self.sps);

        // add two signals together
        let src_i = arr.get_real_array_wrapped();
        let src_q = arr.get_imag_array_wrapped();

        builder.add_f32(&src_i, &samples.get_complex_f32().get_real_array_wrapped());
        builder.add_f32(&src_q, &samples.get_complex_f32().get_imag_array_wrapped());
    }

    fn run(&mut self, _samples: &mut ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }
    fn stop(&self, samples: &mut ElementParameter) -> bool { false }

    fn is_source(&self) -> bool {
        true
    }
}

impl SignalAdder {
    pub fn new(frequency: f32, sample_rate: f32, sps: usize) -> SignalAdder {
        SignalAdder {
            sps,
            sample_rate,
            frequency,
        }
    }
}