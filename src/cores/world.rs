use super::*;

lazy_static! {
    pub(crate) static ref WORLD: World = World {
        entities_manager: ENTITIES_MANAGER.clone(),
    };
}
#[derive(Clone)]
pub struct World {
    entities_manager: EntitiesManager,
}

impl World {
    pub(crate) fn destroy_entity(&self, _entity: Entity) {

    }
}
