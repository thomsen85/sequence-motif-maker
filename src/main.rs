use std::fs::File;
use std::io::Write;

use clap::Parser;
use rand::Rng;
use rand::seq::IndexedRandom;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "10000")]
    sequences: usize,

    #[clap(long, default_value = "50")]
    average_length: usize,

    #[clap(long, default_value = "0.03")]
    error_rate: f64,

    #[clap(long)]
    motif: String,

    #[clap(long)]
    output: String,

    #[clap(long, default_value = "true")]
    generate_header: bool,

    #[clap(long, default_value = "0")]
    random_n_start_sequences: usize,
}
fn main() {
    let args = Args::parse();

    let mut file = File::create(&args.output).expect("Could not create file");

    assert!(
        args.random_n_start_sequences <= args.sequences,
        "Number of random sequences cannot be greater than total sequences"
    );
    assert!(
        args.average_length > args.motif.len(),
        "Average length must be greater than motif length"
    );

    if args.random_n_start_sequences > 0 {
        for _ in 0..args.random_n_start_sequences {
            if args.generate_header {
                writeln!(file, ">TEST(+) HEADER").expect("Could not write to file");
            }

            let random_sequence = generate_sequence(args.average_length, 1., &args.motif);
            writeln!(file, "{}", random_sequence).expect("Could not write to file");
        }
    }

    for _ in 0..(args.sequences - args.random_n_start_sequences) {
        if args.generate_header {
            writeln!(file, ">TEST(+) HEADER").expect("Could not write to file");
        }

        let sequence = generate_sequence(args.average_length, args.error_rate, &args.motif);
        writeln!(file, "{}", sequence).expect("Could not write to file");
    }

    println!(
        "Generated {} sequences with average length of {} and error rate of {}",
        args.sequences, args.average_length, args.error_rate
    );
}

// >chr1:28111-30072(+) H168H3K4me3X1_peak_1

fn generate_sequence(average_length: usize, error_rate: f64, motif: &str) -> String {
    let mut sequence = String::new();
    let mut rng = rand::rng();

    let motif_poistion = rng.random_range(0..average_length - motif.len());

    for i in 0..(average_length - motif.len()) {
        if i == motif_poistion {
            for motif_base in motif.chars() {
                if rng.random_bool(error_rate) {
                    let random_base = ['A', 'C', 'G', 'T'].choose(&mut rng).unwrap();
                    sequence.push(*random_base);
                } else {
                    sequence.push(motif_base);
                }
            }
        } else {
            let random_base = ['A', 'C', 'G', 'T'].choose(&mut rng).unwrap();
            sequence.push(*random_base);
        }
    }

    sequence
}
