/** ⚖ submitPluginTurn: lane ordering, coalescing, teardown and per-actor serialization. */
import { flushMicrotasks, createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTurnSchedulerLanesTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { submitPluginTurn, teardownPluginActor } = dependencies;
  const { withFakeShardClient } = createPluginRuntimeSupport(dependencies);
    //#region 🧪️terra-web-plugin-runtime
    describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
      it("dispatches a queued Interactive-lane turn before an already-queued UserVisible-lane turn for the SAME actor, regardless of arrival order", async () => {
        const dispatchOrder: string[] = [];
        await withFakeShardClient(
          async (_actorId, events) => {
            dispatchOrder.push((events[0]!.payload as { readonly marker: string }).marker);
            return { uiPatches: [], effects: [], nextWake: null };
          },
          async () => {
            const actorId = "turn-test-lane-actor";
            const userVisible = submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: "user-visible-1" } }], "UserVisible", "surface-visible");
            const interactive = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "interactive-1" } }], "Interactive");
            await Promise.all([userVisible, interactive]);
            expect(dispatchOrder).toEqual(["interactive-1", "user-visible-1"]);
          },
        );
      });

      it("collapses a 200-call coalescing burst to a single dispatched turn, resolving EVERY waiter (not just the last) with the winning result", async () => {
        let dispatchCount = 0;
        await withFakeShardClient(
          async (_actorId, events) => {
            dispatchCount += 1;
            return { uiPatches: [], effects: [{ tag: "notify", val: { message: String((events[0]!.payload as { readonly marker: number }).marker) } }], nextWake: null };
          },
          async () => {
            const actorId = "turn-test-coalesce-actor";
            const results = await Promise.all(
              Array.from({ length: 200 }, (_, index) => submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: index } }], "UserVisible", "surface-visible")),
            );
            expect(dispatchCount).toBe(1);
            for (const result of results) expect(result.effects).toEqual(results[0]!.effects);
          },
        );
      });

      it("teardown rejects queued turn waiters before disposing the actor transport", async () => {
        let release!: () => void;
        const gate = new Promise<unknown>((resolve) => {
          release = () => resolve({ uiPatches: [], effects: [], nextWake: null });
        });
        await withFakeShardClient(
          async () => gate,
          async () => {
            const actorId = "turn-test-teardown-actor";
            const inFlight = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "in-flight" } }], "Interactive");
            await flushMicrotasks();
            const queued = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "queued" } }], "Interactive");
            teardownPluginActor(actorId);
            await expect(queued).rejects.toThrow(`plugin actor ${actorId} disposed`);
            release();
            await expect(inFlight).resolves.toMatchObject({ uiPatches: [], effects: [] });
          },
        );
      });

      it("surfaces Rejected once an actor's mailbox is genuinely full of distinct turns, instead of growing without bound", async () => {
        await withFakeShardClient(
          () => new Promise(() => {}),
          async () => {
            const actorId = "turn-test-rejected-actor";
            const settled: Array<"accepted" | "rejected"> = [];
            for (let index = 0; index < 41; index += 1) {
              submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: `t-${index}` } }], "Interactive").then(
                () => settled.push("accepted"),
                () => settled.push("rejected"),
              );
            }
            await flushMicrotasks();
            expect(settled.filter((outcome) => outcome === "rejected").length).toBeGreaterThan(0);
          },
        );
      });

      it("never dispatches a second turn for an actor before the first settles, even while a DIFFERENT actor's turns run concurrently", async () => {
        const inFlightByActor = new Map<string, number>();
        const maxInFlightByActor = new Map<string, number>();
        await withFakeShardClient(
          async (actorId) => {
            inFlightByActor.set(actorId, (inFlightByActor.get(actorId) ?? 0) + 1);
            maxInFlightByActor.set(actorId, Math.max(maxInFlightByActor.get(actorId) ?? 0, inFlightByActor.get(actorId)!));
            await flushMicrotasks(2);
            inFlightByActor.set(actorId, inFlightByActor.get(actorId)! - 1);
            return { uiPatches: [], effects: [], nextWake: null };
          },
          async () => {
            const actorA = "turn-test-interleave-a";
            const actorB = "turn-test-interleave-b";
            await Promise.all([
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorB, [{ kind: "app-command", payload: {} }], "Interactive"),
            ]);
            expect(maxInFlightByActor.get(actorA)).toBe(1);
            expect(maxInFlightByActor.get(actorB)).toBe(1);
          },
        );
      });
    });
  //#endregion 🧪️terra-web-plugin-runtime
}
