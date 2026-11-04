import { describe, expect, it } from "vitest";
import type { ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import { awaitSavedV1, deriveSavedStateV1, guardDocumentCloseV1, saveFailureTextV1, SAVE_FAILURE_LABELS_V1, type SavedStateV1 } from "../../🧱️elements/🛠️ShellHelpers/💾️saved-state/🟦️.ts";
import { computeSyncPillState, syncPillText } from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { syncStatusLabelV1 } from "../../🧱️elements/🔄️ShellSync/🟦️.tsx";

const live = { kind: "live", peerCount: 1 } as const;
const status = (patch: Partial<ArtifactSyncStatus>): ArtifactSyncStatus => ({ persisted: true, pendingMutations: 0, remote: live, ...patch });

describe("one derived saved state", () => {
  it("is saved only when persisted, nothing pending and no failure", () => {
    expect(deriveSavedStateV1(status({}))).toEqual({ kind: "saved" });
    expect(deriveSavedStateV1(null)).toEqual({ kind: "unknown" });
    expect(deriveSavedStateV1(status({ persisted: false }))).toEqual({ kind: "unsaved", pending: 0, failure: null });
    expect(deriveSavedStateV1(status({ writeFailure: { code: "folder-write-failed" } })).kind).toBe("unsaved");
  });

  it("the pill never reports Persisted for a failed or unacknowledged write, and the popover agrees", () => {
    const failed = status({ persisted: false, writeFailure: { code: "folder-write-failed" } });
    expect(computeSyncPillState(failed)).toEqual({ kind: "unsaved", failure: { code: "folder-write-failed" } });
    expect(syncPillText(computeSyncPillState(failed), "en")).toBe("Unsaved: The folder refused the write.");
    expect(syncPillText(computeSyncPillState(failed), "de")).toBe("Nicht gespeichert: Der Ordner hat das Schreiben abgelehnt.");
    const unacknowledged = status({ persisted: false });
    expect(syncPillText(computeSyncPillState(unacknowledged), "en")).toBe("Unsaved");
    const texts = { live: "live", connecting: "c", reconnecting: "r", offline: "o", peers: "1 peer", saved: "Saved", unsaved: "Unsaved", pending: "p" };
    for (const s of [failed, unacknowledged, status({})]) {
      const pillSaysSaved = computeSyncPillState(s).kind === "persisted";
      expect(syncStatusLabelV1(s, texts)!.includes("Saved")).toBe(pillSaysSaved);
    }
  });

  it("words every typed failure in en and de and shows an unknown code raw", () => {
    for (const code of Object.keys(SAVE_FAILURE_LABELS_V1)) {
      expect(saveFailureTextV1({ code }, "en")).not.toBe(saveFailureTextV1({ code }, "de"));
      expect(saveFailureTextV1({ code }, "fr")).toBe(`${SAVE_FAILURE_LABELS_V1[code]!.en} | ${SAVE_FAILURE_LABELS_V1[code]!.de}`);
    }
    expect(saveFailureTextV1({ code: "novel" }, "en")).toBe("The save was not acknowledged.");
    expect(saveFailureTextV1({ code: "novel", message: "disk detached" }, "de")).toBe("Das Speichern wurde nicht bestätigt. disk detached");
    expect(saveFailureTextV1({ code: "folder-write-failed" }, "de-DE")).toBe(SAVE_FAILURE_LABELS_V1["folder-write-failed"].de);
  });
});

describe("guardDocumentCloseV1", () => {
  const unsaved: SavedStateV1 = { kind: "unsaved", pending: 1, failure: null };
  const port = (saved: () => SavedStateV1, flush: "flushed" | "failed" | "cancelled", confirm: boolean) => {
    const log: string[] = [];
    return { log, port: { saved, signal: new AbortController().signal, flush: async () => { log.push("flush"); return flush; }, confirmDiscard: async () => { log.push("confirm"); return confirm; } } };
  };

  it("closes a saved document without flushing or asking", async () => {
    const { log, port: p } = port(() => ({ kind: "saved" }), "flushed", false);
    expect(await guardDocumentCloseV1(p)).toBe("close");
    expect(log).toEqual([]);
  });

  it("flushes first and closes once acknowledged", async () => {
    let state: SavedStateV1 = unsaved;
    const { log, port: p } = port(() => state, "flushed", false);
    const flushing = { ...p, flush: async () => { log.push("flush"); state = { kind: "saved" }; return "flushed" as const; } };
    expect(await guardDocumentCloseV1(flushing)).toBe("close");
    expect(log).toEqual(["flush"]);
  });

  it("prompts after a failed or cancelled flush and honours the answer", async () => {
    for (const outcome of ["failed", "cancelled"] as const) {
      const keep = port(() => unsaved, outcome, false);
      expect(await guardDocumentCloseV1(keep.port)).toBe("stay");
      expect(keep.log).toEqual(["flush", "confirm"]);
      expect(await guardDocumentCloseV1(port(() => unsaved, outcome, true).port)).toBe("close");
    }
  });

  it("does not trust a flush that left the document unsaved", async () => {
    const { log, port: p } = port(() => unsaved, "flushed", false);
    expect(await guardDocumentCloseV1(p)).toBe("stay");
    expect(log).toEqual(["flush", "confirm"]);
  });
});

describe("awaitSavedV1", () => {
  const unsaved: SavedStateV1 = { kind: "unsaved", pending: 1, failure: null };
  it("resolves flushed as soon as the state turns saved", async () => {
    let calls = 0;
    expect(await awaitSavedV1({ saved: () => (++calls > 2 ? { kind: "saved" } : unsaved), signal: new AbortController().signal, timeoutMs: 1000, pollMs: 1 })).toBe("flushed");
  });
  it("fails at the deadline and cancels on abort", async () => {
    expect(await awaitSavedV1({ saved: () => unsaved, signal: new AbortController().signal, timeoutMs: 20, pollMs: 5 })).toBe("failed");
    const abort = new AbortController();
    setTimeout(() => abort.abort(), 10);
    expect(await awaitSavedV1({ saved: () => unsaved, signal: abort.signal, timeoutMs: 5000, pollMs: 5 })).toBe("cancelled");
  });
});

describe("guard and await hardening", () => {
  it("treats a rejecting flush as a failed flush and asks before closing", async () => {
    const log: string[] = [];
    const decision = await guardDocumentCloseV1({ saved: () => ({ kind: "unsaved", pending: 1, failure: null }), signal: new AbortController().signal, flush: async () => { throw new Error("worker gone"); }, confirmDiscard: async () => { log.push("confirm"); return false; } });
    expect(decision).toBe("stay");
    expect(log).toEqual(["confirm"]);
  });

  it("leaves no abort listener behind after polling", async () => {
    let live = 0;
    const listeners = new Set<() => void>();
    const signal = { aborted: false, addEventListener: (_type: string, listener: () => void) => { live += 1; listeners.add(listener); }, removeEventListener: (_type: string, listener: () => void) => { if (listeners.delete(listener)) live -= 1; } } as unknown as AbortSignal;
    let calls = 0;
    await awaitSavedV1({ saved: () => (++calls > 4 ? { kind: "saved" } : { kind: "unsaved", pending: 1, failure: null }), signal, timeoutMs: 1000, pollMs: 1 });
    expect(live).toBe(0);
  });
});
