/** 🏷️ Shell label, command-id and manifest label resolution: the leaf every other `ShellHelpers` module reads translated text from.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { isIconName } from "@semio-tech/framework/assets";
// #region 🔌️Adapters
import {
  type ActionArgDef,
  type ActionDefinition,
  type AppCatalogue,
  type AppRole,
  type ArtifactKindChoice,
  artifactKindChoices,
  CONTEXT_MENU_GROUP_ID_PREFIX,
  CONTEXT_MENU_OVERFLOW_CATEGORY,
  type DialogDefinition,
  encodeArtifactKindChoice,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  type IntroductionDefinition,
  type IntroductionStepDefinition,
  type LocalizedLabel,
  type PluginAppLabelsOverlay,
  SHELL_LOCALES,
  type ShellLocale,
  type ToolDefinition
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import {
  Icon,
  type IconName,
  resolveTranslationLabel,
  UI_RIBBON_PARENT_CATEGORIES,
  type UiChromeTerminologyId,
  uiI18n,
  type UiLabel,
  type UiTranslationKey
} from "@semio-tech/ui-react";
import React from "react";
import { wireLabel } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

//#region ShellHelpers
export function shellTabIcon(iconId: IconName | string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    const iconName: IconName =
      iconId === FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID
        ? "file-text"
        : iconId === FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID
          ? "panel-catalogue"
          : iconId === FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID
            ? "panel-inspection"
            : iconId === FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID
              ? "panel-parameters"
              : isIconName(iconId)
                ? iconId
                : "circle-dot";
    return <Icon icon={iconName} size={size} />;
  };
}

/** 🌐️ Resolves a chrome translation key outside hook context (tree builders run there). Both bundles
 * are guaranteed complete for every key via `satisfies UiTranslationSchema`, so `?? key` is unreachable in
 * practice — kept only as a last-resort literal rather than a thrown error. `options` supports i18next
 * interpolation for keys with `{{placeholders}}`. */
export function shellLabel(key: UiTranslationKey, options?: Record<string, unknown>): UiLabel {
  return wireLabel(resolveTranslationLabel(uiI18n.t(key, options)) ?? key);
}

/** 🌐️ {@link shellLabel} for a key the chrome bundles actually define, or `null` — for a caller holding an open
 * id space (a control id) that only sometimes names a chrome label. */
export function shellLabelIfDefined(key: string): UiLabel | null {
  return uiI18n.exists(key) ? shellLabel(key as UiTranslationKey) : null;
}

/**
 * 🌐️ Points the shared port {@link shellLabel} reads at `locale`. A shell has TWO i18n ports: its own
 * `ShellScope` instance, which `useUiTranslation`/`useLabel` resolve through, and this shared module port,
 * which is the only thing a tree builder running outside hook context can read. Moving the scope instance
 * alone relabels the hook-rendered chrome (fullscreen, the sync pill, the fold buttons) and pins every
 * builder-produced label — the panel tab names, the Settings branch with its General/Theme/Hotkeys children,
 * the Display/Tool/Command category names — at whatever language this module booted in, which renders one
 * shell in two languages at once. Both ports move together or neither does.
 */
export function syncShellLabelLocale(locale: Parameters<typeof uiI18n.changeLanguage>[0]): void {
  void uiI18n.changeLanguage(locale);
}

/** 🌐️ The language the shared port {@link shellLabel} reads is standing at RIGHT NOW. Every
 * builder memo in `🏛️ShellHost` that calls `shellLabel` — the OS command catalogue, the command and
 * tool category trees, the panel tab names — resolves its text during render, while
 * {@link syncShellLabelLocale} used to be called only from a post-paint effect. So on the render in
 * which `uiLocale` changed, each of those memos recomputed against the OLD language and, because
 * `uiLocale` never changes again, kept that text forever: measured 2026-09-22 on the live `s` shell
 * as a History row for `os.setLocale` still reading "Set Locale" in a German shell that has
 * "Sprache festlegen" for it. Comparing this against the wanted locale during render is what lets a
 * caller move the port BEFORE the memos read it, which is the discipline `initUiLocaleSync`'s own
 * doc already demands for boot. */
export function shellLabelLocale(): string {
  return uiI18n.resolvedLanguage ?? uiI18n.language ?? "";
}

/** 🌍️ {@link shellLabelLocale} as a {@link ShellLocale}: the language framework-owned en/de texts (kernel notices, number
 * refusals) are read in — the first shell locale until the port stands at one. */
export function currentShellLocaleV1(): ShellLocale {
  return SHELL_LOCALES.find((candidate) => candidate === shellLabelLocale()) ?? SHELL_LOCALES[0];
}

/** 🗂️ EN/DE label for a `UI_RIBBON_PARENT_CATEGORIES` id, resolved off the SAME `ui.ribbon.parent.*`
 * chrome bundle the ribbon itself reads (`uiRibbonParentEn`/`uiRibbonParentDe`) — the TypeScript twin
 * of `ui_wgpu::wgpu::ribbon_parent_label`. `undefined` for any id outside the closed 20-id taxonomy;
 * callers fall back to whatever raw id or overlay label they hold. The one resolver behind every
 * ribbon-parent lookup in this shell ({@link resolveUtilityGroupLabel}, `actionCategoryLabel`,
 * {@link contextMenuGroupLabel}) so the taxonomy never grows a second string table. */
export function ribbonParentLabel(category: string): UiLabel | undefined {
  const known = UI_RIBBON_PARENT_CATEGORIES.find((value) => value === category);
  return known === undefined ? undefined : shellLabel(`ui.ribbon.parent.${known}`);
}

/** 🗂️ Display label for a `menu.group.<category>` row, which the guest emits with `label: None` on
 * purpose (the host owns the chrome vocabulary — see `plugin/🦀️.rs`'s `Menu::group` doc and the wgpu
 * twin `shell_context_menu_item_from_spec`). Taxonomy categories resolve through
 * {@link ribbonParentLabel}; the overflow row `organizeContextMenu` synthesizes (`menu.group.more`)
 * is the one id outside the taxonomy and resolves through `ui.contextMenu.more`. `undefined` for any
 * row that is not a group row, or a group row whose category is unknown. */
export function contextMenuGroupLabel(id: string): UiLabel | undefined {
  if (!id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX)) return undefined;
  const category = id.slice(CONTEXT_MENU_GROUP_ID_PREFIX.length);
  return category === CONTEXT_MENU_OVERFLOW_CATEGORY ? shellLabel("ui.contextMenu.more") : ribbonParentLabel(category);
}

//#region 👁️✏️SurfaceRoleLabels
/** 👁️✏️ Frozen bilingual text pair (contract freeze §5) — English first, no default language. These
 * strings are net-new chrome vocabulary and the domain-neutral `uiChromeTranslationBundles` dictionary
 * they'd normally register into lives outside this lease (`🖱️ui/📦️packages/🟦️typescript/🎯️targets/
 * ⚛️react/📦️component.tsx`), so they resolve directly off `uiLocale` here — the same
 * `{ native: { en, de }, reuse: { en, de } }`-shaped idiom `resolveManifestLabel` already gives a
 * plugin's own `LocalizedLabel`, just constructed locally instead of decoded off the wire. */
export type FrozenLabel = { readonly en: string; readonly de: string };

export function frozenLabelText(pair: FrozenLabel, locale: string): string {
  return locale === "de" ? pair.de : pair.en;
}

/** 👁️✏️ Window title chip / read-only badge text — contract freeze §5. */
const SURFACE_ROLE_CHIP_LABEL: Readonly<Record<AppRole, FrozenLabel>> = {
  viewer: { en: "Viewer", de: "Betrachter" },
  editor: { en: "Editor", de: "Editor" },
};
export function surfaceRoleChipText(role: AppRole, locale: string): string {
  return frozenLabelText(SURFACE_ROLE_CHIP_LABEL[role], locale);
}

/** 👁️✏️ Accessible name of the navbar role `ButtonGroup` (`playground.navbar.roles`) — the group needs
 * its OWN name because each button is named by the target app's own `AppDefinition.label`
 * ("Editor"/"Viewer"), which says what you switch TO but not what axis the group controls. */
const SURFACE_ROLE_GROUP_LABEL: FrozenLabel = { en: "Surface role", de: "Oberflächenrolle" };
export function surfaceRoleGroupText(locale: string): string {
  return frozenLabelText(SURFACE_ROLE_GROUP_LABEL, locale);
}

/** 🎛️ Accessible name of the navbar mode `ButtonGroup` (`playground.navbar.modes`) — same reasoning as
 * {@link surfaceRoleGroupText}: the items are named by the plugin's own mode labels. */
const APP_MODE_GROUP_LABEL: FrozenLabel = { en: "Mode", de: "Modus" };
export function appModeGroupText(locale: string): string {
  return frozenLabelText(APP_MODE_GROUP_LABEL, locale);
}

const OPEN_ARTIFACT_WITH_LABEL: FrozenLabel = { en: "Open with…", de: "Öffnen mit…" };
export function openArtifactWithText(locale: string): string {
  return frozenLabelText(OPEN_ARTIFACT_WITH_LABEL, locale);
}

const SET_AS_DEFAULT_LABEL: FrozenLabel = { en: "Set as default", de: "Als Standard festlegen" };
export function setAsDefaultText(locale: string): string {
  return frozenLabelText(SET_AS_DEFAULT_LABEL, locale);
}

const DEFAULT_APPS_SETTINGS_TAB_LABEL: FrozenLabel = { en: "Default apps", de: "Standard-Apps" };
export function defaultAppsSettingsTabText(locale: string): string {
  return frozenLabelText(DEFAULT_APPS_SETTINGS_TAB_LABEL, locale);
}
/** 👁️✏️ {@link defaultAppsSettingsTabText} as a {@link UiLabel} — for `PanelTabNode.name`, which
 * (unlike a plain tree-item `label`) always takes the `wireLabel`-wrapped shape `shellLabel` returns. */
export function defaultAppsSettingsTabLabel(locale: string): UiLabel {
  return wireLabel(defaultAppsSettingsTabText(locale));
}

/** 👁️✏️ Not itself a contract-frozen string (the freeze pins the chip/"Open with…"/"Set as
 * default"/"Default apps" vocabulary, not this one) — the text for the non-blocking notice contract
 * freeze §2.3/§5 requires when a `"viewer.read-only"` fault surfaces, or a viewer-role dispatch is
 * blocked client-side before it ever reaches the host. */
const VIEWER_READ_ONLY_NOTICE_LABEL: FrozenLabel = { en: "This is a read-only viewer — editing is disabled.", de: "Dies ist ein schreibgeschützter Betrachter – Bearbeiten ist deaktiviert." };
export function viewerReadOnlyNoticeText(locale: string): string {
  return frozenLabelText(VIEWER_READ_ONLY_NOTICE_LABEL, locale);
}

/** 👁️✏️ The `SettingsDefaultApps` table's "no pin" option — no existing `ui.common.*` chrome key
 * covers a bare "None" (checked: `ui.common.close`/`.none` are not registered), so this follows the
 * same local-resolution idiom as the rest of this region rather than adding one to the out-of-lease
 * chrome dictionary for a single call site. */
const NONE_OPTION_LABEL: FrozenLabel = { en: "None", de: "Keine" };
export function noneOptionText(locale: string): string {
  return frozenLabelText(NONE_OPTION_LABEL, locale);
}

/** 👁️✏️ Palette command ids, frozen (contract freeze §5) — `owner: "os"`, no `os.` prefix (unlike the
 * wire `AppCommand`s in `💻️os/🎮️commands/`, which these are NOT the same thing as: selecting either
 * one opens the shell's own "Open with…" picker scoped to that role, it doesn't itself send
 * `os.open-artifact-with` over the app channel — the picker's own row click does that). */
export const OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID = "open-artifact-with-viewer";
export const OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID = "open-artifact-with-editor";

/** 🧵️ Reveals the `os.task-manager` window — the action-line half of audit
 * `📓️g5-ux-completeness-audit.md` ranked item 1. Prefixed `os.` because it IS an os-owned command
 * (`isOsCommandAddress`), unlike the two frozen artifact ids above. */
export const OPEN_TASK_MANAGER_COMMAND_ID = "os.openTaskManager";

/** 🔗️ The action-line/palette entry for the hub workspace (sign in, browse and switch spaces,
 * invite, redeem). `ShellHost` owns the surface, so it handles this id in its own `onCommand`
 * beside the other os commands that need more context than `dispatchOsCommand` has.
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU3. */
export const OPEN_HUB_COMMAND_ID = "os.openHub";
//#endregion 👁️✏️SurfaceRoleLabels

//#region 🔖️CheckInAndSyncStatus
/** 📌️ The framework-reserved controller of the history body's explicit `#s-checkin` button (`🔌️plugin/🦀️.rs`
 * `FRAMEWORK_CHECKIN_CONTROLLER_ID`): the shell answers its `submit` itself — no artifact app holds a check-in — with
 * the message it carries, else `"check-in"`, exactly as the wgpu shell's `handle_checkin_action`. */
export const FRAMEWORK_CHECKIN_CONTROLLER_ID = "framework.checkin";
//#endregion 🔖️CheckInAndSyncStatus

/** 🧭️ The five panel tabs the framework itself owns (never app-supplied) — routed through the typed chrome schema instead of the plugin overlay so a locale-locked shell can never show their English manifest label. */
const FRAMEWORK_PANEL_TAB_LABEL_KEYS: Readonly<Record<string, UiTranslationKey>> = {
  [FRAMEWORK_PANEL_TAB_ARTIFACT_ID]: "ui.panel.artifact",
  [FRAMEWORK_PANEL_TAB_CATALOGUE_ID]: "ui.panel.catalogue",
  [FRAMEWORK_PANEL_TAB_INSPECTION_ID]: "ui.panel.inspection",
  [FRAMEWORK_PANEL_TAB_PARAMETERS_ID]: "ui.panel.parameters",
  [FRAMEWORK_PANEL_TAB_HISTORY_ID]: "ui.panel.history",
};

/** 🧭️ Framework-owned panel tabs resolve through the chrome schema (`shellLabel`); every other app-declared tab still resolves through the plugin overlay (`resolveAppLabel`). */
export function resolvePanelTabLabel(overlay: PluginAppLabelsOverlay, tabId: string, fallback: string): string {
  const chromeKey = FRAMEWORK_PANEL_TAB_LABEL_KEYS[tabId];
  return chromeKey ? shellLabel(chromeKey) : resolveAppLabel(overlay, "panelTab", tabId, fallback);
}

/** 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🛍️ Stable empty catalogue reference so scene hosts depending on `AppCatalogueContext` don't
 * re-render before the first `catalogue` section fetch resolves. */
export const EMPTY_APP_CATALOGUE: AppCatalogue = Object.freeze({});

/** 🗺️ Synthesizes a full `LocalizedLabel` matrix from a user-authored string by broadcasting it across all cells (native/reuse × en/de), matching Rust's `LocalizedLabel::data(...)`. Also accepts an existing `LocalizedLabel` idempotently. */
export function synthesizeLocalizedLabel(label: string | LocalizedLabel): LocalizedLabel {
  if (typeof label !== "string") return label;
  return {
    native: { en: label, de: label },
    reuse: { en: label, de: label },
  };
}

/** 🗺️ Resolves an exact manifest terminology/locale cell or already-resolved/user-authored text. Malformed cells remain unresolved; no language is selected implicitly. */
export function resolveManifestLabel(label: unknown, terminology: string, locale: string): string {
  if (typeof label === "string") return label;
  if (!label || typeof label !== "object" || !("native" in label) || !("reuse" in label)) return "";
  if (terminology !== "native" && terminology !== "reuse") return "";
  if (locale !== "en" && locale !== "de") return "";
  const row = label[terminology];
  if (!row || typeof row !== "object" || !("en" in row) || !("de" in row)) return "";
  const value = row[locale];
  return typeof value === "string" ? value : "";
}

/** 🗣️ Resolves a window-kind/panel-tab/mode/action/utility/example/actionArg/dialog/introduction/group id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
export function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode" | "action" | "utility" | "example" | "actionArg" | "dialog" | "introduction" | "group", id: string, fallback: string): string {
  const map =
    kind === "windowKind"
      ? overlay.windowKindLabels
      : kind === "panelTab"
        ? overlay.panelTabLabels
        : kind === "mode"
          ? overlay.modeLabels
          : kind === "action"
            ? overlay.actionLabels
            : kind === "utility"
              ? overlay.utilityLabels
              : kind === "example"
                ? overlay.exampleLabels
                : kind === "actionArg"
                  ? overlay.actionArgLabels
                  : kind === "dialog"
                    ? overlay.dialogLabels
                    : kind === "introduction"
                      ? overlay.introductionLabels
                      : overlay.groupLabels;
  return map[id] ?? fallback;
}

/** 🗣️ Resolves one action-arg's label + (for `select` controls) its options' labels from the overlay's `actionArgLabels` map, keyed `"{scopeId}.{argId}"` / `"{scopeId}.{argId}.option.{value}"`. `scopeId` is an action id for staged/palette forms, a dialog id for dialog args, or a command id for command args. `ActionArgDef.label`/`ActionArgOption.label` are manifest `LocalizedLabel` fields, resolved for `terminology`/`locale` before the overlay's (always-empty, see the `AppLabelsOverlay` deletion note) fallback lookup even applies. */
export type ResolvedActionArgDef = Omit<ActionArgDef, "label" | "schema" | "description"> & {
  readonly label: string;
  readonly description?: string;
  readonly schema: Exclude<ActionArgDef["schema"], { kind: "string" }> | (Omit<Extract<ActionArgDef["schema"], { kind: "string" }>, "options"> & { readonly options: { value: string; label: string }[] });
};

export type ResolvedActionDefinition = Omit<ActionDefinition, "label" | "args"> & { readonly label: string; readonly args: ResolvedActionArgDef[] };
export type ResolvedToolDefinition = Omit<ToolDefinition, "label"> & { readonly label: string };

/** 🎚️ A free-text string argument carries no choice list at all: the manifest wire form declares
 * `options` `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so an absent field is exactly
 * an empty option set rather than a malformed schema. */
type ActionArgStringSchema = Extract<ActionArgDef["schema"], { kind: "string" }>;
export function actionArgStringOptions<T>(schema: { readonly options?: readonly T[] }): readonly T[] {
  return schema.options ?? [];
}

function resolveArtifactKindChoiceLabel(choice: { readonly kindId: string; readonly label: { readonly en: string; readonly de: string } }, locale: string): string {
  if (locale === "en") return choice.label.en;
  if (locale === "de") return choice.label.de;
  return choice.kindId;
}

export function resolveActionArgDef(def: ActionArgDef, scopeId: string, overlay: PluginAppLabelsOverlay, terminology: string, locale: string, manifests: readonly { readonly apps: readonly unknown[] }[], selectedArtifactKinds?: readonly ArtifactKindChoice[]): ResolvedActionArgDef {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, resolveManifestLabel(def.label, terminology, locale));
  const description = def.description === undefined ? undefined : resolveManifestLabel(def.description, terminology, locale);
  if (def.schema.kind !== "string") return { ...def, label, description, schema: def.schema };
  if (def.schema.format?.kind === "artifactKind") {
    const choices = selectedArtifactKinds ?? artifactKindChoices(manifests, def.schema.format.roles);
    const options = choices.map((choice) => ({ value: encodeArtifactKindChoice(choice), label: resolveArtifactKindChoiceLabel(choice, locale) }));
    return { ...def, label, description, schema: { ...def.schema, options } };
  }
  const options = actionArgStringOptions(def.schema).map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, resolveManifestLabel(option.label, terminology, locale)) }));
  return { ...def, label, description, schema: { ...def.schema, options } };
}

/** 🗣️ Resolves a `DialogDefinition`'s title/body/submitLabel/cancelLabel/choices/args from the overlay's `dialogLabels`/`actionArgLabels` maps, keyed by the dialog's own id. `title`/`body`/`submitLabel`/`cancelLabel` are all manifest `LocalizedLabel` fields. */
export function resolveDialogDefinition(dialog: DialogDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string, manifests: readonly { readonly apps: readonly unknown[] }[] = [], selectedArtifactKinds?: readonly ArtifactKindChoice[]): Omit<DialogDefinition, "args"> & { readonly args: ResolvedActionArgDef[] } {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, resolveManifestLabel(dialog.title, terminology, locale)),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, resolveManifestLabel(dialog.body, terminology, locale)) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, resolveManifestLabel(dialog.submitLabel, terminology, locale)),
    cancelLabel: dialog.cancelLabel ? resolveAppLabel(overlay, "dialog", `${dialog.id}.cancel`, resolveManifestLabel(dialog.cancelLabel, terminology, locale)) : dialog.cancelLabel,
    ...(dialog.choices ? { choices: dialog.choices.map((choice) => ({
      ...choice,
      label: resolveAppLabel(overlay, "dialog", `${dialog.id}.choice.${choice.id}.label`, resolveManifestLabel(choice.label, terminology, locale)),
      ...(choice.description ? { description: resolveAppLabel(overlay, "dialog", `${dialog.id}.choice.${choice.id}.description`, resolveManifestLabel(choice.description, terminology, locale)) } : {}),
    })) } : {}),
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay, terminology, locale, manifests, selectedArtifactKinds)),
  };
}

/** 🗣️ Resolves an `IntroductionDefinition`'s title and every step's title/body labels from the
 * overlay's `introductionLabels` map. `title`/`body` are manifest `LocalizedLabel` fields;
 * `IntroductionInteraction.label` is a short checklist caption that is still a plain `String` on the Rust
 * side (not part of the `LocalizedLabel` migration), so it is left as-is. */
export function resolveIntroductionDefinition(introduction: IntroductionDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string): IntroductionDefinition {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", resolveManifestLabel(introduction.title, terminology, locale)),
    steps: introduction.steps.map(
      (step): IntroductionStepDefinition => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, resolveManifestLabel(step.title, terminology, locale)),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, resolveManifestLabel(step.body, terminology, locale)),
        interactions: (step.interactions ?? []).map((interaction, index) => ({
          ...interaction,
          label: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.interaction.${index}.label`, interaction.label),
        })),
        ordered: step.ordered ?? false,
      }),
    ),
  };
}

/** 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
export function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}
//#endregion ShellHelpers
