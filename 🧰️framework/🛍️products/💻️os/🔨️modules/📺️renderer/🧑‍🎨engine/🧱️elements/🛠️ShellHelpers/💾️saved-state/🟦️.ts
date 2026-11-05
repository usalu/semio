import type { ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import { localizedLabelTextV1 } from "../../🏛️ShellHost/📣️replay-refusal/🟦️.ts";

/** 💾️ The ONE saved state every surface (pill, popover, tab marker, close guard, page unload guard) reads. It follows the acknowledged write, never a text draft and never the remote link. */
export type SavedStateV1 =
  | Readonly<{ kind: "unknown" }>
  | Readonly<{ kind: "saved" }>
  | Readonly<{ kind: "unsaved"; pending: number; failure: Readonly<{ code: string; message?: string }> | null }>;

/** 💾️ Saved only when the store acknowledged everything admitted: `persisted` and nothing pending. */
export function deriveSavedStateV1(status: ArtifactSyncStatus | null): SavedStateV1 {
  if (status === null) return { kind: "unknown" };
  if (status.persisted && status.pendingMutations === 0 && status.writeFailure === undefined) return { kind: "saved" };
  return { kind: "unsaved", pending: status.pendingMutations, failure: status.writeFailure ?? null };
}

/** 🛑️ Whether leaving the document now could lose an admitted edit. Unknown counts as safe: nothing was ever admitted. */
export function isUnsavedV1(state: SavedStateV1): boolean {
  return state.kind === "unsaved";
}

export const SAVE_FAILURE_CODES_V1 = ["folder-write-failed", "folder-unreadable", "external-divergence", "hub-rejected", "quota-exceeded"] as const;
export type SaveFailureCodeV1 = (typeof SAVE_FAILURE_CODES_V1)[number];

export const SAVE_FAILURE_LABELS_V1: Readonly<Record<SaveFailureCodeV1, Readonly<Record<"en" | "de", string>>>> = {
  "folder-write-failed": { en: "The folder refused the write.", de: "Der Ordner hat das Schreiben abgelehnt." },
  "folder-unreadable": { en: "The folder could not be read.", de: "Der Ordner konnte nicht gelesen werden." },
  "external-divergence": { en: "The folder changed outside this app.", de: "Der Ordner wurde außerhalb dieser App geändert." },
  "hub-rejected": { en: "The hub rejected the change.", de: "Der Hub hat die Änderung abgelehnt." },
  "quota-exceeded": { en: "The storage quota is exhausted.", de: "Das Speicherkontingent ist erschöpft." },
};

const SAVE_FAILURE_GENERIC_V1: Readonly<Record<"en" | "de", string>> = { en: "The save was not acknowledged.", de: "Das Speichern wurde nicht bestätigt." };

/** 🗣️ The typed failure in words. A code no table row knows never leaks as an internal id: it reads as the generic failure, followed by the failure's own message when it carries one. */
export function saveFailureTextV1(failure: Readonly<{ code: string; message?: string }>, locale: string): string {
  const row = (SAVE_FAILURE_LABELS_V1 as Readonly<Record<string, Readonly<Record<"en" | "de", string>>>>)[failure.code];
  if (row !== undefined) return localizedLabelTextV1(row, locale);
  const generic = localizedLabelTextV1(SAVE_FAILURE_GENERIC_V1, locale);
  return failure.message === undefined || failure.message === "" ? generic : `${generic} ${failure.message}`;
}

export type CloseGuardPortV1 = Readonly<{
  saved: () => SavedStateV1;
  isCurrent: () => boolean;
  flush: (signal: AbortSignal) => Promise<"flushed" | "failed" | "cancelled">;
  confirmDiscard: () => Promise<boolean>;
  signal: AbortSignal;
}>;

/** 🚪️ Leaving a document requires acknowledgment or explicit discard; cancelling keeps it open. */
export async function guardDocumentCloseV1(port: CloseGuardPortV1): Promise<"close" | "stay"> {
  if (port.signal.aborted || !port.isCurrent()) return "stay";
  if (!isUnsavedV1(port.saved())) return "close";
  let outcome: "flushed" | "failed" | "cancelled";
  try {
    outcome = await port.flush(port.signal);
  } catch {
    outcome = "failed";
  }
  if (port.signal.aborted || !port.isCurrent() || outcome === "cancelled") return "stay";
  if (outcome === "flushed" && port.saved().kind === "saved") return "close";
  const discard = await port.confirmDiscard();
  return discard && !port.signal.aborted && port.isCurrent() ? "close" : "stay";
}

/** ⏳️ Waits until the acknowledged state turns saved (the worker retries writes with backoff in the background and publishes status), a bounded `timeoutMs` passes, or `signal` cancels. */
export async function awaitSavedV1(input: Readonly<{ saved: () => SavedStateV1; signal: AbortSignal; timeoutMs: number; pollMs?: number; progress?: (state: SavedStateV1) => void }>): Promise<"flushed" | "failed" | "cancelled"> {
  const deadline = Date.now() + input.timeoutMs;
  for (;;) {
    const state = input.saved();
    input.progress?.(state);
    if (input.signal.aborted) return "cancelled";
    if (state.kind === "saved") return "flushed";
    if (Date.now() >= deadline) return "failed";
    await new Promise<void>((resolve) => {
      const timer = setTimeout(() => { input.signal.removeEventListener("abort", wake); resolve(); }, input.pollMs ?? 100);
      const wake = (): void => { clearTimeout(timer); resolve(); };
      input.signal.addEventListener("abort", wake, { once: true });
    });
  }
}
