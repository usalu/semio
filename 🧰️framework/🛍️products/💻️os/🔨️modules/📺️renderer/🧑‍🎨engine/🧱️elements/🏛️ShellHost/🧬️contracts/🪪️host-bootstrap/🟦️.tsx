import type { ArtifactBootstrapWorkerEvent, BackboneWorkerResponse, DocumentScope, GisMapInferencePortStatusV1 } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, documentExecutionTargetStatusRoleV1, documentRuntimeKeyV1, gisMapInferencePortRoleV1, gisMapInferencePortTerminalV1 } from "@semio-tech/framework-os";
import React from "react";

export interface HostAppIdentity {
  readonly id: string;
  readonly role: string;
  readonly dialect?: { readonly artifactKind: string };
}

export interface HostAppAliases {
  readonly landingAppId: string;
  readonly hostAppId: string;
}

export interface ResolvedHostApps<T extends HostAppIdentity> {
  readonly landing: T;
  readonly host: T;
}

const HOST_APP_RESOLUTION_CACHE = new WeakMap<object, Map<string, ResolvedHostApps<HostAppIdentity>>>();

function resolveHostAlias<T extends HostAppIdentity>(apps: readonly T[], alias: string, label: "landing" | "host"): T {
  const direct = apps.filter((app) => app.id === alias);
  const candidates = direct.length > 0
    ? direct
    : apps.filter((app) => app.role === "editor" && app.dialect?.artifactKind.split(".").at(-1) === alias);
  if (candidates.length === 0) throw new Error(`required ${label} alias "${alias}" is absent from the live host manifest`);
  if (candidates.length !== 1) throw new Error(`required ${label} alias "${alias}" is ambiguous in the live host manifest`);
  return candidates[0]!;
}

/** 🪪️ Resolves required host aliases once against live manifest objects, preserving exact identity. */
export function resolveRequiredHostApps<T extends HostAppIdentity>(apps: readonly T[], aliases: HostAppAliases): ResolvedHostApps<T> {
  const key = `${aliases.landingAppId}\u0000${aliases.hostAppId}`;
  const cache = HOST_APP_RESOLUTION_CACHE.get(apps);
  const cached = cache?.get(key);
  if (cached) return cached as ResolvedHostApps<T>;
  const landing = resolveHostAlias(apps, aliases.landingAppId, "landing");
  const host = resolveHostAlias(apps, aliases.hostAppId, "host");
  if (landing === host || landing.id === host.id) throw new Error("required landing and host aliases resolved to the same canonical app");
  const resolved = { landing, host };
  const nextCache = cache ?? new Map<string, ResolvedHostApps<HostAppIdentity>>();
  nextCache.set(key, resolved);
  if (!cache) HOST_APP_RESOLUTION_CACHE.set(apps, nextCache);
  return resolved;
}

export type BootstrapUiStatus = ArtifactBootstrapWorkerEvent;
export type BootstrapUiAction = BootstrapUiStatus | { readonly kind: "snapshot-replaced"; readonly documentId: string; readonly scope?: DocumentScope } | { readonly kind: "detached"; readonly documentId: string; readonly scope?: DocumentScope };
export type BootstrapUiState = Readonly<Record<string, BootstrapUiStatus>>;

function lifecycleKey(documentId: string, scope?: DocumentScope): string {
  return scope === undefined ? documentId : documentRuntimeKeyV1({ kind: "hub", ...scope });
}

/** 🛰️ Replaces one document status atomically and clears only after replacement or detach. */
export function reduceBootstrapUiState(current: BootstrapUiState, action: BootstrapUiAction): BootstrapUiState {
  const key = lifecycleKey(action.documentId, action.scope);
  if (action.kind !== "snapshot-replaced" && action.kind !== "detached") return { ...current, [key]: action };
  if (!(key in current)) return current;
  const next = { ...current };
  delete next[key];
  return next;
}

const COPY = {
  en: {
    progress: (value: Extract<BootstrapUiStatus, { kind: "artifact-bootstrap-progress" }>) =>
      `Restoring document: ${value.receivedBytes} of ${value.totalBytes} bytes; ${value.receivedChunks} of ${value.totalChunks} chunks.`,
    cancel: "Cancel restore",
    rebootstrap: "The server requires a fresh authoritative restore. Stale document UI was discarded while reconnecting.",
    failed: "Document restore failed",
  },
  de: {
    progress: (value: Extract<BootstrapUiStatus, { kind: "artifact-bootstrap-progress" }>) =>
      `Dokument wird wiederhergestellt: ${value.receivedBytes} von ${value.totalBytes} Bytes; ${value.receivedChunks} von ${value.totalChunks} Blöcken.`,
    cancel: "Wiederherstellung abbrechen",
    rebootstrap: "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen.",
    failed: "Dokumentwiederherstellung fehlgeschlagen",
  },
} as const;

/** ♿ Bilingual exact-unit status for one bounded bootstrap transfer. */
export function BootstrapStatusNotice({
  status,
  locale,
  onCancel,
}: {
  readonly status: BootstrapUiStatus;
  readonly locale: "en" | "de";
  readonly onCancel: (documentId: string) => void;
}) {
  const copy = COPY[locale];
  if (status.kind === "artifact-bootstrap-progress") {
    const text = copy.progress(status);
    return (
      <section role="status" aria-live="polite" aria-label={text} data-semio-bootstrap-status={status.documentId}>
        <p>{text}</p>
        <progress aria-label={text} value={status.receivedBytes} max={status.totalBytes} />
        <button type="button" onClick={() => onCancel(status.documentId)}>{copy.cancel}</button>
      </section>
    );
  }
  const text = status.kind === "artifact-rebootstrap-required" ? copy.rebootstrap : `${copy.failed}: ${status.message}`;
  return <section role="alert" aria-live="assertive" data-semio-bootstrap-status={status.documentId}>{text}</section>;
}

//#region 🪪️ExecutionTargetLease
export type ExecutionTargetUiStatus = Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>;
export type ExecutionTargetUiAction = ExecutionTargetUiStatus | { readonly kind: "execution-target-cleared"; readonly documentId: string; readonly scope?: DocumentScope };
export type ExecutionTargetUiState = Readonly<Record<string, ExecutionTargetUiStatus>>;

/** 🪪️ Replaces one document's execution-target status atomically and clears it only on detach. The
 * status is never persisted into the shared document. */
export function reduceExecutionTargetUiState(current: ExecutionTargetUiState, action: ExecutionTargetUiAction): ExecutionTargetUiState {
  const key = lifecycleKey(action.documentId, action.scope);
  if (action.kind === "execution-target-status") return { ...current, [key]: action };
  if (!(key in current)) return current;
  const next = { ...current };
  delete next[key];
  return next;
}

/** ♿ Bilingual execution-target live region: verification progress announces politely, every
 * integrity, stale, cancellation and renderer-unavailable outcome asserts. The rendered text is the
 * complete UI payload — no origin, path, receipt, grant, digest or user identity. */
export function ExecutionTargetStatusNotice({
  status,
  locale,
}: {
  readonly status: ExecutionTargetUiStatus;
  readonly locale: "en" | "de";
}) {
  const text = DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[status.code][locale];
  const role = documentExecutionTargetStatusRoleV1(status.code);
  return role === "status" ? (
    <section role="status" aria-live="polite" aria-label={text} data-semio-execution-target-status={status.documentId}>
      <p>{text}</p>
      {status.progress ? <progress aria-label={text} value={status.progress.completedBytes} max={status.progress.totalBytes} /> : null}
    </section>
  ) : (
    <section role="alert" aria-live="assertive" data-semio-execution-target-status={status.documentId}>{text}</section>
  );
}
//#endregion 🪪️ExecutionTargetLease

//#region 💡️InferencePort
export interface InferencePortOwnerV1 {
  readonly operationEpoch: number;
  readonly runtimeKey: string;
  readonly scope: DocumentScope;
}

/** 🗂️ Accepts one private inference status only for its exact epoch and retained Hub scope. */
export function inferencePortStatusRuntimeKeyV1(
  owner: InferencePortOwnerV1 | null,
  operationEpoch: number,
  message: Extract<BackboneWorkerResponse, { readonly kind: "inference-port-status" }>,
): string | null {
  if (owner === null || message.operationEpoch !== operationEpoch || message.operationEpoch !== owner.operationEpoch) return null;
  if (message.scope.spaceId !== owner.scope.spaceId || message.scope.documentId !== owner.scope.documentId) return null;
  const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...message.scope });
  return runtimeKey === owner.runtimeKey ? runtimeKey : null;
}

/** 🧹 Retains an inference owner when a different scoped document closes. */
export function retainInferencePortOwnerAfterCloseV1(owner: InferencePortOwnerV1 | null, runtimeKey: string): InferencePortOwnerV1 | null {
  return owner?.runtimeKey === runtimeKey ? null : owner;
}

export type InferencePortUiAction =
  | { readonly kind: "propose" }
  | { readonly kind: "cancel" }
  | { readonly kind: "approve" }
  | { readonly kind: "close" };

/** ♿ Bilingual host-owned inference port. Work in flight announces politely, every terminal
 * asserts, progress is a real `<progress>` with the server's own bounded counters, and Cancel and
 * Approve are ordinary keyboard-reachable buttons — Approve exists only while a proposal is
 * actually offered and no cancel has been requested. The rendered text is the complete UI payload:
 * no job transport, origin, path, receipt, proposal body or user identity appears, and nothing here
 * is persisted into the document. Focus moves to the region when it opens and returns to whatever
 * held it before when it closes. */
export function InferencePortPanel({
  status,
  locale,
  onAction,
}: {
  readonly status: GisMapInferencePortStatusV1;
  readonly locale: "en" | "de";
  readonly onAction: (action: InferencePortUiAction) => void;
}) {
  const headingRef = React.useRef<HTMLHeadingElement | null>(null);
  React.useEffect(() => {
    const restore = document.activeElement;
    headingRef.current?.focus();
    return () => {
      if (restore instanceof HTMLElement && restore.isConnected) restore.focus();
    };
  }, []);
  const control = GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1;
  const phaseText = GIS_MAP_INFERENCE_PORT_TEXT_V1[status.phase][locale];
  const text = status.code === null ? phaseText : `${phaseText} ${GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1[status.code][locale]}`;
  const role = gisMapInferencePortRoleV1(status.phase);
  const terminal = gisMapInferencePortTerminalV1(status.phase);
  const cancellable = !terminal && status.phase !== "idle" && !status.cancelRequested;
  const approvable = status.phase === "offered" && status.proposalHash !== null && status.preview?.proposalHash === status.proposalHash && status.preview.jobId === status.jobId && !status.cancelRequested;
  return (
    <section aria-label={control.heading[locale]} data-semio-inference-port={status.phase}>
      <h2 ref={headingRef} tabIndex={-1}>{control.heading[locale]}</h2>
      <p role={role} aria-live={role === "status" ? "polite" : "assertive"}>{text}</p>
      {status.preview ? (
        <dl data-semio-inference-preview={status.preview.regionId}>
          <dt>{control.region[locale]}</dt><dd>{status.preview.regionId}</dd>
          <dt>{control.longitude[locale]}</dt><dd>{status.preview.ring[0][0]}–{status.preview.ring[2][0]}</dd>
          <dt>{control.latitude[locale]}</dt><dd>{status.preview.ring[0][1]}–{status.preview.ring[2][1]}</dd>
        </dl>
      ) : null}
      {status.total > 0 && !terminal ? <progress aria-label={control.progress[locale]} value={status.completed} max={status.total} /> : null}
      {status.phase === "idle" ? (
        <button type="button" onClick={() => onAction({ kind: "propose" })}>{control.heading[locale]}</button>
      ) : null}
      {cancellable ? <button type="button" onClick={() => onAction({ kind: "cancel" })}>{control.cancel[locale]}</button> : null}
      {approvable ? <button type="button" onClick={() => onAction({ kind: "approve" })}>{control.approve[locale]}</button> : null}
      <button type="button" onClick={() => onAction({ kind: "close" })}>{control.close[locale]}</button>
    </section>
  );
}
//#endregion 💡️InferencePort
