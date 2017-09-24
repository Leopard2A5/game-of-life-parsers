# Game of Life parsers

[![Build Status](https://travis-ci.org/Leopard2A5/game-of-life-parsers.svg?branch=master)](https://travis-ci.org/Leopard2A5/game-of-life-parsers)
[Documentation](https://docs.rs/game-of-life-parsers/)

Collection of parsers for Conway's game of life.

Currently supported file formats:

* Life 1.05

## Usage
```rust
extern crate game_of_life_parsers;
// use std::fs::File;
use game_of_life_parsers::{Parser, Life105Parser};

fn main() {
    // let file = File::open("file.life").unwrap();
    let file = "#N\n#P 0 0\n..*".as_bytes();
    let mut parser = Life105Parser::new();
    let game_descriptor =  parser.parse(file).unwrap();
    for live_cell in game_descriptor.live_cells() {
        // iterate overe live cells
    }
}
```
