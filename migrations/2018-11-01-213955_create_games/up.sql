CREATE TABLE games(
    gameid INTEGER NOT NULL,
    gameuser INTEGER NOT NULL,
    gamestarted DATETIME NOT NULL,
    gamefinished DATETIME NOT NULL,
    PRIMARY KEY(gameid, gameuser),
    FOREIGN KEY(gameuser) REFERENCES users(userid)
);
