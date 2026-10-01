import type { ArtifactBootstrapWorkerEvent, BackboneWorkerResponse, DocumentScope } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1, documentRuntimeKeyV1 } from "@semio-tech/framework-os";
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
  return scope === undefined ? documentId : documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
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
    reopen: "The document is reopening from its last confirmed state; your last change was not applied.",
    failed: "Document restore failed",
    exhausted: "This document could not be reopened after repeated failures. Close it and open it again.",
  },
  de: {
    progress: (value: Extract<BootstrapUiStatus, { kind: "artifact-bootstrap-progress" }>) =>
      `Dokument wird wiederhergestellt: ${value.receivedBytes} von ${value.totalBytes} Bytes; ${value.receivedChunks} von ${value.totalChunks} Blöcken.`,
    cancel: "Wiederherstellung abbrechen",
    rebootstrap: "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen.",
    reopen: "Das Dokument wird aus seinem letzten bestätigten Stand neu geöffnet; deine letzte Änderung wurde nicht angewendet.",
    failed: "Dokumentwiederherstellung fehlgeschlagen",
    exhausted: "Dieses Dokument ließ sich nach wiederholten Fehlern nicht neu öffnen. Schließe es und öffne es erneut.",
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
  const text = status.kind === "artifact-rebootstrap-required" ? (status.message === "actor-lost" ? copy.reopen : copy.rebootstrap) : status.code === "recovery-exhausted" ? copy.exhausted : `${copy.failed}: ${status.message}`;
  return <section role="alert" aria-live="assertive" data-semio-bootstrap-status={status.documentId}>{text}</section>;
}

//#region 🪪️ExecutionTargetLease
/** 🛑️ The one control a running execution-target install offers: closing the document aborts the worker's
 * `docAbort`, which stops the hub download, discards every verified byte and settles as `cancelled`. A 15 MB
 * component plus a 20 MB browser actor is an expensive operation, and every expensive operation can be cancelled. */
const EXECUTION_TARGET_COPY = {
  en: { cancel: "Cancel opening" },
  de: { cancel: "Öffnen abbrechen" },
} as const;

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
 * complete UI payload — no origin, path, receipt, grant, digest or user identity.
 *
 * The stage is published as a data attribute rather than as text: the localized sentence stays one
 * sentence for a reader, while an operator (and every headless probe) can tell a load that is still
 * decoding from one that has stopped. It is a stage name from a closed vocabulary and carries no
 * origin, path or digest either. */
export function ExecutionTargetStatusNotice({
  status,
  locale,
  onCancel,
}: {
  readonly status: ExecutionTargetUiStatus;
  readonly locale: "en" | "de";
  readonly onCancel: () => void;
}) {
  const text = DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[status.code][locale];
  const role = documentExecutionTargetStatusRoleV1(status.code);
  return role === "status" ? (
    <section role="status" aria-live="polite" aria-label={text} data-semio-execution-target-status={status.documentId} {...(status.progress ? { "data-semio-execution-target-stage": status.progress.stage } : {})}>
      <p>{text}</p>
      {status.progress ? <progress aria-label={text} value={status.progress.completedBytes} max={status.progress.totalBytes} /> : null}
      <button type="button" data-semio-execution-target-cancel={status.documentId} onClick={onCancel}>{EXECUTION_TARGET_COPY[locale].cancel}</button>
    </section>
  ) : (
    <section role="alert" aria-live="assertive" data-semio-execution-target-status={status.documentId} {...(status.diagnostic ? { "data-semio-execution-target-diagnostic": status.diagnostic } : {})}>{text}</section>
  );
}
//#endregion 🪪️ExecutionTargetLease

//#region 💡️InferencePort
export interface InferencePortOwnerV1 {
  readonly owner: string;
  readonly serviceId: string;
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
  if (owner === null || message.status.owner !== owner.owner || message.status.serviceId !== owner.serviceId || message.operationEpoch !== operationEpoch || message.operationEpoch !== owner.operationEpoch) return null;
  if (message.scope.spaceId !== owner.scope.spaceId || message.scope.documentId !== owner.scope.documentId) return null;
  const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
  return runtimeKey === owner.runtimeKey ? runtimeKey : null;
}

/** 🧹 Retains an inference owner when a different scoped document closes. */
export function retainInferencePortOwnerAfterCloseV1(owner: InferencePortOwnerV1 | null, runtimeKey: string): InferencePortOwnerV1 | null {
  return owner?.runtimeKey === runtimeKey ? null : owner;
}

export type ShellHistoryUndoRouteV1 = "remote" | "local" | "blocked" | "none";

/** ↩️ Chooses the newest exact history owner without folding a Hub durable inverse into guest-local
 * history. A submitting remote member blocks only when it is at least as new as local history. */
export function shellHistoryUndoRouteV1(
  remote: Readonly<{ phase: "unavailable" | "available" | "submitting" | "applied" | "failed"; canUndo: boolean; order: number }> | null,
  local: Readonly<{ canUndo: boolean; order: number }>,
): ShellHistoryUndoRouteV1 {
  if (remote?.phase === "submitting" && remote.order >= local.order) return "blocked";
  if (remote?.canUndo && (!local.canUndo || remote.order >= local.order)) return "remote";
  if (local.canUndo) return "local";
  return remote?.canUndo ? "remote" : "none";
}

export type InferencePortUiAction = Readonly<{ kind: string; payload?: unknown }>;

/** 🪟 Owner-supplied presentation; the host never decodes domain payloads. */
export interface InstalledServicePresentationV1 {
  readonly owner: string;
  readonly serviceId: string;
  probe?(source: import("../🟦️.tsx").ServiceMountedViewV1 | null): unknown;
  render(payload: unknown, locale: "en" | "de", onAction: (action: InferencePortUiAction) => void): React.ReactNode;
}

/** ♿ Mounts the installed owner's accessible view without interpreting its status. */
export function InstalledServicePanelV1({ status, presentation, locale, onAction }: { readonly status: import("@semio-tech/framework-os").InstalledServiceStatusV1; readonly presentation: InstalledServicePresentationV1; readonly locale: "en" | "de"; readonly onAction: (action: InferencePortUiAction) => void }) {
  if (status.owner !== presentation.owner || status.serviceId !== presentation.serviceId) throw new Error("installed-service.foreign-presentation");
  return <>{presentation.render(status.payload, locale, onAction)}</>;
}
//#endregion 💡️InferencePort
