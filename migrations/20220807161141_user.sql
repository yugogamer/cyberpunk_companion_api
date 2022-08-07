-- Add migration script here
CREATE TABLE IF NOT EXISTS accounts(
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE NOT NULL,
    email TEXT NOT NULL UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL,
    last_login TIMESTAMP
);
CREATE TABLE IF NOT EXISTS groupes(
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS accounts_to_groupes(
    id_account INTEGER NOT NULL REFERENCES accounts(id),
    id_groupe INTEGER NOT NULL REFERENCES groupes(id),
    PRIMARY KEY (id_account, id_groupe)
);
CREATE TABLE IF NOT EXISTS invite_links(
    token TEXT NOT NULL,
    id_account INTEGER NOT NULL REFERENCES accounts(id),
    id_groupe INTEGER NOT NULL REFERENCES groupes(id)
);