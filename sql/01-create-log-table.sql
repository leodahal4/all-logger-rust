DROP TABLE IF EXISTS log;
CREATE TABLE log (
  id BIGSERIAL,

  log TEXT NOT NULL,
  error BOOLEAN NOT NULL DEFAULT false,
  priority INT NOT NULL DEFAULT 4,
  send BOOLEAN NOT NULL DEFAULT false,
  extra TEXT,
  subdomain VARCHAR NOT NULL,
  appname VARCHAR NOT NULL
);