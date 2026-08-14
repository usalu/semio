// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🐚️ShellScope/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header


// #region 🔌️Adapters
import * as React from "react";
import { I18nextProvider } from "react-i18next";
import i18next from "i18next";
import { type StoragePort, createBrowserStoragePort, ephemeralBox, ephemeralSet } from "@semio-tech/framework";
import { type UiLocale, createShellI18nInstance } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { type MergeMode } from "../../../🕹️interaction/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🐚️ShellScope
/** @emoji 🐚️ Per-shell replacement for the old `(globalThis).__selectionMode` global plus its
 * `window`-wide `"semio:selectionOptionsChanged"` broadcast — those meant one shell's selection-mode
 * change silently reconfigured every other mounted shell's WASM session too. Keyed by the same
 * {@link MergeMode} union `marqueeModeFromModifiers`/`selectionMergeIds` already use (declared in
 * `📦️index.tsx` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a unified the old
 * bespoke `SelectionMergeMode` onto the `🕹️interaction` module's `MergeMode`). */
export interface SelectionModeStore {
  get(): MergeMode;
  set(mode: MergeMode): void;
  /** Registers a callback invoked whenever `set` changes the mode. Returns an unsubscribe function. */
  subscribe(callback: () => void): () => void;
}

function createSelectionModeStore(): SelectionModeStore {
  let mode: MergeMode = "replace";
  const subscribers = new Set<() => void>();
  return {
    get: () => mode,
    set: (next) => {
      if (next === mode) return;
      mode = next;
      for (const callback of subscribers) callback();
    },
    subscribe: (callback) => {
      subscribers.add(callback);
      return () => subscribers.delete(callback);
    },
  };
}

/** @emoji 🐚️ Per-mounted-`FrameworkOsShell` scope — the seam every document-global mechanism (element
 * ids, theming, i18n, keybindings, storage, portals, ...) threads through so several shells can coexist
 * on one page. Populated incrementally: theming/i18n/keyboard fields land with their own waves. */
export interface ShellScope {
  readonly shellId: string;
  /** The shell's own root element (the `.semio-scope` div `FrameworkOsShell` renders into). */
  readonly rootRef: { current: HTMLElement | null };
  /** Fixed-position overlay layer, last child of the shell root — the portal target for menus, drag
   * ghosts, and dialogs so they never escape to `document.body` and collide with another shell. */
  readonly portalLayerRef: { current: HTMLElement | null };
  readonly storage: StoragePort;
  /** True only for the shell `bootFrameworkOs` mounts as the sole app on its own page — gates the
   * handful of behaviors that are legitimately page-global (document title, browser history sync). */
  readonly ownsPage: boolean;
  /** `querySelector` scoped to this shell's root, replacing document-wide lookups. */
  query(selector: string): HTMLElement | null;
  /** `querySelectorAll` scoped to this shell's root, replacing document-wide lookups. */
  queryAll(selector: string): HTMLElement[];
  readonly selection: SelectionModeStore;
  /** This shell's own i18next instance (see `createShellI18nInstance`) — `useUiTranslation`/`useLabel`
   * pick it up automatically via the nearest `I18nextProvider` ancestor (`FrameworkOsShell` renders one
   * around its subtree), so no call site outside `ShellScope` plumbing itself needs to change. */
  readonly i18n: typeof i18next;
}

const shellScopeAutoIdSeq = ephemeralBox("framework.modules.ui.elements.core.ShellScope.component.tsx.shellScopeAutoIdSeq", 0);

/** @emoji 🐚️ Creates a fresh {@link ShellScope}. Call once per shell mount (e.g. from a lazy `useState`
 * initializer) — the scope's identity must stay stable for the shell instance's lifetime. */
export function createShellScope(options: { readonly shellId?: string; readonly storage: StoragePort; readonly ownsPage?: boolean; readonly initialLocale?: UiLocale }): ShellScope {
  const shellId = options.shellId ?? `shell-${(shellScopeAutoIdSeq.current += 1)}`;
  const rootRef: { current: HTMLElement | null } = { current: null };
  const portalLayerRef: { current: HTMLElement | null } = { current: null };
  return {
    shellId,
    rootRef,
    portalLayerRef,
    storage: options.storage,
    ownsPage: options.ownsPage ?? false,
    query: (selector) => rootRef.current?.querySelector<HTMLElement>(selector) ?? null,
    queryAll: (selector) => (rootRef.current ? Array.from(rootRef.current.querySelectorAll<HTMLElement>(selector)) : []),
    selection: createSelectionModeStore(),
    i18n: createShellI18nInstance(options.initialLocale ?? "en"),
  };
}

export const ShellScopeContext = React.createContext<ShellScope | null>(null);

/** @emoji 🐚️ Also wraps `children` in an `I18nextProvider` bound to `scope.i18n` — the only wiring
 * `useUiTranslation`/`useLabel` (which call plain `useTranslation()`) need to resolve this shell's own
 * translations instead of the shared `uiI18n` singleton; no call site elsewhere changes. */
export function ShellScopeProvider({ scope, children }: { readonly scope: ShellScope; readonly children: React.ReactNode }): React.ReactElement {
  return React.createElement(ShellScopeContext.Provider, { value: scope }, React.createElement(I18nextProvider, { i18n: scope.i18n }, children));
}

/** @emoji 🐚️ Reads the enclosing shell's scope — throws outside a {@link ShellScopeProvider} rather than
 * silently falling back to page-global state, so a missing provider fails loudly during development. */
export function useShellScope(): ShellScope {
  const scope = React.useContext(ShellScopeContext);
  if (!scope) throw new Error("[DEBUG] useShellScope called outside a ShellScopeProvider");
  return scope;
}

/** @emoji 🐚️ Like {@link useShellScope} but returns `null` outside a provider — for the rare leaf element
 * usable both inside a shell and standalone (e.g. a docs-site embed of a single component). */
export function useShellScopeOptional(): ShellScope | null {
  return React.useContext(ShellScopeContext);
}

/** @emoji 🐚️ Falls back to a plain (unnamespaced) browser storage port for the handful of standalone
 * hooks (`useUiTerminology`, `setUiLocale`, …) usable both inside a `ShellScopeProvider` and outside one
 * (a "TS-native product" that hasn't been wrapped yet) — matches pre-scoping behavior for the latter. */
export function shellScopeStorageOrBrowserFallback(scope: ShellScope | null): StoragePort {
  return scope?.storage ?? createBrowserStoragePort();
}
// #endregion 🐚️ShellScope

// #region 🐚️ShellActivity
/** @emoji 🐚️ Which registered shell root most recently received a `pointerdown`/`focusin` — generalizes
 * the `🪟️WindowChrome` region's `surfaceActiveRoots` tracker (which does the same thing for
 * panel/pane/window activity within ONE page) to the shell level, so a page hosting several mounted
 * shells can tell which one the user is actually interacting with. */
const shellActivityRoots = ephemeralSet<HTMLElement>("framework.modules.ui.elements.core.ShellScope.component.tsx.shellActivityRoots");
const activeShellRootValue = ephemeralBox<HTMLElement | null>("framework.modules.ui.elements.core.ShellScope.component.tsx.activeShellRootValue", null);
const shellActivitySubscribers = ephemeralSet<() => void>("framework.modules.ui.elements.core.ShellScope.component.tsx.shellActivitySubscribers");
const shellActivityListenersInstalled = ephemeralBox("framework.modules.ui.elements.core.ShellScope.component.tsx.shellActivityListenersInstalled", false);

function setActiveShellRoot(next: HTMLElement | null): void {
  if (activeShellRootValue.current === next) return;
  activeShellRootValue.current = next;
  shellActivitySubscribers.forEach((notify) => notify());
}

function resolveActiveShellRoot(target: EventTarget | null): HTMLElement | null {
  let node: Node | null = target instanceof Node ? target : null;
  while (node) {
    if (node instanceof HTMLElement && shellActivityRoots.has(node)) return node;
    node = node.parentNode;
  }
  return null;
}

function installShellActivityListeners(): void {
  if (shellActivityListenersInstalled.current || typeof document === "undefined") return;
  shellActivityListenersInstalled.current = true;
  const onActivity = (event: Event) => {
    const root = resolveActiveShellRoot(event.target);
    if (root) setActiveShellRoot(root);
  };
  document.addEventListener("pointerdown", onActivity, true);
  document.addEventListener("focusin", onActivity, true);
}

/** @emoji 🐚️ Registers `root` as a candidate "active shell" — called once per mounted `FrameworkOsShell`.
 * The first (and, on a single-shell page, only) registered root starts active so keyboard dispatch works
 * immediately, before any pointer/focus activity. Returns an unregister function. */
export function registerShellActivityRoot(root: HTMLElement): () => void {
  installShellActivityListeners();
  shellActivityRoots.add(root);
  if (activeShellRootValue.current === null) setActiveShellRoot(root);
  return () => {
    shellActivityRoots.delete(root);
    if (activeShellRootValue.current === root) setActiveShellRoot(shellActivityRoots.values().next().value ?? null);
  };
}

/** @emoji 🐚️ The shell root most recently interacted with, among registered roots — `null` before any
 * shell has registered. */
export function activeShellRoot(): HTMLElement | null {
  return activeShellRootValue.current;
}

/** @emoji 🐚️ True when `rootRef.current` is the page's {@link activeShellRoot} — re-renders on activity
 * changes so shell-gated hotkeys (introduction Next/Back/Skip, …) enable/disable with focus instead of
 * reading a stale snapshot once at mount. Outside any registered shell, returns true so single-shell /
 * storybook call sites keep working without an activity root. */
export function useIsActiveShellRoot(rootRef: { readonly current: HTMLElement | null }): boolean {
  return React.useSyncExternalStore(
    (onStoreChange) => {
      shellActivitySubscribers.add(onStoreChange);
      return () => {
        shellActivitySubscribers.delete(onStoreChange);
      };
    },
    () => {
      const root = rootRef.current;
      if (!root || shellActivityRoots.size === 0) return true;
      return activeShellRootValue.current === root;
    },
    () => true,
  );
}

/**
 * @emoji 🐚️ A `document`-level `keydown` listener gated to one shell: fires `handler` only when the
 * event's target is inside `rootRef.current`, or — for a keystroke that lands on `document`/`body` with
 * nothing focused (the common case for a global hotkey) — when this shell is {@link activeShellRoot}.
 * Replaces the old pattern of an unconditional `window`/`document` keydown listener per shell, under
 * which every mounted shell fired its bound action (and could `preventDefault()` out from under another
 * shell) for every keystroke on the page regardless of which shell the user was actually using.
 */
export function useShellKeydown(rootRef: { readonly current: HTMLElement | null }, handler: (event: KeyboardEvent) => void, deps: readonly unknown[]): void {
  const handlerRef = React.useRef(handler);
  handlerRef.current = handler;
  const root = rootRef.current;
  React.useEffect(() => {
    if (!root) return;
    const unregister = registerShellActivityRoot(root);
    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target;
      const withinThisRoot = target instanceof Node && root.contains(target);
      if (!withinThisRoot && activeShellRoot() !== root) return;
      handlerRef.current(event);
    };
    document.addEventListener("keydown", onKeyDown);
    return () => {
      document.removeEventListener("keydown", onKeyDown);
      unregister();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- `deps` is the caller's own dependency list, spread intentionally
  }, [root, ...deps]);
}

/** 🐚️ Stable no-op root for {@link useShellKeydown} call sites rendered outside a `ShellScopeProvider` (unit tests, storybook) — the hook's `if (!root) return;` guard makes this permanently inert rather than throwing. */
export const NULL_SHELL_ROOT_REF: { readonly current: HTMLElement | null } = { current: null };
// #endregion 🐚️ShellActivity
