use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

use crate::{ComponentID, ComponentRef, WeakComponentRef, WeakSceneRef};

trait ComponentHelpers {
    fn _set_parent(&mut self, comp: WeakComponentRef);
    fn _get_parent(&self) -> Option<WeakComponentRef>;
    
    fn _insert_child(&mut self, c: WeakComponentRef);
}

pub trait Component: ComponentHelpers + Send + Sync {
    fn add_child(&mut self, comp: WeakComponentRef) -> Result<(), String>;
    fn get_child(&self, name: &str) -> Option<WeakComponentRef>;
    fn rm_child(&mut self, name: &str) -> Result<(), String>;

    fn set_name(&mut self, name: &str);
    fn get_name(&self) -> String;

    fn get_parent(&self) -> Option<WeakComponentRef>;
    fn set_parent(&mut self, comp: WeakComponentRef) -> Result<(), String>;

    fn get_ref(&self) -> Option<WeakComponentRef>;
    fn get_id(&self) -> ComponentID;
}

enum Child {
    Child(WeakComponentRef),
    ChildArray(Vec<WeakComponentRef>),
}

pub struct Comp {
    id: ComponentID,
    name: String,
    parent: Option<WeakComponentRef>,
    self_ref: Option<WeakComponentRef>,
    children: HashMap<String, Child>,
}

impl Comp {
    pub fn new(name: &str, scene: Option<WeakSceneRef>) -> ComponentRef {
        let comp = Arc::new(RwLock::new(Comp {
            name: name.to_string(),
            parent: None,
            self_ref: None,
            children: HashMap::new(),
            id: 0,
        }));
        
        if let Some(w) = scene {
            if let Some(s) = w.upgrade() {
                comp.write().unwrap().id = s.write().unwrap().subscribe_component(comp.clone());
            }
        }
        
        let c = Arc::downgrade(&comp);
        comp.write().unwrap().self_ref = Some(c);
        
        comp
    }
}

impl ComponentHelpers for Comp {
    fn _set_parent(&mut self, comp: WeakComponentRef) {
        self.parent = Some(comp);
    }

    fn _get_parent(&self) -> Option<WeakComponentRef> {
        self.parent.clone()
    }

    fn _insert_child(&mut self, c: WeakComponentRef) {
        if let Some(upgraded) = c.upgrade() {
            let id = upgraded.read().unwrap().get_id();
            self.children.insert(id, Child::Child(c));
        }
    }
}

impl Component for Comp {
    fn add_child(&mut self, c: WeakComponentRef) -> Result<(), String> {
        let id = c.read().unwrap().get_name();
        if self.children.contains_key(&id) {
            return Err(format!("component {id} already exists"));
        }
        if let Some(self_ref) = self.get_ref() {
            c.write().unwrap().set_parent(self_ref)?;
        }
        self.children.insert(id, c);
        Ok(())
    }

    fn get_ref(&self) -> Option<WeakComponentRef> {
        self.self_ref.clone()
    }

    fn get_id(&self) -> ComponentID {
        self.id
    }

    fn get_child(&self, name: &str) -> Option<WeakComponentRef> {
        self.children.get(name).map(|c| Arc::downgrade(c))
    }

    fn rm_child(&mut self, name: &str) -> Result<(), String> {
        if self.children.contains_key(name) {
            self.children.remove(name);
            Ok(())
        } else {
            Err(format!("child {name} was not found"))
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_parent(&self) -> Option<WeakComponentRef> {
        self.parent.clone()
    }

    /// deletes itself from current parent's children list and set up a new component as a parent
    fn set_parent(&mut self, c: WeakComponentRef) -> Result<(), String> {
        // check if 'c' is a valid reference
        if let Some(p) = c.upgrade() {
            // check if this component has a parent
            if let Some(current_parent) = &self.parent {
                // return Err() if 'c' already is the current parent
                if Weak::ptr_eq(&current_parent, &c) {
                    return Err(format!(
                        "{} already is the parent of this component",
                        p.read().unwrap().get_name()
                    ));
                }
                // remove itself reference from old parent
                if let Some(upgraded) = current_parent.upgrade() {
                    upgraded.write().unwrap().rm_child(&self.name)?;
                }
                self.parent = Some(c);
                // add itself into new parent children list
                if let Some(weak) = self.get_ref() {
                    if let Some(self_ref) = weak.upgrade() {
                        p.write().unwrap().add_child(self_ref)?;
                    }
                }
                return Ok(());
            }
            // if not:
            self.parent = Some(c);
            // add itself into new parent children list
            if let Some(weak) = self.get_ref() {
                if let Some(self_ref) = weak.upgrade() {
                    p.write().unwrap().add_child(self_ref)?;
                }
            }
            return Ok(());
        }
        Err("invalid component: invalid reference".to_string())
    }
}
