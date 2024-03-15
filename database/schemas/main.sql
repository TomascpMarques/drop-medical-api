create table if not exists user(
  id blob primary key,
  name Text not null,
  email Text not null,
  password Text not null
);
