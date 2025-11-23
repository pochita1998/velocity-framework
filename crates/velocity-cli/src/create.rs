use std::fs;
use std::path::PathBuf;
use colored::*;

/// Create a new Velocity project from a template
pub fn create_project(name: &str, template: &str) -> anyhow::Result<()> {
    let project_path = PathBuf::from(name);

    // Check if directory already exists
    if project_path.exists() {
        return Err(anyhow::anyhow!("Directory '{}' already exists", name));
    }

    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
    println!("{} {}", "⚡".bright_yellow(), format!("Creating Velocity Project").bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
    println!();

    // Create project structure
    fs::create_dir_all(&project_path)?;
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("dist"))?;
    fs::create_dir_all(project_path.join("public"))?;

    println!("{}  Creating project: {}", "✓".bright_green(), name.bright_white());
    println!("{}  Template: {}", "✓".bright_green(), template.bright_white());
    println!();

    // Write index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{} - Velocity App</title>
  <link rel="stylesheet" href="/public/style.css">
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/dist/index.js"></script>
</body>
</html>"#, name);

    fs::write(project_path.join("index.html"), index_html)?;
    println!("{}  index.html", "✓".bright_green());

    // Write package.json
    let package_json = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "type": "module",
  "scripts": {{
    "dev": "velocity dev",
    "build": "velocity build"
  }}
}}"#, name);

    fs::write(project_path.join("package.json"), package_json)?;
    println!("{}  package.json", "✓".bright_green());

    // Write example based on template
    let (app_code, styles) = match template {
        "counter" => (
            get_counter_template(),
            get_counter_styles()
        ),
        "minimal" => (
            get_minimal_template(),
            get_minimal_styles()
        ),
        _ => (
            get_counter_template(),
            get_counter_styles()
        ),
    };

    fs::write(project_path.join("src/index.tsx"), app_code)?;
    println!("{}  src/index.tsx", "✓".bright_green());

    fs::write(project_path.join("public/style.css"), styles)?;
    println!("{}  public/style.css", "✓".bright_green());

    // Copy runtime
    let runtime = include_str!("../../velocity-wasm/pkg/velocity_wasm.js");
    fs::write(project_path.join("dist/velocity-runtime.js"), runtime)?;
    println!("{}  dist/velocity-runtime.js", "✓".bright_green());

    // Write README
    let readme = format!(r#"# {}

A Velocity Framework project.

## Quick Start

```bash
# Start development server
velocity dev

# Build for production
velocity build
```

## Learn More

- [Velocity Documentation](https://github.com/pochita1998/velocity-framework)
- [Examples](https://github.com/pochita1998/velocity-framework/tree/main/examples)

Built with ⚡ by Velocity
"#, name);

    fs::write(project_path.join("README.md"), readme)?;
    println!("{}  README.md", "✓".bright_green());

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
    println!("{} {}", "🎉".bright_yellow(), "Project created successfully!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
    println!();
    println!("Next steps:");
    println!("  {}  cd {}", "→".bright_cyan(), name.bright_white());
    println!("  {}  velocity dev", "→".bright_cyan());
    println!();

    Ok(())
}

fn get_counter_template() -> &'static str {
    r#"import { createSignal, createMemo, render, createElement } from 'velocity-runtime';

function Counter() {
  const [count, setCount] = createSignal(0);
  const doubled = createMemo(() => count() * 2);

  return (
    <div class="container">
      <h1>⚡ Velocity Counter</h1>
      <div class="counter">{count}</div>
      <div class="info">Doubled: {doubled}</div>
      <div class="buttons">
        <button class="btn btn-secondary" onClick={() => setCount(c => c - 1)}>
          - Decrement
        </button>
        <button class="btn btn-primary" onClick={() => setCount(0)}>
          Reset
        </button>
        <button class="btn btn-secondary" onClick={() => setCount(c => c + 1)}>
          + Increment
        </button>
      </div>
    </div>
  );
}

render(() => <Counter />, document.getElementById('root') as HTMLElement);
"#
}

fn get_counter_styles() -> &'static str {
    r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.container {
  background: white;
  padding: 3rem;
  border-radius: 1rem;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  text-align: center;
  max-width: 500px;
  width: 100%;
}

h1 {
  font-size: 2rem;
  margin-bottom: 2rem;
  color: #1a202c;
}

.counter {
  font-size: 4rem;
  font-weight: bold;
  color: #667eea;
  margin: 2rem 0;
}

.info {
  font-size: 1.2rem;
  color: #718096;
  margin-bottom: 2rem;
}

.buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-primary {
  background: #667eea;
  color: white;
}

.btn-primary:hover {
  background: #764ba2;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-secondary {
  background: #edf2f7;
  color: #4a5568;
}

.btn-secondary:hover {
  background: #e2e8f0;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}
"#
}

fn get_minimal_template() -> &'static str {
    r#"import { render, createElement } from 'velocity-runtime';

function App() {
  return (
    <div class="container">
      <h1>Hello Velocity!</h1>
      <p>Edit src/index.tsx to get started.</p>
    </div>
  );
}

render(() => <App />, document.getElementById('root') as HTMLElement);
"#
}

fn get_minimal_styles() -> &'static str {
    r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  padding: 2rem;
  background: #f7fafc;
}

.container {
  max-width: 800px;
  margin: 0 auto;
  background: white;
  padding: 2rem;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

h1 {
  color: #1a202c;
  margin-bottom: 1rem;
}

p {
  color: #4a5568;
}
"#
}
