/** 🪟️ Window panes: the action categories, the category tree and the window action pane. The window measures, utility bar options, staged argument controls, keyboard chords and clipboard activation live in the child folders.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}.
 */
// #region 🔌️Adapters
import { type ActionDefinition, type ActionDescriptor, type AppDefinition, type AppWindowKindDefinition, type ArtifactKindChoice, effectiveActionArgs, type PluginAppLabelsOverlay, resolveWindowActions, SHELL_LOCALES, unresolvedActionArgs } from "@semio-tech/framework";
import { childElementId, Icon, type IconName, Tree, type TreeDataItem, type TreeDataSection, UI_TERMINOLOGY_NATIVE } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { type ReactElement, type ReactNode } from "react";
import { type ActionPaneState, actionStageKey } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { type ShellAction } from "../../🐚️Shell/🎛️shell-actions/🟦️.ts";
import { EMPTY_APP_LABELS_OVERLAY, resolveActionArgDef, resolveAppLabel, type ResolvedActionDefinition, resolveManifestLabel, ribbonParentLabel, shellLabel } from "../🏷️labels/🟦️.tsx";
import { FRAMEWORK_RESERVED_ACTION_IDS } from "../🔩️plumbing/🆔️action-ids/🟦️.ts";
import { actionRequiresStagedForm, renderStagedArgControl } from "./📝️staged-args/🟦️.tsx";

//#region ShellHelpers
//#region 🧰️WindowActionPane
/** 🗂️ Category id for one action: declared category, else `"history"` for history actions, else `"actions"` (mirrors the command-palette fallback at {@link resolveCommands}'s sibling `searchItems` builder). */
export function actionCategoryId(action: Pick<ActionDefinition, "category" | "kind">): string {
  return action.category ?? (action.kind === "history" ? "history" : "actions");
}

/** 🗂️ Resolves an action category's display label: the app's own group-label overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids ({@link ribbonParentLabel}), else the raw id (mirrors {@link resolveUtilityGroupLabel}). */
function actionCategoryLabel(category: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  return resolveAppLabel(appLabelsOverlay, "group", category, ribbonParentLabel(category) ?? category);
}

/** 🗂️ Ordered, deduped categories from resolved actions (sibling of {@link commandCategories}). */
export function actionCategories(actions: readonly ActionDefinition[], appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const action of actions) {
    const id = actionCategoryId(action);
    if (seen.has(id)) continue;
    seen.add(id);
    categories.push({ id, label: actionCategoryLabel(id, appLabelsOverlay) });
  }
  return categories;
}

/**
 * 🎛️ Category sections of one window's Actions rail (Tree twin of {@link buildCommandCategoryTree}):
 * one section per category, zero-arg actions fire directly, arg-carrying actions toggle a sibling form
 * section — exactly {@link buildCommandCategoryTree}'s list/form split, localized per category so
 * multiple categories can render side by side. Only one action (across all categories) is expanded at a
 * time, per `expandedActionId`.
 */
export function buildActionCategoryTree(
  windowId: string,
  controllerId: string,
  actions: readonly ResolvedActionDefinition[],
  expandedActionId: string | null,
  stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  disabled: boolean,
  onExpandedChange: (actionId: string | null) => void,
  onStageArg: (actionId: string, argId: string, value: unknown) => void,
  onResetArgs: (actionId: string) => void,
  onExecute: (descriptor: ActionDescriptor) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): TreeDataSection[] {
  const categories = actionCategories(actions, appLabelsOverlay);
  const expandedAction = expandedActionId ? actions.find((action) => action.id === expandedActionId) : undefined;
  const sections: TreeDataSection[] = [];
  for (const category of categories) {
    const categoryActions = actions.filter((action) => actionCategoryId(action) === category.id);
    sections.push({
      id: `action.category.${category.id}`,
      label: category.label,
      defaultOpen: true,
      items: categoryActions.map((action): TreeDataItem => {
        const icon = action.iconId ? <Icon icon={action.iconId as IconName} size="small" /> : undefined;
        // 🧯️ The armed-utility gate is about the APP's own verbs — a brush that owns the pointer must not
        // have "Delete Selection" pressed out from under it. It is NOT about the framework's own reserved
        // rows: `mod+z` undoes while the brush is armed (the chord is bound outside this pane), so a pane
        // that refuses `#action.undo` at the same moment is inconsistent with its own keybinding. That
        // inconsistency is the whole defect — `pointer-events-none` swallows the click AND the handler
        // short-circuits, so the row reported as inert with no fault and no console line while
        // `framework.history.undo` (ungated) undid the same document (measured 2026-09-17, ◻️2d battery
        // `undo-action-row-changes-document`; every lane after the first `armUtility` was affected).
        const rowDisabled = disabled && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.id);
        const rowClassName = rowDisabled ? "pointer-events-none opacity-50" : undefined;
        if (!actionRequiresStagedForm(action)) {
          return { id: `action.${action.id}`, label: action.label, icon, className: rowClassName, onClick: () => !rowDisabled && onExecute({ controllerId, action: action.id }) };
        }
        const expanded = expandedActionId === action.id;
        return {
          id: `action.${action.id}`,
          label: `${action.label}…`,
          icon: icon ?? <Icon icon={expanded ? "chevron-down" : "chevron-right"} size="small" />,
          className: rowClassName,
          onClick: () => !rowDisabled && onExpandedChange(expanded ? null : action.id),
        };
      }),
    });
    if (expandedAction && actionCategoryId(expandedAction) === category.id) {
      const staged = stagedArgsByKey[actionStageKey(windowId, expandedAction.id)] ?? {};
      const effective = effectiveActionArgs(expandedAction.args, staged);
      const missing = unresolvedActionArgs(expandedAction.args, effective);
      sections.push({
        id: `action.category.${category.id}.form`,
        // 🏷️ The staged-argument form is a SECTION of the Actions pane and every section a user reads
        // carries a name. It had none, so the export dialog's own argument list announced as an unnamed
        // group to a screen reader and read as an untranslated row to the German pass (measured on
        // generation3d 6018: `actions-pane-de` → `unlabelled: ["action.category.actions.form"]`). The
        // expanded action's OWN label is already resolved against the host's locale, and it is what the
        // form is for — no second string to translate, and it says WHICH action is being staged.
        label: expandedAction.label,
        defaultOpen: true,
        items: expandedAction.args.map(
          (def): TreeDataItem => ({
            id: `action.${expandedAction.id}.arg.${def.id}`,
            label: def.label,
            description: def.description,
            control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedAction.id, def.id, value), disabled),
          }),
        ),
        actions: [
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "execute"),
            icon: <Icon icon="check" size="small" />,
            text: shellLabel("ui.common.execute"),
            disabled: disabled || missing.length > 0,
            reason: missing.map((id) => expandedAction.args.find((def) => def.id === id)!.label).join(", ") || undefined,
            onClick: () => { if (!disabled && missing.length === 0) onExecute({ controllerId, action: expandedAction.id, args: effective }); },
          },
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "reset"),
            icon: <Icon icon="undo" size="small" />,
            text: shellLabel("ui.common.reset"),
            disabled,
            onClick: () => onResetArgs(expandedAction.id),
          },
        ],
      });
    }
  }
  return sections;
}

/** 🎛️ Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ResolvedActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
  readonly appLabelsOverlay?: PluginAppLabelsOverlay;
};

/**
 * 🎛️ The per-window Actions rail body (P1/P2), grouped into categories like the command panel. Zero-arg
 * actions fire directly; arg-carrying actions expand a locally-buffered staged form (same inline
 * property-row controls as utility measures) — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay } = props;
  const sections = buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay);
  // 📜️ The rail is its OWN scroll container, not a block that grows past the pane.
  //
  // `Tree` clips itself (`data-slot="tree"` carries `overflow-hidden`), so a rail taller than the pane
  // hid its own last rows instead of handing them to the pane body's scroller: measured on generation3d
  // at 1600×1000 with 32 rows and the app panels raised, `[data-slot="tree"]` rendered 960 px tall inside
  // a 901 px pane body and the expanded `addWidget` form's `kind` combobox sat at y=1038, below the
  // 1000 px viewport — `click` timed out in the shared bar probe and the verb dispatched with no staged
  // argument (ticket 26/09/18, B3c §4.3's reproduction, re-measured by FL1 in `🗑️generated/fl1-*-rail-*.txt`).
  // `min-h-0 flex-1` lets the pane body shrink this box to the band it actually has, and `overflow-y-auto`
  // makes the rows past that band reachable — by wheel, by keyboard focus, and by a probe's
  // `scrollIntoViewIfNeeded`. `shrink-0` on the tree is the other half and not optional: as a flex item the
  // tree would otherwise be shrunk to the band too and clip the very rows this is trying to reach (measured
  // in between: pane 862, tree offsetHeight 862 with scrollHeight 960, nothing scrollable anywhere).
  // A pane whose body has no definite height is unchanged: `flex-1` has nothing to shrink against and the
  // rail renders exactly as before.
  return (
    <div data-slot="window-action-pane" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-y-auto overscroll-contain">
      <Tree className="shrink-0" sections={sections} showLines={false} sortableSections={false} />
    </div>
  );
}

/** 🧰️ Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
export type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰️ Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no panel-eligible action (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 *
 * 🧹️ Panel-eligible means `inPalette` — the SAME curation `resolveCommands` gives the palette and
 * `buildShellContextMenuItems` gives the shell fallback menu: an app declares its raw dispatch verbs
 * (`worldPointerDown`, `registerBrushMesh`, `targetBrushSuggestions`, …) and the framework declares its reserved
 * ones (`interactionSelect`, `noteShellCommand`, `setActiveUtility`, …) as window actions purely so a
 * surface can dispatch them, and a rail that renders them buries the user's own verbs: the puzzle3d
 * perspective rail carried 96 rows and put `Export` at y=1990 inside an 807 px band, which is what
 * `export-only` could not press (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §3).
 */
export function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  manifests: readonly { readonly apps: readonly unknown[] }[] = [],
  selectedArtifactKinds?: readonly ArtifactKindChoice[],
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind).filter((action) => action.inPalette);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, terminology, locale)),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay, terminology, locale, manifests, selectedArtifactKinds)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
      appLabelsOverlay={appLabelsOverlay}
    />
  );
}
//#endregion 🧰️WindowActionPane
//#endregion ShellHelpers
