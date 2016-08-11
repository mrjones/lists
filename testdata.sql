USE lists;

DELETE FROM item_annotations;
DELETE FROM items;
DELETE FROM list_users;
DELETE FROM lists;
DELETE FROM users;

INSERT INTO users (id, name) VALUES (1, "matt");
INSERT INTO users (id, name) VALUES (2, "cristina");

INSERT INTO lists (id, name) VALUES (1, "matts list 1");
INSERT INTO lists (id, name) VALUES (2, "cristina list 1");
INSERT INTO lists (id, name) VALUES (3, "matts list 2");
INSERT INTO lists (id, name) VALUES (4, "a shared list");

INSERT INTO list_users (user_id, list_id) VALUES (1, 1);
INSERT INTO list_users (user_id, list_id) VALUES (1, 3);
INSERT INTO list_users (user_id, list_id) VALUES (1, 4);

INSERT INTO list_users (user_id, list_id) VALUES (2, 2);
INSERT INTO list_users (user_id, list_id) VALUES (2, 4);

INSERT INTO items (id, list_id, name, description) VALUES (1, 1, "first item", "desc2");
INSERT INTO items (id, list_id, name, description) VALUES (2, 1, "second item", "desc 2");
INSERT INTO items (id, list_id, name, description) VALUES (3, 1, "google", "is a nice place to work");


INSERT INTO item_annotations (id, item_id, kind, body) VALUES (1, 3, "LINK", "http://www.google.com");
