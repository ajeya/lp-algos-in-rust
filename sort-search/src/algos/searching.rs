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
