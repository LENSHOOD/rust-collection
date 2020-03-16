use rand::Rng;
use std::collections::HashMap;
use crate::display::Print;

pub struct RandomIntegerVector {
    int_vec: Vec<i32>
}

impl RandomIntegerVector {
    pub fn build_random_integer_vector() -> RandomIntegerVector {
        let mut vec: Vec<i32> = Vec::new();
        for _i in 0..rand::thread_rng().gen_range(5, 25) {
            let tmp = rand::thread_rng().gen_range(1, 101);
            vec.push(tmp);
        }

        RandomIntegerVector {
            int_vec: vec
        }
    }

    pub fn get_int_vec(&self) -> &Vec<i32> {
        &self.int_vec
    }

    pub fn get_mean(&self) -> f32 {
        let mut mean = 0;
        for i in &self.int_vec {
            mean += *i;
        }

        mean as f32 / self.int_vec.len() as f32
    }

    pub fn get_median(&self) -> i32 {
        let mut sorted_vec: Vec<i32> = self.int_vec.clone();
        sorted_vec.sort();

        let median_index: usize = sorted_vec.len() / 2;
        sorted_vec[median_index]
    }

    pub fn get_mode(&self) -> i32 {
        let mut mode_map = HashMap::new();
        for i in &self.int_vec {
            match mode_map.get(i) {
                Some(count) => {mode_map.insert(i, count + 1);},
                None => {mode_map.insert(i, 0);}
            }
        }

        let mut max = 0;
        let mut max_key = 0;
        for (k, v) in &mode_map {
            if *v > max {
                max = *v;
                max_key = **k;
            }
        }

        max_key
    }
}

impl Print for RandomIntegerVector {
    fn print(&self) -> String {
        return format!("The generated integer array is {:?}\n\
        The mean of integer array is {}\n\
        The median of integer array is {}\n\
        The mode of integer array is {}\n",
                self.get_int_vec(),
                self.get_mean(),
                self.get_median(),
                self.get_mode());
    }
}