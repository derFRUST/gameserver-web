use crate::gameserver::model;

impl From<juniper::ID> for model::Id {
    fn from(id: juniper::ID) -> Self {
        model::Id::new(id.to_string())
    }
}

impl From<&juniper::ID> for model::Id {
    fn from(id: &juniper::ID) -> Self {
        model::Id::new(id.to_string())
    }
}

impl From<model::Id> for juniper::ID {
    fn from(id: model::Id) -> Self {
        juniper::ID::from(id.to_string().clone())
    }
}

impl From<&model::Id> for juniper::ID {
    fn from(id: &model::Id) -> Self {
        juniper::ID::from(id.to_string().clone())
    }
}
