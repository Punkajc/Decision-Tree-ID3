Decision-Tree-ID3
===========

An implementation of the ID3 Decision Tree algorithm that just so happens to coincide with Homework 1 for Fall 2014 CSCE 478/878 at UNL.

#### How to Build
Make sure Rust and Cargo are installed. If you don't have them, use the following to get the Rust Nightly:
```
curl https://static.rust-lang.org/rustup.sh | sudo bash
```
Then get the project and build it:
```
git clone https://github.com/DrKwint/Decision-Tree-ID3.git
cargo build
```

#### Documentation
HTML documentation can be built with rustdoc, i.e.
```
rustdoc src/lib.rs
```

### Data
Data is taken from the [UCI Machine Learning Repository](http://archive.ics.uci.edu/ml/).

Currently bundled with:

* [Congressional Voting Records](http://archive.ics.uci.edu/ml/datasets/Congressional+Voting+Records)
* [Monks Problems](http://archive.ics.uci.edu/ml/datasets/MONK%27s+Problems)

#### Dependencies
* Rust - http://www.rust-lang.org/
* Cargo - http://crates.io/
* BurntSushi/rust-csv - https://github.com/BurntSushi/rust-csv

#### License
MIT License - fork, modify and use however you want.
