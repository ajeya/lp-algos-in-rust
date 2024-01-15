extern crate sort_search;

use sort_search::algos::sorting;
use sort_search::utils;

use std::time::Instant;

fn main() {
    println!("initialize");

    let mut prng = utils::Prng::new();
    prng.randomize();

    println!("saving values into vector");
    let vv = utils::make_random_vec(100000, 10000);

    // Bubble sort
    let mut v = vv.clone();
    utils::check_sorted(&v);
    let start = Instant::now();
    sorting::bubble::sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("bubble sorting done, time taken: {duration:?}");

    // quick sort
    let mut v = vv.clone();
    // let mut v = vec![94, 91, 93, 84, 82, 20];
    utils::check_sorted(&v);
    let start = Instant::now();
    sorting::quick::sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("quick sorting done, time taken: {duration:?}");

    // counting sort
    let mut v = vv.clone();
    // let mut v = vec![94, 91, 93, 84, 82, 20];
    utils::check_sorted(&v);
    let start = Instant::now();
    sorting::counting::sort(&mut v);
    let duration = start.elapsed();
    utils::check_sorted(&v);

    println!("counting sorting done, time taken: {duration:?}");
}

// Unit tests
#[cfg(test)]
mod tests {
    use sort_search::algos::sorting;

    fn verify(vec: &mut Vec<i32>, sorted: &Vec<i32>) {
        let algos = vec![
            sorting::bubble::sort,
            sorting::quick::sort,
            sorting::counting::sort,
        ];
        for algo in algos {
            algo(vec);
            assert_eq!(sorted, vec)
        }
    }

    #[test]
    fn sort() {
        let mut vec = vec![0, 528, 183, 776, 372, 957, 235, 62, 468, 709, 991];
        let sorted = vec![0, 62, 183, 235, 372, 468, 528, 709, 776, 957, 991];

        verify(&mut vec, &sorted);
    }

    #[test]
    fn alread_sorted() {
        let mut vec = vec![0, 528, 183, 776, 372, 957, 235, 62, 468, 709, 991];
        let sorted = vec![0, 62, 183, 235, 372, 468, 528, 709, 776, 957, 991];

        verify(&mut vec, &sorted);
    }

    #[test]
    fn empty_vec() {
        let mut vec: Vec<i32> = vec![];
        let sorted: Vec<i32> = vec![];

        verify(&mut vec, &sorted);
    }
}
