import React from "react";
import { SemioFaultError, dialectCoordinate, resolveWindowActions, type AppDefinition } from "@semio-tech/framework";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "@semio-tech/framework-os";
import type { PluginOperationCompletion, PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import type { ViewModel } from "../../🐚️Shell/🟦️.tsx";

export const DIRECTORY_PROJECTION_RECEIPT_SCHEMA = "semio.space.home.directory-projection-receipt.v1";

export type DirectoryProjectionReceiptV1 = Readonly<{
  schema: typeof DIRECTORY_PROJECTION_RECEIPT_SCHEMA;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  throughSeqInclusive: number;
  receiptSha256: string;
}>;

type DirectoryEventPageBootstrapV1 = Extract<BackboneWorkerResponse, { readonly kind: "directory-event-page" }>;

export type DirectoryBootstrapPendingV1 = Readonly<{
  canonicalJson: string;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  receiptSha256: string;
  throughSeqInclusive: number;
}>;

/** ⏱️ Ceiling on how long ONE page's typed operation may take to publish its terminal receipt. It
 * bounds a completion frame that never arrives (a faulted shard, a retired instance) — past it the
 * page is re-offered, never waited on for ever. There is no poll behind it: the runtime's own
 * continuation drain advances the operation and `subscribeOperationCompletions` delivers the terminal
 * publication, so this timer only ever fires when that delivery was lost. */
export const DIRECTORY_BOOTSTRAP_SETTLE_DEADLINE_MS = 30_000;

export type DirectoryHomeOwnerV1 = {
  plugin: PluginWasmHandle;
  app: AppDefinition;
  instanceId: number;
  viewState: ViewModel;
  identity: DirectoryHomeIdentityV1;
  ownsInstance: boolean;
  opened: boolean;
  closed: boolean;
  bootstrapEpoch: number;
  abort: AbortController;
  detachInputAbort: () => void;
  pending: DirectoryBootstrapPendingV1 | null;
};

export type DirectoryHomeIdentityV1 = Readonly<{
  userId: string;
  displayName: string;
}>;

export type DirectoryBootstrapUiState =
  | Readonly<{ kind: "idle" }>
  | Readonly<{ kind: "pending"; throughSeqInclusive: number; cancellable: true }>
  | Readonly<{ kind: "retrying"; throughSeqInclusive: number }>
  | Readonly<{ kind: "fault"; code: string }>;

export type DirectoryBootstrapApplyResult = Readonly<{
  state: DirectoryBootstrapUiState;
  receipt?: DirectoryProjectionReceiptV1;
}>;

const SHA256 = /^[0-9a-f]{64}$/u;
const MAX_SAFE = Number.MAX_SAFE_INTEGER;

function exactRecord(value: unknown, keys: readonly string[]): value is Readonly<Record<string, unknown>> {
  return value !== null
    && typeof value === "object"
    && !Array.isArray(value)
    && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort());
}

/** 🧾️ Parses the closed receipt record returned only by the retained typed-operation terminal. */
export function parseDirectoryProjectionReceiptV1(value: unknown): DirectoryProjectionReceiptV1 | null {
  const keys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"] as const;
  if (!exactRecord(value, keys)) return null;
  if (value.schema !== DIRECTORY_PROJECTION_RECEIPT_SCHEMA) return null;
  if (typeof value.sessionBindingSha256 !== "string" || !SHA256.test(value.sessionBindingSha256)) return null;
  if (!Number.isSafeInteger(value.authorizationGeneration) || Number(value.authorizationGeneration) < 1 || Number(value.authorizationGeneration) > MAX_SAFE) return null;
  if (!Number.isSafeInteger(value.throughSeqInclusive) || Number(value.throughSeqInclusive) < 0 || Number(value.throughSeqInclusive) > MAX_SAFE) return null;
  if (typeof value.receiptSha256 !== "string" || !SHA256.test(value.receiptSha256)) return null;
  return value as DirectoryProjectionReceiptV1;
}

function actionAvailable(app: AppDefinition, actionId: string): boolean {
  return app.windowKinds.some((window) => resolveWindowActions(app, window).some((action) => action.id === actionId));
}

/** 🏠️ The Home surface a directory owner feeds: the visible session app when it is the landing app itself or the landing
 * dialect's other role (the read-only Home viewer), and it declares the sealed-page feed — else `null`. The viewer folds
 * the same pages into its own config store, so a Home opened as a viewer lists the signed-in human's hub spaces instead
 * of only this device's rows (ticket 26/09/23 S16, audit s13 §6 #16). Rows: `🧫️fixtures/📇️directory-bootstrap/🔣️.json`
 * `ownerSurfaces`. */
export function directoryHomeOwnerAppV1(landing: AppDefinition, visible: AppDefinition): AppDefinition | null {
  const sameSurface = visible.id === landing.id || (visible.dialect !== undefined && landing.dialect !== undefined && dialectCoordinate(visible.dialect) === dialectCoordinate(landing.dialect));
  return sameSurface && actionAvailable(visible, "applyDirectoryEventPage") ? visible : null;
}

function identityField(value: string): boolean {
  return value.length > 0 && value.length <= 256 && value.trim() === value && !/[\u0000-\u001f\u007f]/u.test(value);
}

function directoryActionInvocation(owner: DirectoryHomeOwnerV1, actionId: string, args: Readonly<Record<string, unknown>>): string {
  const windowInstanceId = owner.viewState.windowId ?? owner.app.windowKinds[0]!.id;
  return JSON.stringify({
    address: {
      pluginId: owner.plugin.pluginId,
      appId: owner.app.id,
      modeId: owner.viewState.activeModeId,
      windowKindId: owner.viewState.activeWindowKindId,
      windowInstanceId,
      actionId,
    },
    arguments: { ...args, windowId: windowInstanceId },
  });
}

/** 🪪️ Opens one visible Home owner with the current host identity in its ephemeral view context. */
export async function openDirectoryHomeOwnerV1(input: Readonly<{
  plugin: PluginWasmHandle;
  app: AppDefinition;
  identity: DirectoryHomeIdentityV1;
  instance?: Readonly<{ instanceId: number; viewState: ViewModel }>;
  baseUrl: string;
  bootstrapEpoch: number;
  locale: string;
  terminology: string;
  signal?: AbortSignal;
  beforeBootstrap?(owner: DirectoryHomeOwnerV1): Promise<void>;
  post(request: BackboneWorkerRequest): void;
}>): Promise<DirectoryHomeOwnerV1> {
  if (!Number.isSafeInteger(input.bootstrapEpoch) || input.bootstrapEpoch < 1) throw new Error("directory-bootstrap.epoch-invalid");
  if (!input.baseUrl || !input.locale || !input.terminology || !identityField(input.identity.userId) || !identityField(input.identity.displayName)) throw new Error("directory-bootstrap.identity-incomplete");
  if (!actionAvailable(input.app, "applyDirectoryEventPage")) throw new Error("directory-bootstrap.home-action-unavailable");
  const windowKindId = input.app.windowKinds[0]?.id;
  const modeId = input.app.defaultModeId ?? input.app.modes[0]?.id;
  if (!windowKindId || !modeId) throw new Error("directory-bootstrap.home-surface-incomplete");
  const abort = new AbortController();
  const cancelFromInput = () => abort.abort(input.signal?.reason ?? "directory-bootstrap-input-cancelled");
  if (input.signal?.aborted) cancelFromInput();
  else input.signal?.addEventListener("abort", cancelFromInput, { once: true });
  const detachInputAbort = () => input.signal?.removeEventListener("abort", cancelFromInput);
  if (abort.signal.aborted) {
    detachInputAbort();
    throw new Error("directory-bootstrap.stale-owner");
  }
  const ownsInstance = input.instance === undefined;
  let instanceId: number;
  try {
    instanceId = input.instance?.instanceId ?? await input.plugin.createApp(input.app.id);
  } catch (error) {
    abort.abort("directory-bootstrap-instance-open-failed");
    detachInputAbort();
    throw error;
  }
  const viewState: ViewModel = {
    ...(input.instance?.viewState ?? {}),
    activeModeId: modeId,
    activeWindowKindId: windowKindId,
    windowId: windowKindId,
    windowInstances: [{ id: windowKindId, windowKindId }],
    locale: input.locale,
    terminology: input.terminology,
    sessionIdentity: { userId: input.identity.userId, displayName: input.identity.displayName },
  };
  const owner: DirectoryHomeOwnerV1 = {
    plugin: input.plugin,
    app: input.app,
    instanceId,
    viewState,
    identity: input.identity,
    ownsInstance,
    opened: false,
    closed: false,
    bootstrapEpoch: input.bootstrapEpoch,
    abort,
    detachInputAbort,
    pending: null,
  };
  try {
    if (owner.abort.signal.aborted) throw new Error("directory-bootstrap.stale-owner");
    await input.beforeBootstrap?.(owner);
    if (owner.abort.signal.aborted) throw new Error("directory-bootstrap.stale-owner");
    input.post({ kind: "directory-bootstrap-open", baseUrl: input.baseUrl, after: 0, bootstrapEpoch: owner.bootstrapEpoch });
    owner.opened = true;
    return owner;
  } catch (error) {
    owner.closed = true;
    owner.abort.abort("directory-bootstrap-owner-open-failed");
    owner.detachInputAbort();
    if (owner.ownsInstance) await owner.plugin.destroyApp(owner.instanceId).catch(() => {});
    throw error;
  }
}

/** 🧹️ Cancels the worker epoch and destroys only a controller-owned plugin instance. */
export async function closeDirectoryHomeOwnerV1(owner: DirectoryHomeOwnerV1, post: (request: BackboneWorkerRequest) => void): Promise<void> {
  if (owner.closed) return;
  owner.closed = true;
  if (owner.opened) post({ kind: "directory-bootstrap-close", bootstrapEpoch: owner.bootstrapEpoch });
  if (!owner.abort.signal.aborted) owner.abort.abort("directory-bootstrap-owner-closed");
  owner.detachInputAbort();
  owner.pending = null;
  if (owner.ownsInstance) await owner.plugin.destroyApp(owner.instanceId).catch(() => {});
}

function receiptMatchesPage(receipt: DirectoryProjectionReceiptV1, page: DirectoryEventPageBootstrapV1): boolean {
  return receipt.sessionBindingSha256 === page.sessionBindingSha256
    && receipt.authorizationGeneration === page.authorizationGeneration
    && receipt.throughSeqInclusive === page.throughSeqInclusive
    && receipt.receiptSha256 === page.receiptSha256;
}

function directoryPageInvocation(owner: DirectoryHomeOwnerV1, canonicalJson: string): string {
  return directoryActionInvocation(owner, "applyDirectoryEventPage", { pageJson: canonicalJson });
}

/** 🎟️ The typed-operation id an admitting reply started, or `null` when it started none.
 * `applyDirectoryEventPage` is a `Migrated`, job-routed verb: its admitting `InvocationResult` carries
 * the `{ operationId, generation }` handle (`🔌️plugin/🦀️.rs`'s `start_typed_command_operation`, both
 * decimal STRINGS) and NOT the verb's result, while `AppFrame::OperationCompleted.operation` carries
 * the same id as a number. This is where the two spellings meet, and it is the ONLY thing this lane
 * reads out of the admission. */
export function startedDirectoryOperationIdV1(output: unknown): number | null {
  if (typeof output !== "object" || output === null || Array.isArray(output)) return null;
  const raw = (output as { readonly operationId?: unknown }).operationId;
  const operation = typeof raw === "string" ? (/^[0-9]+$/u.test(raw) ? Number(raw) : Number.NaN) : typeof raw === "number" ? raw : Number.NaN;
  return Number.isSafeInteger(operation) && operation >= 0 ? operation : null;
}

/** ♻️ Whether a refusal is one this page may be re-offered after. The wire already answers it: every
 * `Fault` carries a `retryable` flag its raiser set (`⚠️diagnostic/🦀️.rs`'s `with_retryable`, `false`
 * by default), so a permanent refusal — an unparsable page, a receipt the guest itself rejected, a
 * retired instance — stops the lane with its own code instead of being re-fetched about once a second,
 * which is the retry storm S4 measured on the live shell (31 `event-page?after=0` in 60 s). */
function directoryRefusalIsTransientV1(error: unknown): boolean {
  return error instanceof SemioFaultError && error.fault.retryable;
}

function directoryRefusalCodeV1(error: unknown): string {
  if (error instanceof SemioFaultError) return error.fault.code;
  return "directory-bootstrap.apply-refused";
}

/** 🚨️ Raised when the operation's terminal publication never reached this shell. Transient by
 * construction — the operation was admitted, so the page is re-offered rather than refused. */
class DirectorySettleDeadlineError extends Error {}

/** 🚨️ Raised when the owner is retired while its operation is still settling, so the wait never
 * outlives the owner it belongs to. The caller's own `aborted` test turns it into `cancelled`. */
class DirectoryOwnerRetiredError extends Error {}

/** 🏁️ Waits for ONE typed operation's terminal publication on the owner's instance.
 *
 * The subscription opens BEFORE the verb is dispatched, because a completion can land in the same
 * message pump turn that resolves the admitting reply: a completion for an operation nobody is waiting
 * on yet is remembered, and the wait resolves from that memory. There is no polling anywhere in here —
 * the runtime's own continuation drain (`drainTypedOperations`) advances the operation and
 * `subscribeOperationCompletions` is its only delivery path. */
function watchDirectoryOperationV1(owner: DirectoryHomeOwnerV1, deadlineMs: number): Readonly<{
  terminalOutputOf(operation: number): Promise<unknown>;
  dispose(): void;
}> {
  const arrived = new Map<number, unknown>();
  let waiting: Readonly<{ operation: number; resolve(output: unknown): void }> | null = null;
  const receive = (completion: PluginOperationCompletion): void => {
    if (waiting && waiting.operation === completion.operation) {
      const settled = waiting;
      waiting = null;
      settled.resolve(completion.terminalOutput);
      return;
    }
    arrived.set(completion.operation, completion.terminalOutput);
  };
  const unsubscribe = owner.plugin.subscribeOperationCompletions(owner.instanceId, receive);
  return {
    terminalOutputOf: (operation) => {
      if (arrived.has(operation)) return Promise.resolve(arrived.get(operation));
      if (owner.abort.signal.aborted) return Promise.reject(new DirectoryOwnerRetiredError("directory-bootstrap: owner retired before its operation settled"));
      return new Promise<unknown>((resolve, reject) => {
        const finish = (): void => {
          clearTimeout(deadline);
          owner.abort.signal.removeEventListener("abort", retired);
          waiting = null;
        };
        const retired = (): void => {
          finish();
          reject(new DirectoryOwnerRetiredError("directory-bootstrap: owner retired before its operation settled"));
        };
        const deadline = setTimeout(() => {
          finish();
          reject(new DirectorySettleDeadlineError(`directory-bootstrap: typed operation ${operation} published no terminal receipt within ${deadlineMs} ms`));
        }, deadlineMs);
        owner.abort.signal.addEventListener("abort", retired, { once: true });
        waiting = { operation, resolve: (output) => { finish(); resolve(output); } };
      });
    },
    dispose: unsubscribe,
  };
}

/** ✅️ Applies one retained page and emits ACK only after the exact typed terminal receipt returns.
 *
 * 🧾️ The receipt comes from the operation's SETTLED terminal publication, never from the admitting
 * reply. `applyDirectoryEventPage` is job-routed (`InteractiveJobClassification::Migrated`): the guest
 * answers the dispatch with a `{ operationId, generation }` handle on the admission turn and emits the
 * `semio.space.home.directory-projection-receipt.v1` event turns later, so reading `response.output`
 * as the receipt parsed `null` for every real page and closed the owner with
 * `directory-bootstrap.receipt-mismatch` — measured live on hub 7611, and the reason Home's space
 * table stayed empty on a hub that has ever indexed a document. */
export async function applyDirectoryEventPageBootstrapV1(
  owner: DirectoryHomeOwnerV1,
  page: DirectoryEventPageBootstrapV1,
  post: (request: BackboneWorkerRequest) => void,
  beforeAcknowledge?: (owner: DirectoryHomeOwnerV1) => Promise<void>,
  settleDeadlineMs: number = DIRECTORY_BOOTSTRAP_SETTLE_DEADLINE_MS,
): Promise<DirectoryBootstrapApplyResult> {
  if (owner.abort.signal.aborted || page.bootstrapEpoch !== owner.bootstrapEpoch) return { state: { kind: "fault", code: "directory-bootstrap.stale-owner" } };
  if (owner.pending) return { state: { kind: "pending", throughSeqInclusive: owner.pending.throughSeqInclusive, cancellable: true } };
  owner.pending = {
    canonicalJson: page.canonicalJson,
    sessionBindingSha256: page.sessionBindingSha256,
    authorizationGeneration: page.authorizationGeneration,
    receiptSha256: page.receiptSha256,
    throughSeqInclusive: page.throughSeqInclusive,
  };
  let settle: ReturnType<typeof watchDirectoryOperationV1> | null = null;
  try {
    settle = watchDirectoryOperationV1(owner, settleDeadlineMs);
    const admission = await owner.plugin.handleAction(owner.instanceId, directoryPageInvocation(owner, page.canonicalJson), { ...owner.viewState, sessionIdentity: { userId: owner.identity.userId, displayName: owner.identity.displayName } });
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    const operation = startedDirectoryOperationIdV1(admission.output);
    if (operation === null) {
      await closeDirectoryHomeOwnerV1(owner, post);
      return { state: { kind: "fault", code: "directory-bootstrap.operation-unstarted" } };
    }
    const receipt = parseDirectoryProjectionReceiptV1(await settle.terminalOutputOf(operation));
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    if (!receipt || !receiptMatchesPage(receipt, page)) {
      await closeDirectoryHomeOwnerV1(owner, post);
      return { state: { kind: "fault", code: "directory-bootstrap.receipt-mismatch" } };
    }
    await beforeAcknowledge?.(owner);
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    owner.pending = null;
    post({
      kind: "directory-bootstrap-ack",
      bootstrapEpoch: owner.bootstrapEpoch,
      sessionBindingSha256: receipt.sessionBindingSha256,
      authorizationGeneration: receipt.authorizationGeneration,
      throughSeqInclusive: receipt.throughSeqInclusive,
      receiptSha256: receipt.receiptSha256,
    });
    return { state: { kind: "idle" }, receipt };
  } catch (error) {
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    owner.pending = null;
    if (!(error instanceof DirectorySettleDeadlineError) && !directoryRefusalIsTransientV1(error)) {
      await closeDirectoryHomeOwnerV1(owner, post);
      return { state: { kind: "fault", code: directoryRefusalCodeV1(error) } };
    }
    post({ kind: "directory-bootstrap-reject", bootstrapEpoch: owner.bootstrapEpoch, receiptSha256: page.receiptSha256 });
    return { state: { kind: "retrying", throughSeqInclusive: page.throughSeqInclusive } };
  } finally {
    settle?.dispose();
  }
}

const DIRECTORY_STATUS_LABELS = {
  en: { pending: "Updating directory through sequence {frontier}", retrying: "Retrying directory update through sequence {frontier}", fault: "Directory update stopped", cancel: "Cancel directory update" },
  de: { pending: "Verzeichnis wird bis Sequenz {frontier} aktualisiert", retrying: "Verzeichnisaktualisierung bis Sequenz {frontier} wird wiederholt", fault: "Verzeichnisaktualisierung angehalten", cancel: "Verzeichnisaktualisierung abbrechen" },
} as const;

function directoryLanguage(locale: string): keyof typeof DIRECTORY_STATUS_LABELS | null {
  const language = locale.split("-")[0];
  return language === "en" || language === "de" ? language : null;
}

/** ♿️ Renders explicit EN/DE status without silently choosing a default language. */
export function DirectoryBootstrapStatusNotice(props: Readonly<{ state: DirectoryBootstrapUiState; locale: string; onCancel(): void }>): React.ReactElement | null {
  if (props.state.kind === "idle") return null;
  const language = directoryLanguage(props.locale);
  if (!language) return <div role="alert" aria-live="assertive" data-directory-bootstrap="locale-missing">{props.locale}</div>;
  const labels = DIRECTORY_STATUS_LABELS[language];
  if (props.state.kind === "fault")
    return (
      <div role="alert" aria-live="assertive" data-directory-bootstrap="fault" data-directory-bootstrap-code={props.state.code}>
        {labels.fault}
      </div>
    );
  const text = labels[props.state.kind].replace("{frontier}", String(props.state.throughSeqInclusive));
  return (
    <div role="status" aria-live="polite" aria-current="true" data-directory-bootstrap={props.state.kind}>
      <span>{text}</span>
      {props.state.kind === "pending" ? <button type="button" onClick={props.onCancel}>{labels.cancel}</button> : null}
    </div>
  );
}
