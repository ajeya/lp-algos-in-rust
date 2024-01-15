extern crate sort_search;

use sort_search::algos;
use sort_search::utils;

use std::time::Instant;

fn main() {
    let count = utils::get_i32("Enter length of item to create: ");
    let max = utils::get_i32("Enter the max value in the list: ");
    let vec = utils::make_random_vec(count, max);
    utils::print_vec(&vec, 40);

    // Bubble sort
    let mut v = vec.clone();
    utils::check_sorted(&v);
    let start = Instant::now();
    algos::bubble_sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("bubble sorting done, time taken: {duration:?}");

    // quick sort
    let mut v = vec.clone();
    // let mut v = vec![94, 91, 93, 84, 82, 20];
    utils::check_sorted(&v);
    let start = Instant::now();
    algos::quick_sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("quick sorting done, time taken: {duration:?}");

    // counting sort
    let mut v = vec.clone();
    // let mut v = vec![94, 91, 93, 84, 82, 20];
    utils::check_sorted(&v);
    let start = Instant::now();
    algos::counting_sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("counting sorting done, time taken: {duration:?}");
}
