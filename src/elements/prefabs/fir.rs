use num_complex::Complex;

use crate::elements::prefabs::dft::fft_shift;
use crate::math::prelude::*;

pub fn fir_filter_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, fir_filter: &ElementParameter) {
    let samples_i_array = samples.get_real_array_wrapped();
    let samples_q_array = samples.get_imag_array_wrapped();

    let scratch = ComplexF32::new(vec![Complex::new(0.0, 0.0); samples_i_array.get_f32_array().len()]);

    let scratch_i_array = scratch.get_real_array_wrapped();
    let scratch_q_array = scratch.get_imag_array_wrapped();

    builder.dft_f32(&samples_i_array, &samples_q_array, &scratch_i_array, &scratch_q_array);

    fft_shift(builder, &scratch);

    builder.pointwise_multiply_f32(fir_filter, &scratch_i_array);
    builder.pointwise_multiply_f32(fir_filter, &scratch_q_array);

    fft_shift(builder, &scratch);

    builder.idft_f32(&scratch_i_array, &scratch_q_array, &samples_i_array, &samples_q_array);
}

/// This macro applies a low pass filter (LPF)
pub fn fir_lpf_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, sample_rate: f32, cutoff_freq: f32, roll_off: f32, scale: f32) {
    // get baud rate
    let sps = samples.to_vec().len();

    // get the frequency increment for each bin index
    let step_size = sample_rate / sps as f32;

    // Create lpf filter
    let mut filter = vec![scale; sps];
    for (index, x) in filter.iter_mut().enumerate() {
        let freq = (index as f32 - ((sps >> 1) as f32)) * step_size;
        if freq > cutoff_freq {
            *x = (-scale * (freq - cutoff_freq) * roll_off) + scale;
            if x.is_sign_negative() {
                *x = 0.0;
            }
        }
    }

    // add filter
    fir_filter_dft(builder, samples, &ElementParameter::new_f32_array(filter.as_slice()));
}

/// This macro applies a high pass filter (HPF)
pub fn fir_hpf_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, sample_rate: f32, cutoff_freq: f32, roll_off: f32, scale: f32) {
    // get baud rate
    let sps = samples.to_vec().len();

    // get the frequency increment for each bin index
    let step_size = sample_rate / sps as f32;

    // Create hpf filter
    let mut filter = vec![0.0; sps];
    for (index, x) in filter.iter_mut().enumerate() {
        let freq = (index as f32 - ((sps >> 1) as f32)) * step_size;
        if freq >= cutoff_freq {
            *x = ((freq - cutoff_freq) * roll_off * scale) + scale;
            if *x > scale {
                *x = scale;
            }
        }
    }

    // add filter
    fir_filter_dft(builder, samples, &ElementParameter::new_f32_array(filter.as_slice()));
}

pub fn fir_bpf_dft(builder: &mut WorkflowBuilder, samples: &ComplexF32, sample_rate: f32, roll_off: f32, lower_frequency_limit: f32, upper_frequency_limit: f32) {
    let sps = samples.to_vec().len();
    let bin_separation = sample_rate / sps as f32;

    let mut new_filter = vec![0.0; sps];

    for (index, value) in new_filter.iter_mut().enumerate() {
        let frequency = (index as f32 - (sps >> 1) as f32) * bin_separation;
        if frequency >= lower_frequency_limit {
            if frequency <= upper_frequency_limit {
                *value = 1.0;
            } else {
                *value = -roll_off * (frequency - upper_frequency_limit) + 1.0;
            }
        } else {
            *value = roll_off * (frequency - lower_frequency_limit) + 1.0;
        }
        if *value < 0.0 {
            *value = 0.0;
        }
    }

    let filter_obj = ElementParameter::new_f32_array(&new_filter.as_slice());
    fir_filter_dft(builder, samples, &filter_obj);
} 