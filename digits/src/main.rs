use std::fs;

fn main() {
    println!("starting");
    let contents =
        fs::read_to_string("sample.csv")
            .expect("Failed to read file");

    let file_lines = contents.lines();
    for line in file_lines {
        let blocks = line.split(',');
        for block in blocks {
            println!("{block}");
        }
        println!("{line}");
    }

    println!("finished");
}
