mod integer_vector;
mod pig_latin;

fn main() {
    let int_vec =
        integer_vector::integer_vector::RandomIntegerVector::build_random_integer_vector();

    println!("The generated integer array is {:?}", int_vec.get_int_vec());
    println!("The mean of integer array is {}", int_vec.get_mean());
    println!("The median of integer array is {}", int_vec.get_median());
    println!("The mode of integer array is {}", int_vec.get_mode());

    let consonant = pig_latin::pig_latin::PigLatin::new("first");
    let vowel = pig_latin::pig_latin::PigLatin::new("apple");

    println!("The pig latin of consonant \"first\" is: {}", consonant.to_pig_latin());
    println!("The pig latin of consonant \"apple\" is: {}", vowel.to_pig_latin());
}
