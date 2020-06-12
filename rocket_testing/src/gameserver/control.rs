use super::model::request::{GameserverCreate, GameserverUpdate};
use super::model::Id;

pub trait GameserverControl: Send + Sync + 'static {
    fn create_server(&self, create: &dyn GameserverCreate) -> Id;
    fn delete_server(&self, id: &Id);
    fn start_server(&self, id: &Id);
    fn stop_server(&self, id: &Id);
    fn update_server(&self, update: &dyn GameserverUpdate);
}
