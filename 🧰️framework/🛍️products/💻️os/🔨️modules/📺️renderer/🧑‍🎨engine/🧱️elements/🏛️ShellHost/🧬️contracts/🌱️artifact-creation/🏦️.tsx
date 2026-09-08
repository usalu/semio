import type { BackboneWorkerResponse, SpaceArtifactCreationCatalogStatusV1, SpaceArtifactCreationPhaseV1 } from "@semio-tech/framework-os";
import React from "react";

export const ARTIFACT_CREATION_PROGRESS_CAPACITY = 8;

export type ArtifactCreationProgressOwnerV1 = Readonly<{
  requestId: string;
  spaceId: string;
  kindId: string;
  name: string;
  runtimeKey: string;
  clientInstanceId: string;
  sessionInstanceId: number;
}>;

export type ArtifactCreationProgressStateV1 = ArtifactCreationProgressOwnerV1 & Readonly<{
  phase: SpaceArtifactCreationPhaseV1;
  cancelRequested: boolean;
}>;

export type ArtifactCreationProgressUiStateV1 = Readonly<Record<string, ArtifactCreationProgressStateV1>>;

export type ArtifactCreationProgressActionV1 =
  | { readonly kind: "issued"; readonly owner: ArtifactCreationProgressOwnerV1 }
  | { readonly kind: "status"; readonly message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }> }
  | { readonly kind: "cancel-requested"; readonly requestId: string; readonly spaceId: string }
  | { readonly kind: "cleared"; readonly requestId: string };

export const ARTIFACT_CREATION_PROGRESS_TEXT_V1 = {
  en: {
    heading: "Creating artifact",
    phases: {
      accepted: "The creation request was accepted.",
      preparing: "The artifact is being created…",
      ready: "The artifact is ready.",
      indeterminate: "The creation outcome is unknown. Refresh the space before trying again.",
      failed: "Artifact creation failed.",
      cancelled: "Artifact creation was cancelled.",
    },
    cancel: "Cancel creation",
    cancelling: "Cancellation requested…",
    catalog: {
      loading: "Loading the available artifact kinds…",
      ready: "The available artifact kinds are current.",
      unavailable: "Artifact kinds are unavailable. Reopen the space before creating an artifact.",
    },
  },
  de: {
    heading: "Artefakt wird erstellt",
    phases: {
      accepted: "Die Erstellungsanfrage wurde angenommen.",
      preparing: "Das Artefakt wird erstellt…",
      ready: "Das Artefakt ist bereit.",
      indeterminate: "Das Ergebnis der Erstellung ist unbekannt. Aktualisiere den Space vor einem neuen Versuch.",
      failed: "Die Artefakterstellung ist fehlgeschlagen.",
      cancelled: "Die Artefakterstellung wurde abgebrochen.",
    },
    cancel: "Erstellung abbrechen",
    cancelling: "Abbruch angefordert…",
    catalog: {
      loading: "Verfügbare Artefaktarten werden geladen…",
      ready: "Die verfügbaren Artefaktarten sind aktuell.",
      unavailable: "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst.",
    },
  },
} as const;

export function artifactCreationProgressLocaleV1(locale: string): keyof typeof ARTIFACT_CREATION_PROGRESS_TEXT_V1 | null {
  const language = locale.toLowerCase().split("-")[0];
  return language === "en" || language === "de" ? language : null;
}

export function artifactCreationProgressTerminalV1(phase: SpaceArtifactCreationPhaseV1): boolean {
  return phase === "ready" || phase === "indeterminate" || phase === "failed" || phase === "cancelled";
}

export function artifactCreationProgressRoleV1(phase: SpaceArtifactCreationPhaseV1): "status" | "alert" {
  return phase === "indeterminate" || phase === "failed" || phase === "cancelled" ? "alert" : "status";
}

function retainArtifactCreationProgressV1(
  current: ArtifactCreationProgressUiStateV1,
  state: ArtifactCreationProgressStateV1,
): ArtifactCreationProgressUiStateV1 {
  const next = { ...current };
  delete next[state.requestId];
  while (Object.keys(next).length >= ARTIFACT_CREATION_PROGRESS_CAPACITY) {
    const ids = Object.keys(next);
    const terminal = ids.find((requestId) => artifactCreationProgressTerminalV1(next[requestId]!.phase));
    if (terminal === undefined) return current;
    delete next[terminal];
  }
  next[state.requestId] = state;
  return next;
}

/** 🌱️ Reduces only exact request/space/kind status into bounded ephemeral Shell UI state. */
export function reduceArtifactCreationProgressUiV1(
  current: ArtifactCreationProgressUiStateV1,
  action: ArtifactCreationProgressActionV1,
): ArtifactCreationProgressUiStateV1 {
  if (action.kind === "issued") return retainArtifactCreationProgressV1(current, { ...action.owner, phase: "accepted", cancelRequested: false });
  const requestId = action.kind === "status" ? action.message.requestId : action.requestId;
  const state = current[requestId];
  if (state === undefined) return current;
  if (action.kind === "cleared") {
    const next = { ...current };
    delete next[action.requestId];
    return next;
  }
  if (action.kind === "cancel-requested") {
    if (state.spaceId !== action.spaceId || state.cancelRequested || artifactCreationProgressTerminalV1(state.phase)) return current;
    return { ...current, [state.requestId]: { ...state, cancelRequested: true } };
  }
  if (state.spaceId !== action.message.spaceId || (action.message.ready !== undefined && action.message.ready.kindId !== state.kindId)) return current;
  return { ...current, [state.requestId]: { ...state, phase: action.message.phase } };
}

/** ♿ Exact-locale, phase-only creation status. Server status has no numeric counters, so this
 * component never invents a percentage or renders a misleading progress meter. */
export function ArtifactCreationProgressNotice({
  state,
  locale,
  onCancel,
}: Readonly<{
  state: ArtifactCreationProgressStateV1;
  locale: string;
  onCancel(requestId: string, spaceId: string): void;
}>): React.ReactElement | null {
  const language = artifactCreationProgressLocaleV1(locale);
  if (language === null) return null;
  const copy = ARTIFACT_CREATION_PROGRESS_TEXT_V1[language];
  const role = artifactCreationProgressRoleV1(state.phase);
  const terminal = artifactCreationProgressTerminalV1(state.phase);
  const status = state.cancelRequested && !terminal ? copy.cancelling : copy.phases[state.phase];
  return (
    <section
      aria-label={`${copy.heading}: ${state.name}`}
      aria-busy={!terminal || undefined}
      aria-live={role === "alert" ? "assertive" : "polite"}
      data-semio-artifact-creation={state.phase}
      role={role}
    >
      <h2>{copy.heading}</h2>
      <p>{state.name}</p>
      <p>{status}</p>
      {!terminal ? (
        <button
          aria-label={`${copy.cancel}: ${state.name}`}
          disabled={state.cancelRequested}
          onClick={() => onCancel(state.requestId, state.spaceId)}
          type="button"
        >
          {state.cancelRequested ? copy.cancelling : copy.cancel}
        </button>
      ) : null}
    </section>
  );
}

/** ♿ Distinguishes an unavailable catalog from an authorized empty presentation. */
export function ArtifactCreationCatalogNotice({ status, locale }: Readonly<{ status: SpaceArtifactCreationCatalogStatusV1; locale: string }>): React.ReactElement | null {
  const language = artifactCreationProgressLocaleV1(locale);
  if (language === null) return null;
  const text = ARTIFACT_CREATION_PROGRESS_TEXT_V1[language].catalog[status.phase];
  const unavailable = status.phase === "unavailable";
  return (
    <section
      aria-busy={status.phase === "loading" || undefined}
      aria-label={text}
      aria-live={unavailable ? "assertive" : "polite"}
      data-semio-artifact-creation-catalog={status.phase}
      role={unavailable ? "alert" : "status"}
    >
      {text}
    </section>
  );
}
