-- Your SQL goes here
CREATE TABLE snippets(
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    snippet VARCHAR NOT NULL,
    mnemonics VARCHAR,
    tags VARCHAR
)