-- CREATE USER 'lists'@'localhost' IDENTIFIED BY 'lists';
-- CREATE DATABASE lists;
-- GRANT ALL ON lists.* TO 'lists'@'localhost';
-- $ mysql -ulists -hlocalhost -plists < setup.sql 

USE lists;

DROP TABLE IF EXISTS item_annotations;
DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS list_users;
DROP TABLE IF EXISTS lists;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
       id BIGINT NOT NULL AUTO_INCREMENT,
       name VARCHAR(255) NOT NULL,
       PRIMARY KEY(id)
);

CREATE TABLE lists (
       id BIGINT NOT NULL AUTO_INCREMENT,
       name VARCHAR(255) NOT NULL,
       PRIMARY KEY(id)
);

CREATE TABLE items (
       id BIGINT NOT NULL AUTO_INCREMENT,
       list_id BIGINT,
       name VARCHAR(255) NOT NULL,
       description LONGTEXT,
       PRIMARY KEY(id),
       FOREIGN KEY (list_id) REFERENCES lists(id)
);

CREATE TABLE list_users (
       id BIGINT NOT NULL AUTO_INCREMENT,
       user_id BIGINT,
       list_id BIGINT,
       PRIMARY KEY(id),
       FOREIGN KEY(user_id) REFERENCES users(id),
       FOREIGN KEY(list_id) REFERENCES lists(id)
);

CREATE TABLE item_annotations (
       id BIGINT NOT NULL AUTO_INCREMENT,
       item_id BIGINT,
       kind ENUM('LINK'),
       body VARCHAR(255),
       PRIMARY KEY(id),
       FOREIGN KEY(item_id) REFERENCES items(id)
);
