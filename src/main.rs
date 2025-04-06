use indicatif::ProgressBar;
use std::env;
use std::fs::File;
use std::io::Write;
mod vectors;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", &args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    println!("{}", file_name);
    let mut file = File::create(file_name)?;

    const RGB_SCALER: f64 = 255.999;
    let image_width: i16 = 256;
    let image_height: i16 = 256;
    let bar = ProgressBar::new((image_height - 1 as i16) as u64);

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let r: f64 = i as f64 / ((image_width - 1) as f64);
            let g: f64 = j as f64 / ((image_height - 1) as f64);
            let b: f64 = 0.0;

            let ir = RGB_SCALER * r as f64;
            let ig = RGB_SCALER * g as f64;
            let ib = RGB_SCALER * b as f64;

            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::vectors;
    #[test]
    fn vectors() {
        let a: vectors::Vec3 = vectors::Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        };

        let b: vectors::Vec3 = vectors::Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        println!("{:?}", a + b);
    }
}
