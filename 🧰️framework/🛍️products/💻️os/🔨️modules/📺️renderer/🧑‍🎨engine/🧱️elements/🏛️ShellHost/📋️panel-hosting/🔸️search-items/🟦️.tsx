/** 🔸️ The search items phase of usePanelHosting. */
import { useMemo } from "react";
import { type LocalizedLabel, panelTabKindId } from "@semio-tech/framework";
import { Icon } from "@semio-tech/ui-react";
import { FRAMEWORK_CATEGORY_COMMAND_ID } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { appBreadcrumb, flattenPanelTabLeaves, resolveArtifactByAppId } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { commandAddressKey, commandCategoryLabel, commandKeybindingChords, detectCommandPlatform } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { resolveAppLabel, resolveManifestLabel, resolvePanelTabLabel, shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { type UISearchItem } from "../../../🔎️ShellSearch/🟦️.tsx";
import type { useBrandState } from "../🔶️brand/🟦️.tsx";

/** 🔹 Builds searchItems of usePanelHosting. */
export function useSearchItems({ session, appLabelsOverlay, uiTerminology, uiLocale, onAction, dispatch, resolvedCommands, mobile, onCommand, hostMode, panel, spacePrograms, loadedPlugins, spawnProgram, hostControllerId, activeWindowId }: Pick<ReturnType<typeof useBrandState>, "session" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "onAction" | "dispatch" | "resolvedCommands" | "mobile" | "onCommand" | "hostMode" | "panel" | "spacePrograms" | "loadedPlugins" | "spawnProgram" | "hostControllerId" | "activeWindowId">) {
  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label as LocalizedLabel | string, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id }),
      });
    }
    for (const { definition, address } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${commandAddressKey(address).replaceAll(":", ".")}`,
        label: (() => { const resolved = resolveManifestLabel(definition.label as LocalizedLabel | string, uiTerminology, uiLocale); return argCarrying ? `${resolved}…` : resolved; })(),
        description: commandKeybindingChords(definition, detectCommandPlatform(typeof navigator !== "undefined" ? `${navigator.platform} ${navigator.userAgent}` : "")).join(",") || undefined,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`];
            if (mobile) {
              dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
              dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
            } else {
              dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
              dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
            }
            dispatch({ type: "SET_COMMAND_EXPANDED", value: commandAddressKey(address) });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(address);
        },
      });
    }
    if (hostMode && panel) {
      const spawnEntryCounts = new Map<string, number>();
      for (const program of spacePrograms) {
        const seen = spawnEntryCounts.get(program.pluginId) ?? 0;
        spawnEntryCounts.set(program.pluginId, seen + 1);
        items.push({
          id: seen === 0 ? `spawn.${program.pluginId}` : `spawn.${program.pluginId}.${program.appId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appBreadcrumb(resolveArtifactByAppId(loadedPlugins, program.appId, program.breadcrumb, uiTerminology))}`,
          description: `${program.pluginId} · ${program.appId}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => void spawnProgram(program),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.hostApp"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "goHome" }),
        },
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, mobile, onAction, onCommand, panel, resolvedCommands, session, spacePrograms, hostMode, uiLocale, uiTerminology, hostControllerId]);
  return { searchItems };
}
