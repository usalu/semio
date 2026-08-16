// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ChromePanels/component.tsx
/** @emoji 🖼️ `ChromePanels` — the framework-owned settings-panel tree builders for the OS shell's
 * chrome: the Display panel (window-kind palette + named-layout tree), the Settings panel (general/
 * driver/theme/hotkeys tabs), and the Marketplace panel (plugins with their nested extensions), plus the small
 * standalone route-not-found and plugin-recovery affordances they share the chrome namespace with.
 * Each panel's `*HostApi` type is the read/write surface `ShellHost` implements to drive it.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useMemo, useState, useSyncExternalStore, type KeyboardEvent, type ReactNode } from "react";
import {
  App,
  Button,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  Icon,
  type IconName,
  Input,
  type PanelTabNode,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  type UiChromeLayout,
  type UiDriver,
  type UiLabel,
  type UiLocale,
  type UiTheme,
  type UiTranslationKey,
  borderElementClass,
  cn,
  formatKeybindingShortcut,
  humanizeControlId,
  parseKeybindingChords,
  resolveControlLabelId,
  resolveThemeAppearancePalettes,
  singleTreeLeaf,
  uiDataLabel,
  windowTemplatePaletteTreeDragController,
} from "@semio-tech/ui-react";
import { type AppRef, type AppRole, type ArtifactDialect, dialectCoordinate, type NamedLayout, type WindowLayout, createNamedLayout } from "@semio-tech/framework";
import { createWorldProjectionTemplates, encodeWorldProjectionTemplateId, type WorldProjectionTemplateDescriptor } from "@semio-tech/infinite-world-r3f";
import { type PluginPanelStatus, type ResolvedShellLocks } from "../Shell/🟦️component.tsx";
import { driverDisplayLabel, shellLabel, shellTabIcon, shellTerminologyLabel, surfaceRoleChipText } from "../ShellHelpers/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️os-chrome-panels

//#region DisplayPanel
export type DisplayHostApi = {
  readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly iconId: IconName; readonly surfaceKind?: string }[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly saveCurrentLayout: (label: string) => void;
  readonly applyNamedLayout: (layoutId: string) => void;
  readonly deleteUserLayout: (layoutId: string) => void;
  readonly layoutSaveLabel: string;
  readonly setLayoutSaveLabel: (value: string) => void;
};

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
export const FRAMEWORK_SETTINGS_PANEL_ID = "framework.settings";
export const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";
export const FRAMEWORK_SETTINGS_THEME_TAB_ID = "framework.settings.theme";
export const FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID = "framework.settings.keybindings";
/** 👁️✏️ Mirrors Rust `PanelTabKind::SettingsDefaultApps.id_str()` (`🛂️manifest/🦀️component.rs:2607`,
 * contract freeze §1). */
export const FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID = "framework.settings.default-apps";

function groupNamedLayoutsToTreeItems(layouts: readonly NamedLayout[], onApply: (layoutId: string) => void, onDeleteUser?: (layoutId: string) => void): TreeDataItem[] {
  const root: TreeDataItem[] = [];
  const folderByKey = new Map<string, TreeDataItem>();
  const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
    id: `framework.display.layout.${entry.id}`,
    label: entry.label,
    onClick: () => onApply(entry.id),
    ...(entry.origin === "user" && onDeleteUser
      ? {
          actions: [
            {
              id: `framework.display.delete.${entry.id}`,
              icon: <Icon icon="trash-2" size="small" />,
              onClick: () => onDeleteUser(entry.id),
            },
          ],
        }
      : {}),
  });
  for (const entry of layouts) {
    if (!entry.groupPath?.length) {
      root.push(layoutLeaf(entry));
      continue;
    }
    let siblings = root;
    let pathKey = "";
    for (let index = 0; index < entry.groupPath.length; index += 1) {
      const segment = entry.groupPath[index]!;
      pathKey = pathKey ? `${pathKey}/${segment}` : segment;
      let folder = folderByKey.get(pathKey);
      if (!folder) {
        folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
        folderByKey.set(pathKey, folder);
        siblings.push(folder);
      }
      const folderItems = folder.items ?? (folder.items = []);
      if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
      else siblings = folderItems;
    }
  }
  return root;
}

/** @emoji 🪟️ Recursively converts a {@link WorldProjectionTemplateDescriptor} tree (Parallel/Perspective taxonomy)
 * into draggable {@link TreeDataItem}s for a window kind's Display "Windows" section — each node drags a
 * `{windowKindId, templateId}` payload that seeds the freshly-opened pane's initial camera (see
 * {@link registerPendingWorldProjection}/{@link decodeWorldProjectionTemplateId}). Branches keep `items`
 * so Orthographic > Plan > Top/… nesting stays expandable while Plan itself remains a drag target.
 *
 * The Display "Windows" section is always docked at the `bottom-left` panel anchor (see
 * `PanelGroup::Display`'s `anchor()`), so `Tree` always renders it with `direction="up"` — by design it
 * reverses sibling order at *every* level (`framework/ui/js/react/index.tsx`'s `direction === "up" ? [...items].reverse() : items`,
 * exercised by existing tests), so the box can grow upward from its anchor without breaking parent/child
 * nesting. We pre-reverse each level here to cancel that out, so the palette still *reads* top-to-bottom as
 * Plan > Top/Bottom/Front/Back/Left/Right (etc.) instead of backwards. */
function worldProjectionTemplatesToTreeItems(templates: readonly WorldProjectionTemplateDescriptor[], windowKindId: string, idPrefix: string): TreeDataItem[] {
  return [...templates]
    .reverse()
    .map((template) => ({
      id: `${idPrefix}.${template.id}`,
      label: template.label,
      icon: displayWindowKindIcon(template.iconId as IconName),
      defaultOpen: false,
      dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId, templateId: encodeWorldProjectionTemplateId(template.args.spec) }) },
      ...(template.children?.length ? { items: worldProjectionTemplatesToTreeItems(template.children, windowKindId, `${idPrefix}.${template.id}`) } : {}),
    }));
}

function displayWindowKindIcon(iconId: IconName): ReactNode {
  return <Icon icon={iconId} size={12} className="size-tiny shrink-0" />;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
  return {
    dragAndDropController: windowTemplatePaletteTreeDragController(),
    sections: host.windowKinds.length
      ? host.windowKinds.map((kind) => ({
          id: `framework.display.windows.${kind.id}`,
          label: kind.label,
          icon: displayWindowKindIcon(kind.iconId),
          defaultOpen: false,
          // 🔃️ `worldProjectionTemplatesToTreeItems` already returns its items pre-reversed for one "up" render
          // level (see its docstring); the plain kind leaf renders *last* raw so the bottom-anchored Tree's own
          // reversal puts it first — reading top-to-bottom: the plain kind leaf, then Parallel, then Perspective.
          items:
            kind.surfaceKind === "world-3d"
              ? [
                  ...worldProjectionTemplatesToTreeItems(createWorldProjectionTemplates({ controllerId: kind.id }), kind.id, `framework.display.windows.${kind.id}.projection`),
                  { id: `framework.display.windows.${kind.id}.kind`, label: kind.label, icon: displayWindowKindIcon(kind.iconId), dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }) } },
                ]
              : [
                  {
                    id: `framework.display.windows.${kind.id}.kind`,
                    label: kind.label,
                    icon: displayWindowKindIcon(kind.iconId),
                    dragData: {
                      [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }),
                    },
                  },
                ],
        }))
      : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
  const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
  const userLayouts = host.userLayouts;
  const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
  const userItems = userLayouts.length
    ? [
        {
          id: "framework.display.layout.group.saved",
          label: shellLabel("ui.display.saved"),
          defaultOpen: false,
          items: groupNamedLayoutsToTreeItems(
            userLayouts,
            (layoutId) => host.applyNamedLayout(layoutId),
            (layoutId) => host.deleteUserLayout(layoutId),
          ),
        },
      ]
    : [];
  return {
    sections: [
      {
        id: "framework.display.layout.save",
        label: shellLabel("ui.display.saveLayout"),
        defaultOpen: false,
        items: [
          {
            id: "framework.display.layout.save.label",
            label: shellLabel("ui.common.name"),
            control: <Input id="framework.display.saveLabel" value={host.layoutSaveLabel} onChange={(event) => host.setLayoutSaveLabel(event.target.value)} placeholder={shellLabel("ui.display.saveLayoutPlaceholder")} />,
          },
          {
            id: "framework.display.layout.save.action",
            label: shellLabel("ui.common.save"),
            control: (
              <Button
                id="framework.display.save"
                size="sm"
                text={shellLabel("ui.display.saveCurrentLayout")}
                disabled={!host.layoutSaveLabel.trim()}
                onClick={() => {
                  const label = host.layoutSaveLabel.trim();
                  if (!label) return;
                  host.saveCurrentLayout(label);
                  host.setLayoutSaveLabel("");
                }}
              />
            ),
          },
        ],
      },
      {
        id: "framework.display.layout.list",
        label: shellLabel("ui.display.layouts"),
        defaultOpen: true,
        items: [...builtinItems, ...userItems],
      },
    ],
  };
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
      icon: shellTabIcon("framework.display.windows"),
      name: shellLabel("ui.display.tab.windows"),
      order: -100,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
      icon: shellTabIcon("framework.display.layout"),
      name: shellLabel("ui.display.tab.layout"),
      order: -99,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
  ];
}
//#endregion DisplayPanel

//#region SettingsPanel
export type SettingsHostApi = {
  readonly appId?: string;
  readonly appLabel?: string;
  readonly controllerId?: string;
  readonly pluginId?: string;
  readonly driverId: string;
  readonly driver: UiDriver;
  readonly driverDirty: boolean;
  readonly drivers: readonly UiDriver[];
  readonly setDriverId: (id: string) => void;
  readonly setDriverField: <K extends keyof Omit<UiDriver, "id" | "label">>(key: K, value: UiDriver[K]) => void;
  readonly saveDriver: (label: string) => void;
  readonly deleteDriver: (id: string) => void;
  readonly driverSaveLabel: string;
  readonly setDriverSaveLabel: (value: string) => void;
  readonly appearance: string;
  readonly setAppearance: (appearance: string) => void;
  readonly layout: UiChromeLayout;
  readonly setLayout: (layout: UiChromeLayout) => void;
  readonly mobileActive: boolean;
  /** 🧭️ Clears the persisted corner-panel arrangement and folds every corner's active path back to its default — undefined when a shell doesn't wire up dock persistence. */
  readonly onResetDock?: () => void;
  readonly locale: UiLocale;
  readonly setLocale: (locale: UiLocale) => void;
  readonly terminology: string;
  readonly setTerminology: (id: string) => void;
  readonly terminologies: readonly string[];
  readonly theme: UiTheme;
  readonly themeId: string;
  readonly themeDirty: boolean;
  readonly themes: readonly UiTheme[];
  readonly setThemeId: (id: string) => void;
  readonly setThemeColor: (key: string, hex: string) => void;
  readonly setThemeSpacing: (key: string, value: string) => void;
  readonly setThemeFontStack: (key: string, value: string) => void;
  readonly setThemeStroke: (key: string, value: number | number[]) => void;
  readonly setThemeRadius: (key: string, value: number) => void;
  readonly setThemeOpacity: (key: string, value: number) => void;
  readonly setThemeMetric: (section: string, key: string, value: number | number[]) => void;
  readonly setThemeAppearancePaint: (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) => void;
  readonly saveTheme: (label: string) => void;
  readonly deleteTheme: (id: string) => void;
  readonly resetTheme: () => void;
  readonly exportTheme: () => void;
  readonly importTheme: () => void;
  readonly themeSaveLabel: string;
  readonly setThemeSaveLabel: (value: string) => void;
  readonly controlKeybindings: ReadonlyMap<string, string>;
  readonly keybindingCaptureControlId: string | null;
  readonly setKeybindingCaptureControlId: (id: string | null) => void;
  readonly setKeybindingOverride: (controlId: string, keys: string) => void;
  readonly resetKeybindingOverride: (controlId: string) => void;
  readonly locks: ResolvedShellLocks;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
  return {
    sections: [
      ...(host.appId || host.appLabel || host.controllerId || host.pluginId
        ? [
            {
              id: "framework.settings.app",
              label: shellLabel("ui.settings.tab.app"),
              defaultOpen: true,
              items: [
                ...(host.appLabel ? [{ id: "framework.settings.app.label", label: `${shellLabel("ui.settings.app.name")}: ${host.appLabel}` }] : []),
                ...(host.appId ? [{ id: "framework.settings.app.id", label: `${shellLabel("ui.settings.app.id")}: ${host.appId}` }] : []),
                ...(host.controllerId ? [{ id: "framework.settings.app.controller", label: `${shellLabel("ui.settings.app.controller")}: ${host.controllerId}` }] : []),
                ...(host.pluginId ? [{ id: "framework.settings.app.plugin", label: `${shellLabel("ui.settings.app.plugin")}: ${host.pluginId}` }] : []),
              ],
            },
          ]
        : []),
      {
        id: "framework.settings.general",
        label: shellLabel("ui.settings.tab.general"),
        defaultOpen: true,
        items: [
          ...(host.locks.appearance
            ? []
            : [
                {
                  id: "framework.settings.appearance",
                  label: shellLabel("ui.settings.tab.appearance"),
                  control: (
                    <Select value={host.appearance} onValueChange={(value) => host.setAppearance(value)}>
                      <SelectTrigger id="framework.settings.appearance" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="system">{shellLabel("ui.settings.appearance.system")}</SelectItem>
                        <SelectItem value="light">{shellLabel("ui.settings.appearance.light")}</SelectItem>
                        <SelectItem value="dark">{shellLabel("ui.settings.appearance.dark")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          {
            id: "framework.settings.layout",
            label: shellLabel("ui.settings.tab.layout"),
            control: host.mobileActive ? (
              <span className="text-sm text-muted-foreground">{shellLabel("settings.layout.mobile")}</span>
            ) : (
              <Select value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
                <SelectTrigger id="framework.settings.layout" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="desktop">{shellLabel("settings.layout.desktop")}</SelectItem>
                  <SelectItem value="tablet">{shellLabel("settings.layout.tablet")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.driver",
            label: shellLabel("ui.settings.tab.driver"),
            control: (
              <Select value={host.driverId} onValueChange={(value) => host.setDriverId(value)}>
                <SelectTrigger id="framework.settings.driver" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.drivers.map((driver) => (
                    <SelectItem key={driver.id} value={driver.id}>
                      {driverDisplayLabel(driver)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          ...(host.locks.locale
            ? []
            : [
                {
                  id: "framework.settings.language",
                  label: shellLabel("ui.settings.tab.language"),
                  control: (
                    <Select value={host.locale} onValueChange={(value) => host.setLocale(value === "de" ? "de" : "en")}>
                      <SelectTrigger id="framework.settings.language" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="en">{shellLabel("ui.settings.language.en")}</SelectItem>
                        <SelectItem value="de">{shellLabel("ui.settings.language.de")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.locks.terminology
            ? []
            : [
                {
                  id: "framework.settings.terminology",
                  label: shellLabel("ui.settings.tab.terminology"),
                  control: (
                    <Select value={host.terminology} onValueChange={(value) => host.setTerminology(value)}>
                      <SelectTrigger id="framework.settings.terminology" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {host.terminologies.map((id) => (
                          <SelectItem key={id} value={id}>
                            {shellTerminologyLabel(id)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.onResetDock
            ? [
                {
                  id: "framework.settings.resetDock.action",
                  label: shellLabel("ui.settings.resetDock"),
                  control: <Button id="framework.settings.resetDock" size="sm" icon="rotate-ccw" text={shellLabel("ui.settings.resetDock")} onClick={() => host.onResetDock?.()} />,
                },
              ]
            : []),
        ],
      },
      {
        id: "framework.settings.driver.editor",
        label: `${shellLabel("ui.settings.tab.driver")}${host.driverDirty ? ` (${shellLabel("settings.driver.dirty")})` : ""}`,
        defaultOpen: false,
        items: [
          driverAxisSelectRow("labels", shellLabel("settings.driver.labels"), host.driver.labels, [
            { value: "full", label: shellLabel("settings.driver.labelsOption.full") },
            { value: "icons", label: shellLabel("settings.driver.labelsOption.icons") },
          ], (next) => host.setDriverField("labels", next as UiDriver["labels"])),
          driverAxisSelectRow("labelTier", shellLabel("settings.driver.labelTier"), host.driver.labelTier, [
            { value: "beginner", label: shellLabel("settings.driver.labelTierOption.beginner") },
            { value: "normal", label: shellLabel("settings.driver.labelTierOption.normal") },
          ], (next) => host.setDriverField("labelTier", next as UiDriver["labelTier"])),
          driverAxisSelectRow("drag", shellLabel("settings.driver.drag"), host.driver.drag, [
            { value: "handle", label: shellLabel("settings.driver.dragOption.handle") },
            { value: "surface", label: shellLabel("settings.driver.dragOption.surface") },
          ], (next) => host.setDriverField("drag", next as UiDriver["drag"])),
          driverAxisSelectRow("chrome", shellLabel("settings.driver.chrome"), host.driver.chrome, [
            { value: "always", label: shellLabel("settings.driver.chromeOption.always") },
            { value: "hover", label: shellLabel("settings.driver.chromeOption.hover") },
          ], (next) => host.setDriverField("chrome", next as UiDriver["chrome"])),
          driverAxisSelectRow("gumball", shellLabel("settings.driver.gumball"), host.driver.gumball, [
            { value: "always", label: shellLabel("settings.driver.gumballOption.always") },
            { value: "hover", label: shellLabel("settings.driver.gumballOption.hover") },
          ], (next) => host.setDriverField("gumball", next as UiDriver["gumball"])),
          driverAxisSelectRow("tooltips", shellLabel("settings.driver.tooltips"), host.driver.tooltips, [
            { value: "full", label: shellLabel("settings.driver.tooltipsOption.full") },
            { value: "minimal", label: shellLabel("settings.driver.tooltipsOption.minimal") },
            { value: "none", label: shellLabel("settings.driver.tooltipsOption.none") },
          ], (next) => host.setDriverField("tooltips", next as UiDriver["tooltips"])),
          driverAxisSelectRow("hotkeys", shellLabel("settings.driver.hotkeys"), host.driver.hotkeys, [
            { value: "inline", label: shellLabel("settings.driver.hotkeysOption.inline") },
            { value: "tooltip", label: shellLabel("settings.driver.hotkeysOption.tooltip") },
            { value: "none", label: shellLabel("settings.driver.hotkeysOption.none") },
          ], (next) => host.setDriverField("hotkeys", next as UiDriver["hotkeys"])),
          {
            id: "framework.settings.driver.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input
                id="framework.settings.driver.saveLabel"
                value={host.driverSaveLabel}
                onChange={(event) => host.setDriverSaveLabel(event.target.value)}
                placeholder={shellLabel("settings.driver.savePlaceholder")}
                className="h-small w-32"
              />
            ),
          },
          {
            id: "framework.settings.driver.save.action",
            label: shellLabel("settings.driver.save"),
            control: (
              <Button
                id="framework.settings.driver.save"
                size="sm"
                text={shellLabel("settings.driver.save")}
                disabled={!host.driverSaveLabel.trim()}
                onClick={() => {
                  const label = host.driverSaveLabel.trim();
                  if (!label) return;
                  host.saveDriver(label);
                  host.setDriverSaveLabel("");
                }}
              />
            ),
          },
          ...(host.driverId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.driver.delete.action",
                  label: shellLabel("settings.driver.delete"),
                  control: <Button id="framework.settings.driver.delete" size="sm" text={shellLabel("settings.driver.delete")} onClick={() => host.deleteDriver(host.driverId)} />,
                },
              ]
            : []),
        ],
      },
    ],
  };
}

function driverAxisSelectRow<K extends keyof Omit<UiDriver, "id" | "label">>(
  key: K,
  label: string,
  value: UiDriver[K],
  options: readonly { readonly value: string; readonly label: string }[],
  onChange: (next: string) => void,
): TreeDataItem {
  return {
    id: `framework.settings.driver.${key}`,
    label,
    control: (
      <Select value={value as string} onValueChange={onChange}>
        <SelectTrigger id={`framework.settings.driver.${key}`} className="h-small w-32" size="sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    ),
  };
}

function rgba8ToHex(rgba: readonly [number, number, number, number]): string {
  const [r, g, b] = rgba;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

function themeColorInputRow(id: string, label: string, hex: string, onChange: (hex: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <input id={id} type="color" className={cn(borderElementClass, "h-small w-16 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => onChange(event.target.value)} />,
  };
}

function themeTextInputRow(id: string, label: string, value: string, onCommit: (value: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <Input id={id} defaultValue={value} onBlur={(event) => onCommit(event.target.value)} className="h-small w-32" />,
  };
}

function themeNumberInputRow(id: string, label: string, value: number | number[], onCommit: (value: number | number[]) => void): TreeDataItem {
  const text = Array.isArray(value) ? value.join(", ") : String(value);
  return {
    id,
    label,
    control: (
      <Input
        id={id}
        defaultValue={text}
        onBlur={(event) => {
          const raw = event.target.value.trim();
          if (raw.includes(",")) {
            const parts = raw
              .split(",")
              .map((part) => Number.parseFloat(part.trim()))
              .filter((n) => !Number.isNaN(n));
            if (parts.length) onCommit(parts);
            return;
          }
          const n = Number.parseFloat(raw);
          if (!Number.isNaN(n)) onCommit(n);
        }}
        className="h-small w-32"
      />
    ),
  };
}

const THEME_PALETTE_GROUP_LABEL_KEYS = {
  board: "ui.settings.theme.group.board",
  map: "ui.settings.theme.group.map",
  canvas: "ui.settings.theme.group.canvas",
  chrome: "ui.settings.theme.group.chrome",
} as const satisfies Record<ThemePaletteGroup, UiTranslationKey>;

function buildThemeAppearanceGroupItems(host: SettingsHostApi, appearance: ThemeAppearanceName, group: ThemePaletteGroup): TreeDataItem[] {
  const refs = host.theme.appearances[appearance][group];
  const resolved = resolveThemeAppearancePalettes(host.theme, appearance)[group];
  return Object.keys(refs)
    .sort()
    .map((paintKey) => {
      const rgba = resolved[paintKey] ?? [0, 0, 0, 255];
      const hex = rgba8ToHex(rgba);
      const alpha = rgba[3] / 255;
      return {
        id: `framework.settings.theme.appearances.${appearance}.${group}.${paintKey}`,
        label: paintKey,
        control: (
          <div className="flex w-full items-center gap-single">
            <input type="color" className={cn(borderElementClass, "h-small w-10 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => host.setThemeAppearancePaint(appearance, group, paintKey, event.target.value, alpha)} />
            <Input
              id={`framework.settings.theme.appearances.${appearance}.${group}.${paintKey}.alpha`}
              defaultValue={alpha.toFixed(2)}
              onBlur={(event) => {
                const nextAlpha = Number.parseFloat(event.target.value);
                if (!Number.isNaN(nextAlpha)) host.setThemeAppearancePaint(appearance, group, paintKey, hex, Math.min(1, Math.max(0, nextAlpha)));
              }}
              className="h-small w-14 shrink-0"
            />
          </div>
        ),
      } satisfies TreeDataItem;
    });
}

function buildSettingsThemeTree(host: SettingsHostApi): TreePanelConfig {
  const colorItems = Object.keys(host.theme.colors)
    .sort()
    .map((key) => themeColorInputRow(`framework.settings.theme.colors.${key}`, key, host.theme.colors[key]!, (hex) => host.setThemeColor(key, hex)));

  const spacingItems = Object.keys(host.theme.spacing)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.spacing.${key}`, key, host.theme.spacing[key]!, (value) => host.setThemeSpacing(key, value)));

  const fontItems = Object.keys(host.theme.fontStacks)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.fonts.${key}`, key, host.theme.fontStacks[key]!, (value) => host.setThemeFontStack(key, value)));

  const strokeItems = Object.keys(host.theme.strokes)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.strokes.${key}`, key, host.theme.strokes[key]!, (value) => host.setThemeStroke(key, value)));

  const radiusItems = Object.keys(host.theme.radii)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.radii.${key}`, key, host.theme.radii[key]!, (value) => host.setThemeRadius(key, typeof value === "number" ? value : value[0]!)));

  const opacityItems = Object.keys(host.theme.opacities)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.opacities.${key}`, key, host.theme.opacities[key]!, (value) => host.setThemeOpacity(key, typeof value === "number" ? value : value[0]!)));

  const metricSections = Object.keys(host.theme.metrics)
    .sort()
    .map(
      (section): TreeDataItem => ({
        id: `framework.settings.theme.metrics.${section}`,
        label: section,
        defaultOpen: false,
        items: Object.keys(host.theme.metrics[section]!)
          .sort()
          .map((key) => themeNumberInputRow(`framework.settings.theme.metrics.${section}.${key}`, key, host.theme.metrics[section]![key]!, (value) => host.setThemeMetric(section, key, value))),
      }),
    );

  const appearanceGroups: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
  const appearanceItems: TreeDataItem[] = (["light", "dark"] as const).map((appearance) => ({
    id: `framework.settings.theme.appearances.${appearance}`,
    label: shellLabel(appearance === "light" ? "ui.settings.theme.appearance.light" : "ui.settings.theme.appearance.dark"),
    defaultOpen: false,
    items: appearanceGroups.map((group) => ({
      id: `framework.settings.theme.appearances.${appearance}.${group}`,
      label: shellLabel(THEME_PALETTE_GROUP_LABEL_KEYS[group]),
      defaultOpen: false,
      items: buildThemeAppearanceGroupItems(host, appearance, group),
    })),
  }));

  return {
    sections: [
      {
        id: "framework.settings.theme.select",
        label: `${shellLabel("ui.settings.theme.select")}${host.themeDirty ? ` (${shellLabel("ui.settings.theme.dirty")})` : ""}`,
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: shellLabel("ui.settings.theme.select"),
            control: (
              <Select value={host.themeId} onValueChange={(value) => host.setThemeId(value)}>
                <SelectTrigger id="framework.settings.theme.select" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.themes.map((theme) => (
                    <SelectItem key={theme.id} value={theme.id}>
                      {theme.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input id="framework.settings.theme.saveLabel" value={host.themeSaveLabel} onChange={(event) => host.setThemeSaveLabel(event.target.value)} placeholder={shellLabel("ui.settings.theme.savePlaceholder")} className="h-small w-32" />
            ),
          },
          {
            id: "framework.settings.theme.save.action",
            label: shellLabel("ui.settings.theme.save"),
            control: (
              <Button
                id="framework.settings.theme.save"
                size="sm"
                text={shellLabel("ui.settings.theme.save")}
                disabled={!host.themeSaveLabel.trim()}
                onClick={() => {
                  const label = host.themeSaveLabel.trim();
                  if (!label) return;
                  host.saveTheme(label);
                  host.setThemeSaveLabel("");
                }}
              />
            ),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: shellLabel("ui.settings.theme.reset"),
            control: <Button id="framework.settings.theme.reset" size="sm" text={shellLabel("ui.settings.theme.reset")} disabled={!host.themeDirty && host.themeId === "semio"} onClick={() => host.resetTheme()} />,
          },
          {
            id: "framework.settings.theme.export.action",
            label: shellLabel("ui.settings.theme.export"),
            control: <Button id="framework.settings.theme.export" size="sm" text={shellLabel("ui.settings.theme.export")} onClick={() => host.exportTheme()} />,
          },
          {
            id: "framework.settings.theme.import.action",
            label: shellLabel("ui.settings.theme.import"),
            control: <Button id="framework.settings.theme.import" size="sm" text={shellLabel("ui.settings.theme.import")} onClick={() => host.importTheme()} />,
          },
          ...(host.themeId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.theme.delete.action",
                  label: shellLabel("ui.settings.theme.delete"),
                  control: <Button id="framework.settings.theme.delete" size="sm" text={shellLabel("ui.settings.theme.delete")} onClick={() => host.deleteTheme(host.themeId)} />,
                },
              ]
            : []),
        ],
      },
      { id: "framework.settings.theme.colors", label: shellLabel("ui.settings.theme.colors"), defaultOpen: false, items: colorItems },
      { id: "framework.settings.theme.spacing", label: shellLabel("ui.settings.theme.spacing"), defaultOpen: false, items: spacingItems },
      { id: "framework.settings.theme.fonts", label: shellLabel("ui.settings.theme.fonts"), defaultOpen: false, items: fontItems },
      { id: "framework.settings.theme.strokes", label: shellLabel("ui.settings.theme.strokes"), defaultOpen: false, items: strokeItems },
      { id: "framework.settings.theme.radii", label: shellLabel("ui.settings.theme.radii"), defaultOpen: false, items: radiusItems },
      { id: "framework.settings.theme.opacities", label: shellLabel("ui.settings.theme.opacities"), defaultOpen: false, items: opacityItems },
      { id: "framework.settings.theme.metrics", label: shellLabel("ui.settings.theme.metrics"), defaultOpen: false, items: metricSections },
      { id: "framework.settings.theme.appearances", label: shellLabel("ui.settings.theme.appearances"), defaultOpen: false, items: appearanceItems },
    ],
  };
}

function chordFromKeyboardEvent(event: KeyboardEvent): string | null {
  if (event.key === "Escape") return null;
  const parts: string[] = [];
  if (event.ctrlKey) parts.push("ctrl");
  if (event.metaKey) parts.push("meta");
  if (event.altKey) parts.push("alt");
  if (event.shiftKey) parts.push("shift");
  const key = event.key.length === 1 ? event.key.toLowerCase() : event.key.toLowerCase();
  if (key === "control" || key === "meta" || key === "alt" || key === "shift") return null;
  parts.push(key === " " ? "space" : key);
  return parts.join("+");
}

function buildSettingsKeybindingsTree(host: SettingsHostApi): TreePanelConfig {
  const chordOwners = new Map<string, string>();
  for (const [controlId, keys] of host.controlKeybindings) {
    const chord = parseKeybindingChords(keys)[0];
    if (chord) chordOwners.set(chord, controlId);
  }
  const rows = [...host.controlKeybindings.entries()].sort(([left], [right]) => left.localeCompare(right));
  return {
    sections: [
      {
        id: "framework.settings.keybindings.list",
        label: shellLabel("ui.settings.tab.keybindings"),
        defaultOpen: true,
        items: rows.map(([controlId, keys]) => {
          const chord = parseKeybindingChords(keys)[0];
          const conflict = chord ? chordOwners.get(chord) !== controlId : false;
          const labelId = resolveControlLabelId(controlId);
          const label = uiDataLabel(humanizeControlId(labelId));
          const capturing = host.keybindingCaptureControlId === controlId;
          return {
            id: `framework.settings.keybindings.${controlId}`,
            label: conflict ? `${label} (${shellLabel("settings.keybindings.conflict")})` : label,
            control: (
              <div className="flex items-center gap-single">
                <span className="min-w-[5rem] font-mono text-xs text-muted-foreground">{formatKeybindingShortcut(keys)}</span>
                <Button
                  id={`framework.settings.keybindings.capture.${controlId}`}
                  size="sm"
                  icon="keyboard"
                  text={capturing ? shellLabel("settings.keybindings.pressKeys") : shellLabel("settings.keybindings.capture")}
                  data-keybinding-capture={controlId}
                  onClick={() => host.setKeybindingCaptureControlId(capturing ? null : controlId)}
                  onKeyDown={(event) => {
                    if (!capturing) return;
                    event.preventDefault();
                    event.stopPropagation();
                    const chord = chordFromKeyboardEvent(event.nativeEvent);
                    if (!chord) {
                      host.setKeybindingCaptureControlId(null);
                      return;
                    }
                    host.setKeybindingOverride(controlId, chord);
                    host.setKeybindingCaptureControlId(null);
                  }}
                />
                <Button id={`framework.settings.keybindings.reset.${controlId}`} size="sm" variant="ghost" icon="rotate-ccw" text={shellLabel("settings.keybindings.reset")} onClick={() => host.resetKeybindingOverride(controlId)} />
              </div>
            ),
          };
        }),
      },
    ],
  };
}

//#region 🔖️DefaultAppsPanel
/** 👁️✏️ One `AppRef` option in a default-apps `Select` — `"none"` clears the pin. */
const DEFAULT_APP_NONE_VALUE = "none";

function encodeDefaultAppValue(app: AppRef): string {
  return `${app.pluginId} ${app.appId}`;
}

function decodeDefaultAppValue(value: string): AppRef | null {
  if (value === DEFAULT_APP_NONE_VALUE) return null;
  const separator = value.indexOf(" ");
  if (separator < 0) return null;
  return { pluginId: value.slice(0, separator), appId: value.slice(separator + 1) };
}

/** 👁️✏️ One dialect's viewer/editor `AppRouter` options for the `SettingsDefaultApps` table — built by
 * `ShellHost` from `AppRouter.entriesFor` + `OpeningPreferences`, one row per role per dialect. */
export type DefaultAppRow = {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly options: readonly { readonly value: string; readonly label: string }[];
  readonly value: string;
};

export type DefaultAppsHostApi = {
  readonly rows: readonly DefaultAppRow[];
  readonly locale: string;
  readonly setDefault: (dialect: ArtifactDialect, role: AppRole, app: AppRef) => void;
  readonly clearDefault: (dialect: ArtifactDialect, role: AppRole) => void;
};

function buildSettingsDefaultAppsTree(host: DefaultAppsHostApi): TreePanelConfig {
  if (host.rows.length === 0) {
    return { sections: [{ id: "framework.settings.defaultApps.empty", items: [{ id: "framework.settings.defaultApps.empty.row", label: shellLabel("ui.settings.unavailable") }] }] };
  }
  return {
    sections: [
      {
        id: "framework.settings.defaultApps.table",
        label: defaultAppsSettingsTabText(host.locale),
        defaultOpen: true,
        items: host.rows.map((row) => {
          const rowId = `framework.settings.defaultApps.${dialectCoordinate(row.dialect)}.${row.role}`;
          return {
            id: rowId,
            label: `${dialectCoordinate(row.dialect)} — ${surfaceRoleChipText(row.role, host.locale)}`,
            control: (
              <Select
                value={row.value}
                onValueChange={(value) => {
                  if (value === DEFAULT_APP_NONE_VALUE) {
                    host.clearDefault(row.dialect, row.role);
                    return;
                  }
                  const app = decodeDefaultAppValue(value);
                  if (app) host.setDefault(row.dialect, row.role, app);
                }}
              >
                <SelectTrigger id={rowId} className="h-small w-48" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={DEFAULT_APP_NONE_VALUE}>{shellLabel("ui.common.none")}</SelectItem>
                  {row.options.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          };
        }),
      },
    ],
  };
}
//#endregion 🔖️DefaultAppsPanel

export function createFrameworkSettingsPanelTab(getHost: () => SettingsHostApi | null, getDefaultAppsHost?: () => DefaultAppsHostApi | null): PanelTabNode {
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

export function useNamedLayoutHost(options: {
  readonly appId: string;
  readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly iconId: IconName; readonly surfaceKind?: string }[];
  readonly builtinLayouts: readonly NamedLayout[];
  readonly currentLayout: WindowLayout | undefined;
  readonly onApplyLayout: (layout: WindowLayout) => void;
  readonly namedLayoutStore: { getSnapshot: () => readonly NamedLayout[]; save: (layout: NamedLayout) => void; remove: (layoutId: string) => void; subscribe: (listener: () => void) => () => void };
}): DisplayHostApi {
  const userLayouts = useSyncExternalStore(
    (listener) => options.namedLayoutStore.subscribe(listener),
    () => options.namedLayoutStore.getSnapshot(),
    () => options.namedLayoutStore.getSnapshot(),
  );
  const [layoutSaveLabel, setLayoutSaveLabel] = useState("");
  return useMemo(
    (): DisplayHostApi => ({
      windowKinds: options.windowKinds,
      namedLayouts: options.builtinLayouts,
      userLayouts,
      saveCurrentLayout: (label) => {
        if (!options.currentLayout) return;
        const id = `user-${Date.now()}`;
        options.namedLayoutStore.save(createNamedLayout(id, label, options.currentLayout, "user"));
      },
      applyNamedLayout: (layoutId) => {
        const layout = [...options.builtinLayouts, ...userLayouts].find((entry) => entry.id === layoutId);
        if (layout) options.onApplyLayout(layout.layout);
      },
      deleteUserLayout: (layoutId) => options.namedLayoutStore.remove(layoutId),
      layoutSaveLabel,
      setLayoutSaveLabel,
    }),
    [options, userLayouts, layoutSaveLabel],
  );
}
//#endregion SettingsPanel

//#region MarketplacePanel
/** @emoji 🧭️ Canvas fallback when studio history resolves to an unknown route. */
export function ShellRouteNotFoundPage({ path, onHome }: { readonly path: string; readonly onHome: () => void }) {
  return (
    <div className="flex h-full min-h-0 flex-col items-center justify-center gap-double p-double" role="alert" data-shell-route-not-found={path}>
      <p className="text-sm text-muted-foreground">{uiDataLabel(`Route not found: ${path}`)}</p>
      <Button size="sm" text={shellLabel("ui.common.home")} onClick={onHome} />
    </div>
  );
}

/** @emoji 🩺️ Plugin crash/quarantine recovery affordances — mirrors `ui_wgpu::ui_recovery_panel`. */
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
  const message = quarantined ? uiDataLabel("This program was quarantined after repeated crashes.") : uiDataLabel("This program crashed.");
  return (
    <div className="flex h-full min-h-0 flex-col gap-double p-double" data-plugin-recovery={pluginId}>
      <p className="text-sm font-medium">{uiDataLabel("Plugin Recovery")}</p>
      <p className="text-sm text-muted-foreground">{message}</p>
      <div className="flex flex-wrap gap-single">
        <Button size="sm" text={uiDataLabel("Restart App")} onClick={onRestart} />
        <Button size="sm" text={uiDataLabel("Disable Plugin")} onClick={onDisable} />
      </div>
    </div>
  );
}

/** 🔌️ One registry entry as the marketplace wants to render it — `sourceId` and `canUninstall` come
 * straight from the shell's `PluginSource`/primary-plugin bookkeeping; `label`/`version` fall back to
 * the bare pluginId when a plugin hasn't loaded far enough to have a manifest yet (`"available"`). */
export type MarketplacePluginEntry = {
  readonly pluginId: string;
  readonly label: string;
  readonly version?: string;
  readonly status: PluginPanelStatus;
  readonly sourceId: string;
  readonly canUninstall: boolean;
};

/** 🧩️ One extension nested beneath the plugin identified by `extendsHost`. */
export type MarketplaceExtensionEntry = {
  readonly extensionId: string;
  readonly label: string;
  readonly version?: string;
  readonly extendsHost: string;
  readonly enabled: boolean;
  readonly status: PluginPanelStatus;
};

export type MarketplaceHostApi = {
  readonly plugins: readonly MarketplacePluginEntry[];
  readonly extensions: readonly MarketplaceExtensionEntry[];
  readonly installPlugin: (pluginId: string) => void;
  readonly uninstallPlugin: (pluginId: string) => void;
  readonly reloadPlugin: (pluginId: string) => void;
  readonly installExtensionFromUrl: (sourceUri: string) => void;
  readonly installExtensionFromFile: (file: File) => void;
  readonly uninstallExtension: (extensionId: string) => void;
  readonly setExtensionEnabled: (extensionId: string, enabled: boolean) => void;
};

export const FRAMEWORK_MARKETPLACE_TAB_ID = "framework.marketplace";

function pluginStatusLabel(status: PluginPanelStatus): UiLabel {
  if (status === "available") return shellLabel("ui.plugins.status.available");
  if (status === "installing") return shellLabel("ui.plugins.status.installing");
  if (status === "loaded") return shellLabel("ui.plugins.status.loaded");
  if (status === "failed") return shellLabel("ui.plugins.status.failed");
  return shellLabel("ui.plugins.status.reloading");
}

function marketplaceExtensionItem(entry: MarketplaceExtensionEntry, host: MarketplaceHostApi): TreeDataItem {
  return {
    id: `framework.marketplace.plugin.${entry.extendsHost}.extension.${entry.extensionId}`,
    label: `${entry.label}${entry.version ? ` · ${entry.version}` : ""} · ${entry.enabled ? uiDataLabel("enabled") : uiDataLabel("disabled")}`,
    loading: entry.status === "installing" || entry.status === "reloading",
    control: (
      <div className="flex items-center gap-1">
        <Button
          id={`framework.marketplace.extension.${entry.extensionId}.enable`}
          size="sm"
          text={entry.enabled ? uiDataLabel("Disable") : uiDataLabel("Enable")}
          disabled={entry.status !== "loaded" && entry.status !== "available"}
          onClick={() => host.setExtensionEnabled(entry.extensionId, !entry.enabled)}
        />
        <Button
          id={`framework.marketplace.extension.${entry.extensionId}.uninstall`}
          size="sm"
          text={uiDataLabel("Uninstall")}
          disabled={entry.status === "installing" || entry.status === "reloading"}
          onClick={() => host.uninstallExtension(entry.extensionId)}
        />
      </div>
    ),
  };
}

function marketplacePluginItem(entry: MarketplacePluginEntry, extensions: readonly MarketplaceExtensionEntry[], host: MarketplaceHostApi): TreeDataItem {
  return {
    id: `framework.marketplace.plugin.${entry.pluginId}`,
    label: `${entry.label}${entry.version ? ` · ${entry.version}` : ""} · ${pluginStatusLabel(entry.status)}`,
    loading: entry.status === "installing" || entry.status === "reloading",
    defaultOpen: extensions.length > 0,
    ...(extensions.length > 0 ? { items: extensions.map((extension) => marketplaceExtensionItem(extension, host)) } : {}),
    control:
      entry.status === "available" || entry.status === "failed" ? (
        <Button id={`framework.marketplace.plugin.${entry.pluginId}.install`} size="sm" text={shellLabel("ui.plugins.action.install")} onClick={() => host.installPlugin(entry.pluginId)} />
      ) : (
        <div className="flex items-center gap-1">
          <Button
            id={`framework.marketplace.plugin.${entry.pluginId}.reload`}
            size="sm"
            text={shellLabel("ui.plugins.action.reload")}
            disabled={entry.status !== "loaded"}
            onClick={() => host.reloadPlugin(entry.pluginId)}
          />
          <Button
            id={`framework.marketplace.plugin.${entry.pluginId}.uninstall`}
            size="sm"
            text={shellLabel("ui.plugins.action.uninstall")}
            disabled={!entry.canUninstall || entry.status !== "loaded"}
            onClick={() => host.uninstallPlugin(entry.pluginId)}
          />
        </div>
      ),
  };
}

function buildMarketplaceTree(host: MarketplaceHostApi): TreePanelConfig {
  const extensionsByHost = new Map<string, MarketplaceExtensionEntry[]>();
  for (const entry of host.extensions) {
    const list = extensionsByHost.get(entry.extendsHost) ?? [];
    list.push(entry);
    extensionsByHost.set(entry.extendsHost, list);
  }
  for (const entries of extensionsByHost.values()) entries.sort((left, right) => left.extensionId.localeCompare(right.extensionId));

  const bySource = new Map<string, MarketplacePluginEntry[]>();
  for (const entry of host.plugins) {
    const list = bySource.get(entry.sourceId) ?? [];
    list.push(entry);
    bySource.set(entry.sourceId, list);
  }
  const listedPluginIds = new Set(host.plugins.map((entry) => entry.pluginId));
  const sections: TreeDataSection[] = [
    {
      id: "framework.marketplace.extensions.install",
      label: uiDataLabel("Install extension"),
      defaultOpen: true,
      items: [
        {
          id: "framework.marketplace.extensions.install.url",
          label: uiDataLabel("From URL"),
          control: (
            <Button
              id="framework.marketplace.extensions.install.url"
              size="sm"
              text={uiDataLabel("Install from URL")}
              onClick={() => {
                const sourceUri = typeof window !== "undefined" ? window.prompt("Extension package URL") : null;
                if (sourceUri?.trim()) host.installExtensionFromUrl(sourceUri.trim());
              }}
            />
          ),
        },
        {
          id: "framework.marketplace.extensions.install.file",
          label: uiDataLabel("From file"),
          control: (
            <label className="inline-flex cursor-pointer">
              <input
                id="framework.marketplace.extensions.install.file"
                type="file"
                accept=".sxt,.semio,application/octet-stream"
                className="sr-only"
                onChange={(event) => {
                  const file = event.target.files?.[0];
                  if (file) host.installExtensionFromFile(file);
                  event.target.value = "";
                }}
              />
              <Button id="framework.marketplace.extensions.install.file.trigger" size="sm" text={uiDataLabel("Install from file")} />
            </label>
          ),
        },
      ],
    },
    ...[...bySource.entries()]
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([sourceId, entries]) => ({
        id: `framework.marketplace.source.${sourceId}`,
        label: `${shellLabel("ui.plugins.source")}: ${sourceId}`,
        defaultOpen: true,
        items: [...entries]
          .sort((left, right) => left.pluginId.localeCompare(right.pluginId))
          .map((entry) => marketplacePluginItem(entry, extensionsByHost.get(entry.pluginId) ?? [], host)),
      })),
    ...[...extensionsByHost.entries()]
      .filter(([hostId]) => !listedPluginIds.has(hostId))
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([hostId, extensions]) => ({
        id: `framework.marketplace.missing-host.${hostId}`,
        label: hostId,
        defaultOpen: true,
        items: extensions.map((extension) => marketplaceExtensionItem(extension, host)),
      })),
  ];
  return { sections };
}

export function createFrameworkMarketplacePanelTab(getHost: () => MarketplaceHostApi | null): PanelTabNode {
  return singleTreeLeaf({
    id: FRAMEWORK_MARKETPLACE_TAB_ID,
    icon: shellTabIcon("store"),
    name: uiDataLabel("Marketplace"),
    order: 1,
    tree: {
      resolveTree: () => {
        const host = getHost();
        return host ? buildMarketplaceTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: uiDataLabel("Marketplace unavailable") }] }] };
      },
    },
  });
}
//#endregion MarketplacePanel
//#endregion 🔖️os-chrome-panels
