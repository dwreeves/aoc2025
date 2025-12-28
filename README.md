# AOC 2025

https://adventofcode.com/2025/

## Overview

I've been coding for over 10 years, but all in high-level languages.
I am interested in learning a lower-level language.
I'm not quite sure which one to pick.
I figured Rust was a good choice as it is quite popular in the Python ecosystem, so I can probably find some use for it.
I've also dabbled in Rust previously.

So far, I have solved **5/12** of the Advent of Code 2025 puzzles in Rust.

Stipulations:

- No vibe-coding, no cheating on solutions algorithmically.
- Rust docs and StackOverflow are allowed. Ignore Gemini Google search results.
- Claude can assist with compiler errors only.
- Use minimal 3rd party libs. So far, only Clap and Clippy.
- No copy + pasting across modules. (Get muscle memory down for typing Rust!) Peeking is fine.

The code here is almost certainly sloppy.
I'm interested in keeping this as a time capsule where I can one day look back and see my progress.

## Learnings

- **Day 1**: This day took the longest and involved the most learning! First, it took a while to get things running the way I want. I wanted a simple Clap-based setup without any frills. Most code examples show `clap::Parser` with a struct and not an enum. Second, I had to brush up on a lot of Rust things: impls, Results, match, iterators, and so on. You can see very much a "path of resistance" in this code, with plenty of questionable things like `map_err` and not using `Self` when I should have. I also relied a lot on Claude Code for this first day just to get everything compiling.
- **Day 2**: More `impl`, and also how to cast strings to numbers. (This is the last time I used Claude Code until day 5.)
- **Day 3**: Casting, constants, `usize`.
- **Day 4**: `saturating_sub()`.
- **Day 5**: I got extremely ambitious for this one. I got tired of just writing basic code and wanted to see how far I could push things. I learned: `by_ref()` for an iterator, `std::cmp::Ordering` + some other traits, and a few more things. I did have to ask Claude for a hint when I tried to use `Vec::binary_search_by` and it pointed me to `Vec::partition_point`. I also had a logical bug that drove me crazy and which I got stuck on that I just couldn't find, and Claude helped out (I was setting `j` to `0` instead of `1`-- oops!).
