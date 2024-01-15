pub mod linear {
    pub fn search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
        for i in 0..vec.len() {
            if vec[i] == target {
                return (i as i32, i as i32 + 1);
            }
        }
        (-1, -1)
    }
}

pub mod binary {
    use std::usize;

    use crate::algos::sorting;

    pub fn search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
        let mut sorted_vec = vec.clone();
        sorting::quick::sort(&mut sorted_vec);

        let mut l: i32 = 0;
        let mut r: i32 = (sorted_vec.len() - 1) as i32;
        let mut count = 0;
        while l <= r {
            count += 1;
            let m: usize = (l as usize + r as usize) / 2;

            if sorted_vec[m] < target {
                l = m as i32 + 1;
            } else if sorted_vec[m] > target {
                r = m as i32 - 1;
            } else {
                return (m as i32, count);
            }
        }
        return (-1, -1);
    }
}
