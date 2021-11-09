SET GLOBAL SQL_MODE = "STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION";  
DROP TABLE IF EXISTS comment;
DROP TABLE IF EXISTS thread;
DROP TABLE IF EXISTS user;
CREATE TABLE user (
 id int(16) PRIMARY KEY AUTO_INCREMENT NOT NULL, 
    username varchar(32) NOT NULL,
     email varchar(32) NOT NULL,
     create_time datetime NOT NULL,  
     password varchar(32) NOT NULL, 
     UNIQUE KEY username (username),
    UNIQUE KEY email (email)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE comment (
 id varchar(32) PRIMARY KEY NOT NULL,
     thread_id varchar(32) NOT NULL,
    from_user_id int(16) NOT NULL,
    to_user_id int(16) NOT NULL,
    status tinyint(2) unsigned DEFAULT 1,
    create_time datetime NOT NULL,

)ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE thread (
  id varchar(32) PRIMARY KEY NOT NULL,
    user_id int(16) NOT NULL,
        status tinyint(2) unsigned DEFAULT 1,
type tinyint(2) unsigned NOT NULL, 
   create_time datetime NOT NULL,

)ENGINE = InnoDB DEFAULT CHARSET = utf8;