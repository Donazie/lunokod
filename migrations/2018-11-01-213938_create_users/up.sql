CREATE TABLE users(
    userid INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    useremail VARCHAR(32) NOT NULL UNIQUE,
    userpassword VARCHAR(255) NOT NULL,
    useradmin BOOLEAN NOT NULL,
    usercreated DATETIME NOT NULL,
    useractive DATETIME
);
