// #region 🧲️Header
// 💻️ framework/ui/modules/⌨️control-keybinding-context/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { ephemeralMap } from "@semio-tech/framework";
import { useHotkeys } from "react-hotkeys-hook";
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
}

/** @emoji ⌨️ Dependency values that keep a control-keybinding callback current. */
export type ControlKeybindingDependencies = ReadonlyArray<unknown>;

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
  const raw = id ? resolveControlKeybindingRaw(id, bindings) ?? SHELL_KEYBINDINGS[id] : undefined;
  return raw ? formatKeybindingShortcut(raw) : undefined;
}

/** @emoji ⌨️ Binds the active chord for a control id. */
export function useControlKeybinding(controlId: string, callback: ControlKeybindingCallback, options?: ControlKeybindingOptions, dependencies?: ControlKeybindingDependencies): void {
  const bindings = useUiKeybindingsByControlId();
  const keys = reactHostPort.useMemo(() => resolveControlKeybindingRaw(controlId, bindings) ?? SHELL_KEYBINDINGS[controlId], [bindings, controlId]);
  const resolvedOptions = options ?? {};
  useHotkeys(keys ?? "", callback, { ...resolvedOptions, enabled: Boolean(keys) && (resolvedOptions.enabled ?? true) }, dependencies ?? []);
}
// #endregion ⌨️ControlKeybindingContext
