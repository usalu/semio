import type { ArtifactBootstrapWorkerEvent } from "@semio-tech/framework-os";
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
export type BootstrapUiAction = BootstrapUiStatus | { readonly kind: "snapshot-replaced"; readonly documentId: string } | { readonly kind: "detached"; readonly documentId: string };
export type BootstrapUiState = Readonly<Record<string, BootstrapUiStatus>>;

/** 🛰️ Replaces one document status atomically and clears only after replacement or detach. */
export function reduceBootstrapUiState(current: BootstrapUiState, action: BootstrapUiAction): BootstrapUiState {
  if (action.kind !== "snapshot-replaced" && action.kind !== "detached") return { ...current, [action.documentId]: action };
  if (!(action.documentId in current)) return current;
  const next = { ...current };
  delete next[action.documentId];
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
