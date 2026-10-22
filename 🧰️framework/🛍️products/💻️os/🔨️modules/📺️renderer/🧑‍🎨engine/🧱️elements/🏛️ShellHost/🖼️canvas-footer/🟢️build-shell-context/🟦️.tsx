/** 🟢️ The build shell context phase of useCanvasAndFooter. */
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { type ContextMenuItemSpec, type LocalizedLabel, organizeContextMenu, resolveWindowActions } from "@semio-tech/framework";
import { type ContextMenuItem, isContextMenuPointerTarget } from "@semio-tech/ui-react";
import { mapContextMenuSpecs } from "../../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { actionCategoryId } from "../../../🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import { actionRequiresStagedForm } from "../../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import { filterDefinitionsForRole } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { openArtifactWithText, resolveAppLabel, resolveManifestLabel, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { buildPluginFilterState } from "../🔶️plugin-filter/🟦️.tsx";
import type { useFooterItemsState } from "../🟠️footer-items/🟦️.tsx";

/** 🔵 Builds buildShellContextMenuItems of useCanvasAndFooter. */
export function useBuildShellContext({ session, activeWindowId, appLabelsOverlay, uiTerminology, uiLocale, keysByActionId, hasOpenArtifactSurfaces, dispatchShellMenuAction, setShellContextMenu }: Pick<ReturnType<typeof buildPluginFilterState>, "session" | "activeWindowId" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "keysByActionId" | "hasOpenArtifactSurfaces" | "setShellContextMenu"> & Pick<ReturnType<typeof useFooterItemsState>, "dispatchShellMenuAction">) {
  /** 🖱️ Builds the shell-level fallback menu: the active window's declared actions (undo/redo, view
   * actions, ...) plus a command-palette opener — shown for any right-click no inner surface claimed
   * (window background, empty panel/navbar/footer space, an app with no scene at all). Arg-carrying
   * actions route through the reserved `"shell.openActionPane"` id (parity with the wgpu shell's
   * `build_shell_context_menu_specs`), the whole spec list runs through `organizeContextMenu`, then
   * `mapContextMenuSpecs` binds it to `dispatchShellMenuAction`. */
  const buildShellContextMenuItems = useCallback((): ContextMenuItem[] => {
    if (!session) return [];
    const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
    const specs: ContextMenuItemSpec[] = [];
    const categoryByActionId = new Map<string, string>();
    if (windowKind) {
      for (const action of filterDefinitionsForRole(resolveWindowActions(session.app, windowKind), session.app.role)) {
        if (!action.inPalette) continue;
        const argCarrying = actionRequiresStagedForm(action);
        categoryByActionId.set(action.id, actionCategoryId(action));
        specs.push({
          id: `shell-menu.action.${action.id}`,
          label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label as LocalizedLabel | string, uiTerminology, uiLocale)) + (argCarrying ? "…" : ""),
          icon: action.iconId,
          shortcut: action.keys ?? keysByActionId.get(action.id),
          destructive: action.kind === "mutation" && action.id.toLowerCase().includes("delete"),
          action: argCarrying ? "shell.openActionPane" : action.id,
          args: argCarrying ? { actionId: action.id } : undefined,
        });
      }
    }
    if (specs.length > 0) specs.push({ id: "shell-menu.separator", separator: true });
    if (hasOpenArtifactSurfaces) {
      specs.push({ id: "shell.openArtifactWith", label: openArtifactWithText(uiLocale), icon: "app-window", action: "shell.openArtifactWith" });
    }
    specs.push({
      id: "shell.openPalette",
      label: shellLabel("ui.search.toggle"),
      icon: "search",
      action: "shell.openPalette",
    });
    const organized = organizeContextMenu(specs, (id) => categoryByActionId.get(id));
    return mapContextMenuSpecs(organized, dispatchShellMenuAction, keysByActionId);
  }, [session, activeWindowId, appLabelsOverlay, keysByActionId, dispatchShellMenuAction, uiTerminology, uiLocale, hasOpenArtifactSurfaces]);

  useEffect(() => {
    const handleContextMenu = (event: globalThis.MouseEvent) => {
      if (isContextMenuPointerTarget(event.target)) return;
      if (event.defaultPrevented) return;
      const items = buildShellContextMenuItems();
      if (items.length === 0) return;
      event.preventDefault();
      setShellContextMenu({ x: event.clientX, y: event.clientY, items });
    };
    window.addEventListener("contextmenu", handleContextMenu);
    return () => window.removeEventListener("contextmenu", handleContextMenu);
  }, [buildShellContextMenuItems]);
  return { buildShellContextMenuItems };
}
