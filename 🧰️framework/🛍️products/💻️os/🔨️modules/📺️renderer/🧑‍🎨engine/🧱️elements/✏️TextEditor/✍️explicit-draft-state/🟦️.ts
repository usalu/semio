/** ✍️ Text editor explicit draft state: the draft settings parser, the draft actions and the begin, change, reconcile, cancel and settle transitions with the failure reason. */
import { composeDraftStepV1, draftChangesJsonV1, DRAFT_SPLICES_ARGUMENT, type ActionDescriptor, type ActionOperationControlV1, type DraftChangeV1 } from "@semio-tech/framework";
import { inputCommitReceiptMatchesPublicationV1, type InputDiagnosticV1, type InputOutcomeV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import type { TextEditorExplicitDraftPending, TextEditorExplicitDraftSettings, TextEditorExplicitDraftState } from "../🏷️editor-types/🟦️.ts";
import { exactU64 } from "../📤️outbox-and-scene-packets/🟦️.ts";

//#region 🔖️TextEditorHost
//#region EditingHelpers
/** 📝 Decodes the strict explicit-commit subset of a text-editor scene's settings. */
export function parseTextEditorExplicitDraftSettings(json: string | undefined): TextEditorExplicitDraftSettings | null {
  if (!json) return null;
  try {
    const value = JSON.parse(json) as Record<string, unknown>;
    if (
      value === null ||
      Array.isArray(value) ||
      value.readOnly !== false ||
      value.commit !== "explicit" ||
      typeof value.editAction !== "string" ||
      value.editAction.length === 0 ||
      typeof value.editArgument !== "string" ||
      value.editArgument.length === 0 ||
      typeof value.applyLabel !== "string" ||
      value.applyLabel.length === 0 ||
      typeof value.discardLabel !== "string" ||
      value.discardLabel.length === 0 ||
      typeof value.conflictLabel !== "string" ||
      value.conflictLabel.length === 0 ||
      typeof value.applyingLabel !== "string" ||
      value.applyingLabel.length === 0 ||
      typeof value.cancelLabel !== "string" ||
      value.cancelLabel.length === 0 ||
      typeof value.failedLabel !== "string" ||
      value.failedLabel.length === 0 ||
      typeof value.syntaxErrorLabel !== "string" ||
      value.syntaxErrorLabel.length === 0 ||
      typeof value.validationErrorLabel !== "string" ||
      value.validationErrorLabel.length === 0 ||
      typeof value.pathLabel !== "string" ||
      value.pathLabel.length === 0 ||
      typeof value.lineLabel !== "string" ||
      value.lineLabel.length === 0 ||
      typeof value.columnLabel !== "string" ||
      value.columnLabel.length === 0 ||
      !exactU64(value.publicationRevision) ||
      (value.editArguments !== undefined && (value.editArguments === null || typeof value.editArguments !== "object" || Array.isArray(value.editArguments)))
    ) return null;
    return {
      readOnly: false,
      editAction: value.editAction,
      editArgument: value.editArgument,
      editArguments: (value.editArguments ?? {}) as Readonly<Record<string, unknown>>,
      commit: "explicit",
      applyLabel: value.applyLabel,
      discardLabel: value.discardLabel,
      conflictLabel: value.conflictLabel,
      applyingLabel: value.applyingLabel,
      cancelLabel: value.cancelLabel,
      failedLabel: value.failedLabel,
      syntaxErrorLabel: value.syntaxErrorLabel,
      validationErrorLabel: value.validationErrorLabel,
      pathLabel: value.pathLabel,
      lineLabel: value.lineLabel,
      columnLabel: value.columnLabel,
      publicationRevision: value.publicationRevision,
    };
  } catch {
    return null;
  }
}

/** 📦 Merges a local draft into its artifact-owned action without mutating static arguments. A window that declares
 * {@link DRAFT_SPLICES_ARGUMENT} as its edit argument receives the draft's change set (the JSON list of ranges the editor made,
 * in the coordinates of the text the draft started from) instead of the draft text. */
export function textEditorExplicitDraftAction(settings: TextEditorExplicitDraftSettings, draft: string, changes: readonly DraftChangeV1[] = []): ActionDescriptor {
  return { controllerId: "", action: settings.editAction, args: { ...settings.editArguments, [settings.editArgument]: settings.editArgument === DRAFT_SPLICES_ARGUMENT ? draftChangesJsonV1(changes) : draft } };
}

/** 🆕️ Starts a source draft at one canonical document publication. */
export function createTextEditorExplicitDraft(surfaceId: string, buffer: string, revision: string): TextEditorExplicitDraftState {
  return { surfaceId, base: buffer, revision, draft: buffer, changes: [], dirty: false, conflicted: false, failed: false, diagnostic: null, pending: null, operation: null, cancelling: false, cancellationAccepted: false };
}

/** 🔄️ Only the exact native receipt acknowledges Apply; equal foreign text preserves a conflict. */
export function reconcileTextEditorExplicitDraft(state: TextEditorExplicitDraftState, surfaceId: string, buffer: string, revision: string): TextEditorExplicitDraftState {
  if (state.surfaceId !== surfaceId) return createTextEditorExplicitDraft(surfaceId, buffer, revision);
  const pending = state.pending;
  if (pending) {
    if (!pending.settled) return state;
    if (pending.outcome && inputCommitReceiptMatchesPublicationV1(pending.outcome, revision)) {
      const draft = state.draft === pending.value ? buffer : state.draft;
      return { ...createTextEditorExplicitDraft(surfaceId, buffer, revision), draft, changes: composeDraftStepV1([], buffer, draft), dirty: draft !== buffer };
    }
    if (pending.outcome?.kind === "applied" && pending.outcome.commit && revision === pending.revision) return state;
    return { ...state, pending: null, operation: null, cancelling: false, cancellationAccepted: false, failed: true, diagnostic: pending.outcome?.kind === "refused" ? pending.outcome.diagnostic ?? null : null, conflicted: revision !== state.revision || buffer !== state.base };
  }
  if (!state.dirty) return createTextEditorExplicitDraft(surfaceId, buffer, revision);
  return { ...state, conflicted: revision !== state.revision || buffer !== state.base };
}

/** ✍️ Editing while Apply is pending preserves the newer draft across its eventual publication. */
export function changeTextEditorExplicitDraft(state: TextEditorExplicitDraftState, surfaceId: string, buffer: string, draft: string, revision: string): TextEditorExplicitDraftState {
  if (state.surfaceId !== surfaceId || (!state.pending && draft === buffer)) return { ...createTextEditorExplicitDraft(surfaceId, buffer, revision), draft, changes: composeDraftStepV1([], buffer, draft), dirty: draft !== buffer };
  return { ...state, draft, changes: composeDraftStepV1(state.changes, state.draft, draft), dirty: draft !== state.base || state.pending !== null, failed: false, diagnostic: null, conflicted: state.pending ? state.conflicted : revision !== state.revision || buffer !== state.base };
}

/** 📮️ Reserves a single Apply owner until both completion and its publication arrive. */
export function beginTextEditorExplicitDraft(state: TextEditorExplicitDraftState): TextEditorExplicitDraftState {
  if (!state.dirty || state.conflicted || state.pending) return state;
  return { ...state, failed: false, diagnostic: null, pending: { token: {}, value: state.draft, changes: state.changes, revision: state.revision, outcome: null, settled: false }, operation: null, cancelling: false, cancellationAccepted: false };
}

const ownsTextEditorExplicitDraftPending = (state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending): boolean => state.pending?.token === pending.token;

/** 🪪️ Binds the first exact admitted operation only to the Apply owner that started it. */
export function startTextEditorExplicitDraftOperation(state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending, operation: ActionOperationControlV1): TextEditorExplicitDraftState {
  return !ownsTextEditorExplicitDraftPending(state, pending) || state.pending!.settled || state.operation !== null ? state : { ...state, operation };
}

/** ⏹️ Marks one exact cancellation request without releasing or discarding the local draft. */
export function requestTextEditorExplicitDraftCancellation(state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending): TextEditorExplicitDraftState {
  return !ownsTextEditorExplicitDraftPending(state, pending) || state.operation === null || state.cancelling || state.pending!.settled ? state : { ...state, cancelling: true };
}

/** 🛡️ Records accepted cancellation; the original terminal receipt still retires the Apply owner. */
export function acceptTextEditorExplicitDraftCancellation(state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending): TextEditorExplicitDraftState {
  if (!ownsTextEditorExplicitDraftPending(state, pending) || !state.cancelling) return state;
  if (state.pending!.settled) return { ...state, pending: null, operation: null, cancelling: false, cancellationAccepted: false, failed: false, diagnostic: null };
  return { ...state, cancellationAccepted: true };
}

/** ↩️ A refused exact cancellation releases only the Cancel control and resumes a terminal receipt already observed. */
export function refuseTextEditorExplicitDraftCancellation(state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending, surfaceId: string, buffer: string, revision: string): TextEditorExplicitDraftState {
  if (!ownsTextEditorExplicitDraftPending(state, pending) || !state.cancelling) return state;
  const resumed = { ...state, cancelling: false, cancellationAccepted: false };
  return state.pending!.settled ? reconcileTextEditorExplicitDraft(resumed, surfaceId, buffer, revision) : resumed;
}

/** 🏁️ Late completion from a retired Apply owner cannot alter another draft. */
export function settleTextEditorExplicitDraft(state: TextEditorExplicitDraftState, pending: TextEditorExplicitDraftPending, outcome: unknown, surfaceId: string, buffer: string, revision: string): TextEditorExplicitDraftState {
  if (!ownsTextEditorExplicitDraftPending(state, pending) || state.surfaceId !== surfaceId) return state;
  const value = outcome as { kind?: unknown; commit?: { operation?: unknown; revision?: unknown } } | null | undefined;
  const answer = value?.kind === "applied" && exactU64(value.commit?.operation) && exactU64(value.commit?.revision) || value?.kind === "refused" ? outcome as InputOutcomeV1 : null;
  const settled = { ...pending, outcome: answer, settled: true };
  if (state.cancellationAccepted) return { ...state, pending: null, operation: null, cancelling: false, cancellationAccepted: false, failed: false, diagnostic: null };
  const next = { ...state, pending: settled };
  return state.cancelling ? next : reconcileTextEditorExplicitDraft(next, surfaceId, buffer, revision);
}

/** 🗣️ Maps a stable native diagnostic code and coordinates through localized explicit-draft labels. */
export function textEditorExplicitDraftFailureV1(settings: TextEditorExplicitDraftSettings, diagnostic: InputDiagnosticV1 | null): string {
  if (diagnostic === null) return settings.failedLabel;
  const label = diagnostic.code === "snapshot-edit.invalid-source" ? settings.syntaxErrorLabel : diagnostic.code.startsWith("snapshot-edit.") ? settings.validationErrorLabel : settings.failedLabel;
  const location = diagnostic.span === undefined ? [] : [`${settings.lineLabel} ${diagnostic.span.line}`, `${settings.columnLabel} ${diagnostic.span.column}`];
  const path = diagnostic.params?.path;
  if (path) location.push(`${settings.pathLabel} ${path}`);
  return location.length === 0 ? label : `${label} ${location.join(", ")}`;
}
//#endregion EditingHelpers
//#endregion 🔖️TextEditorHost
