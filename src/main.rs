use num_complex::Complex;
use rustfft::{FftPlanner, Fft};

fn main() {
    // Generate a Gaussian function with complex variance
    let points = gaussian_wave(Complex::new(0.0, 1.0), 100);

    // Apply Fourier Transform to the Gaussian data
    let ft_points = apply_fourier_transform(&points);

    println!("Fourier Transform Output: {:?}", ft_points);
}

/// Generates a Gaussian wave with a specified complex variance and size
fn gaussian_wave(variance: Complex<f64>, size: usize) -> Vec<Complex<f64>> {
    let mut wave = Vec::new();
    let mu = Complex::new(0.0, 0.0);  // Assume mean is at the origin
    for i in 0..size {
        let x = Complex::new(i as f64, 0.0);  // Sample points along the real axis
        let exponent = -((x - mu).norm_sqr()) / (2.0 * variance.re);  // Gaussian exponent
        wave.push(Complex::new(exponent.exp(), 0.0));  // Create Gaussian amplitude
    }
    wave
}

/// Applies a Fourier Transform to the input data using rustfft
fn apply_fourier_transform(data: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(data.len());
    let mut buffer = data.to_vec();
    fft.process(&mut buffer);
    buffer
}