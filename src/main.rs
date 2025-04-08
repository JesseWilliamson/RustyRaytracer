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

    let image_width: i16 = 256;
    let image_height: i16 = 256;
    let bar = ProgressBar::new((image_height - 1 as i16) as u64);

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_color = vectors::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            pixel_color.write_color(&mut file)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::vectors;
    #[test]
    fn vectors() {
        let a = vectors::Vec3::new(0.0, 3.0, 0.0);
        let b = vectors::Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(a + b, vectors::Vec3::new(1.0, 4.0, 1.0));
        assert_eq!(a - b, vectors::Vec3::new(-1.0, 2.0, -1.0));
        assert_eq!(b.length_squared(), 3.0);
        let c = vectors::Color::new(0.9, 0.1, 0.5);
        let mut stdout = std::io::stdout();
        c.write_color(&mut stdout).unwrap();
    }
}
