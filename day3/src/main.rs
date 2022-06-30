use std::collections::HashMap;

fn main() {
    test_case_1();
    test_case_2();
    test_case_3();
}

fn test_case_1() {
    //Test case 1: Khởi tạo đầu tiên danh sách phải rỗng
    let mut school = School::new();
    println!("Test case 1: {:?}", school.get_list_student());
}

fn test_case_2() {
    // Test case 2:
    // Thêm sinh viên có tên "Lee" với điểm số là 2
    // Thì tất cả các điểm số hiện có của trường là 2
    // Nếu thêm sinh viên khác "Nancy" với điểm số là 3
    // Thì các điểm số hiện tại là [2,3]
    let mut school = School::new();
    school.add_student("Lee", 2);
    println!("Test case 2.1: {:?}", school.grades());
    school.add_student("Nancy", 3);
    println!("Test case 2.2: {:?}", school.grades());
}

fn test_case_3() {
    // Test case 3:
    // Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
    // với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
    // vì cần tên theo alphabet
    let mut school = School::new();
    school.add_student("Bob", 4);
    school.add_student("Alice", 4);
    school.add_student("Tom", 5);
    println!("Test case 3.1: {:?}", school.get_list_student());
    println!("Test case 3.2: {:?}", school.get_list_grade(4));
}

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str, grade: u32) {
        self.students.insert(name.to_string(), grade);
        
    }

    pub fn get_list_student(&self) -> &HashMap<String, u32> {
        &self.students 
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut list_grade = Vec::new();
        for grade in self.students.values() {
            list_grade.push(*grade);
        }
        list_grade.sort();
        list_grade.dedup();
        list_grade
    }

    pub fn get_list_grade(&self, grade: u32) -> Vec<String> {
        let mut list_same_grade = Vec::new();
        for (name, grade_student) in self.students.iter() {
            if *grade_student == grade {
                list_same_grade.push(name.to_string());
            }
        }
        list_same_grade.sort();
        list_same_grade
    }
}
