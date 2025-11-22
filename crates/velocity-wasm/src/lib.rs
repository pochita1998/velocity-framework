use wasm_bindgen::prelude::*;
use web_sys::{console, Element, HtmlElement, Node};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// Use wee_alloc as the global allocator for smaller WASM size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Signal System
// ============================================================================

type EffectFn = Rc<dyn Fn()>;
type SignalId = usize;
type EffectId = usize;

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
    static CURRENT_EFFECT: RefCell<Option<EffectId>> = RefCell::new(None);
}

struct Runtime {
    next_signal_id: SignalId,
    next_effect_id: EffectId,
    signals: HashMap<SignalId, SignalState>,
    effects: HashMap<EffectId, Effect>,
}

struct SignalState {
    value: JsValue,
    subscribers: Vec<EffectId>,
}

struct Effect {
    func: EffectFn,
    dependencies: Vec<SignalId>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            next_signal_id: 0,
            next_effect_id: 0,
            signals: HashMap::new(),
            effects: HashMap::new(),
        }
    }

    fn create_signal(&mut self, initial_value: JsValue) -> SignalId {
        let id = self.next_signal_id;
        self.next_signal_id += 1;

        self.signals.insert(
            id,
            SignalState {
                value: initial_value,
                subscribers: Vec::new(),
            },
        );

        id
    }

    fn read_signal(&mut self, id: SignalId) -> JsValue {
        // Track dependency
        if let Some(effect_id) = CURRENT_EFFECT.with(|e| *e.borrow()) {
            if let Some(signal) = self.signals.get_mut(&id) {
                if !signal.subscribers.contains(&effect_id) {
                    signal.subscribers.push(effect_id);
                }
            }
            if let Some(effect) = self.effects.get_mut(&effect_id) {
                if !effect.dependencies.contains(&id) {
                    effect.dependencies.push(id);
                }
            }
        }

        self.signals
            .get(&id)
            .map(|s| s.value.clone())
            .unwrap_or(JsValue::UNDEFINED)
    }

    fn write_signal(&mut self, id: SignalId, value: JsValue) -> Vec<EffectId> {
        // Collect subscribers BEFORE updating the value
        let subscribers: Vec<EffectId> = self.signals
            .get(&id)
            .map(|s| s.subscribers.clone())
            .unwrap_or_default();

        // Update the signal value
        if let Some(signal) = self.signals.get_mut(&id) {
            signal.value = value;
        }

        // Return the subscribers to notify (caller will run effects)
        subscribers
    }

    fn create_effect(&mut self, func: EffectFn) -> EffectId {
        let id = self.next_effect_id;
        self.next_effect_id += 1;

        let effect = Effect {
            func,
            dependencies: Vec::new(),
        };

        self.effects.insert(id, effect);

        id
    }

    fn run_effect(id: EffectId) {
        // Prepare the effect (clear dependencies, set context)
        RUNTIME.with(|runtime| {
            let mut r = runtime.borrow_mut();

            // Clone dependencies before clearing to avoid borrow issues
            let old_deps: Vec<SignalId> = r.effects
                .get(&id)
                .map(|e| e.dependencies.clone())
                .unwrap_or_default();

            // Clear old subscribers
            for signal_id in old_deps {
                if let Some(signal) = r.signals.get_mut(&signal_id) {
                    signal.subscribers.retain(|&e| e != id);
                }
            }

            // Clear dependencies
            if let Some(effect) = r.effects.get_mut(&id) {
                effect.dependencies.clear();
            }
        });

        // Set current effect context
        CURRENT_EFFECT.with(|e| *e.borrow_mut() = Some(id));

        // Clone the effect function so we can call it without holding a borrow
        let effect_fn = RUNTIME.with(|runtime| {
            runtime.borrow().effects.get(&id).map(|e| e.func.clone())
        });

        // Run the effect WITHOUT holding any borrow on Runtime
        if let Some(func) = effect_fn {
            func();
        }

        // Clear current effect context
        CURRENT_EFFECT.with(|e| *e.borrow_mut() = None);
    }
}

// ============================================================================
// Public API
// ============================================================================

#[wasm_bindgen]
pub struct Signal {
    id: SignalId,
}

#[wasm_bindgen]
impl Signal {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_value: JsValue) -> Signal {
        let id = RUNTIME.with(|runtime| runtime.borrow_mut().create_signal(initial_value));
        Signal { id }
    }

    #[wasm_bindgen(js_name = get)]
    pub fn get(&self) -> JsValue {
        RUNTIME.with(|runtime| runtime.borrow_mut().read_signal(self.id))
    }

    #[wasm_bindgen(js_name = set)]
    pub fn set(&self, value: JsValue) {
        // Get the list of subscribers to notify
        let subscribers = RUNTIME.with(|runtime| {
            runtime.borrow_mut().write_signal(self.id, value)
        });

        // Run effects after releasing the borrow
        for effect_id in subscribers {
            Runtime::run_effect(effect_id);
        }
    }
}

#[wasm_bindgen(js_name = createSignal)]
pub fn create_signal(initial_value: JsValue) -> Vec<JsValue> {
    let signal = Signal::new(initial_value);
    let signal_ref = Rc::new(RefCell::new(signal));

    let getter_signal = signal_ref.clone();
    let getter = Closure::wrap(Box::new(move || {
        getter_signal.borrow().get()
    }) as Box<dyn Fn() -> JsValue>);

    let setter_signal = signal_ref.clone();
    let setter = Closure::wrap(Box::new(move |value: JsValue| {
        setter_signal.borrow().set(value);
    }) as Box<dyn Fn(JsValue)>);

    let result = vec![
        getter.as_ref().clone(),
        setter.as_ref().clone(),
    ];

    getter.forget();
    setter.forget();

    result
}

#[wasm_bindgen(js_name = createEffect)]
pub fn create_effect(func: &js_sys::Function) {
    let func_clone = func.clone();
    let effect_fn = Rc::new(move || {
        match func_clone.call0(&JsValue::NULL) {
            Ok(_) => {},
            Err(e) => {
                console::error_2(&"Effect error:".into(), &e);
            }
        }
    });

    let effect_id = RUNTIME.with(|runtime| {
        runtime.borrow_mut().create_effect(effect_fn)
    });

    // Run the effect immediately after creating it
    Runtime::run_effect(effect_id);
}

// ============================================================================
// DOM Utilities
// ============================================================================

#[wasm_bindgen(js_name = createElement)]
pub fn create_element(tag: &str) -> Result<HtmlElement, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let element = document.create_element(tag)?;
    Ok(element.dyn_into::<HtmlElement>()?)
}

#[wasm_bindgen(js_name = createTextNode)]
pub fn create_text_node(text: &str) -> Result<Node, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    Ok(document.create_text_node(text).into())
}

#[wasm_bindgen(js_name = setText)]
pub fn set_text(element: &Element, text: &str) {
    element.set_text_content(Some(text));
}

#[wasm_bindgen(js_name = appendChild)]
pub fn append_child(parent: &Node, child: &Node) -> Result<(), JsValue> {
    parent.append_child(child)?;
    Ok(())
}

#[wasm_bindgen(js_name = setAttribute)]
pub fn set_attribute(element: &Element, name: &str, value: &str) -> Result<(), JsValue> {
    element.set_attribute(name, value)
}

#[wasm_bindgen(js_name = addClass)]
pub fn add_class(element: &Element, class: &str) -> Result<(), JsValue> {
    element.class_list().add_1(class)
}

#[wasm_bindgen(js_name = removeClass)]
pub fn remove_class(element: &Element, class: &str) -> Result<(), JsValue> {
    element.class_list().remove_1(class)
}

// ============================================================================
// Hydration Support (Phase 4)
// ============================================================================

/// Mark a component as an island (interactive zone that needs hydration)
#[wasm_bindgen(js_name = markIsland)]
pub fn mark_island(element: &Element, island_id: &str) -> Result<(), JsValue> {
    element.set_attribute("data-island", island_id)?;
    element.set_attribute("data-hydrate", "pending")?;
    Ok(())
}

/// Serialize signal state for SSR
#[wasm_bindgen(js_name = serializeSignalState)]
pub fn serialize_signal_state(signal_id: usize) -> JsValue {
    RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        if let Some(signal) = runtime.signals.get(&signal_id) {
            signal.value.clone()
        } else {
            JsValue::NULL
        }
    })
}

/// Deserialize and restore signal state during hydration
#[wasm_bindgen(js_name = deserializeSignalState)]
pub fn deserialize_signal_state(signal_id: usize, value: JsValue) {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        if let Some(signal) = runtime.signals.get_mut(&signal_id) {
            signal.value = value;
        }
    });
}

/// Check if an element needs hydration
#[wasm_bindgen(js_name = needsHydration)]
pub fn needs_hydration(element: &Element) -> bool {
    element.get_attribute("data-hydrate")
        .map(|val| val == "pending")
        .unwrap_or(false)
}

/// Mark an element as hydrated
#[wasm_bindgen(js_name = markHydrated)]
pub fn mark_hydrated(element: &Element) -> Result<(), JsValue> {
    element.set_attribute("data-hydrate", "complete")?;
    Ok(())
}

/// Get all islands in the DOM that need hydration
#[wasm_bindgen(js_name = getIslandsToHydrate)]
pub fn get_islands_to_hydrate() -> Result<js_sys::Array, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let selector = "[data-hydrate='pending']";
    let node_list = document.query_selector_all(selector)?;

    let result = js_sys::Array::new();
    for i in 0..node_list.length() {
        if let Some(node) = node_list.get(i) {
            result.push(&node);
        }
    }

    Ok(result)
}

// ============================================================================
// Initialization
// ============================================================================

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Velocity WASM Runtime initialized".into());
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello from Velocity WASM, {}!", name).into());
}
