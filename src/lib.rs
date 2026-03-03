use std::sync::{Arc, RwLock, Weak};

pub mod aero;
pub mod comp;
pub mod scene;

pub use comp::{Comp, Component};

pub use scene::Scene;

type ComponentRef = Arc<RwLock<dyn Component + Send + Sync>>;
type WeakComponentRef = Weak<RwLock<dyn Component + Send + Sync>>;
type ComponentID = u32;

type WeakSceneRef = Weak<RwLock<Scene>>;
