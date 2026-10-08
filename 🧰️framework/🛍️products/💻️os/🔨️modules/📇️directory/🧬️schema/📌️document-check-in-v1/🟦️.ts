// #region Header
/** 📌️ Browser twin of the hub-materialized Check In contract (`🦀️.rs`; JSON Schema
 * `schema://os.directory/DocumentCheckInV1` and `DocumentCheckInStatusV1`). */
// #endregion Header

export const DOCUMENT_CHECK_IN_MAX_BYTES = 4096;
export const DOCUMENT_CHECK_IN_SCHEMA_V1 = "semio.hub.document-check-in/v1";
export const DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1 = "semio.hub.document-check-in-status/v1";
const ID_MAX_BYTES = 256;

/** 🌊️ One edited, committed ledger point: counters and content chain the hub published, and its tip edit id. */
export type EditedArtifactFrontierV1 = Readonly<{
  documentId: string;
  headEditOrdinal: number;
  headEditId: string;
  lastCommitSeq: number;
  chainSha256: string;
}>;

/** 📥️ A Check In names the head it checks in; scope comes from the route, author from the session. */
export type DocumentCheckInV1 = Readonly<{
  schema: typeof DOCUMENT_CHECK_IN_SCHEMA_V1;
  requestId: string;
  head: EditedArtifactFrontierV1;
}>;

export type DocumentCheckInPhaseV1 = "accepted" | "materializing" | "publishing" | "ready" | "failed" | "cancelled";
export type DocumentCheckInRefusalV1 = "unknown-head" | "stale-head" | "active-checkpoint-changed" | "ledger-not-replayable" | "codec-refused" | "authority-changed" | "unavailable";

export type DocumentCheckInReadyV1 = Readonly<{
  checkpointId: string;
  parentCheckpointId: string;
  baseline: EditedArtifactFrontierV1;
}>;

export type DocumentCheckInStatusV1 = Readonly<{
  schema: typeof DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1;
  requestId: string;
  phase: DocumentCheckInPhaseV1;
  progress: Readonly<{ completedUnits: number; totalUnits: number }>;
  ready?: DocumentCheckInReadyV1;
  refusal?: DocumentCheckInRefusalV1;
}>;

export const PHASES: ReadonlySet<string> = new Set(["accepted", "materializing", "publishing", "ready", "failed", "cancelled"]);
const TERMINAL: ReadonlySet<string> = new Set(["ready", "failed", "cancelled"]);
export const REFUSALS: ReadonlySet<string> = new Set(["unknown-head", "stale-head", "active-checkpoint-changed", "ledger-not-replayable", "codec-refused", "authority-changed", "unavailable"]);
export const encoder = new TextEncoder();

export function record(value: unknown): Readonly<Record<string, unknown>> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Readonly<Record<string, unknown>>) : null;
}

export function exactKeys(value: Readonly<Record<string, unknown>>, required: readonly string[], optional: readonly string[] = []): boolean {
  return required.every((key) => key in value) && Object.keys(value).every((key) => required.includes(key) || optional.includes(key));
}

export function requestId(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{32}$/u.test(value) && /[1-9a-f]/u.test(value);
}

function hash(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{64}$/u.test(value) && /[1-9a-f]/u.test(value);
}

function text(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && encoder.encode(value).length <= ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f]/u.test(value);
}

function positive(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 1;
}

export function natural(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

/** 🌊️ An edited, committed ledger point in its exact wire grammar. */
export function isEditedArtifactFrontierV1(value: unknown): value is EditedArtifactFrontierV1 {
  const frontier = record(value);
  return (
    frontier !== null &&
    exactKeys(frontier, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainSha256"]) &&
    text(frontier.documentId) &&
    positive(frontier.headEditOrdinal) &&
    text(frontier.headEditId) &&
    positive(frontier.lastCommitSeq) &&
    hash(frontier.chainSha256)
  );
}

export function canonicalFrontier(frontier: EditedArtifactFrontierV1): EditedArtifactFrontierV1 {
  return { documentId: frontier.documentId, headEditOrdinal: frontier.headEditOrdinal, headEditId: frontier.headEditId, lastCommitSeq: frontier.lastCommitSeq, chainSha256: frontier.chainSha256 };
}

/** 📤️ The canonical request body; throws for a request the hub would refuse unread. */


/** 📥️ One exact canonical request; `null` for padding, reordering, overposting or bounds. */


export function ready(value: unknown): value is DocumentCheckInReadyV1 {
  const candidate = record(value);
  return candidate !== null && exactKeys(candidate, ["checkpointId", "parentCheckpointId", "baseline"]) && hash(candidate.checkpointId) && hash(candidate.parentCheckpointId) && isEditedArtifactFrontierV1(candidate.baseline);
}



/** 🧾️ One exact hub status: only ready names a checkpoint, only failed a refusal, progress never overruns. */


/** 📦️ One status that crossed a structured-clone or pack boundary as a plain object: the exact key
 * set, then the same canonical-status law the hub's JSON is held to. */
export function documentCheckInStatusFromValueV1(input: unknown): DocumentCheckInStatusV1 | null {
  const value = record(input);
  const progress = record(value?.progress);
  if (
    value === null ||
    progress === null ||
    !exactKeys(value, ["schema", "requestId", "phase", "progress"], ["ready", "refusal"]) ||
    !exactKeys(progress, ["completedUnits", "totalUnits"]) ||
    value.schema !== DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1 ||
    !requestId(value.requestId) ||
    typeof value.phase !== "string" ||
    !PHASES.has(value.phase) ||
    !natural(progress.completedUnits) ||
    !natural(progress.totalUnits) ||
    progress.completedUnits > progress.totalUnits ||
    (value.ready !== undefined && !ready(value.ready)) ||
    (value.refusal !== undefined && (typeof value.refusal !== "string" || !REFUSALS.has(value.refusal))) ||
    (value.phase === "ready") !== (value.ready !== undefined) ||
    (value.phase === "failed") !== (value.refusal !== undefined)
  )
    return null;
  return value as DocumentCheckInStatusV1;
}

/** 🏁️ Ready, failed and cancelled never change again. */
export function isTerminalDocumentCheckInPhaseV1(phase: DocumentCheckInPhaseV1): boolean {
  return TERMINAL.has(phase);
}
