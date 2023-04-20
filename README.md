# AM-1

AM-1 is a command-line tool for calculating quotas for spaced repetition systems. Given a number of flashcards to study, a deadline for when they need to be studied by, and a number of boxes to divide the flashcards into, AM-1 calculates the number of new and review cards that should be studied each day to meet the deadline and keep up with the spaced repetition schedule.

## Installation

AM-1 is written in Rust, so you'll need to have Rust installed in order to use it. You can install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

Once Rust is installed, you can clone the repository and build AM-1 using Cargo, Rust's package manager:

```sh
git clone https://github.com/your_username/AM-1.git
cd AM-1
cargo build --release
```

## Usage

To use AM-1, run the binary with two command-line arguments: the deadline date in RFC3339 format, and the number of flashcards to study.

```sh
./target/release/am_1 '2023-05-05T00:00:00-00:00' 100
```

This will calculate the quotas for studying 100 flashcards by May 5th, 2023, and print them to the console.
Contributing

Contributions are welcome! Please submit bug reports and feature requests via the Issues tab, and feel free to submit pull requests for bug fixes or new features.


## Analysis
An analysis of the AM-1 algorithm by representing it as a Markov Chain is provided in the Mathematica file in this repository.

## Acknowledgements
This project was built for MATH UN2015 with Professor George Dragomir at Columbia University. We thank Professor Dragomir for a great semester and for the opportunity to do this project.
