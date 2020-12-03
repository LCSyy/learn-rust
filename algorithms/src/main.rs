use rand;
use algorithms::{
    selection_sort,
    bubble_sort,
    insertion_sort,
    shell_sort,
    merge_sort,
    qsort,
};

const NUM_SIZE: usize = 100;

fn random_arrays(c: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for _ in 0..c {
        v.push(rand::random::<i32>());
    }

    v
}

fn main() {
    // println!("seletion:");
    // let mut a = random_arrays(NUM_SIZE);
    // selection_sort(&mut a);
    // for n in a.iter() {
    //     print!("{} ", n);
    // }
    // println!("");

    // println!("bubble sort:");
    // let mut a = random_arrays(NUM_SIZE);
    // bubble_sort(&mut a);
    // for n in a.iter() {
    //     print!("{} ", n);
    // }
    // println!("");

    // println!("insertion sort:");
    // let mut a = random_arrays(NUM_SIZE);
    // insertion_sort(&mut a);
    // for n in a.iter() {
    //     print!("{} ", n);
    // }
    // println!("");
    
    // println!("shell sort:");
    // let mut a = random_arrays(NUM_SIZE);
    // shell_sort(&mut a);
    // for n in a.iter() {
    //     print!("{} ", n);
    // }
    // println!("");

    // println!("merge sort:");
    // let mut a = random_arrays(NUM_SIZE*10);
    // merge_sort(&mut a);
    // for n in a.iter() {
    //     print!("{} ", n);
    // }
    // println!("");

    println!("quick sort:");
    let mut a = random_arrays(NUM_SIZE);
    qsort(&mut a);
    for n in a.iter() {
        print!("{} ", n);
    }
    println!("");
}
