CREATE TABLE players(
    playeragent INTEGER NOT NULL,
    playergame INTEGER NOT NULL,
    playerrank INTEGER NOT NULL,
    PRIMARY KEY(playeragent, playergame),
    FOREIGN KEY(playeragent) REFERENCES agents(agentid),
    FOREIGN KEY(playergame) REFERENCES games(gameid)
) WITHOUT ROWID;
