pub mod bubble {
    pub fn sort(vec: &mut Vec<i32>) {
        loop {
            let mut swapped = false;
            for j in 1..vec.len() {
                let i = j - 1;
                if vec[i] > vec[j] {
                    let tmp = vec[j];
                    vec[j] = vec[i];
                    vec[i] = tmp;
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

    fn swap(vec: &mut Vec<i32>, i: usize, j: usize) {
        let tmp = vec[j];
        vec[j] = vec[i];
        vec[i] = tmp;
    }

    fn partition(vec: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
        let pivot = vec[hi];

        let mut i = lo;

        for j in lo..hi {
            if vec[j] > pivot {
                continue;
            }

            // swap the values
            swap(vec, i, j);

            i += 1;
        }

        swap(vec, i, hi);
        // incase every number is larger than pivot

        i
    }

    fn sort_helper(vec: &mut Vec<i32>, lo: usize, hi: usize) {
        if lo < hi {
            let pivot_index = partition(vec, lo, hi);

            if pivot_index > 0 {
                sort_helper(vec, lo, pivot_index - 1);
            }
            sort_helper(vec, pivot_index + 1, hi)
        }
    }

    pub fn sort(vec: &mut Vec<i32>) {
        if vec.len() < 2 {
            return;
        }

        sort_helper(vec, 0, vec.len() - 1)
    }
}
