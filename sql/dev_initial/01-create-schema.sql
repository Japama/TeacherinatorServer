---- Base app schema
-- Departments
CREATE TABLE "departments"
(
    id    BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    name  varchar(256)             NOT NULL UNIQUE,


    -- Timestamps
    cid   bigint                   NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid   bigint                   NOT NULL,
    mtime timestamp with time zone NOT NULL
);

-- Users
CREATE TABLE "users"
(
    id              BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    username        varchar(128)             NOT NULL UNIQUE,
    is_admin        bool                     NOT NULL DEFAULT false,
    last_checkin    TIME                     NOT NULL DEFAULT '00:00:00',
    last_checkout   TIME                     NOT NULL DEFAULT '00:00:00',
    in_center       bool                     NOT NULL DEFAULT false,
    active          bool                              DEFAULT true,
    department_id   BIGINT                            DEFAULT 1,
    substituting_id BIGINT UNIQUE                     DEFAULT null,
    substitutions   BIGINT                            DEFAULT 0,


    -- Auth
    pwd             varchar(256),
    pwd_salt        uuid                     NOT NULL DEFAULT gen_random_uuid(),
    token_salt      uuid                     NOT NULL DEFAULT gen_random_uuid(),

    -- Timestamps
    cid             bigint                   NOT NULL,
    ctime           timestamp with time zone NOT NULL,
    mid             bigint                   NOT NULL,
    mtime           timestamp with time zone NOT NULL,

    FOREIGN KEY (department_id) REFERENCES departments (id)
);

-- Subjects
CREATE TABLE "subjects"
(
    id               BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    name             varchar(256)             NOT NULL,
    department_id    BIGINT                   NOT NULL DEFAULT 0,
    is_guard         bool                     NOT NULL DEFAULT false,
    is_complementary bool                     NOT NULL DEFAULT false,

    -- Timestamps
    cid              bigint                   NOT NULL,
    ctime            timestamp with time zone NOT NULL,
    mid              bigint                   NOT NULL,
    mtime            timestamp with time zone NOT NULL,

    FOREIGN KEY (department_id) REFERENCES departments (id)
);


-- Groups
CREATE TABLE "groups"
(
    id         BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    course     int                      NOT NULL, -- 1º, 2º
    stage      int                      NOT NULL, -- ESO, BACHILLER, FP
    year       int                      NOT NULL, -- 2024/2025
    letter     varchar(20), -- A, B, C
    tutor_name varchar(128),

    -- Timestamps
    cid        bigint                   NOT NULL,
    ctime      timestamp with time zone NOT NULL,
    mid        bigint                   NOT NULL,
    mtime      timestamp with time zone NOT NULL,

    UNIQUE (course, stage, year, letter)
);



CREATE TABLE buildings
(
    id         BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    building_name        varchar(256),

    -- Timestamps
    cid        bigint                   NOT NULL,
    ctime      timestamp with time zone NOT NULL,
    mid        bigint                   NOT NULL,
    mtime      timestamp with time zone NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (building_name)
);


CREATE TABLE classroom_types
(
    id         BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    type_name        varchar(256),

    -- Timestamps
    cid        bigint                   NOT NULL,
    ctime      timestamp with time zone NOT NULL,
    mid        bigint                   NOT NULL,
    mtime      timestamp with time zone NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (type_name)
);

-- Room
CREATE TABLE "classrooms"
(
    id          BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    building    BIGINT,
    floor       int,
    number      int,
    name        varchar(256) UNIQUE,
    type_c      BIGINT,
    description varchar(256),

    -- Timestamps
    cid         bigint                   NOT NULL,
    ctime       timestamp with time zone NOT NULL,
    mid         bigint                   NOT NULL,
    mtime       timestamp with time zone NOT NULL,

    FOREIGN KEY (building) REFERENCES buildings (id),
    FOREIGN KEY (type_c) REFERENCES classroom_types (id),
    UNIQUE (building, floor, number)
);


-- Schedules
CREATE TABLE schedules
(
    id       BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    user_id  BIGINT REFERENCES users (id) ON DELETE CASCADE,
    group_id BIGINT REFERENCES groups (id),
    course   INT                      NOT NULL, -- 2024/2025

    -- Timestamps
    cid      bigint                   NOT NULL,
    ctime    timestamp with time zone NOT NULL,
    mid      bigint                   NOT NULL,
    mtime    timestamp with time zone NOT NULL,
    PRIMARY KEY (id, course),
    UNIQUE (group_id, course),
    UNIQUE (user_id, course)
) PARTITION BY RANGE (course);

CREATE TABLE schedules_2022 PARTITION OF schedules FOR VALUES FROM (2022) TO (2023);
CREATE TABLE schedules_2023 PARTITION OF schedules FOR VALUES FROM (2023) TO (2024);
CREATE TABLE schedules_2024 PARTITION OF schedules FOR VALUES FROM (2024) TO (2025);
CREATE TABLE schedules_2025 PARTITION OF schedules FOR VALUES FROM (2025) TO (2026);
CREATE TABLE schedules_2026 PARTITION OF schedules FOR VALUES FROM (2026) TO (2027);

-- Schedule_hora
CREATE TABLE schedule_hours
(
    id             BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    schedule_id    BIGINT                   NOT NULL,
    subject_name   varchar(256)             NOT NULL,
    classroom_name varchar(256)             NOT NULL,
    week_day       INT                      NOT NULL,
    n_hour         INT                      NOT NULL,
    course         INT                      NOT NULL,
    notes          varchar(256),

    -- Timestamps
    cid            bigint                   NOT NULL,
    ctime          timestamp with time zone NOT NULL,
    mid            bigint                   NOT NULL,
    mtime          timestamp with time zone NOT NULL,
    PRIMARY KEY (id, course),
    FOREIGN KEY (schedule_id, course) REFERENCES schedules (id, course) ON DELETE CASCADE,
    UNIQUE (schedule_id, course, week_day, n_hour)
) PARTITION BY RANGE (course);

CREATE TABLE center_schedule_hours
(
    id         BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
    n_hour     INT                      NOT NULL,
    start_time TIME                     NOT NULL,
    end_time   TIME                     NOT NULL,

    -- Timestamps
    cid        bigint                   NOT NULL,
    ctime      timestamp with time zone NOT NULL,
    mid        bigint                   NOT NULL,
    mtime      timestamp with time zone NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (n_hour)
);


CREATE TABLE schedule_hours_2022 PARTITION OF schedule_hours FOR VALUES FROM (2022) TO (2023);
CREATE TABLE schedule_hours_2023 PARTITION OF schedule_hours FOR VALUES FROM (2023) TO (2024);
CREATE TABLE schedule_hours_2024 PARTITION OF schedule_hours FOR VALUES FROM (2024) TO (2025);
CREATE TABLE schedule_hours_2025 PARTITION OF schedule_hours FOR VALUES FROM (2025) TO (2026);
CREATE TABLE schedule_hours_2026 PARTITION OF schedule_hours FOR VALUES FROM (2026) TO (2027);

