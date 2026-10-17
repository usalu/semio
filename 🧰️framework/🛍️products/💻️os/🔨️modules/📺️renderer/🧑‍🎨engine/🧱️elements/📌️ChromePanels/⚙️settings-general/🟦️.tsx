/** ⚙️ Settings chrome panel, General tab: the settings host api, the merge policy options and the general settings tree. */
import { Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, type ThemeAppearanceName, type ThemePaletteGroup, type TreeDataItem, type TreePanelConfig, type UiChromeLayout, type UiDriver, type UiLocale, type UiTheme, type UiTranslationKey } from "@semio-tech/ui-react";
import { type MergePolicy } from "@semio-tech/framework";
import { type ResolvedShellLocks } from "../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { driverDisplayLabel } from "../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { shellLabel, shellTerminologyLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

//#region 🔖️os-chrome-panels
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
  /** ⌨️ The localized name of a bound control the shell's chrome bundles do not name — an app action or an OS command —
   * in the live language, or `null`. */
  readonly controlKeybindingLabel: (controlId: string) => string | null;
  readonly keybindingCaptureControlId: string | null;
  readonly setKeybindingCaptureControlId: (id: string | null) => void;
  readonly setKeybindingOverride: (controlId: string, keys: string) => void;
  readonly resetKeybindingOverride: (controlId: string) => void;
  readonly locks: ResolvedShellLocks;
  /** ⚖️ Persisted `os.config.merge-policy` (contract freeze §C3/§C9) — `undefined` omits the row
   * entirely (an embedded shell that doesn't own OS config), matching `onResetDock`'s idiom. */
  readonly mergePolicy?: MergePolicy;
  readonly setMergePolicy?: (policy: MergePolicy) => void;
};

const MERGE_POLICY_OPTIONS: readonly MergePolicy[] = ["LaissezFaire", "Normal", "Vigilant"];

/** ⚖️ `ui.mutation.policy.<key>.label` for a `MergePolicy` — `LaissezFaire` → `laissezFaire`, others
 * lowercase 1:1. */
function mergePolicyLabelKey(policy: MergePolicy): UiTranslationKey {
  const key = policy === "LaissezFaire" ? "laissezFaire" : policy === "Vigilant" ? "vigilant" : "normal";
  return `ui.mutation.policy.${key}.label` as UiTranslationKey;
}

export function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
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
                    <Select id="framework.settings.appearance" value={host.appearance} onValueChange={(value) => host.setAppearance(value)}>
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
              <Select id="framework.settings.layout" value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
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
              <Select id="framework.settings.driver" value={host.driverId} onValueChange={(value) => host.setDriverId(value)}>
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
                    <Select id="framework.settings.language" value={host.locale} onValueChange={(value) => host.setLocale(value === "de" ? "de" : "en")}>
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
                    <Select id="framework.settings.terminology" value={host.terminology} onValueChange={(value) => host.setTerminology(value)}>
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
          ...(host.mergePolicy && host.setMergePolicy
            ? [
                {
                  id: "framework.settings.mergePolicy",
                  label: shellLabel("ui.mutation.policy.setting.label"),
                  control: (
                    <Select id="framework.settings.mergePolicy" value={host.mergePolicy} onValueChange={(value) => host.setMergePolicy?.(value as MergePolicy)}>
                <SelectTrigger id="framework.settings.mergePolicy" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {MERGE_POLICY_OPTIONS.map((policy) => (
                          <SelectItem key={policy} value={policy}>
                            {shellLabel(mergePolicyLabelKey(policy))}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  ),
                },
              ]
            : []),
          ...(host.onResetDock
            ? [
                {
                  id: "framework.settings.resetDock.action",
                  label: shellLabel("ui.settings.resetDock"),
                  control: <Button id="framework.settings.resetDock" icon="rotate-ccw" text={shellLabel("ui.settings.resetDock")} onClick={() => host.onResetDock?.()} />,
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
              <Button icon="save"
                id="framework.settings.driver.save"
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
                  control: <Button icon="trash-2" id="framework.settings.driver.delete" text={shellLabel("settings.driver.delete")} onClick={() => host.deleteDriver(host.driverId)} />,
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
      <Select id={`framework.settings.driver.${key}`} value={value as string} onValueChange={onChange}>
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
//#endregion SettingsPanel
//#endregion 🔖️os-chrome-panels
