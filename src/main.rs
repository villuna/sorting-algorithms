extern crate rand;
extern crate stopwatch;

mod sort;
mod vis;

use rand::Rng;
use sort::{ bubble_sort, selection_sort, comb_sort, quick_sort, heap_sort, merge_sort };
use std::env;

fn main() {
    let mut rng = rand::thread_rng();
    let mut l = [0; 30];

    for i in 0..l.len() {
        l[i] = i;
    }

    let mut args: Vec<String> = env::args().collect();
    
    // Remove the name argument
    args.remove(0);

    if args.len() == 0 {
        args = vec![
            String::from("bubble"), String::from("selection"), String::from("comb"), String::from("quick"), String::from("heap"), String::from("merge")
        ];
    }

    for arg in args.iter() {
        rng.shuffle(&mut l);
        
        handle_arg(&arg, &mut l)
    }
}

fn handle_arg(arg: &str, l: &mut [usize]) {
    match arg {
        "help" => {
            println!("Actions (if none are specified the program will cycle through all algorithms)\n");
            println!("bubble:\t\tBubble Sort");
            println!("selection:\tSelection Sort");
            println!("comb:\t\tComb Sort");
            println!("quick:\t\tQuick Sort");
            println!("heap:\t\tHeap Sort");
            println!("merge:\t\tMerge Sort");
            std::process::exit(0);
        },

        "bubble"    => bubble_sort(l),
        "selection" => selection_sort(l),
        "comb"      => comb_sort(l),
        "quick"     => quick_sort(l),
        "heap"      => heap_sort(l),
        "merge"     => merge_sort(l),

        _ => {},
    }
}
