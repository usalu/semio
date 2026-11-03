/** 🎯️ Keyboard focus of the grips in the React shell: which grip of an open viewport has the keyboard, and how Enter, Tab, Shift+Tab, Ctrl/⌘+Arrow and Escape move it — the rules of the native shell (`semio_framework::walk_grips` and the plugin's `handle_key`, `🛂️manifest/🧱️app-surface/📱️shell-hooks/🎯️grip-walk`), answered from the same language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json`.
 *
 * 🧱️ Domain-neutral: a viewport states its grips as elements `[data-grip-id][data-grip-entity]` inside a `[data-grip-domain]` host. The focus is ephemeral and local to this user. The focused grip is marked `data-grip-focused="true"` (and `aria-current`), which the stylesheet (`PlatformPrefs` region) draws as the focus cross, heavier under more contrast. */

/** 🕹️ One grip the walk can reach: its handle id and the id of the entity that owns it. */
export type GripRefV1 = Readonly<{ id: string; entity: string }>;

/** 🚶️ How a key moves the grip focus. */
export type GripWalkV1 = "forward" | "backward" | "forwardInEntity" | "backwardInEntity";

/** 🚶️ The outcome of a walk: the grip that takes the focus, the end of the walk (Tab ran past either end) or nothing to walk. */
export type GripStepV1 = Readonly<{ to: string }> | "leave" | "nothing";

/** 🚶️ The next focus: Tab and Shift+Tab walk the whole selection in order and leave at either end; Ctrl/⌘+Arrow cycles only the grips of the focused grip's entity and wraps; without a (known) current grip they start at the first or last grip, or of the first entity. */
export function walkGripsV1(grips: readonly GripRefV1[], current: string | null, how: GripWalkV1): GripStepV1 {
  const first = grips[0];
  const last = grips[grips.length - 1];
  if (first === undefined || last === undefined) return "nothing";
  const forward = how === "forward" || how === "forwardInEntity";
  const at = current === null ? -1 : grips.findIndex((grip) => grip.id === current);
  if (how === "forward" || how === "backward") {
    if (at < 0) return { to: (forward ? first : last).id };
    const next = grips[forward ? at + 1 : at - 1];
    return next === undefined ? "leave" : { to: next.id };
  }
  const entity = at < 0 ? first.entity : grips[at]!.entity;
  const ring = grips.filter((grip) => grip.entity === entity);
  const place = at < 0 ? -1 : ring.findIndex((grip) => grip.id === grips[at]!.id);
  const index = place < 0 ? (forward ? 0 : ring.length - 1) : (place + (forward ? 1 : ring.length - 1)) % ring.length;
  return { to: ring[index]!.id };
}

/** 🗂️ The grips last stated and the focus, per interaction domain. A focus on a grip that is gone (the selection changed) is dropped when the grips are noted again. */
export function createGripFocusRegistryV1() {
  const grips = new Map<string, readonly GripRefV1[]>();
  const focus = new Map<string, string>();
  const registry = {
    note(domain: string, next: readonly GripRefV1[]): void {
      const handle = focus.get(domain);
      if (handle !== undefined && !next.some((grip) => grip.id === handle)) focus.delete(domain);
      grips.set(domain, next);
    },
    noted: (domain: string): readonly GripRefV1[] => grips.get(domain) ?? [],
    focused: (domain: string): string | null => focus.get(domain) ?? null,
    set(domain: string, handle: string | null): void {
      if (handle === null) focus.delete(domain);
      else focus.set(domain, handle);
    },
    walk(domain: string, how: GripWalkV1): GripStepV1 {
      const step = walkGripsV1(registry.noted(domain), registry.focused(domain), how);
      if (step === "leave") focus.delete(domain);
      else if (step !== "nothing") focus.set(domain, step.to);
      return step;
    },
  };
  return registry;
}

export type GripFocusRegistryV1 = ReturnType<typeof createGripFocusRegistryV1>;

/** 🧭️ The one registry of this page. */
export const gripFocusRegistryV1: GripFocusRegistryV1 = createGripFocusRegistryV1();

const HOST_V1 = "[data-grip-domain]";

/** 🔎️ The grips a viewport host states, in document (Tab) order. */
export function gripsOfHostV1(host: Element): readonly GripRefV1[] {
  return [...host.querySelectorAll<HTMLElement>("[data-grip-id][data-grip-entity]")].map((element) => ({ id: element.dataset.gripId!, entity: element.dataset.gripEntity! }));
}

/** 🎯️ Marks the focused grip of `host` (and only it). */
export function markFocusedGripV1(host: Element, handle: string | null): void {
  for (const element of host.querySelectorAll<HTMLElement>("[data-grip-id]")) {
    const focused = element.dataset.gripId === handle;
    if (focused) {
      element.setAttribute("data-grip-focused", "true");
      element.setAttribute("aria-current", "true");
    } else {
      element.removeAttribute("data-grip-focused");
      element.removeAttribute("aria-current");
    }
  }
}

type GripKeyV1 = Readonly<{ key: string; ctrlKey: boolean; metaKey: boolean; altKey: boolean; shiftKey: boolean }>;

/** ⌨️ What a key does to the grip walk of a viewport: Enter starts the walk at the first grip, Tab and Shift+Tab walk it, Ctrl/⌘+Right/Down and Left/Up cycle inside the entity, Escape ends it; `null` for any other key and for everything but Enter while no grip has the focus. */
export function gripWalkOfKeyV1(event: GripKeyV1, focused: boolean): GripWalkV1 | "start" | "end" | null {
  const primary = (event.ctrlKey || event.metaKey) && !event.altKey;
  if (focused && primary && !event.shiftKey) {
    if (event.key === "ArrowRight" || event.key === "ArrowDown") return "forwardInEntity";
    if (event.key === "ArrowLeft" || event.key === "ArrowUp") return "backwardInEntity";
  }
  if (event.ctrlKey || event.metaKey || event.altKey) return null;
  if (event.key === "Enter" && !focused && !event.shiftKey) return "start";
  if (!focused) return null;
  if (event.key === "Tab") return event.shiftKey ? "backward" : "forward";
  return event.key === "Escape" ? "end" : null;
}

/** ⌨️ Applies a key to the viewport host holding `target`: reads its grips, walks the focus and marks the focused grip. `true` when the key was the walk's (the caller prevents its default), `false` when it belongs to the shell — also when Tab ran past either end. */
export function handleGripFocusKeyV1(event: GripKeyV1 & { readonly target: EventTarget | null }, registry: GripFocusRegistryV1 = gripFocusRegistryV1): boolean {
  const host = event.target instanceof Element ? event.target.closest(HOST_V1) : null;
  const domain = host?.getAttribute("data-grip-domain");
  if (host === null || host === undefined || domain === null || domain === undefined) return false;
  registry.note(domain, gripsOfHostV1(host));
  const move = gripWalkOfKeyV1(event, registry.focused(domain) !== null);
  if (move === null) return false;
  if (move === "end") {
    registry.set(domain, null);
    markFocusedGripV1(host, null);
    return true;
  }
  const step = registry.walk(domain, move === "start" ? "forward" : move);
  markFocusedGripV1(host, registry.focused(domain));
  return step !== "leave" && step !== "nothing";
}
