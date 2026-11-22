/** 💼️ The spawned-job ledger against the language-neutral `💼️job-ledger` vectors the Rust ledger also drives. */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fixture from "../🧫️fixtures/💼️job-ledger/🔣️.json";
import { beginSpawnedJobV1, cancelSpawnedJobV1, resetSpawnedJobsForTestsV1, spawnedJobsSnapshotV1, SPAWNED_JOB_STEP_PUBLISH_MS, type SpawnedJobLedgerEntryV1 } from "../../🧱️elements/🔌️PluginRuntime/💼️job-ledger/🟦️.ts";

type Op =
  | { op: "begin"; job: { key: string; pluginId: string; actorId: string; instanceId: number; job: number; kind: string; startedAtMs: number }; opened: boolean }
  | { op: "step"; key: string; steps: number; progressed: boolean; atMs: number }
  | { op: "end"; key: string }
  | { op: "cancel"; key: string; accepted: boolean }
  | { op: "cancelCalls"; key: string; count: number }
  | { op: "snapshot"; rows: readonly Record<string, unknown>[] };

describe("spawned job ledger vectors", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    resetSpawnedJobsForTestsV1();
  });
  afterEach(() => {
    resetSpawnedJobsForTestsV1();
    vi.useRealTimers();
  });
  for (const row of fixture.cases) {
    it(row.id, () => {
      const handles = new Map<string, SpawnedJobLedgerEntryV1>();
      const cancels = new Map<string, number>();
      for (const op of row.ops as readonly Op[]) {
        switch (op.op) {
          case "begin": {
            const opened = !spawnedJobsSnapshotV1().some((live) => live.key === op.job.key);
            const handle = beginSpawnedJobV1({ ...op.job, job: BigInt(op.job.job), cancel: () => cancels.set(op.job.key, (cancels.get(op.job.key) ?? 0) + 1) });
            expect(opened).toBe(op.opened);
            if (opened) handles.set(op.job.key, handle);
            break;
          }
          case "step":
            handles.get(op.key)?.step(op.steps, op.progressed, op.atMs);
            vi.advanceTimersByTime(SPAWNED_JOB_STEP_PUBLISH_MS);
            break;
          case "end":
            handles.get(op.key)?.end();
            break;
          case "cancel":
            expect(cancelSpawnedJobV1(op.key)).toBe(op.accepted);
            break;
          case "cancelCalls":
            expect(cancels.get(op.key) ?? 0).toBe(op.count);
            break;
          case "snapshot":
            expect(spawnedJobsSnapshotV1().map(({ key, steps, reportsProgress, progressAtMs, cancelling }) => ({ key, steps, reportsProgress, progressAtMs, cancelling }))).toEqual(op.rows);
            break;
        }
      }
    });
  }
});
