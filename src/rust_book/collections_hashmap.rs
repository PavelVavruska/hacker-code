use std::collections::HashMap;

/// Using a hash map and vectors, create a text interface
/// to allow a user to add employee names to a department in a company.
/// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
/// Then let the user retrieve a list of all people in a department
/// or all people in the company by department, sorted alphabetically.
pub fn add_employee_to_department(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department: String,
) {
    let department_vec = company.entry(department).or_insert([].to_vec());
    department_vec.push(employee);
}

pub fn get_employees_from_department(
    company: &mut HashMap<String, Vec<String>>,
    department: &String,
) -> Vec<String> {
    if let Some(department_list) = company.get(department) {
        let mut dept_vec = department_list.clone();
        dept_vec.sort();
        return dept_vec;
    }
    [].to_vec()
}

pub fn get_all_employees_from_company(company: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for (department_name, emp_list) in company {
        let mut dept_vec = emp_list.clone();
        dept_vec.sort();
        let mut one_line: String =
            String::from(format!("department: {} employees: ", department_name));
        let mut first = true;
        for employee in emp_list {
            if !first {
                one_line.push_str(", "); // add comma after not last element
            }
            one_line.push_str(employee.as_str());
            first = false;
        }
        output.push(one_line);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{add_employee_to_department, get_all_employees_from_company};
    use std::collections::HashMap;

    #[test]
    fn test_get_consonant() {
        let name1 = String::from("Sally");
        let name2 = String::from("Amir");

        let name3 = String::from("Pavel");
        let name4 = String::from("George");

        let department1 = String::from("Engineering");
        let department2 = String::from("Sales");

        let mut company: HashMap<String, Vec<String>> = HashMap::new();

        // create company manually to test against
        let mut result_company: HashMap<String, Vec<String>> = HashMap::new();
        result_company.insert(department1.clone(), [name1.clone(), name3.clone()].to_vec());
        result_company.insert(department2.clone(), [name2.clone(), name4.clone()].to_vec());

        // create company through methods
        add_employee_to_department(&mut company, name1, department1.clone());
        add_employee_to_department(&mut company, name3, department1.clone());
        add_employee_to_department(&mut company, name2, department2.clone());
        add_employee_to_department(&mut company, name4, department2.clone());

        assert_eq!(result_company, company);

        // test string output for whole company
        let mut result2: Vec<String> = Vec::new();
        result2.push(String::from(
            "department: Engineering employees: Sally, Pavel",
        ));
        result2.push(String::from("department: Sales employees: Amir, George"));

        assert_eq!(result2, get_all_employees_from_company(&company));
    }
}
