CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    hashed_password varchar not null,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);

create unique index users_username on users(username);

create table auth_tokens (
    id uuid primary key,
    user_id uuid not null references users(id),
    token varchar not null,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);

create unique index auth_tokens_token on auth_tokens(token);
