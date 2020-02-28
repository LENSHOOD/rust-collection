mod integer_vector;
mod pig_latin;
mod company;
mod display;

use integer_vector::integer_vector::RandomIntegerVector;
use pig_latin::pig_latin::PigLatin;
use company::company::Company;
use crate::company::company::EmployeeDescription;
use display::display::Print;

fn main() {
    let int_vec =
        RandomIntegerVector::build_random_integer_vector();

    println!("{}", int_vec.print());

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
