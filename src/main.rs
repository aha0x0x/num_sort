use std::time::SystemTime;
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return
    }

    let input = Path::new(&args[1]);
    let input_file : File = File::open(input).expect("input not found");

    let start = SystemTime::now();

    let br = BufReader::new(input_file);
    let mut numbers : Vec<i32> = br.lines().map(|s| s.unwrap().parse().unwrap()).collect();
    numbers.sort();

    let mut output = env::temp_dir();
    output.push("rsorted-".to_string() + input.file_name().unwrap().to_str().unwrap());

    let output_file : File = File::create(output.as_path()).expect("failed to create output");
    let mut bw = BufWriter::new(output_file);

    // find a better way
    let snumbers : Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    for snumber in &snumbers {
        bw.write(snumber.as_bytes()).unwrap();
        bw.write(b"\n").unwrap();
    }

    bw.flush().unwrap();
    let end = SystemTime::now();
    let difference = end.duration_since(start)
        .expect("SystemTime::duration_since failed");

    println!("input: {:?}", input);
    println!("output: {:?}", output);
    println!("took {:?} ", difference);

}
