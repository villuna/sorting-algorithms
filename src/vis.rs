extern crate colored;

use self::colored::*;
use std::{ thread, time };

pub fn print_list_vis(list: &mut [usize], yellow: Vec<i32>, green: Vec<i32>) {
    let columns = list.len();

    for i in (list[min(list)]..list[max(list)] + 1).rev() {
        print!(" ");

        for j in 0..list.len() {
            let item = list[j];

            if item == i {
                print!("{}", "█".white());
            }

            else if item > i {
                if yellow.contains(&(j as i32)) {
                    print!("{}", "█".yellow());
                }

                else if green.contains(&(j as i32)) {
                    print!("{}", "█".green());
                }
                
                else {
                    print!("{}", "█".red());
                }
            }
            
            else {
                print!(" ");
            }

            print!(" ");
        }
        
        println!("");
    }
}

fn max<T: PartialOrd>(list: &mut [T]) -> usize {
    let mut m = 0;
    
    for i in 1..list.len() {
        if list[i] > list[m] {
            m = i;
        }
    }
    
    return m;
}

fn min<T: PartialOrd>(list: &mut [T]) -> usize {
    let mut m = 0;
    
    for i in 1..list.len() {
        if list[i] < list[m] {
            m = i;
        }
    }
    
    return m;
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}
