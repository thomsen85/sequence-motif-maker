# Sequence Motif Maker

Tooling for generating test data for de-novo sequence motifs

## How to use

Use the --help to see options

```
cargo run --release -- --help

    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/sequence-motif-maker --help`
Usage: sequence-motif-maker [OPTIONS] --motif <MOTIF> --output <OUTPUT>

Options:
      --sequences <SEQUENCES>            [default: 10000]
      --average-length <AVERAGE_LENGTH>  [default: 50]
      --error-rate <ERROR_RATE>          [default: 0.03]
      --motif <MOTIF>
      --output <OUTPUT>
      --generate-header
  -h, --help                             Print help
```

## Example usage

```
cargo run --release -- --motif AACCGGTT --output test_data.fa --generate-header --sequences 10
```
