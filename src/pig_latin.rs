pub mod pig_latin {
    const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    pub struct PigLatin {
        original_word: String
    }

    impl PigLatin {
        pub fn new(str_literal: &str) -> PigLatin {
            PigLatin {
                original_word: String::from(str_literal)
            }
        }

        pub fn to_pig_latin(&self) -> String {
            let mut mut_str = self.original_word.clone();
            let first_letter = mut_str.remove(0);
            for vowel_letter in &VOWEL {
                if vowel_letter == &first_letter {
                    return first_letter.to_string() + &mut_str + "-hay"
                }
            }

            mut_str + "-" + &first_letter.to_string() + "ay"
        }
    }
}