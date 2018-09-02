CREATE EXTENSION citext;

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email citext NOT NULL UNIQUE,
  passhash TEXT NOT NULL,
  user_id UUID NOT NULL UNIQUE,
  bio TEXT,
  image TEXT
);
