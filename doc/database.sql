CREATE DATABASE `online_forum`;

USE online_forum;

-- online_forum.email_code definition
CREATE TABLE `email_code` (
    `email` varchar(254) NOT NULL,
    `code` int NOT NULL,
    `get_date` datetime NOT NULL,
    PRIMARY KEY (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- online_forum.post_section definition
CREATE TABLE `post_section` (
    `section_id` int NOT NULL AUTO_INCREMENT,
    `section_name` varchar(100) NOT NULL,
    `section_name_zh` varchar(100) NOT NULL,
    PRIMARY KEY (`section_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- online_forum.user_authority definition
CREATE TABLE `user_authority` (
    `auth_id` int unsigned NOT NULL AUTO_INCREMENT,
    `auth_level` int NOT NULL,
    `auth_name` varchar(100) NOT NULL,
    PRIMARY KEY (`auth_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- online_forum.user_table definition
CREATE TABLE `user_table` (
    `user_id` int unsigned NOT NULL AUTO_INCREMENT,
    `email` varchar(254) NOT NULL,
    `user_name` varchar(20) NOT NULL,
    `password` varchar(100) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
    `regist_time` datetime NOT NULL,
    `auth_id` int unsigned NOT NULL,
    `user_nickname` varchar(100) DEFAULT NULL,
    `avatar` varchar(100) DEFAULT NULL,
    `description` text CHARACTER SET utf8 COLLATE utf8_general_ci,
    `lastest_sign` datetime DEFAULT NULL,
    `token` varbinary(16) DEFAULT NULL,
    PRIMARY KEY (`user_id`),
    UNIQUE KEY `user_un` (`email`, `user_name`, `user_nickname`, `token`),
    KEY `user_FK` (`auth_id`),
    CONSTRAINT `user_FK` FOREIGN KEY (`auth_id`) REFERENCES `user_authority` (`auth_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- online_forum.post_table definition
CREATE TABLE `post_table` (
    `post_id` int NOT NULL AUTO_INCREMENT,
    `user_id` int unsigned NOT NULL,
    `section_id` int NOT NULL,
    `post_title` varchar(100) NOT NULL,
    `post_time` datetime NOT NULL,
    `post_update_time` datetime DEFAULT NULL,
    `post_content` text NOT NULL,
    PRIMARY KEY (`post_id`),
    KEY `post_table_FK` (`section_id`),
    KEY `post_table_FK_1` (`user_id`),
    CONSTRAINT `post_table_FK` FOREIGN KEY (`section_id`) REFERENCES `post_section` (`section_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `post_table_FK_1` FOREIGN KEY (`user_id`) REFERENCES `user_table` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- online_forum.comment_table definition
CREATE TABLE `comment_table` (
    `comment_id` int NOT NULL AUTO_INCREMENT,
    `post_id` int NOT NULL,
    `user_id` int unsigned NOT NULL,
    `reply_to` int DEFAULT NULL,
    `comment_time` datetime NOT NULL,
    `comment_content` text NOT NULL,
    PRIMARY KEY (`comment_id`),
    KEY `comment_table_FK` (`user_id`),
    KEY `comment_table_FK_1` (`post_id`),
    KEY `comment_table_FK_2` (`reply_to`),
    CONSTRAINT `comment_table_FK` FOREIGN KEY (`user_id`) REFERENCES `user_table` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `comment_table_FK_1` FOREIGN KEY (`post_id`) REFERENCES `post_table` (`post_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `comment_table_FK_2` FOREIGN KEY (`reply_to`) REFERENCES `comment_table` (`comment_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb3;

-- insert auth

INSERT INTO online_forum.user_authority (auth_level, auth_name) VALUES(1, '管理员');
INSERT INTO online_forum.user_authority (auth_level, auth_name) VALUES(2, '用户');
INSERT INTO online_forum.user_authority (auth_level, auth_name) VALUES(3, '游客');

-- insert post section

INSERT INTO online_forum.post_section (section_name, section_name_zh) VALUES('notify', '论坛通告');
INSERT INTO online_forum.post_section (section_name, section_name_zh) VALUES('tech-share', '技术分享');
INSERT INTO online_forum.post_section (section_name, section_name_zh) VALUES('tech-QA', '技术问答');
INSERT INTO online_forum.post_section (section_name, section_name_zh) VALUES('chat', '灌水闲聊');
INSERT INTO online_forum.post_section (section_name, section_name_zh) VALUES('issue', '论坛反馈');

-- insert root user
INSERT INTO
    online_forum.user_table (
        email,
        user_name,
        password,
        regist_time,
        auth_id,
        user_nickname
    )
VALUES
    (
        'root',
        'root',
        '123456789',
        now(),
        1,
        'root'
    );