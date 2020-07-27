create table guild_config
(
    guild_id bigint not null
        constraint guild_config_pkey
            primary key,
    prefix   text   not null
);

alter table guild_config
    owner to postgres;