# collatz
Collatz conjecture generator that formats the output for Graphviz, written in Rust.
<img width="950" height="262" alt="image" src="https://github.com/user-attachments/assets/5f8c4942-8849-40d8-a45f-722f1154f7cc" />
<img width="272" height="2144" alt="image" src="https://github.com/user-attachments/assets/c6ae2f78-1b4a-470f-8ee7-182d3e25f388" />


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

## Q&A
- How big numbers does this support?
  - By default, up to the u128 integer limit(340282366920938463463374607431768211455). You'll need a lot of ram to get there. u64 is also fine and you probably won't cross the u32 limit. However, if you want, you can use num_bigint and go up to 3.079 x 10^22212093154093428519. Of course, at this point it is silly.
- Will there be GPU acceleration?
  - I am currently working on a GPU accelerated implementation, which could be about 15X faster than the current fastest implementation, so hopefully, yes. If you would like to help me out let me know.
- Which version should I use?
  - Depends on your hardware, but most likely, you should use rayon as it is faster even on older hardware. You'll be fine no matter what you use.
