-- Add migration script here
ALTER TABLE oauth_clients ADD user_id int(10) unsigned NOT NULL;
ALTER TABLE oauth_clients ADD CONSTRAINT `fk_oauth_clients_users` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);
