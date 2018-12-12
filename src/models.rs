
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Student {
    id: i32,
    class_id: i32,
    grades_id:i32,
    name: String,
}

#[derive(Queryable, Serialize, Debug, Clone,FromForm)]
pub struct Class {
    id: i32,
    teacher: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct StudentssWithClass {
    student_id: i32,
    class_id: i32,
    student: String,
    teacher: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct StudentsGrade {
    student_id: i32,
    class_id: i32,
    student: String,
    teacher: String,
}
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct StudentsGradeAndClass {
    student_id: i32,
    class_id: i32,
    student: String,
    teacher: String,
    grade:i32,
}


