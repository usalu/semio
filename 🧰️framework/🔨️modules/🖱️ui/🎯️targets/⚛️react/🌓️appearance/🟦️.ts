/** 🌓️ The document-level surface chrome of every semio page on its own: appearance (system, light, dark) as `.dark`
 * and base colors, the device as `data-ui-device` and `.touch`, the driver's DOM axes with the hover-reveal controller,
 * the persisted appearance and layout preferences, `useMediaQuery`, and the app-shell suppression of browser defaults —
 * importable without the rest of the React target (`@semio-tech/ui-react/chrome`); the barrel re-exports it unchanged.
 *
 * @see ../../../📱️device/🟦️.ts — the device vocabulary and breakpoints
 * @see ../../../🧱️elements/🚗️UiDriver/🟦️.tsx — the driver axes
 * @see ../🟦️.tsx — the barrel that re-exports this module
 */

import { createBrowserStoragePort, ephemeralBox, ephemeralMap, type StoragePort } from "@semio-tech/framework";
import { clearStylingAppearanceRoot, setStylingAppearanceRoot, stylingAppearanceRootElement, subscribeStylingAppearanceRoot } from "@semio-tech/ui-styling";
import { availableViewportHeightPx, type ElementsSurfaceDevice } from "../../../📱️device/🟦️.ts";
import { reactHostPort } from "../../../🧱️elements/🔌️Ports/🟦️.tsx";
import { readStoredUiDriver, setUiDriverProvider, type UiDriver, type UiDriverReveal } from "../../../🧱️elements/🚗️UiDriver/🟦️.tsx";
import { createDOMEventBinding } from "../../../🔨️modules/👂️dom-event-binding/🟦️.ts";
import { applyUiFormControlBrowserDefaults } from "../../../🔨️modules/📝️form-control-presentation/🟦️.ts";

// #region 🌈️SurfaceChrome
/** @emoji 🌈️ Document-level UI chrome shared by Elements shells: appearance (system/light/dark), device (desktop/tablet/mobile), and driver — mirrors sketchpad `Appearance` / `Device` behavior on `documentElement`. */
export type ElementsSurfaceAppearance = "system" | "light" | "dark";

/** @emoji 🚫️ Whether a page keeps the browser's own context menu and form-control assistance (`native`: documents such as
 * a quiz site) or suppresses them for app-shell chrome (`suppressed`, the default). */
export type ElementsSurfaceBrowserDefaults = "suppressed" | "native";

export interface ElementsSurfaceChromeInput {
  appearance: ElementsSurfaceAppearance;
  device: ElementsSurfaceDevice;
  driver: UiDriver;
  browserDefaults?: ElementsSurfaceBrowserDefaults;
}

/** @emoji 🐚️ Resolves an explicit surface-chrome root (a shell's own root — e.g. its `ShellScope.rootRef`)
 * or falls back to `document.documentElement` for the page-owning case; every entry point below takes
 * this same optional-root shape so a single-shell page's existing call sites (which pass none) keep
 * their exact current behavior unchanged. `undefined` in a non-browser environment (SSR/vitest without
 * a document) rather than throwing. */
export function resolveElementsSurfaceChromeRoot(root?: HTMLElement): HTMLElement | undefined {
  return root ?? (typeof document !== "undefined" ? document.documentElement : undefined);
}

/** @emoji 🐚️ Paints a surface-chrome root's own background/foreground/color-scheme — every root, not
 * just `documentElement`, so an embedded shell's own `.semio-scope` div is visually correct even before
 * any descendant renders. When the root IS `documentElement` (the page-owning case), also mirrors onto
 * `document.body` exactly as before this was made root-scoped — unchanged behavior for that case. */
function applyElementsSurfaceChromeBaseColors(root: HTMLElement, scheme: "light" | "dark"): void {
  root.style.backgroundColor = "var(--base)";
  root.style.color = "var(--foreground)";
  root.style.colorScheme = scheme;
  if (typeof document !== "undefined" && root === document.documentElement && document.body) {
    document.body.style.backgroundColor = "var(--base)";
    document.body.style.color = "var(--foreground)";
    document.body.style.colorScheme = scheme;
  }
}

function clearElementsSurfaceChromeBaseColors(root: HTMLElement): void {
  root.style.backgroundColor = "";
  root.style.color = "";
  root.style.colorScheme = "";
  if (typeof document !== "undefined" && root === document.documentElement && document.body) {
    document.body.style.backgroundColor = "";
    document.body.style.color = "";
    document.body.style.colorScheme = "";
  }
}

/** @emoji 🌓️ Resolves whether {@link ElementsSurfaceAppearance} is dark for the current system preference. */
export function resolveElementsSurfaceChromeDark(appearance: ElementsSurfaceAppearance): boolean {
  if (appearance === "dark") return true;
  if (appearance === "light") return false;
  if (typeof window === "undefined" || typeof window.matchMedia !== "function") return false;
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

/** @emoji 🌓️ True when a surface-chrome root (`document.documentElement` by default) currently carries
 * the dark surface chrome class. */
export function isElementsSurfaceChromeDarkApplied(rootOverride?: HTMLElement): boolean {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  return root?.classList.contains("dark") ?? false;
}

type ElementsSurfaceChromeLease = { readonly id: number; readonly input: ElementsSurfaceChromeInput };

const elementsSurfaceChromeLeaseSeq = ephemeralBox("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeLeaseSeq", 0);
/** @emoji 🐚️ One independent lease stack per surface-chrome root — was a single page-global stack, which
 * meant a second mounted shell's appearance/driver/device lease silently won (last-wins) over the
 * first's for the WHOLE page instead of just its own subtree. */
const elementsSurfaceChromeLeasesByRoot = ephemeralMap<HTMLElement, ElementsSurfaceChromeLease[]>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeLeasesByRoot");
const elementsSurfaceChromeDeferredClearFrames = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeDeferredClearFrames");
const elementsSurfaceChromeDomBindings = ephemeralBox<ReturnType<typeof createDOMEventBinding> | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeDomBindings", null);
const elementsSurfaceChromeSystemListenersInstalled = ephemeralBox("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeSystemListenersInstalled", false);
const elementsSurfaceBrowserDefaultBindings = ephemeralBox<ReturnType<typeof createDOMEventBinding> | null>("framework.modules.ui.targets.react.appearance.elementsSurfaceBrowserDefaultBindings", null);

function activeElementsSurfaceChromeInput(root: HTMLElement): ElementsSurfaceChromeInput | undefined {
  const leases = elementsSurfaceChromeLeasesByRoot.get(root);
  return leases?.[leases.length - 1]?.input;
}

function syncElementsSurfaceChromeProviders(input: ElementsSurfaceChromeInput | undefined): void {
  if (input) {
    setUiDriverProvider(() => input.driver);
    return;
  }
  setUiDriverProvider(() => readStoredUiDriver(createBrowserStoragePort()));
}

function applyElementsSurfaceChromeDriverDom(root: HTMLElement, driver: UiDriver): void {
  root.dataset.uiDriver = driver.id;
  root.dataset.uiLabels = driver.labels;
  root.dataset.uiDrag = driver.drag;
  root.dataset.uiChromeReveal = driver.chrome;
  root.dataset.uiGumballReveal = driver.gumball;
  root.dataset.uiTooltips = driver.tooltips;
  syncUiChromeRevealController(root, driver.chrome);
}

function clearElementsSurfaceChromeDriverDom(root: HTMLElement): void {
  delete root.dataset.uiDriver;
  delete root.dataset.uiLabels;
  delete root.dataset.uiDrag;
  delete root.dataset.uiChromeReveal;
  delete root.dataset.uiGumballReveal;
  delete root.dataset.uiTooltips;
  teardownUiChromeRevealController(root);
}

// #region 🫥️ChromeReveal
/** @emoji 🫥️ Extra radius (px) around a reveal region's own rect that still counts as "inside" — makes the invisible-until-hovered bar reachable. */
const CHROME_REVEAL_ACTIVATION_BAND_PX = 24;
/** @emoji 🫥️ Screen-edge band (px) that reveals a region anchored to that edge (navbar top, footer bottom), even before the cursor reaches the region's own rect. */
const CHROME_REVEAL_EDGE_BAND_PX = 8;

/** @emoji 🐚️ One independent reveal controller per surface-chrome root — was a single page-global
 * controller, which meant hovering ANY mounted shell revealed hover-reveal chrome for EVERY shell that
 * had opted into it (and a pointer-move over shell B's DOM would drive shell A's reveal state). */
const chromeRevealBindingsByRoot = ephemeralMap<HTMLElement, ReturnType<typeof createDOMEventBinding>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealBindingsByRoot");
const chromeRevealFrameByRoot = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealFrameByRoot");
const chromeRevealLastPointByRoot = ephemeralMap<HTMLElement, { x: number; y: number }>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealLastPointByRoot");

function chromeRevealStackAncestor(region: HTMLElement): HTMLElement | null {
  return region.closest<HTMLElement>('[data-slot="window-chrome-stack"], [data-slot="mode-dock-stack"]');
}

function chromeRevealRegionRevealed(region: HTMLElement, x: number, y: number): boolean {
  const rect = region.getBoundingClientRect();
  if (x >= rect.left - CHROME_REVEAL_ACTIVATION_BAND_PX && x <= rect.right + CHROME_REVEAL_ACTIVATION_BAND_PX && y >= rect.top - CHROME_REVEAL_ACTIVATION_BAND_PX && y <= rect.bottom + CHROME_REVEAL_ACTIVATION_BAND_PX) {
    return true;
  }
  const regionName = region.dataset.uiRevealRegion;
  if (regionName === "navbar" && y <= CHROME_REVEAL_EDGE_BAND_PX) return true;
  if (regionName === "footer" && typeof window !== "undefined" && y >= availableViewportHeightPx({ innerHeight: window.innerHeight, visualHeight: window.visualViewport?.height }) - CHROME_REVEAL_EDGE_BAND_PX) return true;
  if (regionName === "window-cap") {
    const stack = chromeRevealStackAncestor(region);
    if (stack) {
      const stackRect = stack.getBoundingClientRect();
      if (x >= stackRect.left && x <= stackRect.right && y >= stackRect.top && y <= stackRect.top + CHROME_REVEAL_EDGE_BAND_PX) return true;
    }
  }
  return false;
}

export function applyChromeRevealAtPoint(root: HTMLElement, x: number, y: number): void {
  root.querySelectorAll<HTMLElement>("[data-ui-reveal-region]").forEach((region) => {
    if (chromeRevealRegionRevealed(region, x, y)) region.dataset.uiRevealed = "true";
    else delete region.dataset.uiRevealed;
  });
}

function scheduleChromeRevealUpdate(root: HTMLElement): void {
  if (chromeRevealFrameByRoot.has(root) || typeof requestAnimationFrame === "undefined") return;
  const frame = requestAnimationFrame(() => {
    chromeRevealFrameByRoot.delete(root);
    const point = chromeRevealLastPointByRoot.get(root);
    if (point) applyChromeRevealAtPoint(root, point.x, point.y);
  });
  chromeRevealFrameByRoot.set(root, frame);
}

function ensureUiChromeRevealController(root: HTMLElement): void {
  if (chromeRevealBindingsByRoot.has(root) || typeof window === "undefined") return;
  const bindings = createDOMEventBinding();
  bindings.listen(window, "pointermove", (event: PointerEvent) => {
    // 🐚️ `pointermove` only ever bubbles to `window` (never scoped to a subtree), so this root only
    // reacts to points actually over its own DOM — otherwise hovering shell B would reveal shell A's chrome.
    if (!(event.target instanceof globalThis.Node) || !root.contains(event.target)) return;
    chromeRevealLastPointByRoot.set(root, { x: event.clientX, y: event.clientY });
    scheduleChromeRevealUpdate(root);
  });
  bindings.listen(root, "focusin", (event: FocusEvent) => {
    const region = (event.target as HTMLElement | null)?.closest<HTMLElement>("[data-ui-reveal-region]");
    if (region) region.dataset.uiRevealed = "true";
  });
  bindings.listen(root, "focusout", (event: FocusEvent) => {
    const region = (event.target as HTMLElement | null)?.closest<HTMLElement>("[data-ui-reveal-region]");
    const related = event.relatedTarget as globalThis.Node | null;
    if (region && (!related || !region.contains(related))) delete region.dataset.uiRevealed;
  });
  chromeRevealBindingsByRoot.set(root, bindings);
}

function teardownUiChromeRevealController(root: HTMLElement): void {
  chromeRevealBindingsByRoot.get(root)?.dispose();
  chromeRevealBindingsByRoot.delete(root);
  const frame = chromeRevealFrameByRoot.get(root);
  if (frame !== undefined && typeof cancelAnimationFrame !== "undefined") cancelAnimationFrame(frame);
  chromeRevealFrameByRoot.delete(root);
  chromeRevealLastPointByRoot.delete(root);
  root.querySelectorAll<HTMLElement>("[data-ui-reveal-region][data-ui-revealed]").forEach((region) => delete region.dataset.uiRevealed);
}

/** @emoji 🫥️ Ensures the pointer/focus reveal tracker is installed iff the driver wants hover-reveal chrome; called whenever driver DOM attrs are (re)applied. */
function syncUiChromeRevealController(root: HTMLElement, chrome: UiDriverReveal): void {
  if (chrome === "hover") ensureUiChromeRevealController(root);
  else teardownUiChromeRevealController(root);
}
// #endregion 🫥️ChromeReveal

function clearElementsSurfaceChromeDom(root: HTMLElement): void {
  clearStylingAppearanceRoot(root);
  root.classList.remove("dark");
  root.classList.remove("touch");
  delete root.dataset.uiDevice;
  clearElementsSurfaceChromeDriverDom(root);
  delete root.dataset.uiAppearance;
  clearElementsSurfaceChromeBaseColors(root);
}

function applyElementsSurfaceChromeAppearanceDom(root: HTMLElement, appearance: ElementsSurfaceAppearance): void {
  const dark = resolveElementsSurfaceChromeDark(appearance);
  root.classList.toggle("dark", dark);
  root.dataset.uiAppearance = dark ? "dark" : "light";
  applyElementsSurfaceChromeBaseColors(root, dark ? "dark" : "light");
  setStylingAppearanceRoot(root);
}

/** @emoji 🌓️ Applies `.dark`/`color-scheme` to a root (`document.documentElement` by default) before
 * React/CSS load (play/static entries); does not register a surface-chrome lease. */
export function bootstrapElementsSurfaceChromeDocument(appearance: ElementsSurfaceAppearance = "system", rootOverride?: HTMLElement): void {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  if (!root) return;
  cancelElementsSurfaceChromeDeferredClear(root);
  applyElementsSurfaceChromeAppearanceDom(root, appearance);
}

function cancelElementsSurfaceChromeDeferredClear(root: HTMLElement): void {
  const frame = elementsSurfaceChromeDeferredClearFrames.get(root);
  if (frame === undefined || typeof cancelAnimationFrame === "undefined") {
    elementsSurfaceChromeDeferredClearFrames.delete(root);
    return;
  }
  cancelAnimationFrame(frame);
  elementsSurfaceChromeDeferredClearFrames.delete(root);
}

function scheduleElementsSurfaceChromeDeferredClear(root: HTMLElement): void {
  if (typeof requestAnimationFrame === "undefined") {
    if (!elementsSurfaceChromeLeasesByRoot.has(root)) clearElementsSurfaceChromeDom(root);
    return;
  }
  cancelElementsSurfaceChromeDeferredClear(root);
  const frame = requestAnimationFrame(() => {
    elementsSurfaceChromeDeferredClearFrames.delete(root);
    if (!elementsSurfaceChromeLeasesByRoot.has(root)) clearElementsSurfaceChromeDom(root);
  });
  elementsSurfaceChromeDeferredClearFrames.set(root, frame);
}

function applyElementsSurfaceChromeDom(root: HTMLElement, input: ElementsSurfaceChromeInput): void {
  cancelElementsSurfaceChromeDeferredClear(root);
  applyElementsSurfaceChromeAppearanceDom(root, input.appearance);
  root.dataset.uiDevice = input.device;
  root.classList.toggle("touch", input.device !== "desktop");
  applyElementsSurfaceChromeDriverDom(root, input.driver);
}

function syncElementsSurfaceChromeDomFromLeaseStack(root: HTMLElement): void {
  const input = activeElementsSurfaceChromeInput(root);
  if (!input) {
    clearElementsSurfaceChromeDom(root);
    return;
  }
  applyElementsSurfaceChromeDom(root, input);
}

/** @emoji 🐚️ One shared `matchMedia` listener re-applies EVERY root with an active `appearance: "system"`
 * lease when the OS preference flips — the media query itself is genuinely page-global (there is only
 * one system preference), but each root's lease stack (and therefore whether it even has a "system"
 * lease) stays independent. */
function ensureElementsSurfaceChromeSystemListeners(): void {
  if (elementsSurfaceChromeSystemListenersInstalled.current || typeof window === "undefined" || typeof document === "undefined") {
    return;
  }
  elementsSurfaceChromeSystemListenersInstalled.current = true;
  const bindings = createDOMEventBinding();
  elementsSurfaceChromeDomBindings.current = bindings;
  if (typeof window.matchMedia !== "function") return;
  const mq = window.matchMedia("(prefers-color-scheme: dark)");
  const onSystemAppearanceChange = (): void => {
    for (const [root, leases] of elementsSurfaceChromeLeasesByRoot) {
      const input = leases[leases.length - 1]?.input;
      if (!input || input.appearance !== "system") continue;
      applyElementsSurfaceChromeDom(root, input);
    }
  };
  bindings.listen(mq, "change", onSystemAppearanceChange);
}

/** @emoji 🚫️ Installs {@link installElementsSurfaceBrowserDefaultSuppression} once per page, the first time a lease
 * asks for app-shell browser defaults — a page whose every lease is `native` never loses its context menu. */
function ensureElementsSurfaceBrowserDefaultSuppression(): void {
  if (elementsSurfaceBrowserDefaultBindings.current !== null || typeof document === "undefined") return;
  const bindings = createDOMEventBinding();
  installElementsSurfaceBrowserDefaultSuppression(bindings);
  elementsSurfaceBrowserDefaultBindings.current = bindings;
}

/**
 * @emoji 🌈️ Imperative surface chrome controller for class-based shells; returns a cleanup that reverts
 * DOM state, browser default input, and the active driver. `rootOverride` scopes this lease to one
 * shell's own root (e.g. its `ShellScope.rootRef`) — omitted, it falls back to `document.documentElement`
 * (the single-shell-per-page case, unchanged from before this was made root-scoped).
 */
export function applyElementsSurfaceChrome(input: ElementsSurfaceChromeInput, rootOverride?: HTMLElement): () => void {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  if (!root) return () => {};
  const lease: ElementsSurfaceChromeLease = { id: ++elementsSurfaceChromeLeaseSeq.current, input };
  const leases = elementsSurfaceChromeLeasesByRoot.get(root) ?? [];
  leases.push(lease);
  elementsSurfaceChromeLeasesByRoot.set(root, leases);
  ensureElementsSurfaceChromeSystemListeners();
  if (input.browserDefaults !== "native") ensureElementsSurfaceBrowserDefaultSuppression();
  syncElementsSurfaceChromeProviders(input);
  syncElementsSurfaceChromeDomFromLeaseStack(root);
  return () => {
    const current = elementsSurfaceChromeLeasesByRoot.get(root);
    const index = current?.findIndex((entry) => entry.id === lease.id) ?? -1;
    if (current && index >= 0) current.splice(index, 1);
    if (!current || current.length === 0) {
      elementsSurfaceChromeLeasesByRoot.delete(root);
      syncElementsSurfaceChromeProviders(undefined);
      scheduleElementsSurfaceChromeDeferredClear(root);
      return;
    }
    syncElementsSurfaceChromeProviders(activeElementsSurfaceChromeInput(root));
    syncElementsSurfaceChromeDomFromLeaseStack(root);
  };
}

/**
 * @emoji 🌓️ Syncs a surface-chrome root (`dark`, `touch`, `data-ui-device`, `data-ui-driver` + axis
 * attrs), base colors, and {@link setUiDriverProvider}; returns `mobile` for {@link AppProps.mobile}.
 * `root` scopes this to one shell — omitted, targets `document.documentElement` as before. Callers reading
 * this from a ref (e.g. `ShellScope.rootRef.current`) must re-render once that ref attaches (`FrameworkOsShell`
 * bumps state in its callback ref for exactly this) — a ref OBJECT in this hook's own deps would never
 * re-trigger the effect once populated, since the object's identity never changes.
 */
export function useElementsSurfaceChrome({ appearance, device, driver, browserDefaults }: ElementsSurfaceChromeInput, root?: HTMLElement): { mobile: boolean } {
  reactHostPort.useLayoutEffect(() => applyElementsSurfaceChrome({ appearance, device, driver, browserDefaults }, root), [appearance, device, driver, browserDefaults, root]);

  return { mobile: device === "mobile" };
}

/**
 * @emoji 🌓️ Observes a surface-chrome root's appearance attributes and runs `sync` on mount and whenever
 * they change. Holds `sync` in a ref so callers can pass an inline arrow without retriggering the effect
 * every render (React 19: unstable `sync` identity → effect → `paintOverlays`/`setState` → re-render →
 * Maximum update depth). `root` scopes the observed element — omitted, observes `document.documentElement`;
 * see {@link useElementsSurfaceChrome}'s doc for why this takes a resolved element, not a ref.
 */
export function useCanvasAppearanceSync(sync: () => void, enabled = true, root?: HTMLElement): void {
  const syncRef = reactHostPort.useRef(sync);
  syncRef.current = sync;
  const [appearanceRoot, setAppearanceRoot] = reactHostPort.useState<HTMLElement | null>(() => root ?? stylingAppearanceRootElement());
  reactHostPort.useEffect(() => {
    const read = () => setAppearanceRoot(root ?? stylingAppearanceRootElement());
    read();
    return subscribeStylingAppearanceRoot(read);
  }, [root]);
  reactHostPort.useEffect(() => {
    const observedRoot = appearanceRoot ?? resolveElementsSurfaceChromeRoot(root);
    if (!enabled || !observedRoot || typeof MutationObserver === "undefined") return;
    const run = () => syncRef.current();
    run();
    const observer = new MutationObserver(run);
    observer.observe(observedRoot, { attributes: true, attributeFilter: ["class", "style", "data-ui-appearance", "data-ui-theme"] });
    return () => observer.disconnect();
  }, [appearanceRoot, enabled, root]);
}

/** @emoji 🧪️ Clears every surface-chrome root's leases and DOM overrides between vitest cases (tests only
 * ever exercise the default `document.documentElement` root, but this clears all of them defensively). */
export function resetElementsSurfaceChromeForTests(): void {
  for (const root of elementsSurfaceChromeDeferredClearFrames.keys()) cancelElementsSurfaceChromeDeferredClear(root);
  for (const root of elementsSurfaceChromeLeasesByRoot.keys()) clearElementsSurfaceChromeDom(root);
  elementsSurfaceChromeLeasesByRoot.clear();
  syncElementsSurfaceChromeProviders(undefined);
  const root = resolveElementsSurfaceChromeRoot();
  if (root) clearElementsSurfaceChromeDom(root);
}
// #endregion 🌈️SurfaceChrome

// #region 🎛️UiChromePrefs
/** @emoji 🌓️ Storage key for surface appearance (system/light/dark). */
export const UI_CHROME_APPEARANCE_STORAGE_KEY = "ui.chrome.appearance";

/** @emoji 🌓️ Reads persisted surface appearance from the given shell's storage — a required param
 * (not a `localStorage` default) since two shells on one page must never read/write each other's
 * appearance through a shared key. */
export function readStoredUiChromeAppearance(storage: StoragePort): ElementsSurfaceAppearance {
  const raw = storage.get(UI_CHROME_APPEARANCE_STORAGE_KEY);
  if (raw === "light" || raw === "dark" || raw === "system") return raw;
  return "system";
}

/** @emoji 🌓️ Persists surface appearance to the given shell's storage. */
export function writeStoredUiChromeAppearance(storage: StoragePort, appearance: ElementsSurfaceAppearance): void {
  storage.set(UI_CHROME_APPEARANCE_STORAGE_KEY, appearance);
}

/** @emoji 📐️ User-selectable layout device; mobile is automatic and excluded here. */
export type UiChromeLayout = "desktop" | "tablet";

/** @emoji 📐️ Storage key for the user-selected desktop/tablet layout. */
export const UI_CHROME_LAYOUT_STORAGE_KEY = "ui.chrome.layout";

/** @emoji 📐️ Reads the persisted layout preference from the given shell's storage, defaulting to desktop. */
export function readStoredUiChromeLayout(storage: StoragePort): UiChromeLayout {
  return storage.get(UI_CHROME_LAYOUT_STORAGE_KEY) === "tablet" ? "tablet" : "desktop";
}

/** @emoji 📐️ Persists the layout preference to the given shell's storage. */
export function writeStoredUiChromeLayout(storage: StoragePort, layout: UiChromeLayout): void {
  storage.set(UI_CHROME_LAYOUT_STORAGE_KEY, layout);
}
// #endregion 🎛️UiChromePrefs

// #region 📱️MediaQuery
/**
 * Hook returning whether a CSS media query currently matches.
 **/
export function useMediaQuery(query: string, defaultValue = false): boolean {
  const getMatches = reactHostPort.useCallback(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return defaultValue;
    }

    return window.matchMedia(query).matches;
  }, [defaultValue, query]);

  const [matches, setMatches] = reactHostPort.useState<boolean>(getMatches);

  reactHostPort.useEffect(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return undefined;
    }

    const mediaQueryList = window.matchMedia(query);
    const bindings = createDOMEventBinding();
    const handleChange = (event: MediaQueryListEvent) => setMatches(event.matches);
    setMatches(mediaQueryList.matches);
    bindings.listen(mediaQueryList, "change", handleChange);

    return () => {
      bindings.dispose();
    };
  }, [query]);

  return matches;
}
// #endregion 📱️MediaQuery

// #region 🚫️BrowserDefaults
/** @emoji 🚫️ Capture-phase listeners: native context menu off everywhere; form-control browser defaults on focus.
 *
 * ⌨️ Tab focus traversal stays the browser's. It used to be suppressed everywhere outside a typing target, which left
 * every chrome control of the shell — navbar, Home, footer, panels, window chips — unreachable without a mouse
 * (measured inside `s`: 30 Tab presses from Home never left `mode-dock-panel-root`; WCAG 2.2 SC 2.1.1, ticket
 * 26/09/23 S15). A program that binds `tab` as a chord still owns it: the shell's keybinding loop calls
 * `preventDefault` on every chord it resolves. */
export function installElementsSurfaceBrowserDefaultSuppression(bindings: ReturnType<typeof createDOMEventBinding>): void {
  if (typeof document === "undefined") return;
  const onContextMenu = (event: Event): void => {
    event.preventDefault();
  };
  const onFocusIn = (event: FocusEvent): void => {
    const target = event.target;
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
      applyUiFormControlBrowserDefaults(target);
    }
  };
  bindings.listen(document, "contextmenu", onContextMenu as EventListener, true);
  bindings.listen(document, "focusin", onFocusIn as EventListener, true);
}
// #endregion 🚫️BrowserDefaults
