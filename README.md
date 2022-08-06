# five_words

Solves the wordle-related puzzle presented in https://www.youtube.com/watch?v=_-AfhLQfb6w

The challenge is to find a set of five, five-letter words, such that no letter is used more than once.

This solution takes ~5s on my laptop. The "fast" solution presented in the video takes ~15 minutes.
We won't mention how long the original solution takes...

This solution uses a number of techniques to improve performance:

- Replacing anagrams with a canonical representation.
- Using a 16-bit index instead of the original word.
- Working with fixed size arrays where possible.
- Using a dense representation of graph edges to improve lookup speed.
- Using directed graph edges to avoid visiting duplicate solutions.
- Early exit if there are not enough words left to form a solution.
- Parallel iteration of the outer loop.

## Running

This is a Rust project. To compile it you must [install Rust](https://www.rust-lang.org/tools/install).

Once you have Rust installed, running the solution is simple:

```rust
cargo run --release
```

If you intend to benchmark the solution, take care not to include benchmarking the compilation step.
