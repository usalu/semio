// #region Header
// framework/ui/elements/core/🚗️UiDriver/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region Adapters
import * as React from "react";
import { type StoragePort, createBrowserStoragePort, ephemeralBox } from "@semio-tech/framework-core";
import { reactHostPort } from "../🔌Ports/🟦️component.tsx";
// #endregion Adapters

// #region UiDriver
/** @emoji 🏷️ Inline caption policy: full icon+label chrome vs icons only. */
export type UiDriverLabels = "full" | "icons";
/** @emoji 🎚️ Which member of a `{normal, beginner}` label pair resolves. */
export type UiDriverLabelTier = "beginner" | "normal";
/** @emoji 🫳️ Drag affordance: dedicated grip handle vs whole-element draggable surface. */
export type UiDriverDrag = "handle" | "surface";
/** @emoji 🫥️ Chrome/gumball visibility: always painted vs revealed only while the cursor is in the region. */
export type UiDriverReveal = "always" | "hover";
/** @emoji 🎙️ Tooltip richness: full (label + manual/tutorial links + hotkey), minimal (label + hotkey), or none. */
export type UiDriverTooltips = "full" | "minimal" | "none";
/** @emoji ⌨️ Hotkey visibility: inline kbd badge on controls, tooltip-only, or hidden. */
export type UiDriverHotkeys = "inline" | "tooltip" | "none";

/** @emoji 🚗️ A named configuration bundle controlling how the UI presents itself. */
export interface UiDriver {
  readonly id: string;
  readonly label: string;
  readonly labels: UiDriverLabels;
  readonly labelTier: UiDriverLabelTier;
  readonly drag: UiDriverDrag;
  readonly chrome: UiDriverReveal;
  readonly gumball: UiDriverReveal;
  readonly tooltips: UiDriverTooltips;
  readonly hotkeys: UiDriverHotkeys;
}

/** @emoji 🚗️ Every affordance visible: drag handles, full labels, chrome and gumball always painted, rich tooltips. */
export const DEFAULT_UI_DRIVER: UiDriver = { id: "default", label: "Default", labels: "full", labelTier: "normal", drag: "handle", chrome: "always", gumball: "always", tooltips: "full", hotkeys: "inline" };
/** @emoji 🚗️ Assumes the user knows the UI: icon-only chrome that reveals on hover, whole-surface drag, no tooltips. */
export const COMPACT_UI_DRIVER: UiDriver = { id: "compact", label: "Compact", labels: "icons", labelTier: "normal", drag: "surface", chrome: "hover", gumball: "hover", tooltips: "none", hotkeys: "none" };

/** @emoji 🚗️ The built-in drivers shipped with the shell. */
export function builtinUiDrivers(): readonly UiDriver[] {
  return [DEFAULT_UI_DRIVER, COMPACT_UI_DRIVER];
}

function requireUiDriverAxis<T extends string>(value: unknown, path: string, allowed: readonly T[]): T {
  if (typeof value === "string" && (allowed as readonly string[]).includes(value)) return value as T;
  throw new Error(`driver.${path} must be one of ${allowed.join(", ")}`);
}

/** @emoji 🔎️ Strictly parses and validates a `UiDriver` (unknown axis values throw). */
export function parseUiDriver(json: unknown): UiDriver {
  if (typeof json !== "object" || json === null) throw new Error("driver must be an object");
  const obj = json as Record<string, unknown>;
  if (typeof obj.id !== "string") throw new Error("driver.id must be a string");
  if (typeof obj.label !== "string") throw new Error("driver.label must be a string");
  return {
    id: obj.id,
    label: obj.label,
    labels: requireUiDriverAxis(obj.labels, "labels", ["full", "icons"] as const),
    labelTier: requireUiDriverAxis(obj.labelTier, "labelTier", ["beginner", "normal"] as const),
    drag: requireUiDriverAxis(obj.drag, "drag", ["handle", "surface"] as const),
    chrome: requireUiDriverAxis(obj.chrome, "chrome", ["always", "hover"] as const),
    gumball: requireUiDriverAxis(obj.gumball, "gumball", ["always", "hover"] as const),
    tooltips: requireUiDriverAxis(obj.tooltips, "tooltips", ["full", "minimal", "none"] as const),
    hotkeys: requireUiDriverAxis(obj.hotkeys, "hotkeys", ["inline", "tooltip", "none"] as const),
  };
}

/** @emoji 💾️ Serializes a `UiDriver` to canonical JSON. */
export function serializeUiDriver(driver: UiDriver): string {
  return JSON.stringify(driver, null, 2);
}

/** @emoji 🚗️ Resolves a driver id against custom drivers, falling back to a builtin, then {@link DEFAULT_UI_DRIVER}. */
export function resolveUiDriver(id: string, custom: Record<string, UiDriver>): UiDriver {
  if (custom[id]) return custom[id];
  const builtin = builtinUiDrivers().find((driver) => driver.id === id);
  return builtin ?? DEFAULT_UI_DRIVER;
}

/** @emoji 🚗️ Storage key for the active driver id (builtin or `custom.<slug>`). */
export const UI_CHROME_DRIVER_STORAGE_KEY = "ui.chrome.driver";

/** @emoji 🚗️ Reads the persisted active driver id from the given shell's storage, defaulting to `"default"`. */
export function readStoredUiDriverId(storage: StoragePort): string {
  return storage.get(UI_CHROME_DRIVER_STORAGE_KEY) || DEFAULT_UI_DRIVER.id;
}

/** @emoji 🚗️ Persists the active driver id to the given shell's storage. */
export function writeStoredUiDriverId(storage: StoragePort, id: string): void {
  storage.set(UI_CHROME_DRIVER_STORAGE_KEY, id);
}

/** @emoji 🚗️ Storage key for the user's saved custom drivers, keyed by driver id. */
export const UI_CUSTOM_DRIVERS_STORAGE_KEY = "ui.drivers.custom";

/** @emoji 🚗️ Reads the user's saved custom drivers from the given shell's storage; discards any entry that fails to parse. */
export function readStoredUiCustomDrivers(storage: StoragePort): Record<string, UiDriver> {
  const raw = storage.get(UI_CUSTOM_DRIVERS_STORAGE_KEY);
  if (!raw) return {};
  try {
    const parsed = JSON.parse(raw) as Record<string, unknown>;
    const out: Record<string, UiDriver> = {};
    for (const [id, value] of Object.entries(parsed)) {
      try {
        out[id] = parseUiDriver(value);
      } catch {
        /* drop invalid saved driver */
      }
    }
    return out;
  } catch {
    return {};
  }
}

/** @emoji 🚗️ Persists the user's saved custom drivers to the given shell's storage. */
export function writeStoredUiCustomDrivers(storage: StoragePort, drivers: Record<string, UiDriver>): void {
  storage.set(UI_CUSTOM_DRIVERS_STORAGE_KEY, JSON.stringify(drivers));
}

/** @emoji 🚗️ Resolves the active driver from the given shell's storage (non-React fallback for non-hook consumers). */
export function readStoredUiDriver(storage: StoragePort): UiDriver {
  return resolveUiDriver(readStoredUiDriverId(storage), readStoredUiCustomDrivers(storage));
}

const UiDriverContext = reactHostPort.createContext<UiDriver | null>(null);

const _uiDriverProvider = ephemeralBox<(() => UiDriver) | null>("framework.modules.ui.elements.core.UiDriver.component.tsx._uiDriverProvider", null);

/** @emoji 🚗️ Registers the active driver resolver for non-React consumers (e.g. {@link resolveTranslationLabel}). */
export function setUiDriverProvider(fn: () => UiDriver): void {
  _uiDriverProvider.current = fn;
}

/** @emoji 🚗️ Resolves the active driver outside React render context. */
export function activeUiDriver(): UiDriver {
  return _uiDriverProvider.current ? _uiDriverProvider.current() : readStoredUiDriver(createBrowserStoragePort());
}

/** @emoji 🚗️ The active driver controlling labels, drag affordances, chrome/gumball reveal, and tooltips. */
export function useUiDriver(): UiDriver {
  const contextValue = reactHostPort.useContext(UiDriverContext);
  if (contextValue !== null) return contextValue;
  return activeUiDriver();
}

/** @emoji 🚗️ Supplies driver state to a subtree, overriding the ambient/stored driver. */
export function UiDriverProvider({ driver, children }: { readonly driver: UiDriver; readonly children: React.ReactNode }): React.ReactElement {
  reactHostPort.useEffect(() => {
    setUiDriverProvider(() => driver);
    return () => setUiDriverProvider(() => readStoredUiDriver(createBrowserStoragePort()));
  }, [driver]);
  return <UiDriverContext.Provider value={driver}>{children}</UiDriverContext.Provider>;
}

/** @emoji 🫳️ True when the driver wants whole-surface dragging (no dedicated grip handle). */
export function useUiDriverDragSurface(): boolean {
  return useUiDriver().drag === "surface";
}

/** @emoji 🤝 Arm native HTML5 `draggable` only while a drag handle is pressed. */
export function useNativeDragArm(): { readonly armed: boolean; readonly arm: () => void } {
  const [armed, setArmed] = reactHostPort.useState(false);
  const arm = reactHostPort.useCallback(() => {
    setArmed(true);
    window.addEventListener("pointerup", () => setArmed(false), { once: true });
  }, []);
  return { armed, arm };
}


/** @emoji 🎙️ The active driver's tooltip richness. */
export function useUiDriverTooltips(): UiDriverTooltips {
  return useUiDriver().tooltips;
}

const _controlLabelIdResolver = ephemeralBox<(id: string) => string>("framework.modules.ui.elements.core.UiDriver.component.tsx._controlLabelIdResolver", (id) => id);

/** @emoji 🏷️ Registers a product-specific mapper from shell control ids (`ui.*`) to i18n keys. */
export function setControlLabelIdResolver(resolver: (id: string) => string): void {
  _controlLabelIdResolver.current = typeof resolver === "function" ? resolver : (id) => id;
}

/** @emoji 🏷️ Maps shell control ids to i18n keys for inline labels (identity until a product resolver is set). */
export function resolveControlLabelId(id: string): string {
  if (typeof _controlLabelIdResolver.current !== "function") _controlLabelIdResolver.current = (value) => value;
  const resolve = _controlLabelIdResolver.current;
  if (id.startsWith("ui.nav.")) {
    const segment = id.slice("ui.nav.".length);
    if (segment === "back" || segment === "forward" || segment === "up") {
      return resolve(`ui.nav.${segment}`);
    }
  }
  if (id === "ui.search.toggle") {
    return resolve("ui.search.toggle");
  }
  if (id === "ui.find.toggle") {
    return resolve("ui.find.toggle");
  }
  if (id === "ui.fullscreen.toggle") {
    return resolve("ui.fullscreen.toggle");
  }
  if (id === "ui.mobilePanel.toggle") {
    return resolve("ui.mobilePanel.toggle");
  }
  if (id.startsWith("ui.panelToggle.")) {
    return resolve(`ui.panelToggle.${id.slice("ui.panelToggle.".length)}`);
  }
  if (id.startsWith("ui.ribbon.group.")) {
    return resolve(`ui.ribbon.parent.${id.slice("ui.ribbon.group.".length)}`);
  }
  if (id.startsWith("ui.ribbon.") && id.includes(".group.")) {
    return resolve(`ui.ribbon.parent.${id.slice(id.lastIndexOf(".group.") + ".group.".length)}`);
  }
  if (id === "ui.windowSearch.suggestions") {
    return resolve("ui.windowSearch.suggestions");
  }
  if (id === "ui.engagement.actions") {
    return resolve("ui.engagement.actions");
  }
  if (id === "search-input" || id === "ui.windowSearch.action") {
    return resolve("ui.windowSearch.action");
  }
  if (id.startsWith("playground.panel.")) {
    return resolve(`ui.panelToggle.${id.slice("playground.panel.".length)}`);
  }
  return resolve(id);
}

/** @emoji 🏷️ Panel kind slug from a panel-toggle control id (`ui.panelToggle.*`, `playground.panel.*`, sketchpad navbar keys). */
export function panelKindFromPanelToggleControlId(id: string): string | undefined {
  if (id.startsWith("ui.panelToggle.")) return id.slice("ui.panelToggle.".length);
  if (id.startsWith("playground.panel.")) return id.slice("playground.panel.".length);
  if (id.startsWith("demo.navbar.panelToggle.")) return id.slice("demo.navbar.panelToggle.".length);
  return undefined;
}

/** @emoji 🏷️ True for internal engagement/search chrome element ids that must not surface as humanized tooltips. */
export function isInternalChromeControlId(id: string | undefined | null): boolean {
  if (!id) return false;
  return id.startsWith("engagement-") || id.startsWith("engagement.") || id.startsWith("search-") || id.startsWith("search.");
}

/** @emoji 🔤️ Turns a control id segment into a short title (e.g. `panelToggle` → `Panel Toggle`). */
export function humanizeControlSegment(segment: string): string {
  const normalized = segment
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[._-]+/g, " ")
    .trim();
  if (!normalized) return segment;
  return normalized.replace(/\b\w/g, (char) => char.toUpperCase());
}

/** @emoji 🔤️ Human-readable caption from the last segment of a dotted control id. */
export function humanizeControlId(id: string): string {
  const segment = id.split(".").filter(Boolean).pop() ?? id;
  return humanizeControlSegment(segment);
}

/** @emoji 🏷️ Turns an internal step id (`first_corner`) into readable status text (`First Corner`). */
export function humanizeEngagementStepId(stepId: string): string {
  const trimmed = stepId.trim();
  if (!trimmed) return "";
  return trimmed.replace(/[._-]+/g, " ").replace(/\b\w/g, (character) => character.toUpperCase());
}
// #endregion UiDriver
