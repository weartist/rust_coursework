
struct Student {
    id: i32,
    name: String,
    age: i32,
    class_id: i32, // class id 有可能为0，即未分配
    courses: Vec<Course>
}

struct Class {
    id: i32,
    name: String,
    students: Vec<Student>
}

struct Course {
    id: i32,
    name: String,
    score: i32
}

struct Club {
    id: i32,
    name: String,
    members: Vec<Student>
}


struct StudentManager {
    students: Vec<Student>
}

impl Student {
    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }
    
    fn get_course(&self, id: i32) -> &Course {
        &self.courses[id as usize - 1]
    }
    
    fn update_course(&mut self, id: i32, name: String, score: i32) {
        let course = &mut self.courses[id as usize - 1];
        course.name = name;
        course.score = score;
    }
    
    fn delete_course(&mut self, id: i32) {
        self.courses.remove(id as usize - 1);
    }
}

impl Class {
    fn add_student(&mut self, student: Student) {
        self.students.push(student);
        student.class_id = self.id;
    }
    
}

impl StudentManager {
    
    fn create_student(&mut self, name: String, age: i32) -> Student {
        let id = self.students.len() as i32 + 1;
        let student = Student { id, name, age, courses: Vec::new() };
        return student 
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
    
    fn get_student(&self, id: i32) -> &Student {
        &self.students[id as usize - 1]
    }
    
    
    fn update_student(&mut self, id: i32, name: String, age: i32, class_id: i32) {
        let student = &mut self.students[id as usize - 1];
        student.name = name;
        student.age = age;
        student.class_id = class_id;
    }
    
    fn delete_student(&mut self, id: i32) {
        self.students.remove(id as usize - 1);
    }
    
}


fn main() {

    // 使用结构体普通初始化
    let superman = Student { id: 1, name: "superman".to_string(), age: 40 };
    // 使用管理类方法初始化
    let hans = StudentManager::create_student("hans".to_string(), 30);
    let batman = StudentManager::create_student("batman".to_string(), 40);

    // 增加学生到管理类
    StudentManager::add_student(superman);
    StudentManager::add_student(hans);
    StudentManager::add_student(batman);
    
    // 根据id获取student
    let cur_student = StudentManager::get_student(2);
    println!("cur_student: {:?}", cur_student);

    // 修改学生信息
    StudentManager::update_student(1, "fake_superman".to_string(), 40);

    //删除学生
    StudentManager::delete_student(1);

    // 创建课程
    let math_course = Course { id: 1, name: "math".to_string(), score: 100 };
    let english_course = Course { id: 2, name: "english".to_string(), score: 90 };

    //学生添加课程
    hans.add_course(math_course);
    han.add_course(english_course);

    let class1 = Class { id: 1, name: "class1".to_string(), students: Vec::new() };
    class1.students.push(hans);
    // 这里因为将hans添加到class1，需要更新hans的班级
    hans.update_student(1, "hans".to_string(), 30, 1);

    // 但这样比较麻烦，所以也可以使用便捷方法,内部对student的class_id自动进行了更新
    class1.add_student(batman);
}
