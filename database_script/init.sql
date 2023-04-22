CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(64) NOT NULL UNIQUE,
  password    VARCHAR(64) NOT NULL,
  deleted_at  TIMESTAMPTZ DEFAULT NULL,
  token       TEXT DEFAULT NULL
);

INSERT INTO users (username, password) VALUES ('deleteduser', '$2b$12$x3hs5oMgjHdcV1GUEElfsO19JtS6.ixJAX9Cj62GyhpdPAIW25sky');
