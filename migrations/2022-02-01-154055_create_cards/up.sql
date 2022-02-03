CREATE TABLE cards (
    id UUID PRIMARY KEY,
    list UUID NOT NULL,
    content TEXT,
    labels TEXT [],

    CONSTRAINT fk_list FOREIGN KEY (list) REFERENCES lists (id)
)
