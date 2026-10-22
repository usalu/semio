/** 🎛️ The OS command registry, command category tree and tabs, and the tool registry tree and tabs.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ActionArgDef,
  type ActionDescriptor,
  actionSemanticsForKind,
  type AppDefinition,
  type AppModeDefinition,
  type ArtifactKindChoice,
  type CommandAddress,
  type CommandDefinition,
  DockLayoutStore,
  DockUiStateStore,
  effectiveActionArgs,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  type Platform,
  type PluginAppLabelsOverlay,
  SHELL_LOCALES,
  unresolvedActionArgs,
  type WindowMeasure
} from "@semio-tech/framework";
import {
  builtinUiDrivers,
  type ElementsSurfaceAppearance,
  Icon,
  type IconName,
  type PanelTabNode,
  singleTreeLeaf,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  UI_TERMINOLOGY_NATIVE,
  type UiChromeLayout,
  type UiDriver,
  type UiLocale,
  type UiTheme
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import * as React from "../../⚛️react-port/🟦️.ts";
import {
  setAppearance,
  setDriver,
  setLayout,
  setLocale,
  setTerminology,
  setTheme,
  type UiPreferencesConfigMutation
} from "@semio-tech/framework/config/mutations";
import { EMPTY_SHELL_LOCKS, type ResolvedShellLocks } from "../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { type ShellAction } from "../../🐚️Shell/🎛️shell-actions/🟦️.ts";
import type { PluginManifest } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import {
  EMPTY_APP_LABELS_OVERLAY,
  OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID,
  OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID,
  OPEN_HUB_COMMAND_ID,
  OPEN_TASK_MANAGER_COMMAND_ID,
  openArtifactWithText,
  resolveActionArgDef,
  type ResolvedActionArgDef,
  type ResolvedToolDefinition,
  resolveManifestLabel,
  shellLabel,
  shellTabIcon,
  shellTerminologyLabel
} from "../🏷️labels/🟦️.tsx";
import { FRAMEWORK_CATEGORY_TOOL_ID } from "../🔩️plumbing/🟦️.ts";
import { windowMeasuresToTreeItems } from "../🪟️window-panes/🎚️window-measures/🟦️.tsx";
import { renderStagedArgControl } from "../🪟️window-panes/📝️staged-args/🟦️.tsx";

//#region ShellHelpers
//#region 🎛️CommandRegistry
/** 🎛️ One command definition paired with its shared, fully qualified address. */
export type ResolvedCommand = {
  readonly definition: Omit<CommandDefinition, "label" | "args"> & { readonly label: string; readonly args: ResolvedActionArgDef[] };
  readonly address: CommandAddress;
};

/** 📍️ Stable key for maps whose command ids may overlap across owners. */
export function commandAddressKey(address: CommandAddress): string {
  const owner = address.owner;
  if (owner === "os") return `os:${address.commandId}`;
  if ("plugin" in owner) return `plugin:${owner.plugin.pluginId}:${address.commandId}`;
  if ("app" in owner) return `app:${owner.app.pluginId}:${owner.app.appId}:${address.commandId}`;
  return `mode:${owner.mode.pluginId}:${owner.mode.appId}:${owner.mode.modeId}:${address.commandId}`;
}

/** 🪪️ Element-id-safe projection of the complete command address. */
function commandElementKey(address: CommandAddress): string {
  return commandAddressKey(address).replaceAll(":", ".");
}

/** 📍️ Whether an addressed command belongs to the operating-system catalog. */
export function isOsCommandAddress(address: CommandAddress): boolean {
  return address.owner === "os";
}

/** 📍️ Plugin segment of a non-OS command owner. */
export function commandOwnerPluginId(owner: CommandAddress["owner"]): string | null {
  if (owner === "os") return null;
  if ("plugin" in owner) return owner.plugin.pluginId;
  if ("app" in owner) return owner.app.pluginId;
  return owner.mode.pluginId;
}

/** ⌨️ Detects the command-keybinding platform from a browser/host platform description. */
export function detectCommandPlatform(value: string): Platform | undefined {
  if (/mac|iphone|ipad/i.test(value)) return "macOs";
  if (/win/i.test(value)) return "windows";
  if (/linux|x11/i.test(value)) return "linux";
  return undefined;
}

/** ⌨️ Resolves a command's portable chords for one host platform. */
export function commandKeybindingChords(definition: Pick<CommandDefinition, "keybindings">, platform?: Platform): string[] {
  return definition.keybindings.filter((binding) => binding.platform === undefined || binding.platform === platform).map((binding) => binding.chord);
}

/**
 * 🎛️ Aggregates every command visible for the current session: os built-ins, the active session's
 * plugin-owned commands, app-owned commands, and commands owned by the active mode. Window-local
 * verbs are actions and therefore never enter this global command resolver — unlike
 * `resolveWindowActions`/`resolveUtilities`, this never takes a window kind.
 */
export function resolveCommands(
  osCommands: readonly CommandDefinition[],
  activePluginManifest: Pick<PluginManifest, "pluginId" | "commands"> | null | undefined,
  app: Pick<AppDefinition, "id" | "commands" | "modes"> | null | undefined,
  activeModeId: string,
  overlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  manifests: readonly { readonly apps: readonly unknown[] }[] = [],
  selectedArtifactKinds?: readonly ArtifactKindChoice[],
): ResolvedCommand[] {
  const resolveDefinition = (definition: CommandDefinition): ResolvedCommand["definition"] => ({
    ...definition,
    label: resolveManifestLabel(definition.label, terminology, locale),
    args: definition.args.map((def) => resolveActionArgDef(def, definition.id, overlay, terminology, locale, manifests, selectedArtifactKinds)),
  });
  const resolved: ResolvedCommand[] = osCommands.map((definition) => ({ definition: resolveDefinition(definition), address: { owner: "os", commandId: definition.id } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { plugin: { pluginId: activePluginManifest!.pluginId } }, commandId: definition.id } });
  }
  if (!app) return resolved;
  const pluginId = activePluginManifest?.pluginId ?? "";
  for (const definition of app.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { app: { pluginId, appId: app.id } }, commandId: definition.id } });
  }
  const activeMode = (app.modes as readonly AppModeDefinition[] | undefined)?.find((mode) => mode.id === activeModeId);
  for (const definition of activeMode?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { mode: { pluginId, appId: app.id, modeId: activeModeId } }, commandId: definition.id } });
  }
  return resolved;
}

/** 🎛️ Chrome-known command category ids that already have a `ui.settings.tab.*` translation key. */
const CHROME_KNOWN_COMMAND_CATEGORIES = new Set(["general", "driver", "app", "appearance", "layout", "language", "terminology", "theme"]);

/** 🎛️ Loose title-case for an open-set command category id (e.g. "appearance" -> "Appearance"). Falls back to this for app/plugin-invented categories that have no fixed framework vocabulary entry. */
function titleizeCommandCategory(category: string): string {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

/** 🎛️ Resolves a command category's display label, reusing the existing `ui.settings.tab.*` keys for chrome-known ids and falling back to a loose title-case for open-set app/plugin categories. */
export function commandCategoryLabel(category: string): string {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category as "general" | "driver" | "app" | "appearance" | "layout" | "language" | "terminology" | "theme"}`) : titleizeCommandCategory(category);
}

/** 🎛️ Ordered, deduped category tabs for the footer command panel, derived from whatever commands actually resolved. */
export function commandCategories(commands: readonly ResolvedCommand[]): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const { definition } of commands) {
    if (!definition.inPalette) continue;
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}

function selectCommandArg(id: string, label: string, options: readonly { readonly value: string; readonly label: string }[]): ActionArgDef {
  return { id, label, schema: { kind: "string", options: options.map((option) => ({ ...option })) }, required: true };
}

/** 🚗️ Translated display name for a built-in driver id; a custom (user-authored) driver has no
 * translation key, so its own {@link UiDriver.label} (genuine runtime data) is the correct fallback. */
export function driverDisplayLabel(driver: UiDriver): string {
  if (driver.id === "default") return shellLabel("settings.driver.default");
  if (driver.id === "compact") return shellLabel("settings.driver.compact");
  return driver.label || driver.id;
}

/**
 * 🎛️ Os-owned built-in commands — app introduction/theme/layout/locale/appearance/driver,
 * handled locally by the shell (never routed to a program). Rebuilt via `useMemo` since the theme and
 * terminology option lists are live state.
 *
 * 👁️✏️ Whether the active session's dialect has at least one `AppRouter` entry for either role —
 * gates `open-artifact-with-viewer`/`open-artifact-with-editor` (contract freeze §5) the same way
 * `hasIntroduction`/`tutorialRecorderAvailable` gate their own optional commands above.
 */
export function buildOsCommands(
  themeList: readonly UiTheme[],
  terminologies: readonly string[],
  hasIntroduction: boolean,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
  driverList: readonly UiDriver[] = builtinUiDrivers(),
  tutorials: readonly { readonly id: string; readonly title: unknown }[] = [],
  tutorialRecorderAvailable = false,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  hasOpenArtifactSurfaces = false,
): CommandDefinition[] {
  const lockedCommandIds = new Set<string>([...(locks.appearance ? ["os.setAppearance"] : []), ...(locks.themeId ? ["os.setThemeId"] : []), ...(locks.locale ? ["os.setLocale"] : []), ...(locks.terminology ? ["os.setTerminology"] : [])]);
  const commands: CommandDefinition[] = [
    {
      id: "os.toggleFullscreen",
      label: shellLabel("ui.fullscreen.toggle"),
      category: "layout",
      iconId: "maximize-2" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      inPalette: true,
      args: [],
      keybindings: [
        { chord: "f11", platform: "windows" },
        { chord: "f11", platform: "linux" },
        { chord: "control+meta+f", platform: "macOs" },
      ],
    },
    ...(hasIntroduction ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), category: "app", iconId: "graduation-cap" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    ...(tutorials.length > 0
      ? [{ id: "os.playTutorial", label: shellLabel("ui.command.playTutorial"), category: "app", iconId: "play" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, keybindings: [], args: [selectCommandArg("tutorialId", shellLabel("tutorial.chapter"), tutorials.map((tutorial) => ({ value: tutorial.id, label: resolveManifestLabel(tutorial.title, terminology, locale) })))] }]
      : []),
    ...(tutorialRecorderAvailable ? [{ id: "os.recordTutorial", label: shellLabel("ui.command.recordTutorial"), category: "app", iconId: "circle" as const, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      category: "appearance",
      iconId: "sun" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("appearance", shellLabel("ui.settings.tab.appearance"), [
          { value: "system", label: shellLabel("ui.settings.appearance.system") },
          { value: "light", label: shellLabel("ui.settings.appearance.light") },
          { value: "dark", label: shellLabel("ui.settings.appearance.dark") },
        ]),
      ],
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      category: "appearance",
      iconId: "palette" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id })),
        ),
      ],
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      category: "layout",
      iconId: "layout" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("layout", shellLabel("ui.settings.tab.layout"), [
          { value: "desktop", label: shellLabel("settings.layout.desktop") },
          { value: "tablet", label: shellLabel("settings.layout.tablet") },
        ]),
      ],
    },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), category: "layout", iconId: "undo" as const, semantics: actionSemanticsForKind("shell"), kind: "shell", inPalette: true, args: [], keybindings: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      category: "language",
      iconId: "globe" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("locale", shellLabel("ui.settings.tab.language"), [
          { value: "en", label: shellLabel("ui.settings.language.en") },
          { value: "de", label: shellLabel("ui.settings.language.de") },
        ]),
      ],
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      category: "language",
      iconId: "type" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) })),
        ),
      ],
    },
    {
      id: "os.setDriver",
      label: shellLabel("ui.command.setDriver"),
      category: "layout",
      iconId: "settings" as const,
      semantics: actionSemanticsForKind("shell"), kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "driver",
          shellLabel("ui.settings.tab.driver"),
          driverList.map((driver) => ({ value: driver.id, label: driverDisplayLabel(driver) })),
        ),
      ],
    },
    {
      id: OPEN_TASK_MANAGER_COMMAND_ID,
      label: shellLabel("ui.command.openTaskManager"),
      category: "general",
      iconId: "cpu" as const,
      semantics: actionSemanticsForKind("shell"),
      kind: "shell",
      inPalette: true,
      args: [],
      keybindings: [],
    },
    {
      id: OPEN_HUB_COMMAND_ID,
      label: shellLabel("ui.command.openHub"),
      category: "general",
      iconId: "users" as const,
      semantics: actionSemanticsForKind("shell"),
      kind: "shell",
      inPalette: true,
      args: [],
      keybindings: [],
    },
    ...(hasOpenArtifactSurfaces
      ? [
          { id: OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "eye" as IconName, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
          { id: OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "pencil" as IconName, semantics: actionSemanticsForKind("shell"), kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
        ]
      : []),
  ];
  return commands.filter((command) => !lockedCommandIds.has(command.id));
}

/** 🎛️ Os-scope command ids that are handled locally by the shell — mirrors {@link buildOsCommands}.
 * Answers whether `commandId` was actually routed, so a replayed chrome command that lands nowhere is
 * reported instead of silently vanishing (see {@link isShellOwnedCommandId}). */
export function dispatchOsCommand(
  commandId: string,
  args: Record<string, unknown> | undefined,
  commitUiPreference: (mutation: UiPreferencesConfigMutation) => void,
  dispatch: (action: ShellAction) => void,
  dockLayoutStore: DockLayoutStore,
  dockUiStateStore: DockUiStateStore,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
): boolean {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return true;
    case "os.setAppearance":
      if (locks.appearance) return true;
      commitUiPreference(setAppearance((args?.appearance as ElementsSurfaceAppearance) ?? "system"));
      return true;
    case "os.setThemeId":
      if (locks.themeId) return true;
      if (typeof args?.themeId === "string") commitUiPreference(setTheme(args.themeId));
      return true;
    case "os.setLayout":
      commitUiPreference(setLayout((args?.layout as UiChromeLayout) ?? "desktop"));
      return true;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return true;
    case "os.setLocale":
      if (locks.locale) return true;
      if (typeof args?.locale === "string") commitUiPreference(setLocale(args.locale as UiLocale));
      return true;
    case "os.setTerminology":
      if (locks.terminology) return true;
      if (typeof args?.terminology === "string") commitUiPreference(setTerminology(args.terminology));
      return true;
    case "os.setDriver":
      if (typeof args?.driver === "string") commitUiPreference(setDriver(args.driver));
      return true;
    case OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID:
    case OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID:
      dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: commandId === OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID ? "viewer" : "editor" });
      dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
      return true;
    default:
      return false;
  }
}

/** 🎛️ Fallback icon for every command-category leaf — categories are open-set strings any plugin/app/mode author can invent, so there's no per-category icon metadata to key off (unlike the framework's own Workbench/Details/Display/Settings categories). */
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");

/**
 * 🎛️ One category's command list (and, if a command is expanded, its staged arg form) as a `TreePanelConfig`
 * — the content a category `PanelTabLeaf` resolves to. A zero-arg command's row fires immediately on click
 * (a plain fire-and-forget tree row, same pattern as {@link groupNamedLayoutsToTreeItems}'s layout rows —
 * no `selectedIds`/`onSelectionChange` override, so it takes `Tree`'s default single-select highlight after
 * firing, same as clicking a Display→Layout row does). An arg-carrying command's row toggles `expandedCommandId`
 * itself (kept as its own exclusive, bespoke state — not `Tree`'s per-row `openStates`, which isn't naturally
 * exclusive across sibling rows) and, when expanded, a synthetic form section (one row per arg, `control`
 * holding the staged input, replacing the old `Field` wrapper since `TreeDataItem` already renders label +
 * description + control in the same two-column layout) is prepended so it renders above the command list —
 * `Tree` reverses top-level `sections` for `direction="up"` (bottom anchors), threaded here via `flowFromAnchor`/
 * `FlowProvider`/`useFlow` down from the hosting `Panel`, not any manual reversal in this function.
 */
export function buildCommandCategoryTree(
  commands: readonly ResolvedCommand[],
  expandedCommandId: string | null,
  stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  onExecute: (entry: ResolvedCommand, executeArgs?: Record<string, unknown>) => void,
  onToggleExpanded: (commandId: string | null) => void,
  onStageArg: (commandId: string, argId: string, value: unknown) => void,
  onResetArgs: (commandId: string) => void,
): TreePanelConfig {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : undefined;
  const expanded = (expandedCommandId ? commands.find((entry) => commandAddressKey(entry.address) === expandedCommandId) : undefined) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded ? commandAddressKey(expanded.address) : null;
  const sections: TreeDataSection[] = [];
  if (expanded && expanded.definition.args.length > 0) {
    const expandedKey = commandAddressKey(expanded.address);
    const expandedElementKey = commandElementKey(expanded.address);
    const staged = stagedArgsByCommandId[expandedKey] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = unresolvedActionArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      label: expanded.definition.label,
      items: expanded.definition.args.map(
        (def): TreeDataItem => ({
          id: `command.${expandedElementKey}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedKey, def.id, value)),
        }),
      ),
      actions: [
        {
          id: `command-${expandedElementKey}-execute`,
          icon: <Icon icon="check" size="small" />,
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          reason: missing.map((id) => expanded.definition.args.find((def) => def.id === id)!.label).join(", ") || undefined,
          onClick: () => { if (missing.length === 0) onExecute(expanded, effective); },
        },
        {
          id: `command-${expandedElementKey}-reset`,
          icon: <Icon icon="undo" size="small" />,
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expandedKey),
        },
      ],
    });
  }
  const listCommands = commands.filter((entry) => commandAddressKey(entry.address) !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry): TreeDataItem => {
        const argCarrying = entry.definition.args.length > 0;
        const entryKey = commandAddressKey(entry.address);
        const elementKey = commandElementKey(entry.address);
        const icon = entry.definition.iconId ? <Icon icon={entry.definition.iconId as IconName} size="small" /> : undefined;
        if (!argCarrying) return { id: `command.${elementKey}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${elementKey}`,
          label: `${entry.definition.label}…`,
          icon: <Icon icon={expandedCommandId === entryKey ? "chevron-down" : "chevron-up"} size="small" />,
          onClick: () => onToggleExpanded(expandedCommandId === entryKey ? null : entryKey),
        };
      }),
    });
  }
  return { sections };
}

/**
 * 🎛️ One `PanelTabLeaf` per resolved command category — consumers wrap these under the Command branch
 * (`FRAMEWORK_CATEGORY_COMMAND_ID`) on `defaultDock.anchors["bottom-middle"]` so the folded chrome shows
 * a single expandable Command toggle. The command palette's fold/active-category/size/persistence is the
 * generic per-anchor `Panel` state (see `buildPanelProps`); this only builds the category tab leaves.
 * Content is a *lazy* `resolveTree` (mirrors {@link createFrameworkDisplayPanelTabs}'s windows tab) so
 * this array — and therefore `defaultDock`'s own memo — never depends on `expandedCommandId`/
 * `stagedArgsByCommandId`, which change on every keystroke while staging a command argument; `resolveTree`
 * reads those fresh off refs at render time instead.
 */
export function buildCommandCategoryTabs(
  resolvedCommands: readonly ResolvedCommand[],
  categories: readonly { readonly id: string; readonly label: string }[],
  expandedCommandIdRef: React.RefObject<string | null>,
  stagedArgsByCommandIdRef: React.RefObject<Readonly<Record<string, Readonly<Record<string, unknown>>>>>,
  onCommand: (address: CommandAddress, args?: Record<string, unknown>) => void,
  dispatch: (action: ShellAction) => void,
): PanelTabNode[] {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.inPalette && entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () =>
          buildCommandCategoryTree(
            categoryCommands,
            expandedCommandIdRef.current,
            stagedArgsByCommandIdRef.current,
            (entry, executeArgs) => onCommand(entry.address, executeArgs),
            (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
            (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
            (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId }),
          ),
      },
    });
  });
}
//#endregion 🎛️CommandRegistry

//#region 🛠️ToolRegistry
/**
 * 🛠️ One tool's measure-tree content — a single headerless section mapped to native `TreeDataItem`s, so
 * Fill opens directly onto count + distribution with the same chrome as left-corner panel trees. The
 * `tool.<id>` leaf tab IS the activation control (see {@link reconcileToolTabSelection}): a selected tool
 * leaf always means that tool is active, so the tree never renders a second activation toggle of its own —
 * that toggle carried the leaf tab's own element id, so the tab and the toggle disagreed about the same
 * state and the first press on an already-selected leaf deactivated the tool instead of arming it. A tool
 * whose program publishes no measures yet resolves to an empty options section.
 */
function buildToolTree(tool: ResolvedToolDefinition, measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown): { readonly sections: TreeDataSection[]; readonly sortableSections: false } {
  return {
    sortableSections: false,
    sections: [
      {
        id: `tool.${tool.id}.options`,
        label: "",
        defaultOpen: true,
        items: measures && measures.length > 0 ? windowMeasuresToTreeItems(measures, onAction) : [],
      },
    ],
  };
}

/**
 * 🛠️ One `PanelTabLeaf` per resolved mode tool — consumers wrap these under the Tool branch
 * (`FRAMEWORK_CATEGORY_TOOL_ID`) on `defaultDock.anchors["bottom-middle"]`, ordered left of the Command
 * branch, so the folded chrome shows a single Tool toggle. Content is a *lazy* `resolveTree` (mirrors
 * `buildCommandCategoryTabs`'s windows tab) so this array — and therefore `defaultDock`'s own memo —
 * never depends on `toolMeasuresByToolId`, which changes on every activation/slider tick; `resolveTree`
 * reads those fresh off the ref at render time instead.
 */
export function buildToolTabs(tools: readonly ResolvedToolDefinition[], toolMeasuresByToolIdRef: React.RefObject<Readonly<Record<string, readonly WindowMeasure[]>>>, onAction: (action: ActionDescriptor) => unknown): PanelTabNode[] {
  return tools.map((tool) =>
    singleTreeLeaf({
      id: `tool.${tool.id}`,
      icon: shellTabIcon(tool.iconId),
      name: tool.label,
      tree: {
        resolveTree: () => {
          const measures = toolMeasuresByToolIdRef.current[tool.id];
          const tree = buildToolTree(tool, measures, onAction);
          return { sections: tree.sections, sortableSections: tree.sortableSections };
        },
      },
    }),
  );
}

/** 🛠️ Which of the two halves of the one tool state changed since the last reconciliation, and therefore what the shell owes the other half (see {@link reconcileToolTabSelection}). */
export type ToolTabSelectionEffect = { readonly kind: "idle" } | { readonly kind: "activate"; readonly toolId: string | null } | { readonly kind: "select"; readonly toolId: string | null };

/** 🛠️ The pair reconciled by {@link reconcileToolTabSelection}: the mode's active tool and the `tool.<id>` leaf selected under the Tool category (`null` = no tool / the category collapsed to its branch). */
export interface ToolTabSelection {
  readonly toolId: string | null;
  readonly selected: string | null;
}

/**
 * 🛠️ Tool activation and the selected `tool.<id>` leaf tab are ONE state with two representations, and this
 * is its single owner. Every route into the panel path — a user press, a restored `DockUiStateStore`
 * arrangement, an introduction step, a dock reset — has to end at the same place, so activation may not
 * hang off the press callback alone: a hydrated path that already selects `tool.fill` used to render a
 * selected tab over a tool that was never activated, after which the user's first press read as a re-press
 * and *collapsed* the leaf instead of arming the tool.
 *
 * Last change wins: when the active tool moved (program `setActiveTool` effect, utility mutual exclusion,
 * introduction keep-alive) the selection follows it; otherwise a moved selection drives activation. Both
 * outcomes record the value they are about to establish, so the follow-up pass sees no change and cannot
 * bounce — and a refused activation self-heals on the next pass by deselecting the leaf. Callers must skip
 * reconciliation entirely (without updating `previous`) while the Tool category is not the active root, so
 * a tool stays live while the user browses another category and is re-reconciled on re-entry.
 */
export function reconcileToolTabSelection(previous: ToolTabSelection | null, activeToolId: string | null, selectedToolId: string | null): { readonly next: ToolTabSelection; readonly effect: ToolTabSelectionEffect } {
  const last = previous ?? { toolId: null, selected: null };
  if (activeToolId !== last.toolId) return { next: { toolId: activeToolId, selected: activeToolId }, effect: selectedToolId === activeToolId ? { kind: "idle" } : { kind: "select", toolId: activeToolId } };
  if (selectedToolId !== last.selected) return { next: { toolId: selectedToolId, selected: selectedToolId }, effect: selectedToolId === activeToolId ? { kind: "idle" } : { kind: "activate", toolId: selectedToolId } };
  return { next: last, effect: { kind: "idle" } };
}

/**
 * 🛠️ Whether a tool the PROGRAM just armed has to reveal its own `tool.<id>` leaf. {@link
 * reconcileToolTabSelection} is skipped wholesale while the Tool category is not the active root — which
 * is right for a tool the user armed and then browsed away from, and wrong for one a program arms on its
 * own (typing `fill 3` on the window's engagement line answers `Effect::SetActiveTool { fill }`): the
 * tool went live with no leaf tab mounted anywhere, so its options were unreachable and nothing on screen
 * said it had armed. Only a MOVE to a real tool reveals — an idle category, a disarm and a re-render on
 * the same tool all owe nothing.
 */
export function programArmedToolRevealV1(previousToolId: string | null, activeToolId: string | null, toolCategoryActive: boolean): boolean {
  if (toolCategoryActive) return false;
  return activeToolId !== null && activeToolId !== previousToolId;
}

/** 🛠️ The mode tool a footer tab id (`tool.<id>`) names — the selected leaf and the active tool are reconciled by {@link reconcileToolTabSelection}. */
export function toolIdFromPanelTabId(tabId: string | undefined): string | null {
  if (!tabId?.startsWith("tool.")) return null;
  const toolId = tabId.slice("tool.".length);
  return toolId.length > 0 ? toolId : null;
}

/**
 * ♻️ Identity the desktop and mobile Tool panels share so a late `framework.section.tools` admission
 * re-resolves lazy `resolveTree` sources. `buildToolTabs` keeps tab object identity across measure
 * ticks (the tree reads a ref); {@link PanelTreeUnitsPane} is memoized, so without this revision the
 * Fill body stays the empty section from the first resolve after the guest's packed tools carrier lands.
 */
export function toolPanelTreeContentRevision<TStaged>(
  activeToolId: string | null,
  toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>,
  actionPaneStagedArgsByKey: TStaged,
): { readonly activeToolId: string | null; readonly toolMeasuresByToolId: Readonly<Record<string, readonly WindowMeasure[]>>; readonly actionPaneStagedArgsByKey: TStaged } {
  return { activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey };
}

/**
 * 🛠️ One press that opens the Tool category must land on a tool leaf (remembered, else the first
 * declared tool). `progressPanelTabSelection` does not auto-descend a branch with no memory; the
 * closed-panel open path used to swallow that press entirely, so Fill never became the selected leaf.
 */
export function toolCategoryOpenPath(path: readonly string[], memory: Readonly<Record<string, string>>, toolTabIds: readonly string[]): readonly string[] {
  if (path.length !== 1 || path[0] !== FRAMEWORK_CATEGORY_TOOL_ID || toolTabIds.length === 0) return path;
  const remembered = memory[FRAMEWORK_CATEGORY_TOOL_ID];
  const child = remembered && toolTabIds.includes(remembered) ? remembered : toolTabIds[0]!;
  return [...path, child];
}

/**
 * 🛠️ A re-press of a selected `tool.<id>` leaf while that tool is inactive must arm it, not collapse
 * the leaf. `progressPanelTabSelection` treats the press as an active-segment re-press and walks up;
 * the host keeps the previous path and returns the tool id that `setActiveTool` must receive.
 */
export function toolLeafInactiveRepress(previousPath: readonly string[], nextPath: readonly string[], activeToolId: string | null): { readonly path: readonly string[]; readonly toolId: string } | null {
  const previousToolId = toolIdFromPanelTabId(previousPath[previousPath.length - 1]);
  const nextToolId = toolIdFromPanelTabId(nextPath[nextPath.length - 1]);
  if (!previousToolId || nextToolId || (activeToolId ?? null) === previousToolId) return null;
  return { path: previousPath, toolId: previousToolId };
}
//#endregion 🛠️ToolRegistry
//#endregion ShellHelpers
