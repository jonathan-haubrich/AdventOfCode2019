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

    let mut output = String::new();

    for i in (0..height).rev() {
        for j in 0..width {
            for k in 0..image.layers {
                match image.data[k][i][j] {
                    0 => {
                        output.push('0');
                        break;
                    },
                    1 => {
                        output.push('1');
                        break;
                    },
                    2 => continue,
                    _ => panic!("Invalid character in layer {} at position {}, {}", k, j, i)
                }
            }
        }
    }

    let image = Image::new(width, height, &output);
    image.print();
}
