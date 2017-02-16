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

GRANT ALL on users TO root;


INSERT INTO users(username,email,password) VALUES('testuser','thisisfake@fake.com','exposed');
INSERT INTO users(username,email,password) VALUES('testuser2','thisisfake2@fake.com','exposed2');
INSERT INTO users(username,email,password) VALUES('testuser3','thisisfake3@fake.com','exposed3');
INSERT INTO users(username,email,password) VALUES('testuser4','thisisfake4@fake.com','exposed4');
INSERT INTO users(username,email,password) VALUES('testuser5','thisisfake5@fake.com','exposed5');
INSERT INTO users(username,email,password) VALUES('testuser6','thisisfake6@fake.com','exposed6');

