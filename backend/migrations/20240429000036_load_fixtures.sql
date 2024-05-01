create table users (
  id text primary key not null,
  username text unique not null,
  name text,
  registration_completed boolean default false not null,
  is_admin boolean not null default false,
  created_at datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
);

insert into users (id, username, name, is_admin) values
  ('2db58326-ddfa-4561-9ae2-232aa5c32277', 'root', 'Root account', true),
  ('082efe61-ce6c-432e-a5c3-8e773338f52c', 'stars', 'Astronomy Fan', false),
  ('ed7e73fd-1a1d-4e80-a3d6-0fbad682a9bf', 'neutrinos', 'Astrophysicist', false),
  ('95d7b3cf-0f84-4753-bdd4-b730182841ee', 'databases', 'Database Expert', false),
  ('dbb1a64a-65fc-45fc-bce8-9710e00f29ad', 'ambient', 'Ambient Audiophile', false),
  ('5cdf23c1-55a2-4235-b666-0d9b48921a04', 'country', 'Country Fan', false),
  ('e137847f-6b27-4801-9df0-d58e5750d2aa', 'emwalker', 'Eric Walker', false);

create table topics (
  id text primary key not null,
  name text unique not null,
  owner_id text not null,
  submissions_restricted_to_editors boolean default false,
  created_at datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
);

-- TODO: Add support for synonyms and other languages
insert into topics (id, owner_id, name, submissions_restricted_to_editors) values
  ('63fa2799-f9ba-41d2-8f8b-49c8eac659fc', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Root topic', true),
  ('921f9bd2-8d9e-43b2-aef8-c90c10e1eb03', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Knowledge, learning and education', false),
  ('49414a5f-46af-4317-b190-cffb2465c9c0', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Art, music, theater, poetry and film', false),
  ('f2717e2a-4445-48a2-9b71-10dc62b967d2', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Music', false),
  ('65aa6f91-cc35-4037-aeac-ec7b8a655b00', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Science', false),
  ('acb6260c-9670-4c23-91f7-75ca61745e0c', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Astronomy', false),
  ('269a7947-daab-4d1c-afa4-13e7b794968e', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Physics', false),
  ('952d7815-08cd-455c-8e31-708e711476df', '2db58326-ddfa-4561-9ae2-232aa5c32277', 'Astrophysics', false);

-- Should there be an owner_id?
-- Should multiple rows be allowed, one for each value of owner_id?
create table topics_topics (
  parent_id text not null references topics(id),
  child_id text not null references topics(id),
  created_at datetime default current_timestamp not null,
  primary key (parent_id, child_id)
);

insert into topics_topics (parent_id, child_id) values
  -- Root/Knowledge
  ('63fa2799-f9ba-41d2-8f8b-49c8eac659fc', '921f9bd2-8d9e-43b2-aef8-c90c10e1eb03'),
  -- Knowledge/Science
  ('921f9bd2-8d9e-43b2-aef8-c90c10e1eb03', '65aa6f91-cc35-4037-aeac-ec7b8a655b00'),
  -- Science/Astronomy
  ('65aa6f91-cc35-4037-aeac-ec7b8a655b00', 'acb6260c-9670-4c23-91f7-75ca61745e0c'),
  -- Science/Physics
  ('65aa6f91-cc35-4037-aeac-ec7b8a655b00', '269a7947-daab-4d1c-afa4-13e7b794968e'),
  -- Astronomy/Astrophysics
  ('acb6260c-9670-4c23-91f7-75ca61745e0c', '952d7815-08cd-455c-8e31-708e711476df'),
  -- Physics/Astrophysics
  ('269a7947-daab-4d1c-afa4-13e7b794968e', '952d7815-08cd-455c-8e31-708e711476df'),
  -- Root/Art
  ('63fa2799-f9ba-41d2-8f8b-49c8eac659fc', '49414a5f-46af-4317-b190-cffb2465c9c0'),
  -- Art/Music
  ('49414a5f-46af-4317-b190-cffb2465c9c0', 'f2717e2a-4445-48a2-9b71-10dc62b967d2');

-- Topic roles are sparsely specified.  If a user is an editor or admin of a parent topic, the user
-- is also an editor or admin of any child topics.
-- Roles:
--   0 - viewer
--   1 - editor
--   2 - admin
create table users_topics (
  user_id text not null references users(id),
  topic_id text not null references topics(id),
  role text check (role in ('viewer', 'editor', 'admin')) not null,
  created_at datetime default current_timestamp not null,
  primary key (user_id, topic_id, role)
);

insert into users_topics (user_id, topic_id, role) values
  -- Root/Root topic/admin
  ('2db58326-ddfa-4561-9ae2-232aa5c32277', '63fa2799-f9ba-41d2-8f8b-49c8eac659fc', 'admin');
