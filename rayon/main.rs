use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let limit: u128 = args[1].trim().parse().expect("Please enter a number");

    let tree = DashMap::new();
    tree.insert(1, 4);
    tree.insert(2, 1);
    tree.insert(3, 10);
    tree.insert(4, 2);

    let next = |a: u128| if a % 2 == 0 { a / 2 } else { a * 3 + 1 };

    (5..=limit).into_par_iter().for_each(|x| {
        let mut y = x;

        while !tree.contains_key(&y) {
            let z = next(y);

            tree.insert(y, z);

            y = z;
        }
    });

    let sorted: BTreeMap<u128, u128> = tree.into_iter().collect();

    println!("digraph G {{");

    for (key, value) in sorted {
        print!("{key} -> {value};");
    }

    println!("\n}}");
}
