DROP DATABASE IF EXISTS spacebardb;
CREATE DATABASE spacebardb;

\c spacebardb;

DROP TABLE IF EXISTS users;
CREATE TABLE users(
	ID SERIAL NOT NULL,
	username VARCHAR(25),
	email VARCHAR(225),
	password VARCHAR(225),
	userID SERIAL,
	PRIMARY KEY(ID)
);

INSERT INTO users(username,email,password) VALUES('testuser','thisisfake@fake.com','exposed');
