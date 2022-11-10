CREATE TABLE IF NOT EXISTS `tiny_link` (
    `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
    `origin_url` varchar(1024) NOT NULL,
    `tiny_code` varchar(10) Default NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tiny_code` (`tiny_code`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
