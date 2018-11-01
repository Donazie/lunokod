CREATE TABLE agents(
    agentid INTEGER NOT NULL,
    agentuser INTEGER NOT NULL,
    agentcode TEXT NOT NULL,
    agentcreated TEXT NOT NULL,
    agentmodified TEXT,
    PRIMARY KEY(agentid, agentuser),
    FOREIGN KEY(agentuser) REFERENCES users(userid)
);
