extern crate rand;
extern crate stopwatch;

mod sort;
mod vis;

use rand::Rng;
use sort::{ bubble_sort, selection_sort, comb_sort, quick_sort, heap_sort };
use vis::print_list_vis;

fn main() {
    let mut rng = rand::thread_rng();
    let mut l = [0; 30];

    for i in 0..l.len() {
        l[i] = i;
    }

    rng.shuffle(&mut l);

    bubble_sort(&mut l);

    rng.shuffle(&mut l);

    selection_sort(&mut l);

    rng.shuffle(&mut l);
    
    comb_sort(&mut l);

    rng.shuffle(&mut l);

    quick_sort(&mut l);

    rng.shuffle(&mut l);

    heap_sort(&mut l);
}
