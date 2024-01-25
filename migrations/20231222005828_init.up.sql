-- extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- enums
CREATE TYPE node_type AS ENUM ('dir', 'file', 'symlink');
-- tables
CREATE TABLE IF NOT EXISTS nodes (
  uuid UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
  node_type node_type,
  name VARCHAR(255) NOT NULL,
  path VARCHAR(255) NOT NULL,
  canonical_path VARCHAR(255),
  size BIGINT NOT NULL,
  created TIMESTAMP NOT NULL,
  modified TIMESTAMP NOT NULL,
  accessed TIMESTAMP NOT NULL,
  sha256 VARCHAR(255),
  parent_uuid UUID,
  notes VARCHAR(512),
  published BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);