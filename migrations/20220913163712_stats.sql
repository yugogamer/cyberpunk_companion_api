-- Add migration script here
create table players(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    family_name TEXT NOT NULL,
    surname TEXT,
    age INTEGER,
    description TEXT,
    image TEXT,
    improvement_points INTEGER NOT NULL DEFAULT 0,
    used_improvement_points INTEGER NOT NULL DEFAULT 0,
    created_on TIMESTAMP NOT NULL DEFAULT now()
);
create table types(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
create table types_states(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
create table stats_template(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    type INTEGER REFERENCES types_states(id)
);
create table skills_template(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    parent_stats_id INTEGER REFERENCES stats_template(id),
    type INTEGER REFERENCES types(id)
);
create table stats(
    base_value INTEGER NOT NULL,
    modifier INTEGER NOT NULL DEFAULT 0,
    player_id INTEGER NOT NULL REFERENCES players(id),
    stats_template_id INTEGER NOT NULL REFERENCES stats_template(id),
    PRIMARY KEY (player_id, stats_template_id)
);
create table skills(
    value INTEGER NOT NULL,
    player_id INTEGER NOT NULL REFERENCES players(id),
    skill_template_id INTEGER NOT NULL REFERENCES skills_template(id),
    PRIMARY KEY (player_id, skill_template_id)
);
INSERT INTO types_states (id, name)
VALUES (0, 'stats'),
    (1, 'language'),
    (2, 'reputation'),
    (3, 'classe');
INSERT INTO types (id, name)
VALUES (0, 'awareness'),
    (1, 'body'),
    (2, 'control'),
    (3, 'education'),
    (4, 'fighting'),
    (5, 'performance'),
    (6, 'ranged_weapon'),
    (7, 'social'),
    (8, 'technique');
INSERT INTO stats_template (id, name, description, type)
VALUES (0, 'int', 'inteligence', 0),
    (1, 'ref', 'reflexe', 0),
    (2, 'dex', 'dexerity', 0),
    (3, 'tech', 'Techno', 0),
    (4, 'cool', 'you are super cool', 0),
    (5, 'will', 'willpower', 0),
    (6, 'luck', 'lucky guy', 0),
    (7, 'move', 'move in a turn', 0),
    (8, 'body', 'your average body type', 0),
    (9, 'emp', '0 you are a psycho', 0);
INSERT INTO skills_template (name, description, parent_stats_id, type)
VALUES -- awareness
    ('concentration', 'concentration', 5, 0),
    (
        'conceal/reveal object',
        'how to hide your weapon',
        0,
        0
    ),
    (
        'lip reading',
        'read on the lip of someone',
        0,
        0
    ),
    ('perception', 'you know', 0, 0),
    ('tracking', 'track someone or an object', 0, 0),
    -- body
    ('athletics', '', 2, 1),
    ('contortionist', 'move in small space', 2, 1),
    ('dance', 'move your body', 2, 1),
    ('endurance', '', 5, 1),
    (
        'resist torture/drugs',
        'how well you resiste',
        5,
        1
    ),
    ('stealth', 'sneaky boy', 2, 1),
    -- control
    (
        'drive land vehichle',
        'drive a cars or a land vehicle',
        1,
        2
    ),
    (
        'pilote air vehichle',
        'drive a plane or a wings',
        1,
        2
    ),
    (
        'pilote sea vehicle',
        'drive a boat or a drakar or a land vehicle',
        1,
        2
    ),
    (
        'riding',
        'ride a cyber horse of fire',
        1,
        2
    ),
    -- education
    ('accounting', '', 0, 3),
    ('animal handling', '', 0, 3),
    ('bureaucracy', '', 0, 3),
    ('business', '', 0, 3),
    ('composition', '', 0, 3),
    ('criminology', '', 0, 3),
    ('cryptohraphy', '', 0, 3),
    ('deduction', '', 0, 3),
    ('education', '', 0, 3),
    ('gamble', '', 0, 3),
    ('library search', '', 0, 3),
    ('tactics', '', 0, 3),
    ('wilderness survival', '', 0, 3),
    -- fighting
    ('brawling', '', 2, 4),
    ('evasion', '', 2, 4),
    ('melee weapon', '', 2, 4),
    -- performance
    ('acting', '', 4, 5),
    -- ranged weapon
    ('archery', 'everithing except firearm', 1, 6),
    ('autofire', '', 1, 6),
    ('handgun', 'everithing except firearm', 1, 6),
    (
        'heavy weapons',
        'everithing except firearm',
        1,
        6
    ),
    (
        'shoulder arms',
        'everithing except firearm',
        1,
        6
    ),
    -- social
    ('bridery', '', 4, 7),
    ('conversation', '', 9, 7),
    ('human perception', '', 9, 7),
    ('interogation', '', 4, 7),
    ('persuasion', '', 4, 7),
    ('personnal grooming', '', 4, 7),
    ('streetwise', '', 4, 7),
    ('trading', '', 4, 7),
    ('wardrobe & style', '', 4, 7),
    -- technique
    ('air vehicle tech', '', 3, 8),
    ('basic tech', '', 3, 8),
    ('cybertech', '', 3, 8),
    ('demolution', '', 3, 8),
    ('electronics/security Tech', '', 3, 8),
    ('first aid', '', 3, 8),
    ('forgery', '', 3, 8),
    ('land vehicle tech', '', 3, 8),
    ('point/draw/sculpt', '', 3, 8),
    ('paramedic', '', 3, 8),
    ('photography/film', '', 3, 8),
    ('pick lock', '', 3, 8),
    ('pick pocket', '', 3, 8),
    ('sea vehicle tech', '', 3, 8),
    ('weapongtech', '', 3, 8);