pub trait Game {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn image(&self) -> &str;
}

#[derive(juniper::GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Started,
    Stopping,
}

pub trait Server {
    fn name(&self) -> &str;
    fn game(&self) -> &str;
    fn status(&self) -> &ServerStatus;
}

#[derive(Clone)]
struct GameData {
    id: String,
    name: String,
    image: String,
}

struct ServerData {
    name: String,
    game: String,
    status: ServerStatus,
}

impl Game for GameData {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn image(&self) -> &str {
        &self.image
    }
}

impl Server for ServerData {
    fn name(&self) -> &str {
        &self.name
    }

    fn game(&self) -> &str {
        &self.game
    }

    fn status(&self) -> &ServerStatus {
        &self.status
    }
}

pub struct Database {
    games: Vec<GameData>,
    servers: Vec<ServerData>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            games: vec![
                GameData {
                    id: "factorio".to_owned(),
                    name: "Factorio Experimental 0.18.18".to_owned(),
                    image: "427520".to_owned(),
                },
                GameData {
                    id: "satisfactory".to_owned(),
                    name: "Satisfactory Early Access".to_owned(),
                    image: "526870".to_owned(),
                },
            ],
            servers: vec![
                ServerData {
                    name: "factorio-01".to_owned(),
                    game: "factorio".to_owned(),
                    status: ServerStatus::Stopped,
                },
                ServerData {
                    name: "satisfactory-01".to_owned(),
                    game: "satisfactory".to_owned(),
                    status: ServerStatus::Started,
                },
            ],
        }
    }

    pub fn get_games(&self) -> Vec<&dyn Game> {
        self.games.iter().map(|x| x as &dyn Game).collect()
    }

    pub fn get_game(&self, id: &str) -> Option<&dyn Game> {
        self.games
            .iter()
            .filter(|&x| x.id == id)
            .map(|x| x as &dyn Game)
            .next()
    }

    pub fn get_servers(&self) -> Vec<&dyn Server> {
        self.servers.iter().map(|x| x as &dyn Server).collect()
    }
}

impl juniper::Context for Database {}
