-- Add migration script here
ALTER TABLE oauth_clients ADD client_id varchar(255) COLLATE utf8_unicode_ci NOT NULL;
ALTER TABLE oauth_clients ADD client_secret varchar(255) COLLATE utf8_unicode_ci NOT NULL;
