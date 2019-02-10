use std::collections::HashMap;
use uuid::Uuid;

/**
Using a hash map and vectors, create a text interface to allow
a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department
or all people in the company by department, sorted alphabetically.
*/
fn main() {
    let mut org = Org {
        departments: HashMap::new(),
        employees: HashMap::new(),
    };

    let sales_id = org.add_department("Sales".to_string());
    assert_eq!(org.departments.contains_key(&sales_id), true);

    let it_id = org.add_department("IT Department".to_string());

    let andrew_id = org.add_employee("Andrew".to_string());
    assert_eq!(org.employees.contains_key(&andrew_id), true);
    let man_id = org.add_employee("Man".to_string());


    let it_department = org.departments.get_mut(&it_id).unwrap();
    it_department.add_member(&andrew_id);
    it_department.add_member(&man_id);

    assert_eq!(it_department.members.contains(&andrew_id), true);

    for id in &it_department.members {
        let employee = org.employees.get(&id).unwrap();
        println!("[IT Department] employee's name: {}", employee.name);
    }
}

struct Org {
    departments: HashMap<Uuid, Department>,
    employees: HashMap<Uuid, Employee>,
}

impl Org {
    fn add_department(&mut self, desc: String) -> Uuid {
        let id = Uuid::new_v4();
        let department = Department {
            id,
            desc,
            members: Vec::new(),
        };
        self.departments.insert(id, department);
        id
    }

    fn add_employee(&mut self, name: String) -> Uuid {
        let id = Uuid::new_v4();
        let employee = Employee {id, name};
        self.employees.insert(id, employee);
        id
    }
}

struct Department {
    id: Uuid,
    desc: String,
    members: Vec<Uuid>,
}

impl Department {
    fn add_member(&mut self, member_id: &Uuid) {
        self.members.push(*member_id)
    }
}

struct Employee {
    id: Uuid,
    name: String,
}
