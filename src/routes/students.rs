use crate::models::{Student,NewStudent};
use crate::sql_pool::Pool;
use diesel::prelude::{QueryDsl, RunQueryDsl};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/students")]
pub fn students(db_conn: State<Pool>) -> Json<Vec<Student>> {
    use crate::schema::students::dsl::*;

    let result = students
        .load::<Student>(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");

    Json(result)
}

#[post("/students",data="<student>")]
pub fn add_student(db_conn: State<Pool>,student:Json<NewStudent>) -> Json<&'static str> {
    use crate::schema::students;

    diesel::insert_into(students::table)
        .values(&student.into_inner())
        .execute(&db_conn.inner().get().expect("no connection"));

    Json("Student Sucessfully added")
}

#[get("/students/<student_id>")]
pub fn student(db_conn: State<Pool>, student_id: i32) -> Json<Student> {
    use crate::schema::students::dsl::*;

    let result = students
        .find(student_id)
        .first::<Student>(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");

    Json(result)
}
