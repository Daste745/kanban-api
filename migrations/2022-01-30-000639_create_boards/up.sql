CREATE TABLE boards (
    id UUID PRIMARY KEY,
    owner UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT,

    CONSTRAINT fk_owner FOREIGN KEY (owner) REFERENCES users (id)
)
