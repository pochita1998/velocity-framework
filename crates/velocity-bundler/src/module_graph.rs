use std::collections::HashMap;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct ModuleGraph {
    modules: HashMap<PathBuf, crate::Module>,
}

#[allow(dead_code)]
impl ModuleGraph {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: crate::Module) {
        self.modules.insert(module.path.clone(), module);
    }

    pub fn get_module(&self, path: &PathBuf) -> Option<&crate::Module> {
        self.modules.get(path)
    }
}
