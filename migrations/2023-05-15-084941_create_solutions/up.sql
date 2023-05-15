-- Your SQL goes here
CREATE TABLE solutions   (
	id serial PRIMARY KEY,
	title VARCHAR ( 32 ) NOT NULL,
	description VARCHAR ( 128 ) NULL,
	code VARCHAR ( 255 ) NOT NULL,
	created_on TIMESTAMP NOT NULL,
	modified_on TIMESTAMP NULL
);