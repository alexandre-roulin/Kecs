mod entities_manager;
mod entity;

type Id = usize;
type Tag = usize;
type GrpTag = usize;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

//Intern use
use crate::cores::{World, WORLD};
pub(crate) use entities_manager::{
    EntitiesManager, 
    ENTITIES_MANAGER
};
use entity::{
    Entity
};
