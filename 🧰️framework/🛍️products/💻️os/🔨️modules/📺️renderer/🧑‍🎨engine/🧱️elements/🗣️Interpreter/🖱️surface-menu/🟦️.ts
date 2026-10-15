/** 🖱️ The scene-host context menu contract: the shell fallback context, the surface menu title keys and opener, the app keybinding context and hooks, and the mapping of plugin menu specs onto menu rows.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}, which re-exports its public names; `ShellHost` re-exports the keybinding hooks and `World3dHost` re-exports `mapContextMenuSpecs`, so import specifiers stay unchanged while `NodeGraph` reads them here.
 */

import { createContext, useCallback, useContext } from "react";
import { formatKeybindingShortcut, type ContextMenuItem, type UiTranslationKey } from "@semio-tech/ui-react";
import { isIconName } from "@semio-tech/assets";
import type { ContextMenuItemSpec, PluginContextMenuRequest } from "@semio-tech/framework";
import { contextMenuGroupLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { wireLabel } from "../🌉️scene-host-contract/🟦️.ts";

export const ShellContextMenuFallbackContext = createContext<(() => ContextMenuItem[]) | undefined>(undefined);

export function useShellContextMenuFallback(): (() => ContextMenuItem[]) | undefined {
  return useContext(ShellContextMenuFallbackContext);
}

export type SurfaceContextMenuResult = {
  readonly items: ContextMenuItem[];
  readonly titleKey: UiTranslationKey;
};

const contextMenuSurfaceTitleKeys = {
  blockList: "ui.surfaceContextMenu.step",
  board2d: "ui.surfaceContextMenu.board",
  canvas2d: "ui.surfaceContextMenu.canvas",
  diffView: "ui.surfaceContextMenu.diff",
  eventFeed: "ui.surfaceContextMenu.event",
  graphTimeline: "ui.surfaceContextMenu.history",
  inkCanvas: "ui.surfaceContextMenu.ink",
  nodeGraph: "ui.surfaceContextMenu.flow",
  paint2d: "ui.surfaceContextMenu.paint",
  table: "ui.surfaceContextMenu.row",
  textEditor: "ui.surfaceContextMenu.editor",
  tiledMap: "ui.surfaceContextMenu.map",
  virtualFileSystem: "ui.surfaceContextMenu.file",
  world3d: "ui.surfaceContextMenu.scene",
} as const satisfies Record<string, UiTranslationKey>;

const contextMenuTargetTitleKeys = {
  architecture: "ui.surfaceContextMenu.architecture",
  attraction: "ui.surfaceContextMenu.attraction",
  block: "ui.surfaceContextMenu.block",
  edge: "ui.surfaceContextMenu.edge",
  entry: "ui.surfaceContextMenu.entry",
  feature: "ui.surfaceContextMenu.feature",
  group: "ui.surfaceContextMenu.group",
  handle: "ui.surfaceContextMenu.handle",
  layer: "ui.surfaceContextMenu.layer",
  node: "ui.surfaceContextMenu.node",
  object: "ui.surfaceContextMenu.object",
  part: "ui.surfaceContextMenu.part",
  path: "ui.surfaceContextMenu.path",
  pixel: "ui.surfaceContextMenu.pixel",
  position: "ui.surfaceContextMenu.position",
  reference: "ui.surfaceContextMenu.reference",
  route: "ui.surfaceContextMenu.route",
  row: "ui.surfaceContextMenu.row",
  slider: "ui.surfaceContextMenu.slider",
  vortex: "ui.surfaceContextMenu.vortex",
} as const satisfies Record<string, UiTranslationKey>;

export function surfaceContextMenuTitleKey(request: PluginContextMenuRequest): UiTranslationKey {
  const hitDomain = request.surface?.hits?.[0]?.domain;
  if (hitDomain && hitDomain in contextMenuTargetTitleKeys) return contextMenuTargetTitleKeys[hitDomain as keyof typeof contextMenuTargetTitleKeys];
  const surfaceKind = request.surface?.kind;
  if (surfaceKind && surfaceKind in contextMenuSurfaceTitleKeys) return contextMenuSurfaceTitleKeys[surfaceKind as keyof typeof contextMenuSurfaceTitleKeys];
  return "ui.surfaceContextMenu.workspace";
}

export async function openSurfaceContextMenu(
  requestContextMenu: ((request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>) | undefined,
  request: PluginContextMenuRequest,
  mapSpecs: (specs: readonly ContextMenuItemSpec[]) => ContextMenuItem[],
  shellFallback: (() => ContextMenuItem[]) | undefined,
): Promise<SurfaceContextMenuResult> {
  const titleKey = surfaceContextMenuTitleKey(request);
  if (!requestContextMenu) return { items: shellFallback?.() ?? [], titleKey };
  try {
    // 🖱️ An EMPTY plugin answer stays empty — it is the surface saying "nothing here", and the shell
    // fallback is deliberately NOT substituted for it (law: "openSurfaceContextMenu keeps an empty
    // plugin answer off the shell fallback"). The fallback is only for a surface with no resolver at all.
    return { items: mapSpecs(await requestContextMenu(request)), titleKey };
  } catch {
    return { items: [], titleKey };
  }
}

const EMPTY_KEYS_BY_ACTION_ID = new Map<string, string>();

/** ⌨️ Last-wins app keybindings for enriching context-menu shortcut labels in scene hosts. */
export const AppKeybindingsContext = createContext<ReadonlyMap<string, string>>(EMPTY_KEYS_BY_ACTION_ID);

/** ⌨️ Resolves action→keys bindings from the nearest {@link AppKeybindingsContext} provider. */
export function useAppKeybindingsByActionId(): ReadonlyMap<string, string> {
  return useContext(AppKeybindingsContext);
}

/** 🖱️ Maps program context-menu specs with app keybinding shortcut enrichment. */
export function useMapContextMenuSpecs(dispatch: (action: string, args?: Record<string, unknown>) => void) {
  const keysByActionId = useAppKeybindingsByActionId();
  return useCallback((specs: readonly ContextMenuItemSpec[]) => mapContextMenuSpecs(specs, dispatch, keysByActionId), [dispatch, keysByActionId]);
}

/** 🖱️ Maps plugin-authored {@link ContextMenuItemSpec} rows onto UI {@link ContextMenuItem} rows, binding select/hover to host `dispatch`.
 *
 * 🗂️ A `menu.group.<category>` row travels from the guest with `label: undefined` by contract — the
 * taxonomy is chrome vocabulary the SHELL owns, not app vocabulary (`plugin/🦀️.rs`'s `Menu::group`,
 * asserted by `plugin-runtime-plugin-builder-contract`'s "group rows travel with no label"). So the
 * label is resolved here from the same EN/DE `ui.ribbon.parent.*` bundle the ribbon reads, exactly
 * like the wgpu target's `shell_context_menu_item_from_spec` does through `ribbon_parent_label` —
 * including its default folder icon. Without this the React shell rendered the raw ids
 * (`menu.group.history`, `menu.group.selection`, …), measured 2026-09-09 21:05. */
export function mapContextMenuSpecs(
  specs: readonly ContextMenuItemSpec[] | null | undefined,
  dispatch: (action: string, args?: Record<string, unknown>) => void,
  keysByActionId?: ReadonlyMap<string, string>,
): ContextMenuItem[] {
  return (specs ?? []).map((spec) => {
    const boundKeys = spec.action ? keysByActionId?.get(spec.action) : undefined;
    const shortcut = spec.shortcut ?? (boundKeys ? formatKeybindingShortcut(boundKeys) : undefined);
    const groupLabel = spec.label === undefined ? contextMenuGroupLabel(spec.id) : undefined;
    return {
      id: spec.id,
      action: spec.action,
      label: spec.label === undefined ? groupLabel : wireLabel(spec.label),
      icon: spec.icon && isIconName(spec.icon) ? spec.icon : groupLabel === undefined ? undefined : "folder",
      color: spec.color,
      shortcut,
      disabled: spec.disabled,
      reason: spec.disabled && spec.reason ? wireLabel(spec.reason) : undefined,
      separator: spec.separator,
      checked: spec.checked,
      destructive: spec.destructive,
      onSelect: spec.action
        ? (event) => {
            const clientX = event && "clientX" in event && typeof event.clientX === "number" ? event.clientX : undefined;
            const clientY = event && "clientY" in event && typeof event.clientY === "number" ? event.clientY : undefined;
            const pointArgs = (spec.action === "openVortexSuggestions" || spec.action === "openHandleSuggestions") && clientX != null && clientY != null ? { x: clientX, y: clientY } : undefined;
            dispatch(spec.action!, { ...spec.args, ...pointArgs });
          }
        : undefined,
      onHover: spec.hoverAction ? () => dispatch(spec.hoverAction!, spec.hoverArgs) : undefined,
      children: spec.children?.length ? mapContextMenuSpecs(spec.children, dispatch, keysByActionId) : undefined,
    };
  });
}
