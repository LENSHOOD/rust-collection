mod integer_vector;
mod pig_latin;
mod company;

use integer_vector::integer_vector::RandomIntegerVector;
use pig_latin::pig_latin::PigLatin;
use company::company::Company;
use crate::company::company::EmployeeDescription;

fn main() {
    let int_vec =
        RandomIntegerVector::build_random_integer_vector();

    println!("The generated integer array is {:?}", int_vec.get_int_vec());
    println!("The mean of integer array is {}", int_vec.get_mean());
    println!("The median of integer array is {}", int_vec.get_median());
    println!("The mode of integer array is {}", int_vec.get_mode());

    let consonant = PigLatin::new("first");
    let vowel = PigLatin::new("apple");

    println!("The pig latin of consonant \"first\" is: {}", consonant.to_pig_latin());
    println!("The pig latin of consonant \"apple\" is: {}", vowel.to_pig_latin());

    let mut company = Company::new();
    company.add(EmployeeDescription::new("Add Sally to Engineering"));
    company.add(EmployeeDescription::new("Add Bob to Engineering"));
    company.add(EmployeeDescription::new("Add Alex to Engineering"));
    company.add(EmployeeDescription::new("Add Amir to Sales"));
    company.add(EmployeeDescription::new("Add Jane to Sales"));
    company.add(EmployeeDescription::new("Add Sherry to Sales"));
    company.add(EmployeeDescription::new("Add Amy to IT"));
    company.add(EmployeeDescription::new("Add Charlie to IT"));

    let people_names =
        company.get_all_people_by(String::from("Engineering"));

    let mut names = String::new();
    for name in &people_names {
        names = names + " " + &name;
    }
    println!("The people in Department Engineering are: {}", names);

    let all_employees = company.get_all_employees();
    let mut employees = String::new();
    for employee in all_employees {
        employees = employees + "\n" + &employee;
    }
    println!("The all employees with department are: {}", employees);
}
