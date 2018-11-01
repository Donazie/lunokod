CREATE TABLE games(
    gameid INTEGER NOT NULL,
    gameuser INTEGER NOT NULL,
    gamestarted TEXT NOT NULL,
    gamefinished TEXT NOT NULL,
    PRIMARY KEY(gameid, gameuser),
    FOREIGN KEY(gameuser) REFERENCES users(userid)
);
