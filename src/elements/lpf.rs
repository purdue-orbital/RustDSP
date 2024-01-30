use crate::elements::element::Element;
use crate::elements::macros::fir::fir_lpf_dft;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

#[derive(Clone)]
pub struct LPF {
    roll_off: f32,
    sample_rate: f32,
    cutoff_frequency: f32,
}

impl Element for LPF {
    #[cfg(feature = "ui")]
    fn build_window(&mut self, _win_builder: &mut WindowBuilder) {}

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        fir_lpf_dft(builder,&samples.get_complex_f32(),self.sample_rate,self.cutoff_frequency,self.roll_off)
    }

    fn run(&mut self, _samples: &ElementParameter) {}

    fn halt(&self) -> bool {
        false
    }

    fn is_source(&self) -> bool {
        false
    }
}

impl LPF {
    pub fn new(cutoff_frequency: f32, sample_rate: f32, roll_off: f32) -> LPF {
        LPF {
            roll_off,
            sample_rate,
            cutoff_frequency,
        }
    }
}