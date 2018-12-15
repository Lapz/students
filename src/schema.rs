table! {
    class (id) {
        id -> Integer,
        teacher -> Text,
    }
}

table! {
    grades (id) {
        id -> Integer,
        grade -> Integer,
    }
}

table! {
    students (id) {
        id -> Integer,
        class_id -> Integer,
        grades_id -> Integer,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    class,
    grades,
    students,
    users,
);
