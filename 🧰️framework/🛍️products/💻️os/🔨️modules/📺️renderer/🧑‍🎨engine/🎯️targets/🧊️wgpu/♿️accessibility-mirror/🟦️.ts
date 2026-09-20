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
  readonly actionable?: boolean;
  readonly focused?: boolean;
  readonly checked?: boolean;
  readonly selected?: boolean;
  readonly expanded?: boolean;
  readonly editable?: boolean;
  readonly controls?: string;
  readonly activeDescendant?: string;
  readonly level?: number;
  readonly valueMin?: number;
  readonly valueMax?: number;
  readonly valueNow?: number;
  readonly valueText?: string;
  readonly busy?: boolean;
};

export type AccessibilityProjectionWindow = { readonly windowId: string; readonly windowGeneration: number; readonly nodes: readonly AccessibilityProjectionNode[] };
export type AccessibilityMirrorTransport = Pick<BrowserFrameTransport, "enqueueLossless" | "introspect">;

export function createAccessibilityMirror(root: HTMLElement, transport: AccessibilityMirrorTransport, tongue: "en" | "de"): { readonly refresh: () => void; readonly dispose: () => void } {
  const mirror = document.createElement("div");
  mirror.id = WGPU_ACCESSIBILITY_MIRROR_ID;
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
    const element = (() => {
      if (node.role === "button" || node.role === "switch" || (node.role === "combobox" && node.editable !== true)) {
        const button = document.createElement("button");
        button.type = "button";
        if (node.role !== "button") button.setAttribute("role", node.role);
        return button;
      }
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
    element.tabIndex = node.focusable === true ? 0 : -1;
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
    if (node.selected !== undefined) element.setAttribute("aria-selected", String(node.selected));
    if (node.expanded !== undefined) element.setAttribute("aria-expanded", String(node.expanded));
    if (node.editable === true) element.setAttribute("aria-autocomplete", "list");
    if (node.controls !== undefined && ids.has(node.controls)) element.setAttribute("aria-controls", ids.get(node.controls)!);
    if (node.activeDescendant !== undefined && ids.has(node.activeDescendant)) element.setAttribute("aria-activedescendant", ids.get(node.activeDescendant)!);
    if (node.level !== undefined) element.setAttribute("aria-level", String(node.level));
    if (node.valueMin !== undefined) element.setAttribute("aria-valuemin", String(node.valueMin));
    if (node.valueMax !== undefined) element.setAttribute("aria-valuemax", String(node.valueMax));
    if (node.valueNow !== undefined) element.setAttribute("aria-valuenow", String(node.valueNow));
    if (node.valueText !== undefined) element.setAttribute("aria-valuetext", node.valueText);
    if (node.busy === true) element.setAttribute("aria-busy", "true");
    if (node.focused === true) element.dataset.focused = "true";
    if (node.focusable === true) element.dataset.focusable = "true";
    if (node.actionable === true) element.dataset.actionable = "true";
    if (element instanceof HTMLInputElement) {
      if (node.valueText !== undefined) element.value = node.valueText;
      if (node.valueMin !== undefined) element.min = String(node.valueMin);
      if (node.valueMax !== undefined) element.max = String(node.valueMax);
      if (node.valueNow !== undefined && node.role !== "textbox" && node.role !== "combobox") element.value = String(node.valueNow);
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
    if (node.actionable === true && !(element instanceof HTMLInputElement)) element.addEventListener("click", (event) => {
      event.stopPropagation();
      transport.enqueueLossless({ kind: "accessibility-activate", ...address });
    });
    if (element instanceof HTMLInputElement) element.addEventListener("input", (event) => {
      event.stopPropagation();
      transport.enqueueLossless({ kind: "accessibility-value", ...address, value: element.value });
    });
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
      const ids = new Map(surface.nodes.map((node) => [node.key, `${WGPU_ACCESSIBILITY_MIRROR_ID}-${surfaceId}-${surface.windowGeneration}-${node.nodeId}`]));
      for (const node of surface.nodes) {
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
      restoringFocus = false;
      return;
    }
    next.focus({ preventScroll: true });
    restoringFocus = false;
  };

  const pull = async (): Promise<void> => {
    lastAt = performance.now();
    const json = await transport.introspect("accessibility");
    if (disposed || json === null || json === published) return;
    let dump: { readonly windows?: readonly AccessibilityProjectionWindow[] };
    try {
      dump = JSON.parse(json) as { readonly windows?: readonly AccessibilityProjectionWindow[] };
    } catch {
      return;
    }
    paint(dump.windows ?? []);
    published = json;
  };

  const refresh = (): void => {
    if (disposed || pending) return;
    pending = true;
    window.setTimeout(() => {
      pending = false;
      if (!disposed) void pull();
    }, Math.max(0, ACCESSIBILITY_REFRESH_FLOOR_MS - (performance.now() - lastAt)));
  };

  return { refresh, dispose: () => { disposed = true; mirror.remove(); } };
}
