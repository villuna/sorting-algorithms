use vis::{ print_list_vis, clear };

use std::thread::sleep;
use std::time::Duration;

fn swap<T: Copy>(list: &mut [T], i1: usize, i2: usize) {
    let temp = list[i1];
    list[i1] = list[i2];
    list[i2] = temp;
}

fn min_index(list: &mut [usize], from: usize) -> usize {
    let mut m = from;
    
    for i in (from + 1)..list.len() {
        if list[i] < list[m] {
            m = i;
        }
        
        clear();
        println!(" Selection Sort\n");
        print_list_vis(list, vec![from as i32, m as i32, i as i32], Vec::new());
        sleep(Duration::from_millis(100));
    }

    return m;
}

pub fn bubble_sort(list: &mut [usize]) {
    let mut swapped = true;
    let mut len = list.len() - 1;

    clear();
    println!(" Bubble Sort\n");
    print_list_vis(list, Vec::new(), Vec::new());
    sleep(Duration::from_millis(1000));
    
    while swapped {
        swapped = false;

        for i in 0..len {
            clear();
            println!(" Bubble Sort\n");
            print_list_vis(list, vec![i as i32, (i + 1) as i32], Vec::new());
            sleep(Duration::from_millis(100));

            if list[i] > list[i + 1] {
                clear();
                println!(" Bubble Sort\n");
                print_list_vis(list, Vec::new(), vec![i as i32, (i + 1) as i32]);

                swap(list, i, i + 1);
                swapped = true;
                sleep(Duration::from_millis(100));
            }

        }

        if len > 0 {
            len -= 1;
        }
    }

    let r: Vec<i32> = (0..list.len() as i32).collect();
    clear();
    println!(" Bubble Sort\n");
    print_list_vis(list, Vec::new(), r);
    sleep(Duration::from_millis(2000));
}

pub fn selection_sort(list: &mut [usize]) {
    clear();
    println!(" Selection Sort\n");
    print_list_vis(list, Vec::new(), Vec::new());
    sleep(Duration::from_millis(1000));

    for i in 0..list.len() {
        let m = min_index(list, i);
        clear();
        println!(" Selection Sort\n");
        print_list_vis(list, vec![i as i32, m as i32], Vec::new());
        sleep(Duration::from_millis(100));

        swap(list, i, m);

        clear();
        println!(" Selection Sort\n");
        print_list_vis(list, Vec::new(), vec![i as i32, m as i32]);
        sleep(Duration::from_millis(100));
    }

    let r: Vec<i32> = (0..list.len() as i32).collect();
    clear();
    println!(" Selection Sort\n");
    print_list_vis(list, Vec::new(), r);
    sleep(Duration::from_millis(2000));
}

pub fn comb_sort(list: &mut [usize]) {
    clear();
    println!(" Comb Sort\n");
    print_list_vis(list, Vec::new(), Vec::new());
    sleep(Duration::from_millis(1000));

    let mut gap = list.len();
    let shrink: f64 = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f64 / shrink).floor() as usize;
        
        if gap > 1 {
            sorted = false;
        }

        else {
            gap = 1;
            sorted = true;
        }

        let mut i = 0;
        while i + gap < list.len() {
            clear();
            println!(" Comb Sort\n");
            print_list_vis(list, vec![i as i32, (i + gap) as i32], Vec::new());
            sleep(Duration::from_millis(100));

            if list[i] > list[i + gap] {
                clear();
                println!(" Comb Sort\n");
                print_list_vis(list, Vec::new(), vec![i as i32, (i + gap) as i32]);

                swap(list, i, i + gap);
                sorted = false;

                sleep(Duration::from_millis(100));
            }

            i += 1;
        }
    }

    let r: Vec<i32> = (0..list.len() as i32).collect();
    clear();
    println!(" Comb Sort\n");
    print_list_vis(list, Vec::new(), r);
    sleep(Duration::from_millis(2000));
}

// Quick Sort

pub fn quick_sort(list: &mut [usize]) {
    clear();
    println!(" QuickSort\n");
    print_list_vis(list, Vec::new(), Vec::new());

    let high = list.len();
    quicksort(list, 0, (high - 1) as i32);

    let r: Vec<i32> = (0..list.len() as i32).collect();
    clear();
    println!(" QuickSort\n");
    print_list_vis(list, Vec::new(), r);
    sleep(Duration::from_millis(2000));
}

fn quicksort(list: &mut [usize], lo: i32, hi: i32) {
    if lo < hi {
        let p = partition(list, lo as i32, hi as i32);
        quicksort(list, lo as i32, p - 1);
        quicksort(list, p + 1, hi as i32);
    }
}

fn partition (list: &mut [usize], low: i32, high: i32) -> i32 {
    let pivot = list[high as usize];
    let mut i = low - 1;
      
    for j in low..high {
        clear();
        println!(" QuickSort\n");
        print_list_vis(list, vec![pivot as i32, i, j], Vec::new());
        sleep(Duration::from_millis(100));

        if list[j as usize] <= pivot {
            i += 1;
            swap(list, i as usize, j as usize);

            clear();
            println!(" QuickSort\n");
            print_list_vis(list, vec![pivot as i32], vec![i, j]);
            sleep(Duration::from_millis(100));
        }
    }

    swap(list, (i + 1) as usize, high as usize);

    clear();
    println!(" QuickSort\n");
    print_list_vis(list, vec![pivot as i32], vec![i + 1, high]);
    sleep(Duration::from_millis(100));

    return i + 1;
}

// Heap Sort

pub fn heap_sort(list: &mut [usize]) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in (0..len/2).rev() {
        heapify(list, i, len);
    }

    for i in (1..len).rev() {
        swap(list, 0, i);
        heapify(list, 0, i);
    }

    let r: Vec<i32> = (0..list.len() as i32).collect();
    clear();
    println!(" Heap Sort\n");
    print_list_vis(list, Vec::new(), r);
    sleep(Duration::from_millis(2000));

}

fn heapify(list: &mut [usize], i: usize, max: usize) {
    let mut largest = i;
    let left = i * 2 + 1;
    let right = i * 2 + 2;

    clear();
    println!(" Heap Sort\n");
    print_list_vis(list, vec![largest as i32], Vec::new());
    sleep(Duration::from_millis(100));
    
    if left < max && list[left] > list[largest] {
        largest = left;

        clear();
        println!(" Heap Sort\n");
        print_list_vis(list, vec![i as i32, left as i32], Vec::new());
        sleep(Duration::from_millis(100));
    }

    if right < max && list[right] > list[largest] {
        largest = right;

        clear();
        println!(" Heap Sort\n");
        print_list_vis(list, vec![i as i32, right as i32], Vec::new());
        sleep(Duration::from_millis(100));

    }

    if largest != i {
        swap(list, largest, i);

        clear();
        println!(" Heap Sort\n");
        print_list_vis(list, Vec::new(), vec![largest as i32, i as i32]);
        sleep(Duration::from_millis(100));

        heapify(list, largest, max);
    }
}

// Merge sort


pub fn merge_sort(list: &mut [usize]) {
    let len = list.len();
    merge_sort_bounded(list, 0, len);
}

fn merge_sort_bounded(list: &mut [usize], from: usize, to: usize) {
    let len = to - from;

    if len <= 1 {
        return;
    }

    let mid: usize = to - (len / 2);

    merge_sort_bounded(list, from, mid);
    merge_sort_bounded(list, mid, to);

    merge(list, from, mid, to);
}

fn merge(list: &mut [usize], from: usize, mid: usize, to: usize) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    clear();
    println!(" Merge Sort\n");
    print_list_vis(list, vec![from as i32, mid as i32], Vec::new());
    sleep(Duration::from_millis(100));

    for i in from..mid {
        left.push(list[i]);
    }
    
    for i in mid..to {
        right.push(list[i]);
    }

    let mut res: Vec<usize> = Vec::new();
    
    while left.len() > 0 && right.len() > 0 {
        if left[0] <= right[0] {
            res.push(left.remove(0));
        }

        else {
            res.push(right.remove(0));
        }
    }

    while left.len() > 0 {
        res.push(left.remove(0));
    }

    while right.len() > 0 {
        res.push(right.remove(0));
    }

    for i in 0..res.len() {
        list[from + i] = res[i];

        clear();
        println!(" Merge Sort\n");
        print_list_vis(list, vec![from as i32, mid as i32], vec![(from + i) as i32]);
        sleep(Duration::from_millis(100));
    }
}
