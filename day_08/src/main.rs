use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part_1(&input)?;
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let mut pixels: Vec<char> = vec![];
    // Image is 25 pixels wide and 6 pixels tall
    let width = 25;
    let height = 6;
    let mut min_zeroes = ::std::usize::MAX;
    let mut min_zeroes_layer = 0;

    for pixel in input.trim().chars() {
        pixels.push(pixel);
    }
    if pixels.len() % (width * height) != 0 {
        err("Input length doesn't match assume width * height")?;
    }

    for i in 0..(pixels.len() / (width * height)) {
        let current_zeroes = pixels[(i * width * height)..((i + 1) * width * height)].iter().filter(|&&v| v == '0').count();
        if current_zeroes < min_zeroes {
            min_zeroes = current_zeroes;
            min_zeroes_layer = i;
        }
    }

    let result = pixels[(min_zeroes_layer * width * height)..((min_zeroes_layer + 1) * width * height)].iter().filter(|&&v| v == '1').count() * pixels[(min_zeroes_layer * width * height)..((min_zeroes_layer + 1) * width * height)].iter().filter(|&&v| v == '2').count();

    writeln!(io::stdout(), "Part 1 : {}", result)?;
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let mut pixels: Vec<char> = vec![];
    // Image is 25 pixels wide and 6 pixels tall
    let width = 25;
    let height = 6;
    let mut layers: Vec<Vec<char>> = vec![];

    for pixel in input.trim().chars() {
        pixels.push(pixel);
    }
    if pixels.len() % (width * height) != 0 {
        err("Input length doesn't match assume width * height")?;
    }

    for i in 0..(pixels.len() / (width * height)) {
        layers.push(pixels[(i * width * height)..((i+1) * (width * height))].to_vec());
    }

    let mut result: Vec<u8> = vec![];
    'outer: for i in 0..(width * height) {
        'inner: for layer in &layers {
            if layer[i] == '0' {
                result.push(0);
                continue 'outer;
            } else if layer[i] == '1' {
                result.push(255);
                continue 'outer;
            }
        }
        result.push(127);
    }

    use image::ColorType;
    use image::png::PNGEncoder;
    use std::fs::File;

    let image_file_path = format!("{}/part_2.png", env!("CARGO_MANIFEST_DIR"));
    let output = File::create(image_file_path.clone())?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(&result, width as u32, height as u32, ColorType::Gray(8))?;

    writeln!(io::stdout(), "Part 2 : To get result, open following image : \"{}\"", image_file_path)?;
    Ok(())
}

fn err(s: &str) -> Result<()> {
    Err(Box::<dyn Error>::from(s.to_string()))
}
