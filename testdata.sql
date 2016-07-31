USE lists;

DELETE FROM items;
DELETE FROM lists;
DELETE FROM list_users;
DELETE FROM users;

INSERT INTO users (id, name) VALUES (1, "matt");

INSERT INTO lists (id, name) VALUES (1, "test list");

INSERT INTO list_users (user_id, list_id) VALUES (1, 1);

INSERT INTO items (id, list_id, name, description) VALUES (1, 1, "first item", "desc2");
INSERT INTO items (id, list_id, name, description) VALUES (2, 1, "second item", "desc 2");
