use rand;

use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use std::marker::PhantomData;

fn main() {
    //let mut input_line = String::new();
    // println!("Input amount of numbers to sort:");
    // let amount = match std::io::stdin().read_line(&mut input_line){
    //     Ok(_) => i64::from_str_radix(&input_line.trim(), 10).unwrap(),
    //     Err(_) => panic!("Error while reading amount of values")
    // };
    let amount = 1_000_000;

    // let mut rnd = rand::thread_rng();
    let mut arr: Vec<i64> = Vec::new();
    for _ in 0..amount {
        arr.push( rand::random::<i64>() )
    }

    // println!("Vector before sort:");
    // for elem in &arr {
    //     println!("{}", elem);
    // }

    merge_sort(&mut arr);

    // println!("Vector after sort:");
    // for elem in &arr {
    //     println!("{}", elem);
    // }
}

fn merge_sort(arr: &mut Vec<i64>) {
    let arr_len = arr.len();
    let arr_slice = arr.as_mut_slice();

    // simple_merge_sort(arr, 0 as usize, arr_len - 1 as usize);

    let rc = Rc::new(arr);
    par_merge_sort(rc, 0 as usize, arr_len - 1 as usize, 4);
}

fn simple_merge_sort(arr: &mut Vec<i64>, lo: usize, hi: usize) {
    if lo == hi {
        return;
    }

    let mi = (hi + lo) / 2;
    simple_merge_sort(arr, lo, mi);
    simple_merge_sort(arr, mi + 1, hi);

    merge(arr, lo, mi, hi);
}

fn par_merge_sort(rc: &mut Rc<&mut Vec<i64>>, lo: usize, hi: usize, threads: i32) {
    if lo == hi {
        return;
    }

    let mi = (hi + lo) / 2_usize;
    if threads == 1 {
        simple_merge_sort(&mut rc, lo, hi);
    } else { //if threads == 2 {
        let thread1_rc = *Rc::from(rc);
        let thread2_rc = *Rc::from(rc);
        let thread_rest = threads / 2;
        let thread_rest_2 = threads - thread_rest;
        let thread1 = thread::spawn( move || {
            par_merge_sort(thread1_rc, lo, mi, thread_rest);
        });
        let thread2 = thread::spawn( move || {
            par_merge_sort(thread2_rc, mi + 1, hi, thread_rest_2);
        });

        thread1.join().unwrap();
        thread2.join().unwrap();
    }

    merge(&mut rc, lo, mi, hi);
}

fn merge(arr: &mut Vec<i64>, lo: usize, mi: usize, hi: usize) {
    let mut lo_arr: Vec<i64> = Vec::new();
    for i in lo..(mi + 1) {
        let elem = *arr.get(i).unwrap();
        lo_arr.push(elem);
    }

    let mut hi_arr: Vec<i64> = Vec::new();
    for i in (mi + 1)..(hi + 1) {
        let elem = *arr.get(i).unwrap();
        hi_arr.push(elem);
    }

    let mut i = 0;
    let mut j = 0;
    let mut counter = lo;

    while i < lo_arr.len() && j < hi_arr.len() {
        let elem_i = *lo_arr.get(i).unwrap();
        let elem_j = *hi_arr.get(j).unwrap();

        if elem_i <= elem_j {
            arr[counter] = elem_i;
            i += 1;
        } else { // elem_j <= elem_i
            arr[counter] = elem_j;
            j += 1;
        }
        counter += 1;
    }

    if j == hi_arr.len() {
        while i < lo_arr.len() {
            let elem_i = *lo_arr.get(i).unwrap();
            arr[counter] = elem_i;
            i += 1;
            counter += 1;
        }
    }
    else { // i == lo_arr.len()
        while j < hi_arr.len() {
            let elem_j = *hi_arr.get(j).unwrap();
            arr[counter] = elem_j;
            j += 1;
            counter += 1;
        }
    }
}
