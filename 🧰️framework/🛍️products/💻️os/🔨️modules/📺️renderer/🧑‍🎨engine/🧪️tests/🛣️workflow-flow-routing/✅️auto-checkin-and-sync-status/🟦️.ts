/** ✅️ The auto check-in scheduler, sync status pill, fault boundary and spawned focus tests. */
import { createElement, type ReactElement } from "react";
import { automaticCheckinWaitsV1 } from "../../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import automaticCheckinCorpus from "../../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️automatic-checkin/🔣️.json";
import { render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { uiDataLabel } from "@semio-tech/ui-react";
import { AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, computeSyncPillState, syncPillText, ShellFaultBoundary, studioPanelFocusingSpawned, viewStateWithSpacePanel } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔄 Registers the auto check-in scheduler, sync status pill, check-in affordance, fault boundary and spawned focus tests. */
export function registerAutoCheckinAndSyncStatusTests(): void {
  describe("AutoCheckinScheduler (§C5 auto check-in)", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    it("3 edits then idle ⇒ exactly one commitCheckpoint", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(2);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(3);
      expect(onCheckpoint).not.toHaveBeenCalled();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 3);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("≥ 200 uncommitted edits ⇒ checkpoint without waiting for idle", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD - 1);
      expect(onCheckpoint).not.toHaveBeenCalled();
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD + 1);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("notify(0) (a landed checkpoint) clears the pending latch for a fresh idle window later", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      scheduler.notify(0);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(2);
    });

    it("cancel() stops a pending idle timer (unmount/session-switch)", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      scheduler.cancel();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);
      expect(onCheckpoint).not.toHaveBeenCalled();
    });

    it("an automatic check-in waits while its document cannot take it and is asked for again, per the shared corpus (live finding O4)", () => {
      for (const row of automaticCheckinCorpus.waits) expect(automaticCheckinWaitsV1(row.document), row.id).toBe(row.waits);
      for (const timeline of automaticCheckinCorpus.timelines) {
        vi.useFakeTimers();
        const start = Date.now();
        let document = { loading: false, attached: false, bound: false };
        const dispatched: number[] = [];
        const deferred: number[] = [];
        const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {
          if (automaticCheckinWaitsV1(document)) {
            deferred.push(Date.now() - start);
            scheduler.defer();
          } else dispatched.push(Date.now() - start);
        }, automaticCheckinCorpus.idleMs, automaticCheckinCorpus.threshold);
        for (const event of timeline.events as readonly { readonly notify?: number; readonly advance?: number; readonly document?: typeof document }[]) {
          if (event.document !== undefined) document = event.document;
          else if (event.notify !== undefined) scheduler.notify(event.notify);
          else vi.advanceTimersByTime(event.advance ?? 0);
        }
        expect([dispatched, deferred], timeline.id).toEqual([timeline.dispatched, timeline.deferred]);
        scheduler.cancel();
        vi.useRealTimers();
      }
    });
  });
  describe("sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...))", () => {
    it("persisted: a live remote with nothing pending", () => {
      const state = computeSyncPillState({ persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "persisted" });
      expect(syncPillText(state, "en")).toBe("Persisted");
      expect(syncPillText(state, "de")).toBe("Gespeichert");
    });

    it("pending(n): a live remote with unacked mutations", () => {
      const state = computeSyncPillState({ persisted: false, pendingMutations: 3, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "pending", count: 3 });
      expect(syncPillText(state, "en")).toBe("Pending (3)");
      expect(syncPillText(state, "de")).toBe("Ausstehend (3)");
    });

    it("remote(connecting|backoff|detached): a non-live remote takes priority over a pending count", () => {
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "en")).toBe("Remote: connecting");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "de")).toBe("Remote: verbindet");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 9, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "en")).toBe("Remote: detached");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "de")).toBe("Remote: getrennt");
    });

    it("no status observed yet reads as remote(detached)", () => {
      expect(computeSyncPillState(null)).toEqual({ kind: "remote", remote: "detached" });
    });
  });
  it("canCheckIn is true only for an editor role — viewer gets no affordance and no auto timer", () => {
    expect(canCheckIn("editor")).toBe(true);
    expect(canCheckIn("viewer")).toBe(false);
    expect(canCheckIn(undefined)).toBe(false);
  });
  it("isolates render faults in ShellFaultBoundary", () => {
    function FaultyChild(): ReactElement {
      throw new Error("boom");
    }
    const { getByRole } = render(createElement(ShellFaultBoundary, { boundaryId: "test", fallbackLabel: uiDataLabel("Fault"), children: createElement(FaultyChild) }));
    expect(getByRole("alert").textContent).toContain("boom");
  });
  it("folds spawned focus into viewState so a subsequent host-effect session write keeps activeSpawnedId", async () => {
    const panel = { activePanelTab: "s-play-catalogue", spawnedApps: [] as const };
    const spawned = { id: "app-draw-1", pluginId: "draw", instanceId: 1, appId: "draw", label: "Semio Emblem", breadcrumb: ["draw"] };
    const focused = studioPanelFocusingSpawned(panel, spawned);
    expect(focused.activeSpawnedId).toBe("app-draw-1");
    expect(focused.spawnedApps).toEqual([spawned]);
    const baseViewState = { panelJson: JSON.stringify(panel) };
    const nextViewState = viewStateWithSpacePanel(baseViewState, focused);
    const { parsePanelState } = await import("../../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts");
    expect(parsePanelState(nextViewState)?.activeSpawnedId).toBe("app-draw-1");
    const refocused = studioPanelFocusingSpawned(focused, { ...spawned, label: "Renamed" });
    expect(refocused.spawnedApps).toHaveLength(1);
    expect(refocused.spawnedApps[0]?.label).toBe("Renamed");
    expect(refocused.activeSpawnedId).toBe("app-draw-1");
  });
}
