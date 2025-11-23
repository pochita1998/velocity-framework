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
// Data Layer (Phase 5) - Resource Management
// ============================================================================

#[allow(dead_code)]
type ResourceId = usize;

thread_local! {
    static RESOURCE_CACHE: RefCell<HashMap<String, ResourceState>> = RefCell::new(HashMap::new());
    static NEXT_RESOURCE_ID: RefCell<ResourceId> = RefCell::new(0);
}

struct ResourceState {
    data: JsValue,
    loading: bool,
    error: Option<String>,
    timestamp: f64,
    refetch_fn: Option<js_sys::Function>,
}

/// Create a resource for async data fetching
#[wasm_bindgen(js_name = createResource)]
pub fn create_resource(
    key: &str,
    fetcher: &js_sys::Function,
) -> js_sys::Array {
    // Check cache first
    let cached = RESOURCE_CACHE.with(|cache| {
        cache.borrow().get(key).map(|state| {
            let result = js_sys::Array::new();
            result.push(&state.data);
            result.push(&JsValue::from_bool(state.loading));
            result.push(&state.error.clone().map(|e| JsValue::from_str(&e)).unwrap_or(JsValue::NULL));
            result
        })
    });

    if let Some(cached_result) = cached {
        return cached_result;
    }

    // Initialize loading state
    RESOURCE_CACHE.with(|cache| {
        cache.borrow_mut().insert(key.to_string(), ResourceState {
            data: JsValue::NULL,
            loading: true,
            error: None,
            timestamp: js_sys::Date::now(),
            refetch_fn: Some(fetcher.clone()),
        });
    });

    // Return initial loading state
    let result = js_sys::Array::new();
    result.push(&JsValue::NULL);
    result.push(&JsValue::from_bool(true));
    result.push(&JsValue::NULL);

    // Trigger async fetch
    let key_clone = key.to_string();
    let fetcher_clone = fetcher.clone();

    wasm_bindgen_futures::spawn_local(async move {
        match call_async_fetcher(&fetcher_clone).await {
            Ok(data) => {
                RESOURCE_CACHE.with(|cache| {
                    if let Some(state) = cache.borrow_mut().get_mut(&key_clone) {
                        state.data = data;
                        state.loading = false;
                        state.error = None;
                        state.timestamp = js_sys::Date::now();
                    }
                });
            }
            Err(err) => {
                RESOURCE_CACHE.with(|cache| {
                    if let Some(state) = cache.borrow_mut().get_mut(&key_clone) {
                        state.loading = false;
                        state.error = Some(format!("{:?}", err));
                        state.timestamp = js_sys::Date::now();
                    }
                });
            }
        }
    });

    result
}

async fn call_async_fetcher(fetcher: &js_sys::Function) -> Result<JsValue, JsValue> {
    let promise = fetcher.call0(&JsValue::NULL)?;
    let promise = js_sys::Promise::from(promise);
    wasm_bindgen_futures::JsFuture::from(promise).await
}

/// Invalidate a resource cache entry
#[wasm_bindgen(js_name = invalidateResource)]
pub fn invalidate_resource(key: &str) {
    RESOURCE_CACHE.with(|cache| {
        cache.borrow_mut().remove(key);
    });
}

/// Refetch a resource
#[wasm_bindgen(js_name = refetchResource)]
pub fn refetch_resource(key: &str) {
    let fetcher = RESOURCE_CACHE.with(|cache| {
        cache.borrow().get(key).and_then(|state| state.refetch_fn.clone())
    });

    if let Some(fetcher) = fetcher {
        invalidate_resource(key);
        create_resource(key, &fetcher);
    }
}

/// Update resource with optimistic value
#[wasm_bindgen(js_name = setResourceOptimistic)]
pub fn set_resource_optimistic(key: &str, value: JsValue) {
    RESOURCE_CACHE.with(|cache| {
        if let Some(state) = cache.borrow_mut().get_mut(key) {
            state.data = value;
        }
    });
}

/// Get current resource state
#[wasm_bindgen(js_name = getResourceState)]
pub fn get_resource_state(key: &str) -> js_sys::Array {
    RESOURCE_CACHE.with(|cache| {
        if let Some(state) = cache.borrow().get(key) {
            let result = js_sys::Array::new();
            result.push(&state.data);
            result.push(&JsValue::from_bool(state.loading));
            result.push(&state.error.clone().map(|e| JsValue::from_str(&e)).unwrap_or(JsValue::NULL));
            result
        } else {
            let result = js_sys::Array::new();
            result.push(&JsValue::NULL);
            result.push(&JsValue::from_bool(false));
            result.push(&JsValue::NULL);
            result
        }
    })
}

/// Clear all resource caches
#[wasm_bindgen(js_name = clearResourceCache)]
pub fn clear_resource_cache() {
    RESOURCE_CACHE.with(|cache| {
        cache.borrow_mut().clear();
    });
}

// ============================================================================
// SSR Support (Phase 6)
// ============================================================================

/// Render component to HTML string for SSR
#[wasm_bindgen(js_name = renderToString)]
pub fn render_to_string(component: &js_sys::Function) -> Result<String, JsValue> {
    // Create a virtual DOM context for SSR
    let result = component.call0(&JsValue::NULL)?;

    // Convert the result to HTML string
    // In a full implementation, this would traverse the component tree
    // and generate HTML with hydration markers

    Ok(format!(
        "<!DOCTYPE html>\
         <html>\
         <head><title>Velocity SSR</title></head>\
         <body>\
         <div id=\"root\" data-server-rendered=\"true\">{}</div>\
         <script type=\"module\" src=\"/velocity-runtime.js\"></script>\
         </body>\
         </html>",
        result.as_string().unwrap_or_default()
    ))
}

/// Render component to readable stream for streaming SSR
#[wasm_bindgen(js_name = renderToStream)]
pub fn render_to_stream(component: &js_sys::Function) -> Result<JsValue, JsValue> {
    // This would return a ReadableStream in a full implementation
    // For now, return the HTML as a promise
    let html = render_to_string(component)?;
    let promise = js_sys::Promise::resolve(&JsValue::from_str(&html));
    Ok(promise.into())
}

/// Hydrate server-rendered content on the client
#[wasm_bindgen(js_name = hydrateRoot)]
pub fn hydrate_root(container_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let container = document
        .get_element_by_id(container_id)
        .ok_or("Container not found")?;

    // Mark as hydrated
    container.set_attribute("data-hydrated", "true")?;

    // Find and hydrate all islands
    let islands = get_islands_to_hydrate()?;

    console::log_1(&format!("Hydrating {} island(s)", islands.length()).into());

    for i in 0..islands.length() {
        let island = islands.get(i);
        if let Ok(element) = island.dyn_into::<Element>() {
            mark_hydrated(&element)?;
        }
    }

    Ok(())
}

/// Check if running in SSR context (no window)
#[wasm_bindgen(js_name = isSSR)]
pub fn is_ssr() -> bool {
    web_sys::window().is_none()
}

/// Serialize app state for hydration
#[wasm_bindgen(js_name = serializeState)]
pub fn serialize_state() -> JsValue {
    let state = js_sys::Object::new();

    // Serialize all signals
    RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        let signals_obj = js_sys::Object::new();

        for (id, signal_state) in runtime.signals.iter() {
            let key = format!("signal_{}", id);
            js_sys::Reflect::set(
                &signals_obj,
                &JsValue::from_str(&key),
                &signal_state.value,
            ).ok();
        }

        js_sys::Reflect::set(&state, &JsValue::from_str("signals"), &signals_obj).ok();
    });

    // Serialize resource cache
    RESOURCE_CACHE.with(|cache| {
        let cache = cache.borrow();
        let resources_obj = js_sys::Object::new();

        for (key, resource) in cache.iter() {
            let resource_obj = js_sys::Object::new();
            js_sys::Reflect::set(&resource_obj, &JsValue::from_str("data"), &resource.data).ok();
            js_sys::Reflect::set(&resource_obj, &JsValue::from_str("loading"), &JsValue::from_bool(resource.loading)).ok();

            js_sys::Reflect::set(
                &resources_obj,
                &JsValue::from_str(key),
                &resource_obj,
            ).ok();
        }

        js_sys::Reflect::set(&state, &JsValue::from_str("resources"), &resources_obj).ok();
    });

    state.into()
}

/// Deserialize and restore app state during hydration
#[wasm_bindgen(js_name = deserializeState)]
pub fn deserialize_state(state: &JsValue) -> Result<(), JsValue> {
    let state_obj = js_sys::Object::from(state.clone());

    // Restore signals
    if let Ok(signals) = js_sys::Reflect::get(&state_obj, &JsValue::from_str("signals")) {
        let signals_obj = js_sys::Object::from(signals);
        let keys = js_sys::Object::keys(&signals_obj);

        for i in 0..keys.length() {
            if let Some(key) = keys.get(i).as_string() {
                if let Some(id_str) = key.strip_prefix("signal_") {
                    if let Ok(id) = id_str.parse::<usize>() {
                        if let Ok(value) = js_sys::Reflect::get(&signals_obj, &JsValue::from_str(&key)) {
                            deserialize_signal_state(id, value);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// Production Polish (Phase 7)
// ============================================================================

thread_local! {
    static ERROR_BOUNDARY_HANDLERS: RefCell<Vec<js_sys::Function>> = RefCell::new(Vec::new());
    static DEVTOOLS_ENABLED: RefCell<bool> = RefCell::new(false);
}

/// Create an error boundary to catch component errors
#[wasm_bindgen(js_name = createErrorBoundary)]
pub fn create_error_boundary(
    component: &js_sys::Function,
    fallback: &js_sys::Function,
) -> js_sys::Function {
    let component_clone = component.clone();
    let fallback_clone = fallback.clone();

    let boundary = Closure::wrap(Box::new(move || -> JsValue {
        match component_clone.call0(&JsValue::NULL) {
            Ok(result) => result,
            Err(error) => {
                console::error_2(&"Error boundary caught:".into(), &error);

                // Trigger error handlers
                ERROR_BOUNDARY_HANDLERS.with(|handlers| {
                    for handler in handlers.borrow().iter() {
                        let _ = handler.call1(&JsValue::NULL, &error);
                    }
                });

                // Return fallback UI
                fallback_clone.call0(&JsValue::NULL).unwrap_or(JsValue::NULL)
            }
        }
    }) as Box<dyn Fn() -> JsValue>);

    let func = boundary.as_ref().clone();
    boundary.forget();
    func.into()
}

/// Register a global error handler
#[wasm_bindgen(js_name = onError)]
pub fn on_error(handler: &js_sys::Function) {
    ERROR_BOUNDARY_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(handler.clone());
    });
}

/// Enable DevTools integration
#[wasm_bindgen(js_name = enableDevTools)]
pub fn enable_dev_tools() {
    DEVTOOLS_ENABLED.with(|enabled| {
        *enabled.borrow_mut() = true;
    });

    // Expose runtime internals to window for DevTools
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    // Create __VELOCITY_DEVTOOLS__ object
    let devtools = js_sys::Object::new();

    // Expose signal inspection
    let signals_fn = Closure::wrap(Box::new(|| -> JsValue {
        RUNTIME.with(|runtime| {
            let runtime = runtime.borrow();
            let signals_obj = js_sys::Object::new();

            for (id, state) in runtime.signals.iter() {
                let signal_info = js_sys::Object::new();
                js_sys::Reflect::set(&signal_info, &JsValue::from_str("value"), &state.value).ok();
                js_sys::Reflect::set(&signal_info, &JsValue::from_str("subscribers"), &JsValue::from_f64(state.subscribers.len() as f64)).ok();

                js_sys::Reflect::set(
                    &signals_obj,
                    &JsValue::from_str(&format!("signal_{}", id)),
                    &signal_info,
                ).ok();
            }

            signals_obj.into()
        })
    }) as Box<dyn Fn() -> JsValue>);

    js_sys::Reflect::set(&devtools, &JsValue::from_str("getSignals"), signals_fn.as_ref()).ok();
    signals_fn.forget();

    // Expose resource inspection
    let resources_fn = Closure::wrap(Box::new(|| -> JsValue {
        RESOURCE_CACHE.with(|cache| {
            let cache = cache.borrow();
            let resources_obj = js_sys::Object::new();

            for (key, resource) in cache.iter() {
                let resource_info = js_sys::Object::new();
                js_sys::Reflect::set(&resource_info, &JsValue::from_str("data"), &resource.data).ok();
                js_sys::Reflect::set(&resource_info, &JsValue::from_str("loading"), &JsValue::from_bool(resource.loading)).ok();
                js_sys::Reflect::set(&resource_info, &JsValue::from_str("timestamp"), &JsValue::from_f64(resource.timestamp)).ok();

                js_sys::Reflect::set(&resources_obj, &JsValue::from_str(key), &resource_info).ok();
            }

            resources_obj.into()
        })
    }) as Box<dyn Fn() -> JsValue>);

    js_sys::Reflect::set(&devtools, &JsValue::from_str("getResources"), resources_fn.as_ref()).ok();
    resources_fn.forget();

    // Attach to window
    js_sys::Reflect::set(&window, &JsValue::from_str("__VELOCITY_DEVTOOLS__"), &devtools).ok();

    console::log_1(&"✨ Velocity DevTools enabled".into());
}

/// Get performance metrics
#[wasm_bindgen(js_name = getMetrics)]
pub fn get_metrics() -> JsValue {
    let metrics = js_sys::Object::new();

    RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        js_sys::Reflect::set(&metrics, &JsValue::from_str("signalCount"), &JsValue::from_f64(runtime.signals.len() as f64)).ok();
        js_sys::Reflect::set(&metrics, &JsValue::from_str("effectCount"), &JsValue::from_f64(runtime.effects.len() as f64)).ok();
    });

    RESOURCE_CACHE.with(|cache| {
        let cache = cache.borrow();
        js_sys::Reflect::set(&metrics, &JsValue::from_str("resourceCount"), &JsValue::from_f64(cache.len() as f64)).ok();
    });

    metrics.into()
}

/// Log a performance mark for benchmarking
#[wasm_bindgen(js_name = mark)]
pub fn mark(name: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(performance) = js_sys::Reflect::get(&window, &JsValue::from_str("performance")) {
            let performance: web_sys::Performance = performance.into();
            let _ = performance.mark(name);
        }
    }
}

/// Measure performance between two marks
#[wasm_bindgen(js_name = measure)]
pub fn measure(name: &str, start_mark: &str, end_mark: &str) -> f64 {
    if let Some(window) = web_sys::window() {
        if let Ok(performance) = js_sys::Reflect::get(&window, &JsValue::from_str("performance")) {
            let performance: web_sys::Performance = performance.into();
            // measure_with_start_mark_and_end_mark returns Result<(), JsValue>
            // We need to get the measure entry from getEntriesByName
            if performance.measure_with_start_mark_and_end_mark(name, start_mark, end_mark).is_ok() {
                let entries = performance.get_entries_by_name(name);
                if entries.length() > 0 {
                    let entry = entries.get(entries.length() - 1);
                    if let Ok(measure) = entry.dyn_into::<web_sys::PerformanceMeasure>() {
                        return measure.duration();
                    }
                }
            }
        }
    }
    0.0
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
