use std::fs;

// An Example has a label (what digit it is), and pixels
struct Example {
    label: u8,
    pixels: Vec<u8>
}

fn main() {
    println!("starting");

    let contents =
        fs::read_to_string("sample.csv")
            .expect("Failed to read file");

    let mut sample: Vec<Example> = Vec::new();

    let file_lines = contents.lines();
    for line in file_lines {

        let blocks = line.split(',');
        let mut numbers: Vec<u8> = Vec::new();

        for block in blocks {
            let n =
                block
                    .parse::<u8>()
                    .expect("Failed to convert to number");
            numbers.push(n);
        }

        let label = numbers[0];
        let pixels: Vec<u8> = numbers[1..].to_vec();

        let example = Example {
            label: label,
            pixels: pixels
        };

        sample.push(example);
    }

    for example in sample {
        let label = example.label;
        let first_pixel = example.pixels[0];
        let last_pixel = example.pixels[example.pixels.len() - 1];
        println!("Label = {label}, pixels = {first_pixel}, ..., {last_pixel}");
    }

    println!("finished");
}
