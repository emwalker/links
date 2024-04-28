create table users (
  id text not null,
  username text unique not null
);

insert into users (id, username) values ('2db58326-ddfa-4561-9ae2-232aa5c32277', 'root');
