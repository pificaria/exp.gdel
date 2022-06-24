use num_complex::Complex;
use chfft::CFft1D;
use quadrature::double_exponential::integrate;

const SAMPLE_RATE: u32 = 48000;
const ABS_ERROR: f64 = 1e-12;

/// TODO DOC
fn phase<F>(tau: F, w: f64) -> f64 where F: Fn(f64) -> f64 {
    -integrate(&tau, 0., w, ABS_ERROR).integral
}

/// TODO DOC
fn generate_ir<F>(opa: bool, samples: usize, tau: F) -> Vec<f64> where F: Fn(f64) -> f64 {
    use std::f64::consts::PI;
    let r_n = (samples as f64).recip();
    let h = samples / 2;
    let lim = if opa { samples } else { h };
    let mut buf: Vec<Complex<f64>> = vec![Complex{ re: 0., im: 0. }; samples];
    for k in 0..lim {
        let r = Complex::new(0., phase(&tau, (k as f64)*r_n) * 2. * PI).exp();
        buf[k] = r;
    }

    /*for k in (h+1)..samples {
        //let x = - ((samples - k) as f64) * r_n;
        let r = Complex::new(0., phase(&tau, x*r_n) * 2. * PI).exp();
        buf[k] = r;
    }*/

    let mut fft = CFft1D::<f64>::with_len(samples);
    fft.backward(buf.as_slice()).iter().map(|x| x.re).collect()
}

fn write_wav(path: &str, vec: &[f64]) -> hound::Result<()> {
    use std::i16;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    for x in vec {
        let sample = ((i16::MAX as f64) * x) as i16;
        writer.write_sample(sample)?;
    }

    writer.finalize()
}

/*fn help(arg0: &str) -> ! {
    use std::process::exit;
    println!("{} output [n] [opa]", arg0);
    exit(1)
}

fn main() {
    use std::env::args;

    let a: Vec<String> = args().collect();
    if a.len() < 2 {
        help(&a[0]);
    } 
    
    let mut time = 16;
    let mut opa = false;
    let path = &a[1];
    if a.len() >= 3 {
        if let Ok(v) = a[2].parse::<u32>() {
            time = v.max(4).min(20);
        }
    }

    if a.len() >= 4 {
        if let Ok(v) = a[3].parse::<bool>() {
            opa = v;
        }
    }

    let n = 2_usize.pow(time);
    println!("[-] generating IR of {} samples ({:.3} seconds) at {}; opa: {:?}", n, (n as f64 / SAMPLE_RATE as f64), path, opa);
    let ir = generate_ir(opa, n, tau::g_tau);
    println!("[-] okay! now writing wav file");
    match write_wav(path, ir.as_slice()) {
        Ok(_) => println!("[-] done!"),
        Err(e) => println!("[-] error: {:?}", e),
    }
}*/

pub fn gdel_writer<F>(path: &str, power: u32, opa: bool, tau: F) where F: Fn(f64) -> f64 {
    let n = 2_usize.pow(power);
    println!("[-] generating IR of {} samples ({:.3} seconds) at {}; opa: {:?}", n, (n as f64 / SAMPLE_RATE as f64), path, opa);
    let ir = generate_ir(opa, n, tau);
    println!("[-] okay! now writing wav file");
    match write_wav(path, ir.as_slice()) {
        Ok(_) => println!("[-] done!"),
        Err(e) => println!("[-] error: {:?}", e),
    }
}
