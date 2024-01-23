-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE usename = 'teacherinator_app_user_db'
   OR datname = 'teacherinator';
DROP
DATABASE IF EXISTS teacherinator;
DROP
USER IF EXISTS teacherinator_app_user_db;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE
USER teacherinator_app_user_db PASSWORD 'dev_only_pwd';
CREATE
DATABASE teacherinator owner teacherinator_app_user_db ENCODING = 'UTF-8';