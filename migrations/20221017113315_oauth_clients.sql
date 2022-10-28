-- Add migration script here
CREATE TABLE `oauth_clients` (
  `client_id` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `client_secret` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `name` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `user_id` int(10) unsigned NOT NULL,
  `scope` varchar(32) COLLATE utf8_unicode_ci NOT NULL,
  `redirect_uri` varchar(256) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`client_id`),
  UNIQUE KEY `redirect_uri` (`redirect_uri`) USING BTREE,
  CONSTRAINT `fk_oauth_clients_users` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci
