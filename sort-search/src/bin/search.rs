extern crate sort_search;

use sort_search::algos;
use sort_search::utils;

fn main() {
    let count = utils::get_i32("Enter length of item to create: ");
    let max = utils::get_i32("Enter the max value in the list: ");

    let vec = utils::make_random_vec(count, max);
    utils::print_vec(&vec, 40);

    loop {
        let target = utils::get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }
        let (index, search_count) = algos::linear_search(&vec, target);
        println!("numbers[{index}] = {target}, {search_count} tests");

        let (index, search_count) = algos::binary_search(&vec, target);
        println!("numbers[{index}] = {target}, {search_count} tests");
    }
}
