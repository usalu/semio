type TestSource = { readonly directory: string; readonly url: string };

/** 🛑 LAW: `ShardClient.cancelJob` sends a REQUEST (`{ kind: "cancelJob", requestId }`, `🟦️.ts`'s own
 * `OutboundMessage` union), so the generated worker owes it a `"result"` and at least one liveness
 * beat — exactly like `startJob`/`stepJob`/`turn`. A `cancelJob` the worker answers with nothing
 * leaves the request outstanding in `pending` forever while no beat is recorded, and the watchdog then
 * reports the one thing that never happened: `the worker was silent for N ms; outstanding: cancelJob
 * … A guest turn that never yields`.
 *
 * 🧊️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42, battery wasm #56: `engagement-abort` PASSed at
 * 720.9 s and pushed `Effect::CancelJob`; the very next outliner interaction waited 30 268 ms and
 * `shard 0` was terminated with `outstanding: cancelJob puzzle#1 started 18603 ms ago`, taking
 * `puzzle#1` and every later verdict with it. The guest's own `jobs::cancel-job` spends ZERO teardown
 * units (`engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one`, puzzle3d): the
 * shard died of an unanswered request, not of guest work.
 * @see 🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts */
export async function registerCancelJobReplyTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: import("../../🟦️.ts").ShardClientTestDependenciesV1,
  _testSource: TestSource,
): Promise<void> {
  const { SHARD_LIVENESS_POLICY } = dependencies;
  const { describe, it, expect } = vitest;

  const workerContext = async (cancelJob: (job: bigint) => Promise<void>) => {
    const vm = await import("node:vm");
    const { shardWorkerSource } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
    const posted: Record<string, unknown>[] = [];
    let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
    const context = vm.createContext({
      WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
      TextDecoder,
      console: { log: () => {}, error: () => {}, warn: () => {} },
      setInterval: () => null,
      clearInterval: () => {},
      self: { postMessage: (message: Record<string, unknown>) => posted.push(message), addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } },
      api: { cancelJob },
    });
    new vm.Script(shardWorkerSource()).runInContext(context);
    new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(context);
    const send = dispatch as unknown as ((event: { data: Record<string, unknown> }) => Promise<void>) | null;
    if (!send) throw new Error("Missing generated worker dispatcher");
    return { posted, send };
  };

  describe("ShardWorkerCancelJobReply", () => {
    it("answers a cancelJob request with a result and a beat, and hands the job on to the guest", async () => {
      const seen: bigint[] = [];
      const { posted, send } = await workerContext(async (job) => {
        seen.push(job);
      });
      await send({ data: { kind: "cancelJob", requestId: "c0", actorId: "a", job: 5n } });
      const result = posted.find((message) => message.kind === "result" && message.requestId === "c0");
      expect(seen).toEqual([5n]);
      expect(result, "a cancelJob request the worker never answers is the outstanding request the watchdog kills the shard over").toBeTruthy();
      expect(result?.ok).toBe(true);
      expect(posted.some((message) => message.kind === "heartbeat"), "an unanswered request is also an unbeaten one — the worker looks dead rather than busy").toBe(true);
      expect(SHARD_LIVENESS_POLICY.missedLimit).toBeGreaterThan(0);
      console.log(`shard-worker.cancel-job-reply posts=${posted.map((message) => message.kind).join(",")}`);
    });

    it("still answers a cancelJob for an actor that is already gone, instead of stranding the request", async () => {
      const { posted, send } = await workerContext(async () => {
        throw new Error("a disposed actor must never be asked");
      });
      await send({ data: { kind: "cancelJob", requestId: "c1", actorId: "gone", job: 9n } });
      const result = posted.find((message) => message.kind === "result" && message.requestId === "c1");
      expect(result, "a cancel for a disposed actor is a no-op the caller must still be told about").toBeTruthy();
      expect(result?.ok).toBe(true);
    });

    it("propagates a guest cancel failure as an error result rather than silence", async () => {
      const { posted, send } = await workerContext(async () => {
        throw new Error("guest cancel trapped");
      });
      await send({ data: { kind: "cancelJob", requestId: "c2", actorId: "a", job: 11n } });
      const result = posted.find((message) => message.kind === "result" && message.requestId === "c2");
      expect(result).toBeTruthy();
      expect(result?.ok).toBe(false);
    });
  });
}
