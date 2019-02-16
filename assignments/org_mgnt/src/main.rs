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
    let org = &mut Org {
        departments: HashMap::new(),
        employees: HashMap::new(),
    };
    let man_id = org.add_employee(String::from("Man"));
    let andrew_id = org.add_employee(String::from("Andrew"));
    let zen_id = org.add_employee(String::from("Min"));

    let it_dep_id = org.add_department(String::from("IT department"));
    let it_deparment = org.departments.get_mut(&it_dep_id).unwrap();
    it_deparment.add_member(&man_id);
    it_deparment.add_member(&andrew_id);
    it_deparment.add_member(&zen_id);

    for emp in org.get_department_members(&it_dep_id, String::from("asc")) {
        println!("[IT department] member: {}", emp.name);
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

    fn get_department_members(&self, dep_id: &Uuid, strategy: String) -> Vec<&Employee> {
        let department = self.departments.get(dep_id).unwrap();
        let mut result = Vec::new();

        for member_id in department.members.iter() {
            result.push(self.employees.get(member_id).unwrap())
        }

        if strategy == "asc" {
            result.sort_by(
                |x, y|
                    x.name.partial_cmp(&y.name).unwrap()
            );
        };

        if strategy == "des" {
            result.sort_by(
                |x, y|
                    x.name.partial_cmp(&y.name).unwrap()
            );
            result.reverse();
        };

        result
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
