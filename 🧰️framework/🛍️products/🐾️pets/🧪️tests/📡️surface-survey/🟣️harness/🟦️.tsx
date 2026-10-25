/** 🟣️ The shared declarations of the 📡️ surface survey suite tests. */
import { vi } from "vitest";
import { type SurveyHost } from "@semio-tech/pets-react";
import vectors from "../../🧫️fixtures/📡️surface-survey/🔣️.json";

export interface Box {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

export interface SceneNode {
  readonly key: string;
  readonly tag: string;
  readonly attributes: Readonly<Record<string, string>>;
  readonly box?: Box;
  readonly overflow?: string;
  readonly children: readonly SceneNode[];
}

export interface Scene {
  readonly name: string;
  readonly viewport: { readonly width: number; readonly height: number };
  readonly frame?: Box;
  readonly focus?: string;
  readonly pointer?: { readonly x: number; readonly y: number };
  readonly options?: { readonly surfaces?: string; readonly keepoutsBesidesDefaults?: string; readonly walls?: string; readonly props?: string };
  readonly nodes: readonly SceneNode[];
  readonly expected: {
    readonly kind: string;
    readonly width: number;
    readonly height: number;
    readonly surfaces: readonly { readonly element: string; readonly x0: number; readonly x1: number; readonly y: number }[];
    readonly keepouts: readonly { readonly x: number; readonly y: number; readonly width: number; readonly height: number }[];
    readonly walls: readonly { readonly element: string; readonly side: 1 | -1; readonly x: number; readonly y0: number; readonly y1: number }[];
    readonly fixtures?: readonly { readonly element: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number }[];
  };
}

export const scenes = vectors.scenes as readonly Scene[];

/** 📦️ Gives an element the box a layout engine would have given it (jsdom lays nothing out). */
export function place<T extends Element>(element: T, box: Box): T {
  element.getBoundingClientRect = () => ({ ...box, x: box.left, y: box.top, right: box.left + box.width, bottom: box.top + box.height, toJSON: () => box }) as DOMRect;
  return element;
}

/** 🪟️ Sizes the viewport the way a browser reports it without scrollbars. */
export function viewport(width: number, height: number): void {
  Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: width });
  Object.defineProperty(document.documentElement, "clientHeight", { configurable: true, value: height });
}

export function build(nodes: readonly SceneNode[], parent: Element, built: Map<string, HTMLElement>): void {
  for (const node of nodes) {
    const element = document.createElement(node.tag);
    for (const [name, value] of Object.entries(node.attributes)) element.setAttribute(name, value);
    if (node.box !== undefined) place(element, node.box);
    if (node.overflow !== undefined) {
      element.style.overflowX = node.overflow;
      element.style.overflowY = node.overflow;
    }
    built.set(node.key, element);
    parent.append(element);
    build(node.children, element, built);
  }
}

export function card(box: Box, name = "data-pet-surface"): HTMLElement {
  const element = place(document.createElement("article"), box);
  element.setAttribute(name, "");
  document.body.append(element);
  return element;
}

/** 🖱️ A pointer event with the fields the feed reads, whether or not this DOM has a `PointerEvent` of its own. */
export function pointer(type: string, init: MouseEventInit & { readonly pointerType?: string; readonly isPrimary?: boolean }): MouseEvent {
  const event = new MouseEvent(type, { bubbles: type !== "pointerleave", cancelable: true, ...init });
  Object.defineProperty(event, "pointerType", { value: init.pointerType ?? "mouse" });
  Object.defineProperty(event, "isPrimary", { value: init.isPrimary ?? true });
  return event;
}

/** 📒️ Records what is listening on a target: every `addEventListener` not yet undone by a matching `removeEventListener`. */
export function ledger(target: EventTarget): { readonly open: () => string[]; readonly options: () => unknown[]; readonly close: () => void } {
  const open: { readonly type: string; readonly listener: unknown; readonly capture: boolean }[] = [];
  const seen: unknown[] = [];
  const capture = (options: unknown): boolean => (typeof options === "boolean" ? options : ((options as AddEventListenerOptions | undefined)?.capture ?? false));
  const add = vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: unknown, options?: unknown) => {
    open.push({ type, listener, capture: capture(options) });
    seen.push(options);
  });
  const remove = vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: unknown, options?: unknown) => {
    const index = open.findIndex((entry) => entry.type === type && entry.listener === listener && entry.capture === capture(options));
    if (index >= 0) open.splice(index, 1);
  });
  return {
    open: () => open.map((entry) => entry.type).sort(),
    options: () => seen,
    close: () => {
      add.mockRestore();
      remove.mockRestore();
    },
  };
}

/** 🏠️ The window as the watchers see it, with whatever observers a case hands in; calls reach the window as it is at that moment. */
export function hostWith(observers: Pick<SurveyHost, "ResizeObserver" | "MutationObserver"> = {}): SurveyHost {
  return {
    document,
    addEventListener: ((...args: Parameters<Window["addEventListener"]>) => window.addEventListener(...args)) as Window["addEventListener"],
    removeEventListener: ((...args: Parameters<Window["removeEventListener"]>) => window.removeEventListener(...args)) as Window["removeEventListener"],
    ...observers,
  };
}
