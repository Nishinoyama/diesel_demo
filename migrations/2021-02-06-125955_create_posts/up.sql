CREATE TABLE posts (
	id serial primary key,
	title varchar(10) not null,
	body text not null,
	published boolean not null default false
);
