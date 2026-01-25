use crate::component::Component;
use crate::types::Transform;
use std::any::Any;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct EntityId(pub usize);

pub struct Entity {
    pub id: EntityId,
    pub tag: Option<String>,
    pub enabled: bool,
    pub marked_for_destruction: bool,
    pub transform: Transform,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            tag: None,
            enabled: true,
            marked_for_destruction: false,
            transform: Transform::default(),
            components: Vec::new(),
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn add_component<C: Component + 'static>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }

    pub fn add_component_boxed(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn get_component<T: Component + 'static>(&self) -> Option<&T> {
        self.components
            .iter()
            .find_map(|c| c.as_any().downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        self.components
            .iter_mut()
            .find_map(|c| c.as_any_mut().downcast_mut::<T>())
    }

    pub fn has_component<T: Component + 'static>(&self) -> bool {
        self.get_component::<T>().is_some()
    }

    pub fn components(&self) -> &[Box<dyn Component>] {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.components
    }
}
