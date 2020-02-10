mod integer_vector;

fn main() {
    let int_vec =
        integer_vector::integer_vector::RandomIntegerVector::build_random_integer_vector();

    println!("The generated integer array is {:?}", int_vec.get_int_vec());
    println!("The mean of integer array is {}", int_vec.get_mean());
    println!("The median of integer array is {}", int_vec.get_median());
    println!("The mode of integer array is {}", int_vec.get_mode());
}
