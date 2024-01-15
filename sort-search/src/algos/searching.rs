pub mod linear {
    pub fn search(vec: &[i32], target: i32) -> (i32, i32) {
        for (i, _) in vec.iter().enumerate() {
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
    use std::cmp;

    pub fn search(vec: &[i32], target: i32) -> (i32, i32) {
        let mut sorted_vec = vec.to_owned();
        sorting::quick::sort(&mut sorted_vec);

        let mut l: i32 = 0;
        let mut r: i32 = (sorted_vec.len() - 1) as i32;
        let mut count = 0;
        while l <= r {
            count += 1;
            let m: usize = (l as usize + r as usize) / 2;

            match sorted_vec[m].cmp(&target) {
                cmp::Ordering::Less => l = m as i32 + 1,
                cmp::Ordering::Greater => r = m as i32 - 1,
                cmp::Ordering::Equal => return (m as i32, count),
            }
        }
        (-1, -1)
    }
}
