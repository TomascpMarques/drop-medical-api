create table if not exists user(
  id blob primary key,
  name Text not null,
  email Text not null UNIQUE,
  password Text not null
);
create unique index user_email_idx on user(email);
create table if not exists user_session(
  id blob PRIMARY KEY, 
  user_id blob not null,
  expires_in text not null,
  FOREIGN KEY(user_id) REFERENCES user(id)
);
create unique index user_sesh_idx on user_session(user_id);
