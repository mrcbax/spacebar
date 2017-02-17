DROP DATABASE IF EXISTS spacebardb;
CREATE DATABASE spacebardb;

\c spacebardb;

DROP TABLE IF EXISTS users;
CREATE TABLE users(
	user_ID SERIAL NOT NULL,
	user_name VARCHAR(25),
	email VARCHAR(225),
	password VARCHAR(225),
	last_barcode INT DEFAULT NULL,
	PRIMARY KEY(user_ID)
);


DROP TABLE IF EXISTS barcode;
CREATE TABLE barcode(
	id_user SERIAL NOT NULL,
	user_barcode text,
	name_barcode text,
	description text,
);


GRANT ALL on users TO root;
GRANT ALL on users_user_id_seq TO root;
GRANT ALL on barcode TO root;
GRANT ALL on barcode_id_user_seq TO root;

INSERT INTO users(user_name,email,password) VALUES('testuser','thisisfake@fake.com','exposed');
INSERT INTO users(user_name,email,password) VALUES('testuser2','thisisfake2@fake.com','exposed2');
INSERT INTO users(user_name,email,password) VALUES('testuser3','thisisfake3@fake.com','exposed3');
INSERT INTO users(user_name,email,password) VALUES('testuser4','thisisfake4@fake.com','exposed4');
INSERT INTO users(user_name,email,password) VALUES('testuser5','thisisfake5@fake.com','exposed5');
INSERT INTO users(user_name,email,password) VALUES('testuser6','thisisfake6@fake.com','exposed6');

