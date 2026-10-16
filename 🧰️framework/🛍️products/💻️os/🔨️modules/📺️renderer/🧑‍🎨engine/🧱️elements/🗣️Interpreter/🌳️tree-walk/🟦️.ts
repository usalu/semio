/** 🌳️ The retained tree walk: tree item collection, row tone classes, control stand-in rule and the status section.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { type TreeDataSection } from "@semio-tech/ui-react";
import { UiDocumentStore, type UiDocumentState } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type Component, type StyleSpec, type UiNodeId, type UiNodeRecord } from "@semio-tech/framework";
import { interpLabel } from "../🧷️interpreter-context/🟦️.ts";
import { nodeDomId } from "../🪪️stable-dom-ids/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
//#region Tree
export type TreeWalkNode = { readonly record: UiNodeRecord; readonly props: Extract<Component, { type: "treeItem" }> };

export function collectTreeItems(state: UiDocumentState, ids: readonly UiNodeId[]): readonly TreeWalkNode[] {
  const out: TreeWalkNode[] = [];
  for (const id of ids) {
    const record = state.nodes.get(id);
    if (!record || record.component.type !== "treeItem") continue;
    out.push({ record, props: record.component });
  }
  return out;
}

/** 🎛️ Non-`treeItem` children of a tree item are its inline row controls (the History panel's
 * `framework.history.undo.run` button, the filter's select) — mounted through {@link UiNodeView}
 * into {@link TreeDataItem.control} so their own bindings stay live; `collectTreeItems` alone
 * silently dropped them (browser-measured 2026-09-10: `.run` never mounted, Undo row inert). */
export function collectTreeItemControls(state: UiDocumentState, ids: readonly UiNodeId[]): readonly UiNodeRecord[] {
  const out: UiNodeRecord[] = [];
  for (const id of ids) {
    const record = state.nodes.get(id);
    if (!record || record.component.type === "treeItem") continue;
    out.push(record);
  }
  return out;
}

/** 🚦️ The text colour a tree row's `style.tone` paints (a history row's outcome, a refused input): a semantic role, and
 * never the only cue — the row's icon and its words say the same. Neutral and the brand roles paint nothing. */
export const TREE_ROW_TONE_CLASSES: Readonly<Partial<Record<NonNullable<StyleSpec["tone"]>, string>>> = {
  info: "text-info",
  success: "text-success",
  warning: "text-warning",
  danger: "text-destructive",
};

/** 🪞️ Whether a row may stand in for its one activatable control: only when it says what the control does — the control's
 * own text or accessible name is the row's label (a button row: "Accept" holding "Accept"). A row reading "Points" never
 * silently adds an item: such a control stays a named button inside the row. */
export function treeRowStandsForControl(control: UiNodeRecord, rowLabel: string | null | undefined): boolean {
  const text = control.component.type === "button" ? control.component.label : undefined;
  return rowLabel != null && rowLabel !== "" && (text === rowLabel || control.accessibility.label === rowLabel);
}

/** 🦴 The one section a tree with NO resolvable `treeSection` child renders, so "still loading"
 * and "genuinely nothing to show" are never the same blank rectangle.
 *
 * 🧯️ A panel body the shell has not received yet is `pendingPanelUiNode()` — a `tree` node with
 * `activity: "loading"` and no children (`🖥️platform/🟦️.ts`) — and `<Tree sections={[]}/>` draws
 * exactly nothing for it, which is how the inspection panel presented as an empty panel in the browser
 * (measured 2026-09-09 21:05) while `refreshUi` reported ok. The root record's own `activity` is the
 * contract's declared mechanism for this state, and it was the one thing this view dropped: a
 * `treeSection` carries `loading`/`waiting` for exactly this purpose. An idle tree with no sections is
 * a real, rendered empty body and says so instead. */
export function treeStatusSection(store: UiDocumentStore, record: UiNodeRecord): TreeDataSection {
  const loading = record.activity === "loading";
  const waiting = record.activity === "waiting";
  const status = loading || waiting ? interpLabel("ui.common.loadingSurface") : interpLabel("ui.common.noData");
  return { id: `${nodeDomId(store, record)}-status`, label: status, defaultOpen: true, loading, waiting, items: [], emptyState: status };
}
//#endregion Tree
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
