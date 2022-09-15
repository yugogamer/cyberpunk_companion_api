-- Add migration script here
create table players_to_groupes(
    player_id INTEGER NOT NULL REFERENCES players(id),
    groupe_id INTEGER NOT NULL REFERENCES groupes(id),
    can_see BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (player_id, groupe_id)
);
create table players_to_accounts(
    player_id INTEGER NOT NULL REFERENCES players(id),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    can_edit BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (player_id, account_id)
);
alter table accounts_to_groupes
add column is_master BOOLEAN NOT NULL DEFAULT false;
alter table players
add column owner INTEGER NOT NULL REFERENCES accounts(id);