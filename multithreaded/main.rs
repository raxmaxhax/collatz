use dashmap::DashMap;
use std::collections::BTreeMap;
use std::env;
use std::sync::Arc;
use std::thread;

fn main() {
    let limit: Vec<String> = env::args().collect();
    let limit: u128 = limit[1].trim().parse().expect("Please enter a number");

    let tree = Arc::new(DashMap::new());

    tree.insert(1, 4);
    tree.insert(2, 1);
    tree.insert(3, 10);
    tree.insert(4, 2);

    let start_x: u128 = 5;

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as u128;

    let chunk_size = (limit - start_x + 1 + num_threads - 1) / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let thread_start = start_x + i * chunk_size;
        let thread_end = std::cmp::min(start_x + (i + 1) * chunk_size - 1, limit);

        if thread_start > limit {
            break;
        }

        let shared_tree = Arc::clone(&tree);

        handles.push(thread::spawn(move || {
            let mut x = thread_start;

            while x <= thread_end {
                let mut curr = x;

                while !shared_tree.contains_key(&curr) {
                    let next_val = if curr % 2 == 0 {
                        curr / 2
                    } else {
                        curr * 3 + 1
                    };
                  
                    shared_tree.insert(curr, next_val);
                    curr = next_val;
                }
                x += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let ordered: BTreeMap<u128, u128> = <DashMap<u128, u128> as Clone>::clone(&tree)
        .into_iter()
        .collect();

    println!("digraph G {{");
    for (key, value) in ordered {
        print!("{key} -> {value};");
    }
    println!("\n}}");
}
