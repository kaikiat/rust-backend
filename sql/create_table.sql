-- CREATE DATABASE leetcode;
-- \connect leetcode;

CREATE TABLE solutions   (
	id serial PRIMARY KEY,
	title VARCHAR ( 128 ) NOT NULL,
	description VARCHAR ( 128 ) NULL,
	code TEXT NOT NULL,
	created_on TIMESTAMP NOT NULL,
	modified_on TIMESTAMP NULL
);