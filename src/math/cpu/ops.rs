use std::f32::consts::PI;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone)]
pub struct Data {
    pub(crate) f32_arrays: Vec<Arc<Mutex<Vec<f32>>>>,
    pub(crate) f32_const: Vec<Arc<RwLock<f32>>>,
}

pub trait CPUOperation {
    fn run(&mut self, data: &mut Data);
}


pub struct ElementwiseMultiplyF32;

impl CPUOperation for ElementwiseMultiplyF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x *= arr1[index];
        }
    }
}

pub struct ElementwiseDivideF32;

impl CPUOperation for ElementwiseDivideF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x /= arr1[index];
        }
    }
}

pub struct ConvolutionF32;

impl CPUOperation for ConvolutionF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let dest = binding.as_mut_slice();

        // run
        for i in 0..arr1.len() {
            for j in 0..arr2.len() {
                dest[i + j] += arr1[i] * arr2[j];
            }
        }
    }
}

pub struct ScalarMultiplyF32;

impl CPUOperation for ScalarMultiplyF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar: f32 = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x *= scalar;
        }
    }
}


pub struct SinF32;

impl CPUOperation for SinF32 {
    fn run(&mut self, data: &mut Data) {
        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x = x.sin();
        }
    }
}

pub struct CosF32;

impl CPUOperation for CosF32 {
    fn run(&mut self, data: &mut Data) {
        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x = x.cos();
        }
    }
}

pub struct SqrtF32;

impl CPUOperation for SqrtF32 {
    fn run(&mut self, data: &mut Data) {
        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x = x.sqrt();
        }
    }
}

pub struct ModF32;

impl CPUOperation for ModF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x %= scalar;
        }
    }
}

pub struct AddF32;

impl CPUOperation for AddF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr2.iter_mut().enumerate() {
            *x += arr1[index];
        }
    }
}

pub struct ScalarAddF32;

impl CPUOperation for ScalarAddF32 {
    fn run(&mut self, data: &mut Data) {
        let scalar: f32 = *data.f32_const[0].read().unwrap();

        for x in data.f32_arrays[0].lock().unwrap().iter_mut() {
            *x += scalar;
        }
    }
}

pub struct CopyF32;

impl CPUOperation for CopyF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let arr1 = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let arr2 = binding.as_mut_slice();

        // run
        for (index, x) in arr1.iter().enumerate() {
            arr2[index] = *x;
        }
    }
}

pub struct FetchF32;

impl CPUOperation for FetchF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let src = binding.as_slice();

        let mut binding = data.f32_arrays[1].lock().unwrap();
        let indexes = binding.as_mut_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let dest = binding.as_mut_slice();

        // run
        for (dest_index,src_index) in indexes.iter().enumerate(){
            dest[dest_index] = src[*src_index as usize];
        }
    }
}

pub struct DFTF32;

impl CPUOperation for DFTF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let i_src = binding.as_slice();

        let binding = data.f32_arrays[1].lock().unwrap();
        let q_src = binding.as_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let i_dest = binding.as_mut_slice();

        let mut binding = data.f32_arrays[3].lock().unwrap();
        let q_dest = binding.as_mut_slice();
        let len = i_src.len();
        let step_save = (-2.0 * PI) / len as f32;

        // run
        for k in 0..len {
            i_dest[k] = 0.0;
            q_dest[k] = 0.0;
            let step = step_save * k as f32;
            for n in 0..len {
                let phi = step * n as f32;

                // Set i value
                i_dest[k] += i_src[n] * phi.cos() - q_src[n] * phi.sin();
                q_dest[k] += i_src[n] * phi.sin() + q_src[n] * phi.cos();
            }
        }
    }
}

pub struct IDFTF32;

impl CPUOperation for IDFTF32 {
    fn run(&mut self, data: &mut Data) {
        let binding = data.f32_arrays[0].lock().unwrap();
        let i_src = binding.as_slice();

        let binding = data.f32_arrays[1].lock().unwrap();
        let q_src = binding.as_slice();

        let mut binding = data.f32_arrays[2].lock().unwrap();
        let i_dest = binding.as_mut_slice();

        let mut binding = data.f32_arrays[3].lock().unwrap();
        let q_dest = binding.as_mut_slice();

        let len = i_src.len();

        let pi_2: f32 = (2.0 * PI) / len as f32;

        // run
        for n in 0..len {
            i_dest[n] = 0.0;
            q_dest[n] = 0.0;

            for k in 0..len {
                let phi = pi_2 * n as f32 * k as f32;

                // Set i value
                i_dest[n] += (i_src[k] * phi.cos()) - (q_src[k] * phi.sin());
                q_dest[n] += (i_src[k] * phi.sin()) + (q_src[k] * phi.cos());
            }

            i_dest[n] /= len as f32;
            q_dest[n] /= len as f32;
        }
    }
}