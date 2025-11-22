// Efficient DOM operations without Virtual DOM

import { createEffect } from './reactivity';

type Child = Node | string | number | boolean | null | undefined | (() => Child);
type Children = Child | Child[];

/**
 * Create an element with props and children
 */
export function createElement(
  tag: string,
  props: Record<string, any> | null,
  ...children: Children[]
): HTMLElement {
  const element = document.createElement(tag);

  // Apply props
  if (props) {
    for (const [key, value] of Object.entries(props)) {
      if (key.startsWith('on')) {
        // Event listener
        const eventName = key.slice(2).toLowerCase();
        element.addEventListener(eventName, value);
      } else if (key === 'ref') {
        // Ref callback
        if (typeof value === 'function') {
          value(element);
        } else if (value) {
          value.current = element;
        }
      } else if (key === 'class' || key === 'className') {
        // Class handling
        if (typeof value === 'function') {
          createEffect(() => {
            element.className = value();
          });
        } else {
          element.className = value;
        }
      } else if (key === 'style') {
        // Style handling
        if (typeof value === 'object') {
          Object.assign(element.style, value);
        } else {
          element.setAttribute('style', value);
        }
      } else if (typeof value === 'function') {
        // Reactive attribute
        createEffect(() => {
          const result = value();
          if (result != null && result !== false) {
            element.setAttribute(key, String(result));
          } else {
            element.removeAttribute(key);
          }
        });
      } else if (value != null && value !== false) {
        element.setAttribute(key, String(value));
      }
    }
  }

  // Append children
  appendChildren(element, children);

  return element;
}

/**
 * Append children to a parent element
 */
function appendChildren(parent: HTMLElement, children: Children[]): void {
  for (const child of children) {
    if (Array.isArray(child)) {
      appendChildren(parent, child);
    } else if (typeof child === 'function') {
      // Reactive child
      const marker = document.createTextNode('');
      parent.appendChild(marker);

      let currentNode: Node | null = marker;

      createEffect(() => {
        const value = child();
        const newNode = normalizeChild(value);

        if (currentNode && currentNode.parentNode) {
          currentNode.parentNode.replaceChild(newNode, currentNode);
          currentNode = newNode;
        }
      });
    } else if (child != null && child !== false && child !== true) {
      parent.appendChild(normalizeChild(child));
    }
  }
}

/**
 * Normalize a child to a DOM node
 */
function normalizeChild(child: Child): Node {
  if (child instanceof Node) {
    return child;
  }
  return document.createTextNode(String(child));
}

/**
 * Insert a node into the DOM
 */
export function insert(parent: Element, child: Child, marker?: Node): void {
  if (typeof child === 'function') {
    createEffect(() => {
      insert(parent, child(), marker);
    });
  } else if (Array.isArray(child)) {
    child.forEach(c => insert(parent, c, marker));
  } else if (child != null && child !== false && child !== true) {
    const node = normalizeChild(child);
    if (marker) {
      parent.insertBefore(node, marker);
    } else {
      parent.appendChild(node);
    }
  }
}

/**
 * Render a component to the DOM
 */
export function render(code: () => any, container: Element): () => void {
  let dispose: (() => void) | null = null;

  dispose = createEffect(() => {
    // Clear container
    container.innerHTML = '';

    // Render component
    const result = code();
    if (result) {
      insert(container, result);
    }
  });

  return () => {
    if (dispose) dispose();
    container.innerHTML = '';
  };
}

/**
 * Create a portal to render children in a different location
 */
export function createPortal(children: Children, container: Element): null {
  createEffect(() => {
    const temp = document.createElement('div');
    appendChildren(temp, [children]);
    container.appendChild(temp.firstChild!);
  });
  return null;
}
