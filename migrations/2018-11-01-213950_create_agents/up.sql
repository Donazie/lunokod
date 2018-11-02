CREATE TABLE agents(
    agentid INTEGER NOT NULL,
    agentuser INTEGER NOT NULL,
    agentcode TEXT NOT NULL,
    agentcreated DATETIME NOT NULL,
    agentmodified DATETIME,
    PRIMARY KEY(agentid, agentuser),
    FOREIGN KEY(agentuser) REFERENCES users(userid)
);
