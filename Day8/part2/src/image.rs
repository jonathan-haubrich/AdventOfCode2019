
#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<Vec<u32>>>,
    pub layers: usize
}

impl Image {
    pub fn new(width: usize, height: usize, data: &String) -> Self {
        if data.len() % (width * height) != 0 {
            panic!("Image data does not match dimensions given");
        }

        let mut image_data: Vec<Vec<Vec<u32>>> = Vec::new();

        let layers = data.len()/(width*height);
        let layer_length = width*height;

        for layer in 0..layers {
            image_data.push(Vec::new());
            let current_layer_start = layer * layer_length;
            for i in 0..height {
                let slice_start = current_layer_start + (i * width);
                let slice_end = slice_start + width;
                image_data[layer].push(data[slice_start..slice_end].chars().map(|c| { c.to_digit(10).unwrap() }).collect());
            }
        }

        Self {
            width,
            height,
            data: image_data,
            layers
        }
    }

    pub fn print(&self) {
        for row in self.data[0].iter().rev() {
            for pixel in row {
                match *pixel {
                    0 => print!("  "),
                    1 => print!("{} ", 1),
                    _ => {}
                };
            }
            println!();
        }
    }
}
