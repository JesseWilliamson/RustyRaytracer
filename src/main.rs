fn main() {
    const RGB_SCALER: f64 = 255.999;
    let image_width: i16 = 256;
    let image_height: i16 = 256;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f64 = i as f64 / ((image_width - 1) as f64);
            let g: f64 = j as f64 / ((image_height - 1) as f64);
            let b: f64 = 0.0;

            let ir = RGB_SCALER * r as f64;
            let ig = RGB_SCALER * g as f64;
            let ib = RGB_SCALER * b as f64;

            println!("{ir} {ig} {ib}");
        }
    }
}
