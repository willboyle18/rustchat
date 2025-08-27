BEGIN;


DROP TABLE if EXISTS messages CASCADE;
DROP TABLE if EXISTS users CASCADE;


CREATE TABLE users (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username    TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE messages (
    message_id  BIGSERIAL PRIMARY KEY,
    user_id     BIGINT REFERENCES users(id) ON DELETE CASCADE,
    text        TEXT NOT NULL CHECK(length(text) <= 4000),
    sent_at     TIMESTAMPTZ NOT NULL default now()
);

CREATE INDEX index_messages_user_sent ON messages (user_id, sent_at DESC);

INSERT INTO users (username, password)
VALUES ('test', 'testpw');

COMMIT;