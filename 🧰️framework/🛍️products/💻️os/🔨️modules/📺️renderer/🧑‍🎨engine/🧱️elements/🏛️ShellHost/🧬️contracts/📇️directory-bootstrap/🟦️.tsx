import React from "react";
import type { AppDefinition } from "@semio-tech/framework";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "@semio-tech/framework-os";
import type { PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import type { ViewModel } from "../../../🐚️Shell/🟦️.tsx";

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
  return app.windowKinds.some((window) => (window.actions ?? []).some((action) => action.id === actionId));
}

function identityField(value: string): boolean {
  return value.length > 0 && value.length <= 256 && value.trim() === value && !/[\u0000-\u001f\u007f]/u.test(value);
}

function invocationTerminal(value: unknown): boolean {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const response = value as Readonly<Record<string, unknown>>;
  if (!Array.isArray(response.mutations) || response.inverseGroup === null || typeof response.inverseGroup !== "object" || Array.isArray(response.inverseGroup)) return false;
  const inverse = response.inverseGroup as Readonly<Record<string, unknown>>;
  return typeof inverse.invocationId === "string" && Array.isArray(inverse.mutations) && Array.isArray(inverse.inverseMutations);
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

/** 🪪️ Binds Hub identity to the visible landing instance before granting bootstrap ownership. */
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
  if (!actionAvailable(input.app, "setClient") || !actionAvailable(input.app, "applyDirectoryEventPage")) throw new Error("directory-bootstrap.home-action-unavailable");
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
    const response = await owner.plugin.handleAction(
      owner.instanceId,
      directoryActionInvocation(owner, "setClient", { clientId: input.identity.userId, clientName: input.identity.displayName }),
      owner.viewState,
    );
    if (!invocationTerminal(response)) throw new Error("directory-bootstrap.identity-terminal-invalid");
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

/** ✅️ Applies one retained page and emits ACK only after the exact typed terminal receipt returns. */
export async function applyDirectoryEventPageBootstrapV1(
  owner: DirectoryHomeOwnerV1,
  page: DirectoryEventPageBootstrapV1,
  post: (request: BackboneWorkerRequest) => void,
  beforeAcknowledge?: (owner: DirectoryHomeOwnerV1) => Promise<void>,
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
  try {
    const response = await owner.plugin.handleAction(owner.instanceId, directoryPageInvocation(owner, page.canonicalJson), owner.viewState);
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    const receipt = parseDirectoryProjectionReceiptV1(response.output);
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
  } catch {
    if (owner.abort.signal.aborted || owner.pending?.receiptSha256 !== page.receiptSha256) return { state: { kind: "fault", code: "directory-bootstrap.cancelled" } };
    post({ kind: "directory-bootstrap-reject", bootstrapEpoch: owner.bootstrapEpoch, receiptSha256: page.receiptSha256 });
    owner.pending = null;
    return { state: { kind: "retrying", throughSeqInclusive: page.throughSeqInclusive } };
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
  if (props.state.kind === "fault") return <div role="alert" aria-live="assertive" data-directory-bootstrap="fault">{labels.fault}</div>;
  const text = labels[props.state.kind].replace("{frontier}", String(props.state.throughSeqInclusive));
  return (
    <div role="status" aria-live="polite" aria-current="true" data-directory-bootstrap={props.state.kind}>
      <span>{text}</span>
      {props.state.kind === "pending" ? <button type="button" onClick={props.onCancel}>{labels.cancel}</button> : null}
    </div>
  );
}
