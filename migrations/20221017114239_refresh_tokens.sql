-- Add migration script here
CREATE TABLE `refresh_tokens` (
  `token` varchar(63) COLLATE utf8_unicode_ci NOT NULL,
  `access_token` varchar(63) COLLATE utf8_unicode_ci NOT NULL,
  `expires_at` datetime NOT NULL,
  PRIMARY KEY (`token`),
  CONSTRAINT `fk_refresh_tokens_access_tokens` FOREIGN KEY (`access_token`) REFERENCES `access_tokens` (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci
