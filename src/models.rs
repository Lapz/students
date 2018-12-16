use crate::schema::class;
use crate::schema::students;
use crate::schema::users;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Student {
    id: i32,
    class_id: i32,
    grades_id: i32,
    name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct Class {
    id: i32,
    teacher: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "class"]
pub struct NewClass {
    teacher: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "students"]
pub struct NewStudent {
    pub class_id: i32,
    pub grades_id: i32,
    pub name: String,
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
    grade: i32,
}
