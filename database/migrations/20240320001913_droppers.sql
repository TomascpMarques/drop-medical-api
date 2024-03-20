-- Add migration script here
CREATE TABLE IF NOT EXISTS "dropper" (
  id serial primary key,
  --
  serial_id uuid default gen_random_uuid() not null unique,
  active boolean default false,
  owner_id uuid not null,
  machine_url text unique,
  name text not null,
  --
  FOREIGN KEY(owner_id) REFERENCES "user"(id)
);
create index if not exists dropper_serialn on "dropper"(serial_id);


CREATE TABLE IF NOT EXISTS "dropper_dispensse_schedule" (
  id serial not null unique,
  machine_id serial not null,
  --
  name varchar(60) not null unique,
  description varchar(120),
  --
  start_date TIMESTAMPTZ not null,
  end_date TIMESTAMPTZ,
  active Boolean default false,
  --
  FOREIGN KEY (machine_id) REFERENCES "dropper"(id),
  PRIMARY KEY (id, machine_id)
);
create index if not exists schedule_name on "dropper_dispensse_schedule"(name);

-- Many to 1 to dropper 
CREATE TABLE IF NOT EXISTS "dropper_pill" (
  id serial primary key,
  --
  pill_name varchar(32) not null,
  section numeric(1) not null,
  machine_id serial not null,
  --
  FOREIGN KEY(machine_id) REFERENCES "dropper"(id),
  unique (pill_name, section, machine_id)
);
create index if not exists pill_name on "dropper_pill"(pill_name);


-- Many to 1 to schedule
-- 1 to 1 to dropper_pills
CREATE TABLE IF NOT EXISTS "dropper_scheduled_dispensse" (
  id serial,
  --
  schedule_id serial not null,
  pill serial not null,
  quantity numeric(2) not null,
  --
  FOREIGN KEY (schedule_id) REFERENCES "dropper_dispensse_schedule"(id),
  FOREIGN KEY (pill) REFERENCES "dropper_pill"(id),
  PRIMARY KEY (id, schedule_id)
);


-- To know where the machine is;
-- To know what schedules the machine has;
-- To know that pills the machine contains;

