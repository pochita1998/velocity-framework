use anyhow::Result;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct BundlerConfig {
    pub root_dir: PathBuf,
    pub out_dir: PathBuf,
    pub minify: bool,
}

pub struct Bundler {
    config: BundlerConfig,
}

impl Bundler {
    pub fn new(config: BundlerConfig) -> Self {
        Self { config }
    }

    pub fn build(&self) -> Result<()> {
        // Create output directory
        fs::create_dir_all(&self.config.out_dir)?;

        // Find entry point
        let entry = self.config.root_dir.join("src/index.tsx");
        if !entry.exists() {
            anyhow::bail!("Entry point not found: src/index.tsx");
        }

        // Process modules
        let modules = self.collect_modules(&entry)?;

        // Bundle
        let bundle = self.bundle_modules(&modules)?;

        // Write output
        let output_path = self.config.out_dir.join("bundle.js");
        fs::write(output_path, bundle)?;

        // Copy index.html if exists
        let html_path = self.config.root_dir.join("index.html");
        if html_path.exists() {
            fs::copy(html_path, self.config.out_dir.join("index.html"))?;
        }

        Ok(())
    }

    fn collect_modules(&self, entry: &PathBuf) -> Result<Vec<crate::Module>> {
        let mut modules = Vec::new();
        let content = fs::read_to_string(entry)?;

        // For now, pass through content as-is
        // JSX transformation will be handled by the runtime bundler (Vite, etc.)
        let transformed = content.clone();

        modules.push(crate::Module {
            path: entry.clone(),
            content: content.clone(),
            transformed,
            dependencies: Vec::new(),
        });

        Ok(modules)
    }

    fn bundle_modules(&self, modules: &[crate::Module]) -> Result<String> {
        let mut bundle = String::new();

        // Add runtime imports
        bundle.push_str("import { createSignal, createEffect, render } from 'velocity-runtime';\n\n");

        // Add all modules
        for module in modules {
            bundle.push_str(&module.transformed);
            bundle.push_str("\n\n");
        }

        Ok(bundle)
    }
}
