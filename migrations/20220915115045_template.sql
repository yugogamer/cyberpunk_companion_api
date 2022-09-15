-- Add migration script here
create table template(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);
create table template_to_stats(
    template_id INTEGER NOT NULL REFERENCES template(id),
    stat_id INTEGER NOT NULL REFERENCES stats_template(id),
    base_value INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (template_id, stat_id)
);
create table template_to_skills(
    template_id INTEGER NOT NULL REFERENCES template(id),
    skill_id INTEGER NOT NULL REFERENCES skills_template(id),
    base_value INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (template_id, skill_id)
);
alter table players
add column template_id INTEGER REFERENCES template(id);
-- seeding
insert into template (name, description)
values (
        'Cyberpunk Red',
        'Cyberpunk red default stats and skills'
    );
insert into template_to_stats (template_id, stat_id, base_value)
values (1, 0, 2),
    (1, 1, 2),
    (1, 2, 2),
    (1, 3, 2),
    (1, 4, 2),
    (1, 5, 2),
    (1, 6, 2),
    (1, 7, 2),
    (1, 8, 2);
insert into template_to_skills (template_id, skill_id, base_value)
values (1, 1, 2),
    (1, 2, 0),
    (1, 3, 0),
    (1, 4, 2),
    (1, 5, 0),
    (1, 6, 2),
    (1, 7, 0),
    (1, 8, 0),
    (1, 9, 0),
    (1, 10, 0),
    (1, 11, 2),
    (1, 12, 0),
    (1, 13, 0),
    (1, 14, 0),
    (1, 15, 0),
    (1, 16, 0),
    (1, 17, 0),
    (1, 18, 0),
    (1, 19, 0),
    (1, 20, 0),
    (1, 21, 0),
    (1, 22, 0),
    (1, 23, 0),
    (1, 24, 2),
    (1, 25, 0),
    (1, 26, 0),
    (1, 27, 0),
    (1, 28, 0),
    (1, 29, 2),
    (1, 30, 2),
    (1, 31, 0),
    (1, 32, 0),
    (1, 33, 0),
    (1, 34, 0),
    (1, 35, 0),
    (1, 36, 0),
    (1, 37, 0),
    (1, 38, 0),
    (1, 39, 2),
    (1, 40, 2),
    (1, 41, 0),
    (1, 42, 2),
    (1, 43, 0),
    (1, 44, 0),
    (1, 45, 0),
    (1, 46, 0),
    (1, 47, 0),
    (1, 48, 0),
    (1, 49, 0),
    (1, 50, 0),
    (1, 51, 0),
    (1, 52, 2),
    (1, 53, 0),
    (1, 54, 0),
    (1, 55, 0),
    (1, 56, 0),
    (1, 57, 0),
    (1, 58, 0),
    (1, 59, 0),
    (1, 60, 0),
    (1, 61, 0);