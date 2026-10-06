import type { BrowserFrameTransport } from "../🚚️browser-frame-transport/🟦️.ts";

export const WGPU_ACCESSIBILITY_MIRROR_ID = "semio-wgpu-accessibility";
const ACCESSIBILITY_REFRESH_FLOOR_MS = 400;

export type AccessibilityProjectionNode = {
  readonly nodeId: number;
  readonly key: string;
  readonly role: string;
  readonly depth: number;
  readonly label?: string;
  readonly description?: string;
  readonly live: string;
  readonly shortcut?: string;
  readonly hidden?: boolean;
  readonly disabled?: boolean;
  readonly focusable?: boolean;
  readonly tabbable?: boolean;
  readonly actionable?: boolean;
  readonly focused?: boolean;
  readonly checked?: boolean;
  readonly pressed?: boolean;
  readonly selected?: boolean;
  readonly expanded?: boolean;
  readonly editable?: boolean;
  readonly multiline?: boolean;
  readonly controls?: string;
  readonly activeDescendant?: string;
  readonly level?: number;
  readonly valueMin?: number;
  readonly valueMax?: number;
  readonly valueNow?: number;
  readonly valueText?: string;
  readonly busy?: boolean;
  readonly valueStep?: number;
  readonly invalid?: boolean;
  readonly setSize?: number;
  readonly posInSet?: number;
  readonly tone?: string;
};

/** 🗝️ The attribute a mirrored control lists its engine-owned keys in, space separated. */
export const WGPU_ACCESSIBILITY_ENGINE_KEYS_ATTRIBUTE = "data-engine-keys";

/** 🎹️ The keys the renderer answers for one mirrored control, so the browser's own handling of them never runs: a slider's
 * and a stepper's number-law keys (design §18: arrows, page keys, Home/End toward a bound the control has) with Enter (commit)
 * and Escape (revert) on a typed field, and Enter/Escape alone on a slider's typed readout (`<key>::editor`), whose arrows stay
 * the native draft stepping. Only a control that publishes its step follows the law; every other control keeps its keys. */
export function accessibilityMirrorEngineKeys(node: Pick<AccessibilityProjectionNode, "role" | "key" | "disabled" | "valueMin" | "valueMax" | "valueStep">): readonly string[] {
  if (node.disabled === true || node.valueStep === undefined) return [];
  if (node.role === "slider") return ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "PageUp", "PageDown", "Home", "End"];
  if (node.role !== "spinbutton") return [];
  if (node.key.endsWith("::editor")) return ["Enter", "Escape"];
  return ["ArrowUp", "ArrowDown", "PageUp", "PageDown", ...(node.valueMin === undefined ? [] : ["Home"]), ...(node.valueMax === undefined ? [] : ["End"]), "Enter", "Escape"];
}

/** ⏎️ Whether Enter and Space on a focused mirrored control become its activation HERE: a native button, a text field and a
 * textarea answer their keys themselves, a tab and a radio own them in their own key handlers — every other actionable
 * element (a tree row, a pressable group) is a plain element the browser activates on no key at all, so the Actions rail was
 * pointer-only (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F18). */
export function accessibilityMirrorActivatesByKey(element: HTMLElement, role: string): boolean {
  return !(element instanceof HTMLButtonElement) && !(element instanceof HTMLInputElement) && !(element instanceof HTMLTextAreaElement) && role !== "tab" && role !== "radio";
}

/** 🚪️ Whether the mirrored control `target` hands `event`'s key to the renderer ({@link accessibilityMirrorEngineKeys}); a
 * chord with Ctrl, Meta or Alt is never a number-law key. */
export function accessibilityMirrorOwnsKey(target: HTMLElement, event: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey">): boolean {
  if (event.ctrlKey || event.metaKey || event.altKey) return false;
  return (target.getAttribute(WGPU_ACCESSIBILITY_ENGINE_KEYS_ATTRIBUTE) ?? "").split(" ").includes(event.key);
}

export type AccessibilityProjectionWindow = { readonly windowId: string; readonly windowGeneration: number; readonly nodes: readonly AccessibilityProjectionNode[] };
export type AccessibilityMirrorTransport = Pick<BrowserFrameTransport, "enqueueLossless" | "introspect">;

export function createAccessibilityMirror(root: HTMLElement, transport: AccessibilityMirrorTransport, tongue: "en" | "de", focusFallback?: HTMLElement, domOwns?: (surface: AccessibilityProjectionWindow, node: AccessibilityProjectionNode) => boolean, mirrorId = WGPU_ACCESSIBILITY_MIRROR_ID): { readonly refresh: () => void; readonly dispose: () => void } {
  const mirror = document.createElement("div");
  mirror.id = mirrorId;
  mirror.dataset.semioWgpuAccessibility = "";
  mirror.setAttribute("role", "region");
  mirror.setAttribute("aria-label", tongue === "de" ? "Semio Bedienelemente" : "Semio controls");
  mirror.style.cssText = "position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0 0 0 0);clip-path:inset(50%);white-space:nowrap;border:0;";
  root.appendChild(mirror);
  let published = "";
  let lastAt = 0;
  let pending = false;
  let disposed = false;
  let restoringFocus = false;

  const projectedElement = (surface: AccessibilityProjectionWindow, node: AccessibilityProjectionNode, ids: ReadonlyMap<string, string>): { readonly element: HTMLElement; readonly description?: HTMLSpanElement } => {
    const element: HTMLElement = (() => {
      if (node.role === "button" || node.role === "switch" || (node.role === "combobox" && node.editable !== true)) {
        const button = document.createElement("button");
        button.type = "button";
        if (node.role !== "button") button.setAttribute("role", node.role);
        return button;
      }
      if (node.role === "textbox" && node.multiline === true) return document.createElement("textarea");
      if (node.role === "textbox" || node.role === "slider" || node.role === "spinbutton" || (node.role === "combobox" && node.editable === true)) {
        const input = document.createElement("input");
        input.type = node.role === "slider" ? "range" : node.role === "spinbutton" ? "number" : "text";
        if (node.role === "combobox") input.setAttribute("role", "combobox");
        return input;
      }
      if (node.role === "form") return document.createElement("form");
      if (node.role === "paragraph") return document.createElement("p");
      if (node.role === "separator") return document.createElement("hr");
      const generic = document.createElement("div");
      generic.setAttribute("role", node.role);
      return generic;
    })();
    element.tabIndex = node.tabbable === true ? 0 : -1;
    element.id = ids.get(node.key) ?? "";
    element.dataset.window = surface.windowId;
    element.dataset.windowGeneration = String(surface.windowGeneration);
    element.dataset.nodeId = String(node.nodeId);
    element.dataset.nodeKey = node.key;
    if (node.role === "option") element.dataset.commandItemId = node.key;
    element.dataset.depth = String(node.depth);
    if (node.label !== undefined) element.setAttribute("aria-label", node.label);
    if (node.role === "paragraph" && node.label !== undefined) element.textContent = node.label;
    if (node.live !== "off") element.setAttribute("aria-live", node.live);
    if (node.shortcut !== undefined) element.setAttribute("aria-keyshortcuts", node.shortcut);
    if (node.hidden === true) element.setAttribute("aria-hidden", "true");
    if (node.disabled === true) element.setAttribute("aria-disabled", "true");
    if (node.checked !== undefined) element.setAttribute("aria-checked", String(node.checked));
    if (node.pressed !== undefined) element.setAttribute("aria-pressed", String(node.pressed));
    if (node.selected !== undefined) element.setAttribute("aria-selected", String(node.selected));
    if (node.expanded !== undefined) element.setAttribute("aria-expanded", String(node.expanded));
    if (node.role === "combobox" && node.editable === true) element.setAttribute("aria-autocomplete", "list");
    if (node.multiline === true) {
      element.setAttribute("aria-multiline", "true");
      element.setAttribute("aria-readonly", String(node.editable !== true));
    }
    if (node.controls !== undefined && ids.has(node.controls)) element.setAttribute("aria-controls", ids.get(node.controls)!);
    if (node.activeDescendant !== undefined && ids.has(node.activeDescendant)) element.setAttribute("aria-activedescendant", ids.get(node.activeDescendant)!);
    if (node.level !== undefined) element.setAttribute("aria-level", String(node.level));
    if (node.valueMin !== undefined) element.setAttribute("aria-valuemin", String(node.valueMin));
    if (node.valueMax !== undefined) element.setAttribute("aria-valuemax", String(node.valueMax));
    if (node.valueNow !== undefined) element.setAttribute("aria-valuenow", String(node.valueNow));
    if (node.valueText !== undefined) element.setAttribute("aria-valuetext", node.valueText);
    if (node.busy === true) element.setAttribute("aria-busy", "true");
    if (node.invalid === true) element.setAttribute("aria-invalid", "true");
    if (node.setSize !== undefined) element.setAttribute("aria-setsize", String(node.setSize));
    if (node.posInSet !== undefined) element.setAttribute("aria-posinset", String(node.posInSet));
    if (node.tone !== undefined) element.dataset.tone = node.tone;
    if (node.focused === true) element.dataset.focused = "true";
    if (node.focusable === true) element.dataset.focusable = "true";
    if (node.actionable === true) element.dataset.actionable = "true";
    if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
      if (node.valueText !== undefined) element.value = node.valueText;
      if (element instanceof HTMLInputElement && node.valueMin !== undefined) element.min = String(node.valueMin);
      if (element instanceof HTMLInputElement && node.valueMax !== undefined) element.max = String(node.valueMax);
      if (element instanceof HTMLInputElement && (node.role === "slider" || node.role === "spinbutton")) element.step = node.valueStep === undefined ? "any" : String(node.valueStep);
      if (element instanceof HTMLInputElement && node.valueNow !== undefined && node.role !== "textbox" && node.role !== "combobox") element.value = String(node.valueNow);
      element.disabled = node.disabled === true;
    }
    if (element instanceof HTMLButtonElement) element.disabled = node.disabled === true;
    if (element instanceof HTMLButtonElement && node.role === "combobox" && node.valueText !== undefined) element.textContent = node.valueText;
    const address = { windowId: surface.windowId, windowGeneration: surface.windowGeneration, nodeId: node.nodeId, nodeKey: node.key };
    if (node.focusable === true) element.addEventListener("focus", () => {
      if (!restoringFocus) transport.enqueueLossless({ kind: "accessibility-focus", ...address });
    });
    if (node.focusable === true) element.addEventListener("blur", () => {
      if (!restoringFocus) transport.enqueueLossless({ kind: "accessibility-blur", ...address });
    });
    if (node.actionable === true && !(element instanceof HTMLInputElement) && !(element instanceof HTMLTextAreaElement)) element.addEventListener("click", (event) => {
      event.stopPropagation();
      transport.enqueueLossless({ kind: "accessibility-activate", ...address });
    });
    if (node.actionable === true && node.focusable === true && accessibilityMirrorActivatesByKey(element, node.role)) element.addEventListener("keydown", (event) => {
      if (event.defaultPrevented || event.isComposing || event.ctrlKey || event.metaKey || event.altKey || (event.key !== "Enter" && event.key !== " ")) return;
      event.preventDefault();
      event.stopPropagation();
      transport.enqueueLossless({ kind: "accessibility-activate", ...address });
    });
    if (node.role === "tab" && node.focusable === true) element.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        event.stopPropagation();
        if (node.actionable === true) transport.enqueueLossless({ kind: "accessibility-activate", ...address });
        return;
      }
      if (event.key !== "ArrowLeft" && event.key !== "ArrowRight" && event.key !== "Home" && event.key !== "End") return;
      event.preventDefault();
      event.stopPropagation();
      const controls = element.getAttribute("aria-controls");
      const tabs = Array.from(element.parentElement?.querySelectorAll<HTMLElement>('[role="tab"]') ?? []).filter((tab) => tab.getAttribute("aria-controls") === controls);
      const index = tabs.indexOf(element);
      if (index < 0 || tabs.length === 0) return;
      const next = event.key === "Home"
        ? tabs[0]
        : event.key === "End"
          ? tabs[tabs.length - 1]
          : tabs[(index + (event.key === "ArrowRight" ? 1 : -1) + tabs.length) % tabs.length];
      next?.focus({ preventScroll: true });
    });
    if (node.role === "radio" && node.focusable === true) element.addEventListener("keydown", (event) => {
      const radios = Array.from(element.parentElement?.querySelectorAll<HTMLElement>(':scope > [role="radio"]') ?? []);
      const index = radios.indexOf(element);
      const step = event.key === "ArrowRight" || event.key === "ArrowDown" ? 1 : event.key === "ArrowLeft" || event.key === "ArrowUp" ? -1 : 0;
      const next = event.key === "Enter" || event.key === " " ? element : step === 0 || index < 0 ? undefined : radios[(index + step + radios.length) % radios.length];
      if (next === undefined) return;
      event.preventDefault();
      event.stopPropagation();
      next.focus({ preventScroll: true });
      next.click();
    });
    if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) element.addEventListener("input", (event) => {
      event.stopPropagation();
      transport.enqueueLossless({ kind: "accessibility-value", ...address, value: element.value });
    });
    const engineKeys = accessibilityMirrorEngineKeys(node);
    if (engineKeys.length > 0) {
      element.setAttribute(WGPU_ACCESSIBILITY_ENGINE_KEYS_ATTRIBUTE, engineKeys.join(" "));
      element.addEventListener("keydown", (event) => {
        if (event.defaultPrevented || event.isComposing || !accessibilityMirrorOwnsKey(element, event)) return;
        transport.enqueueLossless({ kind: "accessibility-focus", ...address });
      });
    }
    if (node.description === undefined) return { element };
    const description = document.createElement("span");
    description.id = `${WGPU_ACCESSIBILITY_MIRROR_ID}-${encodeURIComponent(surface.windowId)}-${surface.windowGeneration}-${node.nodeId}-desc`;
    description.dataset.descriptionFor = node.key;
    description.textContent = node.description;
    element.setAttribute("aria-describedby", description.id);
    return { element, description };
  };

  const paint = (surfaces: readonly AccessibilityProjectionWindow[]): void => {
    const active = document.activeElement instanceof HTMLElement && document.activeElement.dataset.window !== undefined
      ? `${document.activeElement.dataset.window}\u0000${document.activeElement.dataset.windowGeneration}\u0000${document.activeElement.dataset.nodeId}\u0000${document.activeElement.dataset.nodeKey}`
      : undefined;
    const roots: HTMLElement[] = [];
    let count = 0;
    let rendererFocus: string | undefined;
    for (const surface of surfaces) {
      const group = document.createElement("div");
      group.setAttribute("role", "group");
      group.dataset.window = surface.windowId;
      group.dataset.windowGeneration = String(surface.windowGeneration);
      const stack: HTMLElement[] = [group];
      const surfaceId = encodeURIComponent(surface.windowId);
      const ids = new Map(surface.nodes.map((node) => [node.key, `${mirrorId}-${surfaceId}-${surface.windowGeneration}-${node.nodeId}`]));
      for (const node of surface.nodes) {
        if (domOwns?.(surface, node)) continue;
        while (stack.length > node.depth + 1) stack.pop();
        const projected = projectedElement(surface, node, ids);
        const parent = stack[node.depth] ?? group;
        parent.appendChild(projected.element);
        if (projected.description !== undefined) parent.appendChild(projected.description);
        stack[node.depth + 1] = projected.element;
        stack.length = node.depth + 2;
        if (node.focused === true) rendererFocus = `${surface.windowId}\u0000${surface.windowGeneration}\u0000${node.nodeId}\u0000${node.key}`;
        count++;
      }
      roots.push(group);
    }
    restoringFocus = true;
    mirror.replaceChildren(...roots);
    mirror.dataset.nodeCount = String(count);
    mirror.dataset.windows = surfaces.map((surface) => surface.windowId).join(" ");
    const focus = rendererFocus ?? active;
    if (focus === undefined) {
      restoringFocus = false;
      return;
    }
    const [windowId, windowGeneration, nodeId, nodeKey] = focus.split("\u0000");
    const next = Array.from(mirror.querySelectorAll<HTMLElement>("[data-node-id]")).find((candidate) => candidate.dataset.window === windowId && candidate.dataset.windowGeneration === windowGeneration && candidate.dataset.nodeId === nodeId && candidate.dataset.nodeKey === nodeKey);
    if (next === undefined) {
      focusFallback?.focus({ preventScroll: true });
      restoringFocus = false;
      return;
    }
    next.focus({ preventScroll: true });
    restoringFocus = false;
  };

  const pull = async (): Promise<void> => {
    lastAt = performance.now();
    const json = await transport.introspect("accessibility");
    if (disposed || json === null) return;
    const ownership = Array.from(root.querySelectorAll<HTMLElement>("[data-media-slot]")).map((host) => `${host.dataset.mediaWindow}:${host.dataset.uiNodeId}:${host.dataset.uiNodeKey}:${host.dataset.mediaSlot}`).join("\u0000");
    const signature = `${json}\u0000${ownership}`;
    if (signature === published) return;
    let dump: { readonly windows?: readonly AccessibilityProjectionWindow[] };
    try {
      dump = JSON.parse(json) as { readonly windows?: readonly AccessibilityProjectionWindow[] };
    } catch {
      return;
    }
    paint(dump.windows ?? []);
    published = signature;
  };

  /** 🧹️ Drops every mirrored node a DOM host took over (`domOwns`) — one walk of the mirror. */
  const retireOwned = (): void => {
    if (!domOwns) return;
    for (const candidate of mirror.querySelectorAll<HTMLElement>("[data-node-id]")) {
      const surface = { windowId: candidate.dataset.window!, windowGeneration: Number(candidate.dataset.windowGeneration), nodes: [] };
      const node = { nodeId: Number(candidate.dataset.nodeId), key: candidate.dataset.nodeKey!, role: candidate.getAttribute("role") ?? "region", depth: 0, live: "off" };
      if (domOwns(surface, node)) { candidate.remove(); published = ""; }
    }
  };

  /** 🔁️ Asks for a fresh projection. The frame worker posts one `frame` message per frame STEP — measured at about
   * 12 000 a second on a live board — and the host calls this for every one of them, so a call costs nothing while a
   * pull is already owed: the ownership walk ({@link retireOwned}) runs at once on the first call of a burst and again
   * when the owed pull starts, never per call. It used to run on EVERY call, ahead of the throttle: half of the main
   * thread at 115 mirrored nodes, more with each History row, until the page's task queue no longer drained and a
   * plain DOM read waited 27 s (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F22). */
  const refresh = (): void => {
    if (disposed || pending) return;
    retireOwned();
    pending = true;
    window.setTimeout(() => {
      pending = false;
      if (disposed) return;
      retireOwned();
      void pull();
    }, Math.max(0, ACCESSIBILITY_REFRESH_FLOOR_MS - (performance.now() - lastAt)));
  };

  return { refresh, dispose: () => { disposed = true; mirror.remove(); } };
}
