#
# DATABASE CREATION FOR: mto_db
#

CREATE DATABASE IF NOT EXISTS mto_db
CHARACTER SET utf8mb3
COLLATE utf8mb3_general_ci;

USE mto_db;

#
# TABLE STRUCTURE FOR: httpbin_request
#

CREATE TABLE IF NOT EXISTS `httpbin_request` (
  `id` int(9) unsigned NOT NULL AUTO_INCREMENT,
  `batch_id` int(9) unsigned NOT NULL,
  `value` varchar(255) NOT NULL,
  `status` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

#
# TABLE STRUCTURE FOR: user
#

START TRANSACTION;
CREATE TABLE IF NOT EXISTS `user` (
  `id` int(9) unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(100) NOT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
INSERT INTO `user` VALUES (1, 'admin', 'admin');
COMMIT;


