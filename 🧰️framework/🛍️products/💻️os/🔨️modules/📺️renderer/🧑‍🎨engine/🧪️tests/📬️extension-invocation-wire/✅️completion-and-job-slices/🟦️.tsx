/** ✅️ The exact completion, isolated job, job slice and task ledger tests. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client";
import { cancelSpawnedJobV1, resetSpawnedJobsForTestsV1, spawnedJobsSnapshotV1 } from "../../../🧱️elements/🔌️PluginRuntime/💼️job-ledger/🟦️.ts";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";
import type { CompletionPublicationHelpers } from "../🛠️requester-helper/🟦️.tsx";

/** 🧾 Registers the exact completion settlement, isolated job completion, job stepping, per slice bound and task ledger cancel tests. */
export function registerCompletionAndJobSliceTests(kit: InvocationWireTestKit, helpers: CompletionPublicationHelpers): void {
  const { DEFAULT_SHARD_BUDGET, encodePackValue, expect, it, yieldPluginUiContinuation } = kit;
  const { withRequester } = helpers;
    it("settles the exact completion and returns frames and host effects", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      const bytes = (value: unknown) => Array.from(encodePackValue(value));
      let instance = 0;
      const submitted: ShardEventEnvelope[][] = [];
      await withRequester(async (_actor, events) => {
        submitted.push([...events]);
        if (submitted.length === 1) return { uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "more-work" } };
        if (submitted.length === 2) return {
          uiPatches: [],
          effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } }],
          nextWake: null, status: { tag: "idle" },
        };
        return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
      }, async (handle, opened) => {
        instance = opened;
        const req = BigInt(fixture.requestIds[2]!);
        const outcome = { ok: encodePackValue(fixture.response) };
        const response = await handle.captureExtensionCompletion!(instance, req).complete(outcome);
        expect(submitted).toEqual([[{ kind: "completed", payload: { req, outcome: { tag: "ok", val: Array.from(outcome.ok) } } }], []]);
        expect(response).toMatchObject({ output: fixture.response, requestedEffects: [{ notify: { message: fixture.completion.notification } }], uiScope: fixture.completion.uiScope, historyPatch: fixture.completion.historyPatch });
      });
    });
    it("publishes isolated job completion without requiring another user command", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🏁️job-completion-publication/🔣️.json");
              for (const row of fixture.cases) {
        let instance = 0;
        const submitted: ShardEventEnvelope[][] = [];
        const completions: unknown[] = [];
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const history = { cursor: 2, upserts: [] };
        let terminal = false;
        let published = false;
        let remaining = row.continuations;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          terminal ||= events.some(event => event.kind === "job-completed");
          if (published) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          if (terminal && remaining-- > 0) return { uiPatches: [], effects: [], nextWake: null, status: { tag: row.handoffStatus } };
          published = terminal;
          return {
            uiPatches: [], nextWake: null, status: { tag: "idle" },
            effects: terminal ? [
              { tag: "notify", val: { message: row.message } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ OperationCompleted: { operation: BigInt(row.operation), revision: BigInt(row.revision), ui_scope: bytes({ kind: "full" }), history_patch: bytes(history) } })) } },
            ] : [
              { tag: "spawn-job", val: { job: BigInt(row.job), kind: "semio.draw.mounted-vector", input: new Uint8Array([1]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(null), diagnostics: bytes([]), ui_scope: bytes({ kind: "none" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } })) } },
            ],
          };
        }, async (handle, opened) => {
          instance = opened;
          const unsubscribe = handle.subscribeOperationCompletions(instance, completion => completions.push(completion));
          try {
            await handle.captureExtensionCompletion!(instance, 1n).complete({ ok: encodePackValue(null) });
            for (let turn = 0; turn < 256 && completions.length === 0; turn += 1) await yieldPluginUiContinuation();
            if (row.continuations === 0) {
              await yieldPluginUiContinuation();
              expect(submitted, row.id).toHaveLength(2);
            }
            expect.soft(completions, row.id).toEqual([{ instanceId: instance, operation: BigInt(row.operation), revision: BigInt(row.revision), uiScope: { kind: "full" }, historyPatch: history, requestedEffects: [{ notify: { message: row.message } }], terminalOutput: undefined }]);
            expect.soft(submitted.length, row.id).toBeGreaterThanOrEqual(2 + row.continuations);
            expect(submitted.slice(1).flat().map(event => event.kind), row.id).toEqual(["job-completed"]);
          } finally { unsubscribe(); }
        }, { jobs: { step: () => ({ status: row.outcome as "done" | "failed", value: new Uint8Array([1]) }) } });
      }
    });
    it("starts an isolated job, steps it to its terminal and feeds job-completed back into the actor", async () => {
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const bytes = (value: unknown) => Array.from(encodePackValue(value));
      const started: Array<{ job: bigint; kind: string; input: readonly number[] }> = [];
      const stepped: bigint[] = [];
      const submitted: ShardEventEnvelope[][] = [];
      let instance = 0;
      await withRequester(async (_actor, events) => {
        submitted.push([...events]);
        if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        return {
          uiPatches: [],
          effects: [
            { tag: "spawn-job", val: { job: 7n, kind: "semio.puzzle3d.fill", input: new Uint8Array([4, 2]), placement: { tag: "isolated" } } },
            { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
          ],
          nextWake: null,
          status: { tag: "idle" },
        };
      }, async (handle, opened) => {
        instance = opened;
        const response = await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
        expect(response.requestedEffects).toEqual([]);
        for (let turn = 0; turn < 64 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
        expect(started).toEqual([{ job: 7n, kind: "semio.puzzle3d.fill", input: [4, 2] }]);
        expect(stepped).toEqual([7n, 7n, 7n]);
        const completion = submitted.flat().find(event => event.kind === "job-completed");
        expect(completion?.payload).toEqual({ job: 7n, outcome: { tag: "ok", val: [1] } });
      }, {
        jobs: {
          start: (job, kind, input) => { started.push({ job, kind, input: Array.from(input) }); },
          step: (job) => { stepped.push(job); return stepped.length < 3 ? { status: "running" } : { status: "done", value: new Uint8Array([1]) }; },
        },
      });
    });
    it("bounds a job per slice and never ends one the guest is still running", async () => {
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const bytes = (value: unknown) => Array.from(encodePackValue(value));
      const legacyHostLifetimeCap = 1 << 16;
      const slicesBeforeTerminal = legacyHostLifetimeCap + 1_024;
      const budgets = new Set<string>();
      const cancelled: bigint[] = [];
      const submitted: ShardEventEnvelope[][] = [];
      let stepped = 0;
      let instance = 0;
      await withRequester(async (_actor, events) => {
        submitted.push([...events]);
        if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        return {
          uiPatches: [],
          effects: [
            { tag: "spawn-job", val: { job: 11n, kind: "semio.puzzle3d.fill", input: new Uint8Array([1]), placement: { tag: "isolated" } } },
            { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
          ],
          nextWake: null,
          status: { tag: "idle" },
        };
      }, async (handle, opened) => {
        instance = opened;
        await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
        for (let turn = 0; turn < 4_096 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
        expect(stepped).toBe(slicesBeforeTerminal);
        expect(cancelled).toEqual([]);
        expect([...budgets]).toEqual([`${DEFAULT_SHARD_BUDGET.fuel}/${DEFAULT_SHARD_BUDGET.wallMs}`]);
        const completion = submitted.flat().find(event => event.kind === "job-completed");
        expect(completion?.payload).toEqual({ job: 11n, outcome: { tag: "ok", val: [9] } });
      }, {
        jobs: {
          step: (_job, budget) => {
            stepped += 1;
            budgets.add(`${budget.fuel}/${budget.deadlineMs}`);
            return stepped < slicesBeforeTerminal ? { status: "running" } : { status: "done", value: new Uint8Array([9]) };
          },
          cancel: (job) => { cancelled.push(job); },
        },
      });
    });
    it("lists a driven job in the task-manager ledger and routes a cancel to the guest's own cancel-job", async () => {
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const bytes = (value: unknown) => Array.from(encodePackValue(value));
      resetSpawnedJobsForTestsV1();
      const cancelled: bigint[] = [];
      const submitted: ShardEventEnvelope[][] = [];
      let instance = 0;
      let released = false;
      await withRequester(async (_actor, events) => {
        submitted.push([...events]);
        if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        return {
          uiPatches: [],
          effects: [
            { tag: "spawn-job", val: { job: 13n, kind: "semio.puzzle3d.fill", input: new Uint8Array([2]), placement: { tag: "isolated" } } },
            { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
          ],
          nextWake: null,
          status: { tag: "idle" },
        };
      }, async (handle, opened) => {
        instance = opened;
        await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
        for (let turn = 0; turn < 256 && (spawnedJobsSnapshotV1()[0]?.steps ?? 0) === 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 5));
        const [row] = spawnedJobsSnapshotV1();
        expect(row).toMatchObject({ kind: "semio.puzzle3d.fill", job: 13n, cancelling: false });
        expect(row!.steps).toBeGreaterThan(0);
        expect(cancelSpawnedJobV1(row!.key)).toBe(true);
        expect(spawnedJobsSnapshotV1()[0]?.cancelling).toBe(true);
        for (let turn = 0; turn < 256 && cancelled.length === 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
        expect(cancelled).toEqual([13n]);
        released = true;
        for (let turn = 0; turn < 1_024 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
        const completion = submitted.flat().find(event => event.kind === "job-completed");
        expect(completion?.payload).toMatchObject({ job: 13n, outcome: { tag: "fault" } });
        for (let turn = 0; turn < 256 && spawnedJobsSnapshotV1().length > 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
        expect(spawnedJobsSnapshotV1()).toEqual([]);
        expect(cancelSpawnedJobV1(row!.key)).toBe(false);
      }, {
        jobs: {
          step: () => (released ? { status: "failed", value: new TextEncoder().encode("job.unknown") } : { status: "running" }),
          cancel: (job) => { cancelled.push(job); },
        },
      });
    });
}
