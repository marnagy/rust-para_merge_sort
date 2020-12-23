use rand;

// use std::rc::Rc;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use crossbeam::thread::Scope;

extern crate crossbeam;

fn main() {
    //let mut input_line = String::new();
    // println!("Input amount of numbers to sort:");
    // let amount = match std::io::stdin().read_line(&mut input_line){
    //     Ok(_) => i64::from_str_radix(&input_line.trim(), 10).unwrap(),
    //     Err(_) => panic!("Error while reading amount of values")
    // };
    let amount = 100;
    let verbose = true;
    let threads_amount = 0_usize;

    // let mut rnd = rand::thread_rng();
    let mut arr: Vec<i64> = Vec::new();
    for _ in 0..amount {
        arr.push(rand::random::<i64>())
    }

    if verbose {
        println!("Vector before sort:");
        for elem in &arr {
            println!("{}", elem);
        }
    }

    par_merge_sort(&mut arr, threads_amount);

    if verbose {
        println!("Vector after sort:");
        for elem in &arr {
            println!("{}", elem);
        }
    }
}


fn par_merge_sort(arr: &mut [i64], thread_count: usize) {
    if thread_count > 0 {
        parallel_merge_sort(arr, 4);
    }
    else{
        simple_merge_sort(arr);
    }
}

/// Used for easier use of merge sort.
/// Single-threaded recursive version.
fn merge_sort(arr: &mut [i64]) {
    par_merge_sort(arr, 0_usize);
}

fn simple_merge_sort(arr: &mut [i64]) {
    if arr.len() == 1 {
        return;
    }

    let mi = arr.len() / 2;
    let (left_arr, right_arr) = arr.split_at_mut(mi);

    simple_merge_sort(left_arr);
    simple_merge_sort(right_arr);

    merge(arr);
}

fn parallel_merge_sort(arr: &mut [i64], threads: i32) {
    let arr_length = arr.len();

    if arr_length == 1 {
        return;
    }

    let mi = arr_length / 2_usize;
    if threads == 1 {
        simple_merge_sort(arr);
    } else {
        //if threads == 2 {
        let (mut left_arr, mut right_arr) = arr.split_at_mut(mi);
        // let mut left_arc = Arc::new(Mutex::new(left_arr));
        // let mut right_arc = Arc::new(Mutex::new(right_arr));
        let thread_rest = threads / 2;
        let thread_rest_2 = threads - thread_rest;
        crossbeam::scope( move |s| {
            s.spawn(move |_| {
                parallel_merge_sort(&mut left_arr, thread_rest);
            });
            s.spawn(move |_| {
                parallel_merge_sort(&mut right_arr, thread_rest_2);
            });
        }).unwrap();

        // thread1.join().unwrap();
        // thread2.join().unwrap();
    }

    merge(arr);
}

fn merge(arr: &mut [i64]) {
    let mi = arr.len() / 2;
    let mut left_vec: Vec<i64> = Vec::new();
    for i in 0..mi {
        left_vec.push(arr[i].clone());
    }

    let mut right_vec: Vec<i64> = Vec::new();
    for i in mi..arr.len() {
        right_vec.push(arr[i].clone());
    }

    let left_arr_temp = left_vec.as_slice();
    let right_arr_temp = right_vec.as_slice();

    let mut i = 0;
    let mut j = 0;
    let mut counter = 0;

    while i < left_arr_temp.len() && j < right_arr_temp.len() {
        let elem_i = left_arr_temp[i];
        let elem_j = right_arr_temp[j];

        if elem_i <= elem_j {
            arr[counter] = elem_i;
            i += 1;
        } else {
            // elem_j <= elem_i
            arr[counter] = elem_j;
            j += 1;
        }
        counter += 1;
    }

    if j == right_arr_temp.len() {
        while i < left_arr_temp.len() {
            let elem_i = left_arr_temp[i];
            arr[counter] = elem_i;
            i += 1;
            counter += 1;
        }
    } else {
        // i == lo_arr.len()
        while j < right_arr_temp.len() {
            let elem_j = right_arr_temp[j];
            arr[counter] = elem_j;
            j += 1;
            counter += 1;
        }
    }
}
