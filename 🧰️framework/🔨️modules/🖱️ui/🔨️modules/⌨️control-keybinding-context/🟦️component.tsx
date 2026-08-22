// #region 🧲️Header
// 💻️ framework/ui/modules/⌨️control-keybinding-context/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { ephemeralMap } from "@semio-tech/framework";
import { formatKeybindingShortcut } from "../⌨️keybinding-text-interpretation/🟦️component.ts";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/🟦️component.tsx";
import { resolveControlLabelId } from "../../🧱️elements/🚗️UiDriver/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region ⌨️ControlKeybindingContext
/** @emoji ⌨️ Action reference accepted by a control keybinding definition. */
export interface ControlKeybindingAction {
  readonly action: string;
}

/** @emoji ⌨️ Application keybinding declaration merged into the control registry. */
export interface ControlKeybindingDefinition {
  readonly action: ControlKeybindingAction;
  readonly keys: string;
}

/** @emoji ⌨️ Opaque provider children rendered by the React adapter. */
type UiKeybindingsProviderChildren = unknown;

/** @emoji ⌨️ Props supplied to the control keybinding context provider. */
interface UiKeybindingsProviderProps {
  readonly bindings: ReadonlyMap<string, string>;
  readonly children: UiKeybindingsProviderChildren;
}

/** @emoji ⌨️ Callback invoked for a matched control keybinding. */
export type ControlKeybindingCallback = () => void;

/** @emoji ⌨️ Supported control-keybinding hook options. */
export interface ControlKeybindingOptions {
  readonly enabled?: boolean;
  readonly enableOnFormTags?: boolean;
  readonly preventDefault?: boolean;
}

/** @emoji ⌨️ Dependency values that keep a control-keybinding callback current. */
export type ControlKeybindingDependencies = ReadonlyArray<unknown>;

interface OwnedHotkeyChord {
  readonly alt: boolean;
  readonly control: boolean;
  readonly key: string;
  readonly meta: boolean;
  readonly shift: boolean;
}

/** @emoji 🍎 Whether the browser platform uses Command as its primary modifier. */
export function isAppleHotkeyPlatform(platform: string): boolean {
  return /mac|iphone|ipad|ipod/i.test(platform);
}

/** @emoji 🧹 Normalizes browser key names and keybinding aliases to one comparison token. */
export function normalizeHotkeyKey(key: string): string {
  if (key === " ") return "space";
  const normalized = key.trim().toLowerCase();
  if (normalized === "spacebar") return "space";
  if (normalized === "esc") return "escape";
  if (normalized === "left") return "arrowleft";
  if (normalized === "right") return "arrowright";
  if (normalized === "up") return "arrowup";
  if (normalized === "down") return "arrowdown";
  return normalized;
}

/** @emoji 🧩 Parses one comma-separated hotkey declaration into strict modifier chords. */
export function parseOwnedHotkeyChords(keys: string, applePlatform: boolean): readonly OwnedHotkeyChord[] {
  const chords: OwnedHotkeyChord[] = [];
  for (const rawChord of keys.split(",")) {
    const tokens = rawChord.split("+").map((token) => normalizeHotkeyKey(token)).filter(Boolean);
    if (tokens.length === 0) continue;
    let alt = false;
    let control = false;
    let meta = false;
    let shift = false;
    let key = "";
    for (const token of tokens) {
      if (token === "alt" || token === "option") alt = true;
      else if (token === "ctrl" || token === "control") control = true;
      else if (token === "meta" || token === "cmd" || token === "command") meta = true;
      else if (token === "mod") {
        if (applePlatform) meta = true;
        else control = true;
      } else if (token === "shift") shift = true;
      else key = token;
    }
    if (key) chords.push({ alt, control, key, meta, shift });
  }
  return chords;
}

/** @emoji 🎯 Matches one keyboard event against an already-normalized owned chord. */
export function keyboardEventMatchesOwnedHotkey(event: KeyboardEvent, chord: OwnedHotkeyChord): boolean {
  return normalizeHotkeyKey(event.key) === chord.key
    && event.altKey === chord.alt
    && event.ctrlKey === chord.control
    && event.metaKey === chord.meta
    && event.shiftKey === chord.shift;
}

/** @emoji ✍️ Whether a keyboard event originated from a form or editable text surface. */
export function isHotkeyFormTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  return target.matches("input,textarea,select,[contenteditable]:not([contenteditable='false'])");
}

/** @emoji 🌙 Owned React hotkey listener with strict chord matching and deterministic cleanup. */
export function useHotkeys(keys: string, callback: ControlKeybindingCallback, options: ControlKeybindingOptions = {}, dependencies: ControlKeybindingDependencies = []): void {
  const callbackRef = reactHostPort.useRef(callback);
  callbackRef.current = callback;
  const platform = typeof navigator === "undefined" ? "" : navigator.platform;
  const chords = reactHostPort.useMemo(() => parseOwnedHotkeyChords(keys, isAppleHotkeyPlatform(platform)), [keys, platform]);
  const enabled = options.enabled ?? true;
  const enableOnFormTags = options.enableOnFormTags ?? false;
  const preventDefault = options.preventDefault ?? false;
  void dependencies;

  reactHostPort.useEffect(() => {
    if (!enabled || chords.length === 0 || typeof window === "undefined") return;
    const onKeyDown = (event: KeyboardEvent): void => {
      if (!enableOnFormTags && isHotkeyFormTarget(event.target)) return;
      if (!chords.some((chord) => keyboardEventMatchesOwnedHotkey(event, chord))) return;
      if (preventDefault) event.preventDefault();
      callbackRef.current();
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [chords, enabled, enableOnFormTags, preventDefault]);
}

/** @emoji ⌨️ Last-wins action-to-keys map from app keybindings. */
export function buildKeysByActionId(keybindings: readonly ControlKeybindingDefinition[]): ReadonlyMap<string, string> {
  const map = new Map<string, string>();
  for (const binding of keybindings) {
    map.set(binding.action.action, binding.keys);
  }
  return map;
}

/** @emoji ⌨️ Shell chrome control ids mapped to default chords. */
export const SHELL_KEYBINDINGS: Readonly<Record<string, string>> = {
  "ui.introduction.skip": "escape",
  "ui.introduction.next": "enter,arrowright",
  "ui.introduction.back": "arrowleft",
  "ui.dialog.cancel": "escape",
  "ui.dialog.submit": "enter",
  "ui.search.toggle": "mod+p",
  "ui.find.toggle": "mod+f",
  "os.toggleFullscreen": "mod+shift+f",
  "ui.nav.back": "mod+[",
  "ui.nav.forward": "mod+]",
  "ui.nav.up": "mod+up",
  "ui.shell.panelAnchor.topLeft": "ctrl+b,meta+b",
  "ui.shell.panelAnchor.topMiddle": "ctrl+m,meta+m",
  "ui.shell.panelAnchor.topRight": "ctrl+shift+b,meta+shift+b",
  "ui.shell.panelAnchor.rightMiddle": "ctrl+shift+m,meta+shift+m",
  "ui.shell.panelAnchor.bottomRight": "ctrl+alt+shift+b,meta+alt+shift+b",
  "ui.shell.panelAnchor.bottomMiddle": "ctrl+alt+m,meta+alt+m",
  "ui.shell.panelAnchor.bottomLeft": "ctrl+alt+b,meta+alt+b",
  "ui.shell.panelAnchor.leftMiddle": "ctrl+alt+shift+m,meta+alt+shift+m",
  "ui.window.close": "mod+shift+w",
  "ui.window.focus": "mod+shift+enter",
  "ui.window.newWindow": "mod+shift+n",
};

/** @emoji ⌨️ Merges shell defaults, app action bindings, and user overrides. */
export function composeControlKeybindings(keysByActionId: ReadonlyMap<string, string>, overrides: Readonly<Record<string, string>>): ReadonlyMap<string, string> {
  const map = new Map<string, string>(Object.entries(SHELL_KEYBINDINGS));
  for (const [actionId, keys] of keysByActionId) {
    if (!map.has(actionId)) map.set(actionId, keys);
  }
  for (const [controlId, keys] of Object.entries(overrides)) {
    map.set(controlId, keys);
  }
  return map;
}

const EMPTY_CONTROL_KEYBINDINGS = ephemeralMap<string, string>("framework.modules.ui.modules.control-keybinding-context.EMPTY_CONTROL_KEYBINDINGS");

const UiKeybindingsContext = reactHostPort.createContext<ReadonlyMap<string, string>>(EMPTY_CONTROL_KEYBINDINGS);

/** @emoji ⌨️ Supplies merged control-id-to-chords bindings to a subtree. */
export function UiKeybindingsProvider({ bindings, children }: UiKeybindingsProviderProps) {
  const content = children as React.ReactNode;
  return <UiKeybindingsContext.Provider value={bindings}>{content}</UiKeybindingsContext.Provider>;
}

/** @emoji ⌨️ Resolves the nearest control-id-to-chords binding map. */
export function useUiKeybindingsByControlId(): ReadonlyMap<string, string> {
  return reactHostPort.useContext(UiKeybindingsContext);
}

/** @emoji ⌨️ Resolves a raw chord string for a control id. */
export function resolveControlKeybindingRaw(id: string | undefined, bindings: ReadonlyMap<string, string>): string | undefined {
  if (!id) return undefined;
  const direct = bindings.get(id);
  if (direct) return direct;
  const labelId = resolveControlLabelId(id);
  if (labelId !== id) return bindings.get(labelId);
  return undefined;
}

/** @emoji ⌨️ Resolves a platform-formatted shortcut label for a control id. */
export function useControlHotkey(id: string | undefined): string | undefined {
  const bindings = useUiKeybindingsByControlId();
  const raw = id
    ? resolveControlKeybindingRaw(id, bindings) ?? SHELL_KEYBINDINGS[id] ?? SHELL_KEYBINDINGS[resolveControlLabelId(id)]
    : undefined;
  return raw ? formatKeybindingShortcut(raw) : undefined;
}

/** @emoji ⌨️ Binds the active chord for a control id. */
export function useControlKeybinding(controlId: string, callback: ControlKeybindingCallback, options?: ControlKeybindingOptions, dependencies?: ControlKeybindingDependencies): void {
  const bindings = useUiKeybindingsByControlId();
  const keys = reactHostPort.useMemo(() => resolveControlKeybindingRaw(controlId, bindings) ?? SHELL_KEYBINDINGS[controlId] ?? SHELL_KEYBINDINGS[resolveControlLabelId(controlId)], [bindings, controlId]);
  const resolvedOptions = options ?? {};
  useHotkeys(keys ?? "", callback, { ...resolvedOptions, enabled: Boolean(keys) && (resolvedOptions.enabled ?? true) }, dependencies ?? []);
}
// #endregion ⌨️ControlKeybindingContext
