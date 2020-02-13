pub mod company {
    use std::collections::HashMap;

    pub struct Company {
        employee: HashMap<Department, People>
    }

    impl Company {
        pub fn new() -> Company {
            Company {
                employee: HashMap::new()
            }
        }

        pub fn add(&mut self, description: String) {
            let desc_list: Vec<&str> = description.as_str().split(' ').collect();
            let name = desc_list[1].to_string();
            let department = desc_list[3].to_string();

            let dept = Department {
                name: department
            };

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