pub mod company {
    use std::collections::HashMap;

    pub struct EmployeeDescription {
        name: String,
        department: String
    }

    impl EmployeeDescription {
        pub fn new(description_str: &str) -> EmployeeDescription {
            let desc_list: Vec<&str> = description_str.split(' ').collect();
            if desc_list.len() != 4 {
                panic!("Invalid description string, \
                    format: \"Add {employee name} to {department name}\"")
            }

            EmployeeDescription {
                name: desc_list[1].to_string(),
                department: desc_list[3].to_string()
            }
        }
    }

    pub struct Company {
        employee: HashMap<Department, People>
    }

    impl Company {
        pub fn new() -> Company {
            Company {
                employee: HashMap::new()
            }
        }

        pub fn add(&mut self, employee_description: EmployeeDescription) {
            let dept = Department {
                name: employee_description.department
            };

            let name = employee_description.name;
            let mutable_employee = &mut self.employee;
            match mutable_employee.get_mut(&dept) {
                Some(people) => {
                    people.add_people_by(name);
                },
                None => {
                    let mut people = People {
                        people_list: Vec::new()
                    };
                    people.add_people_by(name);
                    mutable_employee.insert(dept, people);
                }
            }
        }

        pub fn get_all_people_by(&self, department_name: String) -> Vec<String> {
            match self.employee.get(&Department {name: department_name}) {
                Some(people) => {people.people_list.clone()},
                None => {Vec::new()}
            }
        }

        pub fn get_all_employees(&self) -> Vec<String> {
            let mut employees: Vec<String> = Vec::new();
            for (dept, people) in &self.employee {
                let mut names = String::new();
                for name in people.people_list.clone() {
                    names = names + " " + &name;
                }
                employees.push(dept.name.clone() + ":" + &names);
            }

            employees.sort_by(|a, b| a.cmp(b));
            employees
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    struct Department {
        name: String
    }

    struct People {
        people_list: Vec<String>
    }

    impl People {
        fn add_people_by(&mut self, name: String) {
            self.people_list.push(name);
        }
    }
}