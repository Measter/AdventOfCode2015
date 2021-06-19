#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1525.txt")?;

    aoc_lib::run(
        &ALLOC,
        "Day 25: Let It Snow",
        &input,
        &|_| Ok("Not Implemented"),
        &|_| Ok("Not Implemented"),
    )
}

#[cfg(test)]
mod tests_1525 {
    use super::*;
}
