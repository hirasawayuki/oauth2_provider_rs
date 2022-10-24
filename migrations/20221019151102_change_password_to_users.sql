-- Add migration script here
ALTER TABLE users MODIFY `password` varchar(100) COLLATE utf8_unicode_ci NOT NULL;
