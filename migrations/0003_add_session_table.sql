
create table if not exists sessions
(
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    exp timestamptz NOT NULL,
    user_id uuid NOT NULL references users (id) on delete cascade,
    user_agent varchar not null default ''
);

alter table sessions
    add constraint sessions_id_pk primary key (id);
