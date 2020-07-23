use super::*;
const BASE_RESIZE: usize = 100;

///
///   
///
///
///
#[allow(dead_code)]
pub struct InnerEntitiesManager {
    world: World,
    next_id: Id,
    free_ids: VecDeque<Id>,
    valid_ids: Vec<bool>,
    tagged_entity_id: Vec<Option<Tag>>,     // [Entity::Id] = Tag
    tagged_entities: Vec<Option<Entity>>,       // [Tag] = Entity::Id
    grouped_entity_id: Vec<Option<Tag>>,    // [Entity::Id] = Tag
    grouped_entities: Vec<Option<Vec<Entity>>>, // [Tag] = [Entity::Id, ...]
}
#[derive(Clone)]
pub struct EntitiesManager(pub Arc<Mutex<InnerEntitiesManager>>);

lazy_static! {
    pub(crate) static ref ENTITIES_MANAGER: EntitiesManager =
        EntitiesManager(Arc::new(Mutex::new(InnerEntitiesManager {
            world: WORLD.clone(),
            next_id: 0,
            free_ids: VecDeque::new(),
            valid_ids: Vec::new(),
            tagged_entity_id: Vec::new(),
            tagged_entities: Vec::new(),
            grouped_entity_id: Vec::new(),
            grouped_entities: Vec::new(),
        })));
}

#[allow(dead_code)]
impl EntitiesManager {
    /* ******************************************************* */
    /* Global function */
    /* ******************************************************* */

    pub(crate) fn create_entity(&self) -> Entity {
        let em = &mut *self.0.lock().unwrap();
        let id = em.free_ids.pop_front().unwrap_or_else(|| {
            em.next_id += 1;
            em.next_id - 1
        });
        let entity = Entity::new(id);

        if em.valid_ids.len() < id.into() {
            em.valid_ids.resize(em.valid_ids.len() + BASE_RESIZE, false);
        }
        
        //TODO Add vector component Mask
        //TODO Assert tag and mask
        em.valid_ids[id] = true;
        entity
    }

    pub(crate) fn destroy_entity(&self, entity: Entity) {
        //TODO Reset component mask

        let em = &*self.0.lock().unwrap();
        let id = entity.get_id();

        //make sure the entity is valid
        assert!(em.valid_ids[id]);

        //Unvalid his `Tag`
        if let Some(Some(tag)) = em.tagged_entity_id.get_mut(id).take() {
            let _ = em.tagged_entities[*tag].take();
        }

        //Unvalid his Group `Tag`
        if let Some(Some(tag)) = em.grouped_entity_id.get_mut(id).take() {
            em.grouped_entities[*tag]
                .as_mut()
                .unwrap()
                .retain(|grp_id| grp_id.get_id() != id);
        }
        //Free the `Id` and push it to the free vector
        em.valid_ids[id] = false;
        em.free_ids.push_back(id);
    }

    /* ******************************************************* */
    /* Getter */
    /* ******************************************************* */

    pub fn get_entity_by_id(&self, id: Id) -> Entity {
        assert!(self.has_entity_by_id(id));
        Entity::new(id)
    }
    pub fn has_entity_by_id(&self, id: Id) -> bool {
        (*self.0.lock().unwrap()).valid_ids[id]
    }
    pub fn get_entities(&self) -> Vec<Entity> {
        self.0
            .lock()
            .unwrap()
            .valid_ids
            .iter()
            .enumerate()
            .filter_map(|(id, valid)| if *valid { Some(Entity::new(id)) } else { None })
            .collect::<Vec<Entity>>()
    }

    /* ******************************************************* */
    /* Kill */
    /* ******************************************************* */
    pub(crate) fn kill_entity(&self, entity: Entity) {
        //TODO Add world destory entity
        self.0.lock().unwrap().world.destroy_entity(entity);
    }

    pub(crate) fn kill_group_by_entity(&self, entity: &Entity) {

    }
    pub(crate) fn kill_group_by_grptag(&self, group_id: Tag) {
        let em = &mut *self.0.lock().unwrap();
        assert!(em.grouped_entities[group_id].is_some());
        let group = em.grouped_entities[group_id].take().unwrap();
        for entity in group {
            em.world.destroy_entity(entity);
        }
    }

    /* ******************************************************* */
    /* Manage Tag with Entity */
    /* ******************************************************* */


    pub fn get_tag_by_entity(&self, entity: &Entity) -> Option<Tag> {
        if let Some(tag) = self.0.lock().unwrap().tagged_entity_id.get(entity.get_id()) {
            *tag
        } else { None }
        
    }

    pub fn has_entity_by_tag(&self, tag: Tag) -> bool {
        if let Some(entity) = self.0.lock().unwrap().tagged_entities.get(tag) {
            entity.is_some()
        } else {
            false
        }
    }

    pub fn get_entity_by_tag(&self, tag: Tag) -> Entity {
        assert!(self.has_entity_by_tag(tag));
        self.0.lock().unwrap().tagged_entities[tag].unwrap()
    }

    pub fn tag_entity(&mut self, entity: &Entity, tag: Tag) {
        let em = &mut *self.0.lock().unwrap();
        let id = entity.get_id();
        let max = std::cmp::max(tag, id);

        if em.tagged_entities.len() < max {
            assert!(em.tagged_entity_id.len() < max);
            em.tagged_entities.resize(max + BASE_RESIZE, None);
            em.tagged_entity_id.resize(max + BASE_RESIZE, None);
        }
        em.tagged_entities[tag] = Some(entity.clone());
        em.tagged_entity_id[id] = Some(tag);
    }

    /* ******************************************************* */
    /* Manage Group with Entity */
    /* ******************************************************* */

    pub fn has_entities_by_grptag(&self, tag: GrpTag) -> bool {
        if let Some(grp) = self.0.lock().unwrap().grouped_entities.get(tag) {
            grp.is_some()
        } else {
            false
        }
    }

    pub fn get_entities_by_grptag(&self, tag: GrpTag) -> Vec<Entity> {
        assert!(self.has_entities_by_grptag(tag));
        self.0
            .lock()
            .unwrap()
            .grouped_entities
            .get(tag)
            .unwrap()
            .as_ref()
            .unwrap()
            .iter()
            .map(|id| id.clone())
            .collect()
    }

    fn group_entity_by_grptag(&self, entity: &Entity, tag: GrpTag) {
        let em = &mut *self.0.lock().unwrap();
        em.grouped_entity_id[entity.get_id()] = Some(tag);
        em.grouped_entities[tag].get_or_insert(vec![]).push(*entity);
    }

    fn has_grptag_by_entity(&self, entity:&Entity)-> bool {
        assert!(self.has_entity_by_id(entity.get_id()));
        let em = &mut *self.0.lock().unwrap();
        em.grouped_entity_id[entity.get_id()].is_some()
    }

    fn get_grptag_by_entity(&self, entity:&Entity) -> GrpTag {
        assert!(self.has_grptag_by_entity(entity));
        let em = &mut *self.0.lock().unwrap();
        em.grouped_entity_id[entity.get_id()].unwrap()
    }
}
