-- Add migration script here
CREATE TABLE `oauth_clients` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(32) COLLATE utf8_unicode_ci,
  `scope` varchar(32) COLLATE utf8_unicode_ci NOT NULL DEFAULT '',
  `redirect_uri` varchar(256) COLLATE utf8_unicode_ci NOT NULL,
  `revoked` tinyint(1) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  UNIQUE KEY `redirect_uri` (`redirect_uri`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci
