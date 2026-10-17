// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/📌️ChromePanels/component.tsx
/** 🖼️ `📌️ChromePanels` — the framework-owned panel tabs of the OS shell's chrome. This file keeps the
 * Settings panel tab that assembles the tab trees, the named-layout host hook, the chat and task manager
 * panel tabs and the small standalone route-not-found and plugin-recovery affordances. The Display panel
 * (`🖥️display-panel`), the Settings tab trees (`⚙️settings-general`, `🎨️settings-theme`,
 * `⌨️settings-keybindings`, `📱️settings-defaults-and-conflicts`), the Marketplace panel
 * (`🏪️marketplace-panel`) and the panel ids (`🆔️panel-ids`) live in the child folders beside it.
 * Each panel's `*HostApi` type is the read/write surface `ShellHost` implements to drive it.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useMemo, useState, type ReactElement } from "react";
import { Button, type IconName, type PanelTabBranch, type PanelTabNode, singleTreeLeaf } from "@semio-tech/ui-react";
import { type NamedLayout, type WindowLayout } from "@semio-tech/framework";
import { defaultAppsSettingsTabLabel, shellLabel, shellTabIcon } from "../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { FRAMEWORK_CHAT_PANEL_ID, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID } from "./🆔️panel-ids/🟦️.ts";
import type { DisplayHostApi } from "./🖥️display-panel/🟦️.tsx";
import { buildSettingsGeneralTree, type SettingsHostApi } from "./⚙️settings-general/🟦️.tsx";
import { buildSettingsThemeTree } from "./🎨️settings-theme/🟦️.tsx";
import { buildSettingsKeybindingsTree } from "./⌨️settings-keybindings/🟦️.tsx";
import { buildConflictsTree, buildSettingsDefaultAppsTree, type ConflictsHostApi, type DefaultAppsHostApi } from "./📱️settings-defaults-and-conflicts/🟦️.tsx";

// #endregion 🔌️Adapters
//#region 🔖️os-chrome-panels
//#region SettingsPanel
export function createFrameworkSettingsPanelTab(getHost: () => SettingsHostApi | null, getDefaultAppsHost?: () => DefaultAppsHostApi | null, getConflictsHost?: () => ConflictsHostApi | null): PanelTabBranch {
  const children: PanelTabNode[] = [
    singleTreeLeaf({
      id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
      icon: shellTabIcon("framework.settings.general"),
      name: shellLabel("ui.settings.tab.general"),
      order: 0,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    }),
    // 🔒️ A locked theme means no theme editing or saving, so the Settings branch omits that child tab.
    ...(getHost()?.locks.themeId
      ? []
      : [
          singleTreeLeaf({
            id: FRAMEWORK_SETTINGS_THEME_TAB_ID,
            icon: shellTabIcon("paintbrush"),
            name: shellLabel("ui.settings.tab.theme"),
            order: 1,
            tree: {
              resolveTree: () => {
                const host = getHost();
                return host ? buildSettingsThemeTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
              },
            },
          }),
        ]),
    singleTreeLeaf({
      id: FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID,
      icon: shellTabIcon("keyboard"),
      name: shellLabel("ui.settings.tab.keybindings"),
      order: 2,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsKeybindingsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    }),
    // 👁️✏️ `PanelTabKind::SettingsDefaultApps` (contract freeze §1/§5) — omitted entirely when the
    // shell has no default-apps host wired up (e.g. an embedded shell that doesn't own OS commands),
    // same "undefined getter → tab absent" idiom the theme lock branch above uses.
    ...(getDefaultAppsHost
      ? [
          singleTreeLeaf({
            id: FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID,
            icon: shellTabIcon("app-window"),
            name: defaultAppsSettingsTabLabel(getDefaultAppsHost?.()?.locale ?? "en"),
            order: 3,
            tree: {
              resolveTree: () => {
                const host = getDefaultAppsHost();
                return host ? buildSettingsDefaultAppsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
              },
            },
          }),
        ]
      : []),
    // ⚖️ `Conflicts` tab (contract freeze §C9) — omitted entirely when the shell has no conflicts
    // host wired up, same "undefined getter → tab absent" idiom `getDefaultAppsHost` uses above.
    ...(getConflictsHost
      ? [
          singleTreeLeaf({
            id: FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID,
            icon: shellTabIcon("triangle-alert"),
            name: shellLabel("ui.conflict.panel"),
            order: 4,
            tree: {
              resolveTree: () => {
                const host = getConflictsHost();
                return host ? buildConflictsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
              },
            },
          }),
        ]
      : []),
  ];
  return {
    kind: "branch",
    id: FRAMEWORK_SETTINGS_PANEL_ID,
    icon: shellTabIcon("settings"),
    name: shellLabel("ui.panelToggle.settings"),
    order: 0,
    children,
  };
}

/** 🗂️ The Display host's layouts: the app's built-in ones and the user's own, which are a projection of the user's
 * `setNamedLayout` preference events — saving and deleting commit one such event (`onSaveUserLayout` /
 * `onDeleteUserLayout`), so a saved layout follows the user to every device like any other shared preference. */
export function useNamedLayoutHost(options: {
  readonly appId: string;
  readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly iconId: IconName; readonly surfaceKind?: string }[];
  readonly builtinLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly currentLayout: WindowLayout | undefined;
  readonly onApplyLayout: (layout: WindowLayout) => void;
  readonly onSaveUserLayout: (layoutId: string, label: string, layout: WindowLayout) => void;
  readonly onDeleteUserLayout: (layoutId: string) => void;
}): DisplayHostApi {
  const [layoutSaveLabel, setLayoutSaveLabel] = useState("");
  return useMemo(
    (): DisplayHostApi => ({
      windowKinds: options.windowKinds,
      namedLayouts: options.builtinLayouts,
      userLayouts: options.userLayouts,
      saveCurrentLayout: (label) => {
        if (!options.currentLayout) return;
        options.onSaveUserLayout(`user-${Date.now()}`, label, options.currentLayout);
      },
      applyNamedLayout: (layoutId) => {
        const layout = [...options.builtinLayouts, ...options.userLayouts].find((entry) => entry.id === layoutId);
        if (layout) options.onApplyLayout(layout.layout);
      },
      deleteUserLayout: (layoutId) => options.onDeleteUserLayout(layoutId),
      layoutSaveLabel,
      setLayoutSaveLabel,
    }),
    [options, layoutSaveLabel],
  );
}

//#endregion SettingsPanel
//#region MarketplacePanel
/** 🧭️ Canvas fallback when studio history resolves to an unknown route. */
export function ShellRouteNotFoundPage({ path, onHome }: { readonly path: string; readonly onHome: () => void }) {
  return (
    <div className="flex h-full min-h-0 flex-col items-center justify-center gap-double p-double" role="alert" data-shell-route-not-found={path}>
      <p className="text-sm text-muted-foreground">{shellLabel("ui.common.routeNotFound", { path })}</p>
      <Button icon="home" text={shellLabel("ui.common.home")} onClick={onHome} />
    </div>
  );
}

/** 🩺️ Plugin crash/quarantine recovery affordances — mirrors `ui_wgpu::ui_recovery_panel`. */
export function PluginRecoveryPanel({
  pluginId,
  quarantined,
  onRestart,
  onDisable,
}: {
  readonly pluginId: string;
  readonly quarantined: boolean;
  readonly onRestart: () => void;
  readonly onDisable: () => void;
}) {
  const message = quarantined ? shellLabel("ui.plugins.recovery.quarantined") : shellLabel("ui.plugins.recovery.crashed");
  return (
    <div className="flex h-full min-h-0 flex-col gap-double p-double" data-plugin-recovery={pluginId}>
      <p className="text-sm font-medium">{shellLabel("ui.plugins.recovery.title")}</p>
      <p className="text-sm text-muted-foreground">{message}</p>
      <div className="flex flex-wrap gap-single">
        <Button icon="rotate-ccw" text={shellLabel("ui.plugins.recovery.restartApp")} onClick={onRestart} />
        <Button icon="eye-off" text={shellLabel("ui.plugins.recovery.disablePlugin")} onClick={onDisable} />
      </div>
    </div>
  );
}

export function createFrameworkChatPanelTab(renderPanel: () => ReactElement): PanelTabNode {
  return singleTreeLeaf({
    id: FRAMEWORK_CHAT_PANEL_ID,
    icon: shellTabIcon("message-square"),
    name: shellLabel("ui.panelToggle.chat"),
    order: 100,
    tree: {
      resolveTree: () => ({
        sections: [],
        emptyState: renderPanel(),
        className: "min-h-0 min-w-0 w-full",
        sortableSections: false,
      }),
    },
  });
}

/** 🧵️ The `os.task-manager` window: one row per actor with suspend/resume/cancel. Built exactly like
 * the chat leaf above (an `emptyState`-hosted panel inside a `singleTreeLeaf`) so it docks, drags,
 * folds and reaches the mobile stack with every other chrome tab rather than inventing a second
 * window system. */
export function createFrameworkTaskManagerPanelTab(renderPanel: () => ReactElement): PanelTabNode {
  return singleTreeLeaf({
    id: FRAMEWORK_TASK_MANAGER_PANEL_ID,
    icon: shellTabIcon("cpu"),
    name: shellLabel("ui.panelToggle.taskManager"),
    order: 2,
    tree: {
      resolveTree: () => ({
        sections: [],
        emptyState: renderPanel(),
        className: "min-h-0 min-w-0 w-full",
        sortableSections: false,
      }),
    },
  });
}
//#endregion MarketplacePanel
//#endregion 🔖️os-chrome-panels
