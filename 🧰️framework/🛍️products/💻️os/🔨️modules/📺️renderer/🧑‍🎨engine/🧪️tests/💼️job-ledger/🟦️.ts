/** 💼️ The plugin runtime's live spawned-job ledger — the task manager's job rows. Pins that a job is listed
 * exactly while its drive runs, that a duplicated `spawn-job` can never close the real drive's row, that a
 * cancel reaches the job's own owner once and only marks the row `cancelling`, and that step updates are
 * coalesced while opening and closing publish at once. */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { beginSpawnedJobV1, cancelSpawnedJobV1, resetSpawnedJobsForTestsV1, spawnedJobsSnapshotV1, subscribeSpawnedJobsV1, SPAWNED_JOB_STEP_PUBLISH_MS } from "../../🧱️elements/🔌️PluginRuntime/💼️job-ledger/🟦️.ts";

const JOB = { key: "actor-1#7", pluginId: "s.puzzle", actorId: "actor-1", instanceId: 3, job: 7n, kind: "semio.puzzle3d.fill", startedAtMs: 1_000 };

describe("spawned job ledger", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    resetSpawnedJobsForTestsV1();
  });
  afterEach(() => {
    resetSpawnedJobsForTestsV1();
    vi.useRealTimers();
  });

  it("lists a job exactly while its drive runs", () => {
    const entry = beginSpawnedJobV1({ ...JOB, cancel: () => undefined });
    expect(spawnedJobsSnapshotV1().map((row) => row.key)).toEqual(["actor-1#7"]);
    expect(spawnedJobsSnapshotV1()[0]).toMatchObject({ steps: 0, cancelling: false, reportsProgress: false, progressAtMs: 1_000 });
    entry.end();
    expect(spawnedJobsSnapshotV1()).toEqual([]);
  });

  it("never lets a duplicated spawn close the real drive's row", () => {
    const real = beginSpawnedJobV1({ ...JOB, cancel: () => undefined });
    const duplicate = beginSpawnedJobV1({ ...JOB, cancel: () => undefined });
    duplicate.end();
    expect(spawnedJobsSnapshotV1()).toHaveLength(1);
    real.end();
    expect(spawnedJobsSnapshotV1()).toHaveLength(0);
  });

  it("coalesces step updates and publishes opening and closing at once", () => {
    const listener = vi.fn();
    subscribeSpawnedJobsV1(listener);
    const entry = beginSpawnedJobV1({ ...JOB, cancel: () => undefined });
    expect(listener).toHaveBeenCalledTimes(1);
    entry.step(64, false, 2_000);
    entry.step(128, true, 3_000);
    expect(listener).toHaveBeenCalledTimes(1);
    expect(spawnedJobsSnapshotV1()[0]!.steps).toBe(0);
    vi.advanceTimersByTime(SPAWNED_JOB_STEP_PUBLISH_MS);
    expect(listener).toHaveBeenCalledTimes(2);
    expect(spawnedJobsSnapshotV1()[0]).toMatchObject({ steps: 128, reportsProgress: true, progressAtMs: 3_000 });
    entry.end();
    expect(listener).toHaveBeenCalledTimes(3);
  });

  it("asks the job's own owner to cancel once and only marks the row cancelling", () => {
    const cancel = vi.fn();
    const entry = beginSpawnedJobV1({ ...JOB, cancel });
    expect(cancelSpawnedJobV1("actor-1#7")).toBe(true);
    expect(cancelSpawnedJobV1("actor-1#7")).toBe(false);
    expect(cancel).toHaveBeenCalledTimes(1);
    expect(spawnedJobsSnapshotV1()[0]!.cancelling).toBe(true);
    expect(cancelSpawnedJobV1("nobody#1")).toBe(false);
    entry.end();
    expect(spawnedJobsSnapshotV1()).toEqual([]);
  });
});
