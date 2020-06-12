use super::model::response::{GameData, GameserverData};
use super::model::Id;

pub trait GameserverView: Send + Sync + 'static {
    fn game_data(&self, id: &Id) -> Box<dyn GameData>;
    fn game_list(&self) -> Vec<Box<dyn GameData>>;

    fn server_data(&self, id: &Id) -> Box<dyn GameserverData>;
    fn server_list(&self) -> Vec<Box<dyn GameserverData>>;
}
