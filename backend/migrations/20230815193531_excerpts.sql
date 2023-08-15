-- Add migration script here
CREATE TABLE IF NOT EXISTS excerpt
(
    id      INTEGER PRIMARY KEY NOT NULL,
    excerpt STRING              NOT NULL
);

CREATE TABLE IF NOT EXISTS image
(
    path    STRING PRIMARY KEY NOT NULL,
    post_id INTEGER NOT NULL,
    FOREIGN KEY(post_id) REFERENCES excerpt(id) ON DELETE CASCADE
);

