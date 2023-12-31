// static SAMPLE_RATE: f32 = 1e5;
// static BAUD_RATE: f32 = 1e4;
//
// #[test]
// pub fn gaussian() {
//     let samples_per_symbol = (SAMPLE_RATE / BAUD_RATE) as usize;
//     let mods = Modulators::new(samples_per_symbol, SAMPLE_RATE, 1000.0);
//
//     let ones = mods.fsk(&[255]);
//     let zeros = mods.fsk(&[0]);
//
//     let noisy_ones = noise_generators::gaussian_noise_generator(&*ones, 25.0).unwrap();
//     let noisy_zeros = noise_generators::gaussian_noise_generator(&*ones, 25.0).unwrap();
//
//     assert_ne!(ones, noisy_ones);
//     assert_ne!(zeros, noisy_zeros);
// }