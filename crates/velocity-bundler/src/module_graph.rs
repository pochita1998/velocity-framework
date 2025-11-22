use std::collections::HashMap;
use std::path::PathBuf;

pub struct ModuleGraph {
    modules: HashMap<PathBuf, crate::Module>,
}

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
