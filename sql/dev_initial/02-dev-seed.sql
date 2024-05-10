-- Departments
INSERT INTO "departments" (id, name, cid, ctime, mid, mtime)
VALUES (1, 'Ninguno', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Matemáticas', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Física', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Educación fisica', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Inglés', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Música', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Castellano', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Valenciano', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Informática', 0, now(), 0, now());
INSERT INTO "departments" (name, cid, ctime, mid, mtime)
VALUES ('Francés', 0, now(), 0, now());

-- Users
-- Admin user (at id = 0)
-- root user (at id = 0)
INSERT INTO "users"
(id, username, cid, ctime, mid, mtime)
VALUES (0, 'root', 0, now(), 0, now());

INSERT INTO "users" (username, is_admin, active, department_id, cid, ctime, mid, mtime)
VALUES ('admin', true, true, 1, 0, now(), 0, now());
INSERT INTO "users" (username, active, department_id, cid, ctime, mid, mtime)
VALUES ('profesor1', true, 1001, 0, now(), 0, now());
INSERT INTO "users" (username, active, department_id, cid, ctime, mid, mtime)
VALUES ('profesor2', true, 1002, 0, now(), 0, now());
INSERT INTO "users" (username, active, department_id, cid, ctime, mid, mtime)
VALUES ('profesor3', true, 1003, 0, now(), 0, now());
INSERT INTO "users" (username, active, department_id, cid, ctime, mid, mtime)
VALUES ('profesor4', true, 1004, 0, now(), 0, now());
INSERT INTO "users" (username, is_admin, active, department_id, cid, ctime, mid, mtime)
VALUES ('secretaria1', true, true, null, 0, now(), 0, now());

-- Subjects
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Matemáticas', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Física', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Educación fisica', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Inglés', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Música', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Castellano', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Valenciano', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Informática', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Francés', 1000, false, false, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Guardia', 1000, true, true, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Guardia Patio', 1000, true, true, 0, now(), 0, now());
INSERT INTO "subjects" (name, department_id, is_guard, is_complementary, cid, ctime, mid, mtime)
VALUES ('Atención a padres', 1000, false, true, 0, now(), 0, now());

-- Groups
INSERT INTO "groups" (course, stage, year, letter, tutor_id, cid, ctime, mid, mtime)
VALUES (1, 1, 2023, 'A', 1000, 0, now(), 0, now());
INSERT INTO "groups" (course, stage, year, letter, tutor_id, cid, ctime, mid, mtime)
VALUES (1, 1, 2023, 'B', 1001, 0, now(), 0, now());

-- Classroom
INSERT INTO "classrooms" (building, floor, number, name, type_c, description, cid, ctime, mid, mtime)
VALUES ('Edificio1', 0, 1, 'Sala 1', 0, 'Habitación normal', 0, now(), 0, now());


-- Insertar datos en la tabla schedules
INSERT INTO schedules (user_id, group_id, course, cid, ctime, mid, mtime)
VALUES (1001, null, 2024, 1001, CURRENT_TIMESTAMP, 1001, CURRENT_TIMESTAMP);
INSERT INTO schedules (user_id, group_id, course, cid, ctime, mid, mtime)
VALUES (1002, null, 2024, 1001, CURRENT_TIMESTAMP, 1001, CURRENT_TIMESTAMP);
INSERT INTO schedules (user_id, group_id, course, cid, ctime, mid, mtime)
VALUES (1003, null, 2024, 1001, CURRENT_TIMESTAMP, 1001, CURRENT_TIMESTAMP);
INSERT INTO schedules (user_id, group_id, course, cid, ctime, mid, mtime)
VALUES (null, 1000, 2024, 1001, CURRENT_TIMESTAMP, 1001, CURRENT_TIMESTAMP);
INSERT INTO schedules (user_id, group_id, course, cid, ctime, mid, mtime)
VALUES (null, 1001, 2024, 1001, CURRENT_TIMESTAMP, 1001, CURRENT_TIMESTAMP);

-- Insertar datos en la tabla schedule_hours
-- LUNES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, notes, cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Info 2', 0, 0, 2024, 'Turuleta', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Info 2', 0, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Mantenimiento','Departamento', 0, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Recreo','Patio', 0, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Atención a familias','Sala de profesores', 0, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Cultura digital','Info 3', 0, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Libre','', 0, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Matemáticas','Info 2', 0, 7, 2024, '14:00:00', '08:55:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Matemáticas','Info 2', 0, 8, 2024, '15:55:00', '09:50:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Mantenimiento','Departamento', 0, 9, 2024, '16:50:00', '10:45:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Recreo','Patio', 0, 10, 2024, '17:45:00', '11:10:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Atención a familias','Sala de profesores', 0, 11, 2024, '18:10:00', '12:05:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Cultura digital','Info 3', 0, 12, 2024, '19:05:00', '13:00:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
-- INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
-- VALUES (1000, 'Libre','', 0, 13, 2024, '20:00:00', '13:55:00', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- MARTES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 1, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Guardia','Aula 02', 1, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 03', 1, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Recreo','Patio', 1, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 1, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 1, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 1, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- MIERCOLES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 2, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 02', 2, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 03', 2, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Recreo','Patio', 2, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 2, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 2, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 2, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- JUEVES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Info 2', 3, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Info 2', 3, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Mantenimiento','Departamento', 3, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Recreo','Patio', 3, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Atención a familias','Sala de profesores', 3, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Cultura digital','Info 3', 3, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Libre','', 3, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- VIERNES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 4, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 02', 4, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 03', 4, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Recreo','Patio', 4, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 4, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 4, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course,cid, ctime, mid, mtime)
VALUES (1000, 'Matemáticas','Aula 01', 4, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- LUNES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, notes, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Info 2', 0, 0, 2024, 'Turuleta', 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Info 2', 0, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Departamento', 0, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Patio', 0, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Sala de profesores', 0, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Info 3', 0, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','', 0, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- MARTES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 1, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 02', 1, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 03', 1, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Patio', 1, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 1, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 1, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 1, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- MIERCOLES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 2, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 02', 2, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 03', 2, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Patio', 2, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 2, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 2, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 2, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- JUEVES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 3, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 02', 3, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 03', 3, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Patio', 3, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 3, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 3, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 3, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- VIERNES
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 4, 0, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 02', 4, 1, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 03', 4, 2, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Patio', 4, 3, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 4, 4, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 4, 5, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO schedule_hours (schedule_id, subject_name, classroom_name, week_day, n_hour, course, cid, ctime, mid, mtime)
VALUES (1001, 'Guardia','Aula 01', 4, 6, 2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);


-- Center schedule hours
-- Lunes
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (0, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- Martes
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (1, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- Miércoles
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (2, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- Jueves
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (3, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

-- Viernes
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (4, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);

INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 0, '08:00:00', '08:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 1, '08:55:00', '09:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 2, '09:50:00', '10:45:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 3, '10:45:00', '11:10:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 4, '11:10:00', '12:05:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 5, '12:05:00', '13:00:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 6, '13:00:00', '13:55:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
INSERT INTO center_schedule_hours (week_day, n_hour, start_time, end_time, course, cid, ctime, mid, mtime)
VALUES (5, 7, '13:55:00', '14:50:00',2024, 0, CURRENT_TIMESTAMP, 0, CURRENT_TIMESTAMP);
