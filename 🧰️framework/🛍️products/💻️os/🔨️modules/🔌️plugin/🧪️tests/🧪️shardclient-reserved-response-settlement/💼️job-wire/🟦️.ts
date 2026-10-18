/** 💼️ The shard client job wire tests. */
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🧳 Registers the job wire tests of the shard client. */
export function registerJobWireTests(kit: ShardClientSettlementTestKit): void {
  const { activateActor, describe, expect, harness, it } = kit;
  describe("ShardClient job wire", () => {
    // 💼️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-J. Nothing in production ever called these three
    // methods, so their declared shapes had drifted from what the shard worker actually needs: `job`
    // and `budget.fuel` are WIT `u64`s (a `number` is rejected by the guest's own lowering),
    // `step-job` answers the `{status, …}` union this module declares (the worker normalizes jco's
    // `{tag, val}` variant into it), and `cancel-job` had no worker route at all — it was posted
    // without a `requestId` and would have hit the worker's `default: throw`.
    it("carries u64 job identities as bigint and settles start, step and cancel against the worker", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "a");

      const started = client.startJob("a", 42n, "semio.puzzle3d.fill", new Uint8Array([1, 2, 3]));
      const startMessage = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: string; readonly job: unknown };
      expect(startMessage.kind).toBe("startJob");
      expect(typeof startMessage.job).toBe("bigint");
      workers[0]!.deliver({ kind: "result", requestId: startMessage.requestId, ok: true, value: undefined });
      await started;

      const stepped = client.stepJob("a", 42n, { fuel: 4000n, deadlineMs: 8 });
      const stepMessage = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: string; readonly job: unknown; readonly budget: { readonly fuel: unknown } };
      expect(stepMessage.kind).toBe("stepJob");
      expect(typeof stepMessage.job).toBe("bigint");
      expect(typeof stepMessage.budget.fuel).toBe("bigint");
      workers[0]!.deliver({ kind: "result", requestId: stepMessage.requestId, ok: true, value: { status: "done", value: new Uint8Array([9]) } });
      expect(await stepped).toEqual({ status: "done", value: new Uint8Array([9]) });

      const cancelled = client.cancelJob("a", 42n);
      const cancelMessage = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: unknown; readonly job: unknown };
      expect(cancelMessage.kind).toBe("cancelJob");
      expect(typeof cancelMessage.requestId).toBe("string");
      expect(typeof cancelMessage.job).toBe("bigint");
      workers[0]!.deliver({ kind: "result", requestId: cancelMessage.requestId as string, ok: true, value: undefined });
      await cancelled;
      client.disposeAll();
    });
  });
}
