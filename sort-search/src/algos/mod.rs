pub mod bubble {
    pub fn sort(vec: &mut [i32]) {
        loop {
            let mut swapped = false;
            for j in 1..vec.len() {
                let i = j - 1;
                if vec[i] > vec[j] {
                    vec.swap(i, j);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

pub mod quick {

    fn partition(vec: &mut [i32]) -> usize {
        let lo = 0;
        let pivot = vec.len() - 1;

        let mut i = lo;

        for j in lo..pivot {
            if vec[j] > vec[pivot] {
                continue;
            }

            // swap the values
            vec.swap(i, j);

            i += 1;
        }

        vec.swap(i, pivot);
        i
    }

    pub fn sort(vec: &mut [i32]) {
        if vec.len() < 2 {
            return;
        }

        let pi = partition(vec);
        sort(&mut vec[0..pi]);
        sort(&mut vec[pi + 1..]);
    }
}

pub mod counting {
    use std::usize;

    pub fn sort(vec: &mut [i32]) {
        let max: i32 = match vec.iter().max() {
            Some(v) => v + 1,
            None => 0,
        };
        let mut count_vec: Vec<usize> = vec![0; max as usize];
        let mut sorted_vec: Vec<i32> = vec![0; vec.len()];

        for i in 0..vec.len() {
            count_vec[vec[i] as usize] += 1;
        }

        // iterate through the counts and add them to the previous ones
        for i in 1..count_vec.len() {
            count_vec[i] += count_vec[i - 1];
        }

        // set the values into sorted_vec:
        for i in 0..vec.len() {
            sorted_vec[count_vec[vec[i] as usize] - 1] = vec[i];
            count_vec[vec[i] as usize] -= 1;
        }

        // for i in 0..sorted_vec.len() {

        // vec[i] = sorted_vec[i];
        // }
        vec[..sorted_vec.len()].copy_from_slice(&sorted_vec[..]);
    }
}
