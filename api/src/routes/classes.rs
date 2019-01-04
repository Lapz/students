use crate::auth::ApiKey;
use crate::models::{Class, NewClass, StudentsGradeAndClass, StudentssWithClass};
use crate::sql_pool::Pool;
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/classes")]
pub fn classes(_key: ApiKey,db_conn: State<'_, Pool>) -> Json<Vec<Class>> {
    use crate::schema::class::dsl::class;

    let result = class
        .load::<Class>(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");
    Json(result)
}

#[post("/classes", data = "<class>")]
pub fn add_class(
    _key: ApiKey,
    db_conn: State<'_, Pool>,
    class: Json<NewClass>,
) -> Json<&'static str> {
    use crate::schema::class;

    diesel::insert_into(class::table)
        .values(&class.into_inner())
        .execute(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");

    Json("Class Sucessfully added")
}

#[get("/classes/<class_id>")]
pub fn class(
    _key: ApiKey,
    db_conn: State<'_, Pool>,
    class_id: i32,
) -> Json<Vec<StudentssWithClass>> {
    use crate::schema::class;
    use crate::schema::students;

    let result = students::table
        .inner_join(class::table.on(class::id.eq(students::class_id)))
        .select((students::id, class::id, students::name, class::teacher))
        .filter(class::id.eq(class_id))
        .load::<StudentssWithClass>(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");

    // Select student.id,student.name,class.teacher from students join class on
    // class.id = student_class.id where class.id = $class_id;

    Json(result)
}

#[get("/classes/<class_id>/grades")]
pub fn class_grades(
    _key: ApiKey,
    db_conn: State<'_, Pool>,
    class_id: i32,
) -> Json<Vec<StudentsGradeAndClass>> {
    use crate::schema::class;
    use crate::schema::grades;
    use crate::schema::students;

    let result = students::table
        .inner_join(class::table.on(class::id.eq(students::class_id)))
        .inner_join(grades::table.on(grades::id.eq(students::grades_id)))
        .select((
            students::id,
            class::id,
            students::name,
            class::teacher,
            grades::grade,
        ))
        .filter(class::id.eq(class_id))
        .load::<StudentsGradeAndClass>(&db_conn.inner().get().expect("no connection"))
        .expect("Error loading posts");

    //select students.id,students.name,class.teacher from students join class on
    //class.id = students.id join grades on grades.id =students.grades_id;

    Json(result)
}
