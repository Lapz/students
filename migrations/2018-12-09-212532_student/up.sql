-- Your SQL goes here

CREATE TABLE class (
    id  INTEGER PRIMARY KEY AUTO_INCREMENT,
    teacher TEXT NOT NULL
);


CREATE TABLE grades (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    grade INTEGER NOT NULL
);

CREATE TABLE students (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    class_id INTEGER NOT NULL,
    grades_id INTEGER NOT NULL,
    name TEXT NOT NULL
);


INSERT INTO grades VALUES(0,95);
INSERT INTO grades VALUES(1,100);

INSERT INTO students VALUES(
    0,
    0,
    'Lenard'
);

INSERT INTO students VALUES(
    0,
    1,
    'Andy'
);


-- INSERT INTO students VALUES(1,0,'Andy');

