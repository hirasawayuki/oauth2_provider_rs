-- Add migration script here
ALTER TABLE users MODIFY `password` varchar(100);
