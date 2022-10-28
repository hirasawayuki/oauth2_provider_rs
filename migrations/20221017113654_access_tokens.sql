-- Add migration script here
CREATE TABLE `access_tokens` (
  `token` varchar(63) COLLATE utf8_unicode_ci NOT NULL,
  `user_id` int(10) unsigned NOT NULL,
  `client_id` varchar(63) COLLATE utf8_unicode_ci NOT NULL,
  `scope` varchar(63) COLLATE utf8_unicode_ci NOT NULL,
  `expires_at` datetime NOT NULL,
  PRIMARY KEY (`token`),
  CONSTRAINT `fk_access_tokens_users` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  CONSTRAINT `fk_access_tokens_oauth_clients` FOREIGN KEY (`client_id`) REFERENCES `oauth_clients` (`client_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci
