use std::fs;

// An Example has a label (what digit it is), and pixels
struct Example {
    label: u8,
    pixels: Vec<u8>
}

fn parse_example (line: &str) {
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

    Example {
        label: label,
        pixels: pixels
    };
    }

fn main() {
    println!("starting");

    let contents =
        fs::read_to_string("sample.csv")
            .expect("Failed to read file");

    let mut sample: Vec<Example> = Vec::new();

    // TODO: rewrite as a map
    // let file_lines = contents.lines();
    // let test =
    //     file_lines
    //         .skip(1)
    //         .map(parse_example);

    let file_lines = contents.lines();
    // skip 1 line (the headers)
    let file_lines = file_lines.skip(1);

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
