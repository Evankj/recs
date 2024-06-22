use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type EntityId = usize;

pub type Component = Box<dyn Any>;

#[derive(Debug)]
pub struct Entity {
    index: EntityId,
    components: HashMap<TypeId, Component>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            index: id,
            components: HashMap::new(),
        }
    }

    /**
     * Will overwrite current entry for this component if present
     **/
    pub fn add_component<T: 'static + Default>(&mut self) -> &mut T {
        let type_id = TypeId::of::<T>();

        let components = &mut self.components;

        components.insert(type_id, Box::new(T::default()));

        return components
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut()
            .unwrap();
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(component) = self.components.get(&type_id) {
            return component.downcast_ref::<T>();
        }
        return None;
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(component) = self.components.get_mut(&type_id) {
            return component.downcast_mut::<T>();
        }
        return None;
    }
}

#[derive(Debug)]
pub struct Bucket {
    free_ids: Vec<EntityId>,
    entities: Vec<Option<Entity>>,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            free_ids: vec![],
        }
    }

    pub fn create_entity(&mut self) -> Result<&mut Entity, &str> {
        let id: EntityId;
        if let Some(index) = self.free_ids.pop() {
            id = index;
        } else {
            id = self.entities.len();
            self.entities.push(None);
        }

        let entity = Entity::new(id);
        self.entities[id] = Some(entity);

        if let Some(ent_ref) = self.entities.get_mut(id) {
            if let Some(ent) = ent_ref {
                return Ok(ent);
            }
        }

        return Err("Failed to create entity!");
    }

    pub fn delete_entity_by_id(&mut self, id: EntityId) -> Result<(), &str> {
        if let Some(entity) = self.entities.get_mut(id) {
            *entity = None;
            self.free_ids.push(id);
            return Ok(());
        };

        return Err("Could not delete entity");
    }

    pub fn get_all_entities(&mut self) -> &mut Vec<Option<Entity>> {
        return &mut self.entities;
    }

    pub fn get_entity_by_id(&mut self, id: EntityId) -> Option<&mut Entity> {
        if let Some(entity) = self.entities.get_mut(id) {
            return entity.as_mut();
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entity_to_bucket() {
        let mut bucket = Bucket::new();
        let _ = bucket.create_entity();
        assert!(bucket.entities.len() == 1);
        let ent = bucket.entities.get(0).unwrap().as_ref().unwrap();
        assert_eq!(ent.index, 0);
    }

    #[test]
    fn test_remove_entity_from_bucket() {
        let mut bucket = Bucket::new();
        let index = bucket.create_entity().unwrap().index;
        let deleted = bucket.delete_entity_by_id(index);
        assert!(deleted.is_ok());
        assert_eq!(*bucket.free_ids.get(0).unwrap(), 0);
    }

    #[test]
    fn test_get_component_from_entity() {
        let mut bucket = Bucket::new();
        let ent = bucket.create_entity().unwrap();

        #[derive(Default)]
        struct TestComponent {
            x: i32,
            y: i32,
        }

        let test_component = ent.add_component::<TestComponent>();
        test_component.x = 1;
        test_component.y = 2;

        let comp = ent.get_component::<TestComponent>().unwrap();

        assert_eq!(comp.x, 1);
        assert_eq!(comp.y, 2);

        let comp_mut = ent.get_component_mut::<TestComponent>().unwrap();

        assert_eq!(comp_mut.x, 1);
        assert_eq!(comp_mut.y, 2);
    }
}
