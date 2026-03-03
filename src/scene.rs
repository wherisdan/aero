use std::{collections::HashMap, sync::Arc};

use crate::comp::Comp;
use crate::{ComponentID, ComponentRef, WeakComponentRef};

pub struct Scene {
    active_components: HashMap<ComponentID, ComponentRef>,
    root_component: WeakComponentRef,
    next_id: ComponentID,
}

impl Scene {
    pub fn new() -> Scene {
        let root = Comp::new("root", None);
        let mut s = Scene {
            active_components: HashMap::new(),
            root_component: Arc::downgrade(&root),
            next_id: 0,
        };
        s.subscribe_component(root);
        s
    }

    pub(crate) fn subscribe_component(&mut self, comp: ComponentRef) -> ComponentID {
        self.active_components.insert(self.next_id, comp);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn get_root_component(&self) -> WeakComponentRef {
        self.root_component.clone()
    }
}
