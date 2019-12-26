use std::env;
use std::fs;
use std::process;

mod image;
use image::Image;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <input> <width> <height>", args[0]);
        process::exit(1);
    }

    let input = fs::read_to_string(&args[1])
        .unwrap_or_else(|err| {
            eprintln!("Failed to open file {}: {}", args[1], err);
            process::exit(1);
        })
        .trim()
        .to_string();

    let width: usize = args[2].parse()
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse width ({}): {}", args[2], err);
            process::exit(1);
        });
    let height: usize = args[3].parse()
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse height ({}): {}", args[3], err);
            process::exit(1);
        });

    let image: Image = Image::new(width, height, &input);

    let mut min_zero_count = (0, std::u32::MAX);
    for (i, layer) in image.data.iter().enumerate() {
        let zero_count: u32 = layer.iter().map(|row| {
            row.iter().map(|i| {
                (*i == 0) as u32
            }).collect::<Vec<u32>>().iter().sum()
        }).collect::<Vec<u32>>().iter().sum();
        if zero_count < min_zero_count.1 {
            min_zero_count = (i, zero_count);
        }
    }

    println!("{:?}", min_zero_count);

    let target_index = min_zero_count.0;

    let target_layer = &image.data[target_index];

    let mut num_ones = 0;
    let mut num_twos = 0;
    target_layer.iter().for_each(|row| {
        row.iter().for_each(|i| {
            match i {
                1 => num_ones += 1,
                2 => num_twos += 1,
                _ => {}
            };
        })
    });

    println!("1s: {} 2s: {} Product: {}", num_ones, num_twos, num_ones * num_twos);
}
