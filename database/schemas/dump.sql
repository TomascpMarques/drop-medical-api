PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS user(
  id blob primary key,
  name Text not null,
  email Text not null,
  password Text not null
);
COMMIT;
