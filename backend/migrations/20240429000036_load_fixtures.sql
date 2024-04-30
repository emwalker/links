create table users (
  id text unique not null,
  username text unique not null
);

insert into users (id, username) values
  ('2db58326-ddfa-4561-9ae2-232aa5c32277', 'root');

create table roles (
  id text unique not null,
  name text unique not null
);

insert into roles (id, name) values
  ('79197f85-fb60-486f-b9fe-0aa0b10dabe2', 'admin'),
  ('99d8335a-1c23-4ad3-a10f-7e63fb3599d2', 'editor'),
  ('793dd5d3-7bf2-41b7-bd18-b2d6ba3d02c2', 'unknown');

create table users_roles (
  user_id text not null,
  role_id text not null,
  unique(user_id, role_id)
);

insert into users_roles (user_id, role_id) values
  ('2db58326-ddfa-4561-9ae2-232aa5c32277', '79197f85-fb60-486f-b9fe-0aa0b10dabe2'),
  ('2db58326-ddfa-4561-9ae2-232aa5c32277', '99d8335a-1c23-4ad3-a10f-7e63fb3599d2');
