/** 🔀️ Shell plumbing, example switching: the active-example action id, whether an app switches or offers examples, the active-example action builder and the remembered and navbar example ids read back from dispatches and history upserts. */
import { type ActionDescriptor } from "@semio-tech/framework";
import { undeclaredActionDiagnostic, type WindowKindActionDeclaration } from "../🆔️action-ids/🟦️.ts";

//#region ShellHelpers
/** 🔄 Catalog example switch does not carry a history_patch on the first Invocation; chrome must re-snapshot. */
export function historyRefreshNeededV1(actionId: string, patch: Readonly<{ upserts?: readonly unknown[] }> | undefined): boolean {
  return actionId === SET_ACTIVE_EXAMPLE_ACTION_ID && (patch?.upserts?.length ?? 0) === 0;
}

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
 * `onAction` funnel — pure so the dispatched id is testable without a session/component. The picker's own
 * `SET_ACTIVE_EXAMPLE_ID` reducer write only moves the LABEL; this descriptor is the only thing that
 * reaches the program, and an empty selection means "the app's default document", never "no argument"
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function buildActiveExampleAction(controllerId: string, exampleId: string): ActionDescriptor {
  return { controllerId, action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: exampleId || "" } };
}

/** 🎨️ The example id the navbar must remember after one dispatch, given what it remembers now. Any
 * `setActiveExample` carrying a non-empty `exampleId` teaches it — not only `NavbarExampleSelect`'s own
 * `onValueChange`, because {@link navbarExampleIdFromHistoryUpserts} spends this memory on REDO, and a
 * `Set Active Example` row can be redone that this shell dispatched from the palette, a context menu, a
 * replayed shell command or the boot load. An empty `exampleId` means "the app's default document",
 * which needs no memory at all (the popped-row branch already answers with `bootExampleId`), so it
 * leaves the memory standing rather than erasing it. */
export function rememberedExampleIdFromDispatchV1(action: { readonly action: string; readonly args?: unknown }, remembered: string): string {
  if (action.action !== SET_ACTIVE_EXAMPLE_ACTION_ID) return remembered;
  const requested = typeof action.args === "object" && action.args !== null ? (action.args as { exampleId?: unknown }).exampleId : undefined;
  return typeof requested === "string" && requested ? requested : remembered;
}

/** 🎨️ Navbar example id implied by a history upsert batch. Chrome-only rows leave the label alone
 * (`undefined`). A live Set Active Example row restores `rememberedExampleId`; a popped row returns
 * `bootExampleId`. */
export function navbarExampleIdFromHistoryUpserts(
  upserts: readonly { readonly actionId?: string; readonly revertible?: boolean }[] | undefined,
  rememberedExampleId: string,
  bootExampleId: string,
): string | undefined {
  const rows = (upserts ?? []).filter((entry) => entry.actionId === SET_ACTIVE_EXAMPLE_ACTION_ID);
  if (rows.length === 0) return undefined;
  if (rows.some((entry) => entry.revertible !== false)) return rememberedExampleId || undefined;
  return bootExampleId;
}
//#endregion ShellHelpers
