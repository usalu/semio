/** 🔀️ Shell plumbing, example switching: the active-example action id, whether an app switches or offers examples, the active-example action builder and the remembered and navbar example ids read back from dispatches and history upserts. */
import { type ActionDescriptor, type HistoryEntry } from "@semio-tech/framework";
import { undeclaredActionDiagnostic, type WindowKindActionDeclaration } from "../🆔️action-ids/🟦️.ts";

//#region ShellHelpers
/** 🔄 Catalog example switch does not carry a history_patch on the first Invocation; chrome must re-snapshot. */
export function historyRefreshNeededV1(actionId: string, patch: Readonly<{ upserts?: readonly unknown[] }> | undefined): boolean {
  return actionId === SET_ACTIVE_EXAMPLE_ACTION_ID && (patch?.upserts?.length ?? 0) === 0;
}

/** ∅ The canonical id of the empty document every app's picker offers first-class: the wire id the guests' `EMPTY_EXAMPLE_ID` constants and the picker's "No example" row share. */
export const EMPTY_EXAMPLE_ID = "";

/** 🎨️ The program action the navbar example picker dispatches. */
export const SET_ACTIVE_EXAMPLE_ACTION_ID = "setActiveExample";

/** 📚️ Whether the app itself declares `setActiveExample`. The navbar no longer uses this as the
 * visibility gate: an editor loads its registered catalogue even when the action was never declared. */
export function appSwitchesExamples(appId: string, windowKinds: readonly WindowKindActionDeclaration[], appActions: readonly { readonly id: string }[] = []): boolean {
  return undeclaredActionDiagnostic(appId, SET_ACTIVE_EXAMPLE_ACTION_ID, windowKinds, null, appActions) === null;
}

/** 📚️ Whether the navbar lists registered examples. An editor always does when the catalogue is
 * non-empty — the framework loads the body. A viewer lists them only when it declares the action,
 * because it does not own the editor's example documents. */
export function appOffersRegisteredExamples(
  app: { readonly id: string; readonly role?: string; readonly windowKinds: readonly WindowKindActionDeclaration[]; readonly actions?: readonly { readonly id: string }[] },
  exampleCount: number,
): boolean {
  if (exampleCount <= 0) return false;
  if (app.role === "editor") return true;
  return appSwitchesExamples(app.id, app.windowKinds, app.actions ?? []);
}

/** 🎨️ An editor may dispatch `setActiveExample` without declaring it. The guest loads the registered catalogue. */
export function frameworkOwnsExampleSwitch(role: string | undefined, action: string): boolean {
  return role === "editor" && action === SET_ACTIVE_EXAMPLE_ACTION_ID;
}

/** 🎨️ Builds the `setActiveExample` descriptor the navbar example picker dispatches through the standard
 * `onAction` funnel. The settled outcome drives the picker label; {@link EMPTY_EXAMPLE_ID} asks the program to load its empty document. */
export function buildActiveExampleAction(controllerId: string, exampleId: string): ActionDescriptor {
  return { controllerId, action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId } };
}

/** 🎨️ The example id a `setActiveExample` dispatch requested, the empty document included; `undefined` for any other action or a missing id. */
export function requestedExampleIdV1(action: { readonly action: string; readonly args?: unknown }): string | undefined {
  if (action.action !== SET_ACTIVE_EXAMPLE_ACTION_ID) return undefined;
  const requested = typeof action.args === "object" && action.args !== null ? (action.args as { exampleId?: unknown }).exampleId : undefined;
  return typeof requested === "string" ? requested : undefined;
}

/** 🎨️ Keeps the explicit example from an applied action, the empty document included, so a redone history row can recover its label. */
export function rememberedExampleIdFromDispatchV1(action: { readonly action: string; readonly args?: unknown }, remembered: string | undefined): string | undefined {
  return requestedExampleIdV1(action) ?? remembered;
}

export type CommittedExampleV1 = Readonly<{ exampleId: string; inputSeq: number }>;

/** 🧾️ Applies an example only after its input settles, keeping the last successful explicit example for redo. */
export function exampleSelectionAfterInputV1(
  action: { readonly action: string; readonly args?: unknown },
  outcome: { readonly kind: string; readonly inputSeq: number },
  previous: CommittedExampleV1 | undefined,
  bootExampleId: string,
  focused: boolean,
): { readonly memory: CommittedExampleV1; readonly visibleExampleId?: string } | undefined {
  if (action.action !== SET_ACTIVE_EXAMPLE_ACTION_ID || outcome.kind !== "applied" || (previous && previous.inputSeq >= outcome.inputSeq)) return;
  const requested = requestedExampleIdV1(action);
  const memory = { exampleId: requested ?? previous?.exampleId ?? bootExampleId, inputSeq: outcome.inputSeq };
  return focused ? { memory, visibleExampleId: requested ?? bootExampleId } : { memory };
}

/** 🎨️ Restores the picker from the applied state of example edits. Whole-document load commands
 * record a history row without an edit, so their settled input supplies the example label. */
export function navbarExampleIdFromHistoryUpserts(
  upserts: readonly Pick<HistoryEntry, "actionId" | "applied" | "editId" | "transitionId" | "transaction" | "opCount">[] | undefined,
  rememberedExampleId: string | undefined,
  bootExampleId: string,
): string | undefined {
  const rows = (upserts ?? []).filter((entry) => entry.actionId === SET_ACTIVE_EXAMPLE_ACTION_ID
    && entry.applied !== undefined
    && (entry.editId !== undefined || entry.transitionId !== undefined || entry.transaction !== undefined || (entry.opCount ?? 0) > 0));
  if (rows.length === 0) return undefined;
  if (rows.some((entry) => entry.applied)) return rememberedExampleId;
  return bootExampleId;
}
//#endregion ShellHelpers
