-- Add migration script here
ALTER TABLE users MODIFY `password` varchar(36) COLLATE utf8_unicode_ci NOT NULL;
