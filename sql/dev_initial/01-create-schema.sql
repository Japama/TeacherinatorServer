---- Base app schema

-- Users
CREATE TABLE "users"
(
    id            BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    username      varchar(128)             NOT NULL UNIQUE,
    isadmin bool                     NOT NULL DEFAULT false,

    -- Auth
    pwd           varchar(256),
    pwd_salt      uuid                     NOT NULL DEFAULT gen_random_uuid(),
    token_salt    uuid                     NOT NULL DEFAULT gen_random_uuid(),

    -- Timestamps
    cid           bigint                   NOT NULL,
    ctime         timestamp with time zone NOT NULL,
    mid           bigint                   NOT NULL,
    mtime         timestamp with time zone NOT NULL
);

-- Departments
CREATE TABLE "departments"
(
    id    BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    name  varchar(256)             NOT NULL,


    -- Timestamps
    cid   bigint                   NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid   bigint                   NOT NULL,
    mtime timestamp with time zone NOT NULL
);

-- Teachers
CREATE TABLE "teachers"
(
    id            BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    user_id       BIGINT                   NOT NULL UNIQUE,
    name          varchar(128)             NOT NULL UNIQUE,
    active        bool,
    department_id BIGINT                   NOT NULL DEFAULT 0,
--     substituting  BIGINT                   UNIQUE,

    -- Timestamps
    cid           bigint                   NOT NULL,
    ctime         timestamp with time zone NOT NULL,
    mid           bigint                   NOT NULL,
    mtime         timestamp with time zone NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (department_id) REFERENCES departments (id)
);

-- Subjects
CREATE TABLE "subjects"
(
    id              BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    name            varchar(256)             NOT NULL,
    department_id   BIGINT                   NOT NULL DEFAULT 0,
    isGuard         bool                     NOT NULL DEFAULT false,
    isComplementary bool                     NOT NULL DEFAULT false,

    -- Timestamps
    cid             bigint                   NOT NULL,
    ctime           timestamp with time zone NOT NULL,
    mid             bigint                   NOT NULL,
    mtime           timestamp with time zone NOT NULL,

    FOREIGN KEY (department_id) REFERENCES departments (id)
);


-- Groups
CREATE TABLE "groups"
(
    id       BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    course   int                      NOT NULL,
    stage    int                      NOT NULL,
    year     int                      NOT NULL,
    letter   varchar(10)              NOT NULL,
    tutor_id int                      NOT NULL,
    -- Timestamps
    cid      bigint                   NOT NULL,
    ctime    timestamp with time zone NOT NULL,
    mid      bigint                   NOT NULL,
    mtime    timestamp with time zone NOT NULL,

    FOREIGN KEY (tutor_id) REFERENCES teachers (id)
);

-- Room
CREATE TABLE "room"
(
    id          BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    building    varchar(20),
    floor       int,
    number      int,
    name        varchar(20),
    type        int,
    description varchar(256),
    -- Timestamps
    cid         bigint                   NOT NULL,
    ctime       timestamp with time zone NOT NULL,
    mid         bigint                   NOT NULL,
    mtime       timestamp with time zone NOT NULL
);


-- GuardType
CREATE TABLE "guardType"
(
    id          BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    name        varchar(256)             NOT NULL,
    playground  bool DEFAULT false,
    replacement bool DEFAULT false,
    description varchar(256)             NOT NULL,
    -- Timestamps
    cid         bigint                   NOT NULL,
    ctime       timestamp with time zone NOT NULL,
    mid         bigint                   NOT NULL,
    mtime       timestamp with time zone NOT NULL
);


-- Schedules
CREATE TABLE schedules
(
    id            BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    teacher_id    INT REFERENCES teachers (id) NOT NULL,
    subject_id    INT REFERENCES subjects (id) NOT NULL,
    day           INT                          NOT NULL,
    start_time    TIME                         NOT NULL,
    end_time      TIME                         NOT NULL,
    academic_year INT                          NOT NULL,

    -- Timestamps
    cid           bigint                       NOT NULL,
    ctime         timestamp with time zone     NOT NULL,
    mid           bigint                       NOT NULL,
    mtime         timestamp with time zone     NOT NULL,
    PRIMARY KEY (id, academic_year),
    UNIQUE (teacher_id, subject_id, day, start_time, academic_year)
) PARTITION BY RANGE (academic_year);
CREATE INDEX idx_teacher_day_start_time ON schedules (teacher_id, day, start_time);
CREATE TABLE schedules_2022 PARTITION OF schedules FOR VALUES FROM
(
    2022
) TO
(
    2023
);


-- Project
CREATE TABLE project
(
    -- PK
    id       BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    -- Properties
    owner_id BIGINT                   NOT NULL,
    name     varchar(256)             NOT NULL,

    -- Timestamps
    cid      bigint                   NOT NULL,
    ctime    timestamp with time zone NOT NULL,
    mid      bigint                   NOT NULL,
    mtime    timestamp with time zone NOT NULL
);

-- Task
CREATE TABLE task
(
    -- PK
    id         BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    -- FK
    project_id BIGINT                   NOT NULL,

    -- Properties
    title      varchar(256)             NOT NULL,
    done       bool                     NOT NULL DEFAULT false,

    -- Timestamps
    cid        bigint                   NOT NULL,
    ctime      timestamp with time zone NOT NULL,
    mid        bigint                   NOT NULL,
    mtime      timestamp with time zone NOT NULL
);

ALTER TABLE task
    ADD CONSTRAINT fk_project
        FOREIGN KEY (project_id) REFERENCES project (id)
            ON DELETE CASCADE;

