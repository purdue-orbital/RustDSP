use num_complex::Complex;

use crate::elements::element::Element;
use crate::math::builder::WorkflowBuilder;
use crate::math::prelude::*;
use crate::ui::charts::builder::WindowBuilder;
use crate::ui::charts::pixel_chart::PixelChart;

#[derive(Clone)]
pub struct WaterfallChart {
    boxed_chart: Option<Box<PixelChart>>,
    len: usize,
    arr: ComplexF32,
}

impl WaterfallChart {
    pub fn new(len: usize) -> WaterfallChart {
        WaterfallChart {
            boxed_chart: None,
            len,
            arr: ComplexF32::new(vec![Complex::new(0.0, 0.0); len]),
        }
    }
}

impl Element for WaterfallChart {
    fn build_window(&mut self, win_builder: &mut WindowBuilder) {
        let chart = PixelChart::new(self.len, 100);

        self.boxed_chart = Some(win_builder.add_chart(chart));
    }

    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter) {
        let complex = samples.get_complex_f32();

        let i_arr = complex.get_real_array_wrapped();
        let q_arr = complex.get_imag_array_wrapped();

        let zero_arr = ComplexF32::new(vec![Complex::new(0.0, 0.0); self.len]);
        let i_zero = zero_arr.get_real_array_wrapped();
        let q_zero = zero_arr.get_imag_array_wrapped();

        let i_out = self.arr.get_real_array_wrapped();
        let q_out = self.arr.get_imag_array_wrapped();

        // reset
        builder.copy_f32(&i_zero, &i_out);
        builder.copy_f32(&q_zero, &q_out);

        // preform fft
        builder.dft_f32(&i_arr, &q_arr, &i_out, &q_out);
    }

    fn run(&mut self, _samples: &ElementParameter) {
        let mut fft_bins: Vec<Complex<f32>> = self.arr.to_vec();

        // divide by 2
        let mut k = fft_bins.len() >> 1;

        // preform fft shift
        if fft_bins.len() % 2 == 1 {
            k += 1
        }

        fft_bins.rotate_right(k);

        // send fft to pixel chart
        for x in fft_bins {

            // we only need the real component as the imaginary component is just phase data
            let normalized = (((x.norm_sqr().sqrt()) / self.len as f32) * 255.0) as u8;

            self.boxed_chart.as_mut().unwrap().add(normalized, 0, 255 - normalized);
        }
    }

    fn halt(&self) -> bool {
        true
    }

    fn is_source(&self) -> bool {
        false
    }
}