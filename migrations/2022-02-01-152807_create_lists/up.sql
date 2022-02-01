CREATE TABLE lists (
    id UUID PRIMARY KEY,
    board UUID NOT NULL,
    name TEXT NOT NULL,

    CONSTRAINT fk_board FOREIGN KEY (board) REFERENCES boards (id)
)
