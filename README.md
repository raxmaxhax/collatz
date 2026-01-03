# collatz
Collatz conjecture generator that formats the output for Graphviz, written in Rust.
<img width="950" height="262" alt="image" src="https://github.com/user-attachments/assets/5f8c4942-8849-40d8-a45f-722f1154f7cc" />

## Performance
- base is the slowest and smallest
- multithreaded is the middleground
- rayon is the fastest and biggest

## Installation
- Download the raw file for your preferred version
- Make a new project with ```cargo new projectname```
- Replace the main.rs file with the one you downloaded.
- Compile with ```cargo build --release```

## Usage
- Run with ```cargo run --release -- x``` replacing x with your desired limit.
- Go to https://dreampuf.github.io/GraphvizOnline/
- Delete the existing digraph.
- Paste the output.
- Watch as the graph is generated.

## Advanced usage
- Instead of copying the output write it to a file.
- Instead of going to https://dreampuf.github.io/GraphvizOnline/ run Graphviz locally on the output.
