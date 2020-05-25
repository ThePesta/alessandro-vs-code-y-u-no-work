-- Your SQL goes here
CREATE TABLE islands (
  id UUID PRIMARY KEY,
  owner_id UUID NOT NULL,
  name TEXT NOT NULL,
  is_active BOOLEAN NOT NULL
);