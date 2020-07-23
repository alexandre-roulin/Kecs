use super::*;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Entity {
    id: Id,
}
#[allow(dead_code)]
impl Entity {
    pub(super) fn new(id: Id) -> Self {
        Entity { id }
    }


    ///Return the ID of Entity
    pub fn get_id(&self) -> Id {
        self.id
    }

    ///Kill the entity in the next `World` update
    pub fn kill(&self) {
        ENTITIES_MANAGER.kill_entity(*self);
    }

    ///Kill all entity in the same group of `self`
    pub fn kill_group(&self) {
        ENTITIES_MANAGER.kill_group_by_entity(self);
    }

    ///Tag the entity with `Tag`
    pub fn tag_by_id(&self, tag: Tag) {
        ENTITIES_MANAGER.tag_entity(self, tag)
    }

    /// Return a option if the current Entity is tagged
    pub fn get_tag(&self) -> Option<Tag> {
        ENTITIES_MANAGER.get_tag_by_entity(self)
    }


    pub fn group_by_id(&self, grpid: GrpTag) {
    }
}
