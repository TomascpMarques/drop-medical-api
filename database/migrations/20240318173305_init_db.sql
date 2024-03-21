create table if not exists "user" (
  id uuid default gen_random_uuid(),
  name varchar(40) not null,
  email varchar(40) not null UNIQUE,
  password Text not null,
  PRIMARY KEY(id)
);

create table if not exists "user_session" (
  id uuid default gen_random_uuid(),
  user_id uuid not null unique,
  expires_in TIMESTAMPTZ not null,
  PRIMARY KEY(id),
  constraint fk_user_id
    FOREIGN KEY(user_id) 
      REFERENCES "user"(id)
);
