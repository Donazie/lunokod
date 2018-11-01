CREATE TABLE users(
    userid INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    useremail TEXT NOT NULL UNIQUE,
    userpassword TEXT NOT NULL,
    useradmin INTEGER NOT NULL,
    usercreated TEXT NOT NULL,
    useractive TEXT
);
