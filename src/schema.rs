table! {
    agents (agentid, agentuser) {
        agentid -> Integer,
        agentuser -> Integer,
        agentcode -> Text,
        agentcreated -> Timestamp,
        agentmodified -> Nullable<Timestamp>,
    }
}

table! {
    games (gameid, gameuser) {
        gameid -> Integer,
        gameuser -> Integer,
        gamestarted -> Timestamp,
        gamefinished -> Timestamp,
    }
}

table! {
    players (playeragent, playergame) {
        playeragent -> Integer,
        playergame -> Integer,
        playerrank -> Integer,
    }
}

table! {
    users (userid) {
        userid -> Integer,
        username -> Text,
        useremail -> Text,
        userpassword -> Text,
        useradmin -> Bool,
        usercreated -> Timestamp,
        useractive -> Nullable<Timestamp>,
    }
}

joinable!(agents -> users (agentuser));
joinable!(games -> users (gameuser));

allow_tables_to_appear_in_same_query!(agents, games, players, users,);
