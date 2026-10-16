/** 🔂 submitPluginTurn: owed settles, retained surface acknowledgement and continuation ceilings. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { WireTurnResult, WireVariant } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerSettleContinuationsTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { encodeActorUiPatchReceipt, patchAckEvents, PLUGIN_UI_CONTINUATION_BATCH_SIZE, PLUGIN_UI_CONTINUATION_LIMIT, PLUGIN_UI_QUIESCENT_CONTINUATIONS, PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT, pluginSurfaceRef, retainedSurfaceId, settlePluginTurn, yieldPluginUiContinuation } = dependencies;
  const { withFakeShardClient } = createPluginRuntimeSupport(dependencies);
    //#region 🧪️terra-web-plugin-runtime
    describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
      /** 🎞️ The owed clause is a GRACE, not a hold: a live tool run keeps the guest in `more-work` for
       * its whole run, and `settleOwesRequestedSurfacesV1` cannot tell that from "still reconciling my
       * surface". Measured on release :6013 with a 100-object fill (ticket
       * 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS): 3 refreshes instead of 12, one `refresh.turn`
       * lasting 1 281 ms, progress frames stopping at +442 ms, and the scene stuck at 36–40 of 100
       * provisional objects 20 s after the run completed. */
      it("lets an owed settle hold its surface for at most one crossing's wall, then yields to the progress lane", async () => {
        const invocation = (inReplyTo: number) => ({ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: inReplyTo, output: [], diagnostics: [], ui_scope: [1], history_patch: [], messages: [], mutations: [], inverse_group: [] } })) } });
        const turn = (effects: unknown[]) => ({ uiPatches: [], effects, nextWake: null, status: { tag: "more-work" } }) as unknown as WireTurnResult;
        const { settleYieldsToRefreshV1, PLUGIN_OPERATION_REFRESH_SLICE_MS, PLUGIN_OPERATION_OWED_SLICE_MS } = dependencies;
        const results = [turn([invocation(0)])];
        expect(PLUGIN_OPERATION_OWED_SLICE_MS, "the grace is the wall the host grants ONE crossing, never a chosen number").toBe(100);
        expect(PLUGIN_OPERATION_OWED_SLICE_MS, "and it is strictly longer than the unowed slice, so the earlier fix still holds").toBeGreaterThan(PLUGIN_OPERATION_REFRESH_SLICE_MS);
        expect(settleYieldsToRefreshV1(results, 0, PLUGIN_OPERATION_REFRESH_SLICE_MS, false, false, true), "inside the grace an owed settle keeps the actor").toBe(false);
        expect(settleYieldsToRefreshV1(results, 0, PLUGIN_OPERATION_OWED_SLICE_MS - 1, false, false, true), "right up to its last millisecond").toBe(false);
        expect(settleYieldsToRefreshV1(results, 0, PLUGIN_OPERATION_OWED_SLICE_MS, false, false, true), "and past it the progress lane is let back in").toBe(true);
        expect(settleYieldsToRefreshV1(results, 0, 60_000, false, false, true), "a run that never publishes the surface can never hold the actor for its whole length again").toBe(true);
        expect(settleYieldsToRefreshV1(results, 0, 60_000, true, false, true), "an acknowledgement still owed is never yielded over").toBe(false);
        expect(settleYieldsToRefreshV1(results, 0, 60_000, false, true, true), "and neither is a REQUIRED surface that has not published").toBe(false);
      });

      /** 🎞️ …and the other half of the same defect: the frames that settle produced must still reach the
       * channel. `retainedUiRefreshEffects` decoded every non-`Error` shell frame and dropped it, so a
       * run's unsolicited `Invocation` progress frames died inside whichever refresh happened to own the
       * actor when they were produced. */
      it("hands a refresh settle's unsolicited progress frames to the channel instead of dropping them", () => {
        const { retainedUiRefreshEffects } = dependencies;
        const shellMessage = (inReplyTo: number) => ({ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: inReplyTo, output: [], diagnostics: [], ui_scope: [1], history_patch: [], messages: [], mutations: [], inverse_group: [] } })) } }) as unknown as WireVariant;
        const published: Uint8Array[][] = [];
        const requested = retainedUiRefreshEffects(7, [shellMessage(0), shellMessage(4), { tag: "notify", val: { message: "x" } } as unknown as WireVariant], (frames) => published.push([...frames]));
        expect(published, "one publication carrying both shell frames").toHaveLength(1);
        expect(published[0], "the unsolicited progress frame AND the correlated answer both belong to the channel").toHaveLength(2);
        expect(requested.length, "a non-frame effect is still the refresh response's own").toBeGreaterThan(0);

        const silent: Uint8Array[][] = [];
        expect(retainedUiRefreshEffects(7, [{ tag: "notify", val: { message: "x" } } as unknown as WireVariant], (frames) => silent.push([...frames]))).toHaveLength(1);
        expect(silent, "a settle that produced no frame publishes nothing").toHaveLength(0);
      });

      it("acknowledges each retained surface before requesting the next bounded publication", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔄️surface-refresh.json");
        const surfaces = fixture.acknowledgement.surfaces;
        const submitted: string[][] = [];
        await withFakeShardClient(
          async (_actor, events) => {
            const acknowledgements = events.filter((event) => event.kind === "patch-ack").map((event) => (event.payload as { surface: { surface: string } }).surface.surface);
            submitted.push(acknowledgements);
            return {
              uiPatches: acknowledgements.includes(surfaces[0]!) ? [{ surface: pluginSurfaceRef(7, surfaces[1]!), revision: 1, baseRevision: 0, ops: [] }] : [],
              effects: [], nextWake: null, status: { tag: "idle" },
              uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 1n }, patchSequence: 1n }),
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "bounded-panel-publication#1",
              { uiPatches: [{ surface: pluginSurfaceRef(7, surfaces[0]!), revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "more-work" }, uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 1n }, patchSequence: 1n }) },
              "UserVisible",
              new Set(surfaces.map((surface) => retainedSurfaceId(7, surface))),
              (turn) => patchAckEvents(turn, turn.uiPatches),
            );
            expect(submitted).toEqual(surfaces.map((surface) => [surface]));
            expect(submitted.every((batch) => batch.length <= fixture.acknowledgement.maxUnacknowledged)).toBe(true);
            expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(surfaces);
          },
        );
      });

      it("rejects an idle actor that did not publish a requested surface instead of retaining a loading placeholder", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔄️surface-refresh.json");
        const { instance, requested, published, missing } = fixture.quiescent;
        await expect(settlePluginTurn(
          "idle-missing-surface#1",
          { uiPatches: published.map((surface) => ({ surface: pluginSurfaceRef(instance, surface), revision: 1, baseRevision: 0, ops: [] })), effects: [], nextWake: null, status: { tag: "idle" } },
          "UserVisible",
          new Set(requested.map((surface) => retainedSurfaceId(instance, surface))),
        )).rejects.toThrow(`missing=${JSON.stringify(missing)}`);
      });

      /** 😴️ Wave W-B14 (ticket 26/09/02): a settle with nothing required used to stop at the FIRST
       * acknowledgement-free continuation, which ended the turn of a mutation whose typed operation
       * was still in flight. An actor that says `more-work` is quiesced FOR THIS TURN only once it
       * has published, acknowledged and emitted nothing for a whole continuation batch — and that is
       * a quiet return, not the stall fault, because nothing was requested of it. */
      it("treats an actor answering more-work with nothing to publish as quiesced only after a whole silent batch", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "empty-required-drain#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
              undefined,
              true,
            );
            expect(continuationCount).toBe(PLUGIN_UI_QUIESCENT_CONTINUATIONS);
            expect(PLUGIN_UI_QUIESCENT_CONTINUATIONS).toBe(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });

      /** 📄️ The drop W-B14 fixes: `addObjectKind`/`addTargetVolume` reached the guest and settled with
       * `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0` — the mutation's own frame is
       * emitted several acknowledgement-free continuations after the command itself completed, and the
       * old stop ended the turn before it, so the publication was dropped. */
      it("collects a mutation frame emitted several acknowledgement-free continuations after the command completed", async () => {
        const documentEffect = { tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload: [1, 2, 3] } };
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const publishing = continuationCount === PLUGIN_UI_QUIESCENT_CONTINUATIONS;
            return { uiPatches: [], effects: publishing ? [documentEffect] : [], nextWake: null, status: { tag: publishing ? "idle" : "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "mutation-publication#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
              undefined,
              true,
            );
            expect(continuationCount).toBe(PLUGIN_UI_QUIESCENT_CONTINUATIONS);
            expect(result.effects).toEqual([documentEffect]);
          },
        );
      });

      /** 🔇️ A settle is silent while it works: its former per-continuation `console.warn` charged
       * 108–244 ms of main thread per example switch (`📓️2026-09-11-wave-B4-main-thread-refresh.md`
       * §5.2, 2 505–6 624 lines). Diagnosis belongs to the faults, which name the actor, the pending
       * surfaces and the bound they hit. */
      it("prints no console warning while a healthy actor settles its continuations", async () => {
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
        try {
          let continuationCount = 0;
          await withFakeShardClient(
            async () => {
              continuationCount += 1;
              const done = continuationCount === 3;
              return {
                uiPatches: [{ surface: pluginSurfaceRef(7, "quiet"), revision: continuationCount, baseRevision: continuationCount - 1, ops: [] }],
                effects: [], nextWake: null, status: { tag: done ? "idle" : "more-work" },
              };
            },
            async () => {
              const result = await settlePluginTurn(
                "quiet-settle#1",
                { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
                "Interactive",
                new Set(),
                undefined,
                true,
              );
              expect(continuationCount).toBe(3);
              expect(result.uiPatches).toHaveLength(3);
            },
          );
          expect(warn).not.toHaveBeenCalled();
        } finally {
          warn.mockRestore();
        }
      });

      /** 🛑️ Wave W-B2 (ticket 26/09/02): a guest that answers `more-work` while publishing,
       * acknowledging and emitting NOTHING is not "still working", it is stalled — and the settle
       * loop used to spend all 4 096 continuations (4 096 host round trips) discovering that, then
       * throw a timeout naming neither the surface nor the reason. It must now stop after the
       * contract-derived zero-progress bound and name the surface it is waiting on. */
      it("fails fast and names the pending surface when a guest answers more-work without publishing anything", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const surface = retainedSurfaceId(7, "puzzle3d-main-perspective");
            await expect(settlePluginTurn(
              "zero-progress-settle#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set([surface]),
            )).rejects.toThrow(`pending=${JSON.stringify([surface])}`);
            expect(continuationCount).toBe(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT).toBeLessThan(PLUGIN_UI_CONTINUATION_LIMIT);
          },
        );
      });

      /** ✅️ The successful path is untouched: any continuation that publishes resets the streak, so a
       * guest still publishing OTHER surfaces while the requested one is pending keeps its full
       * continuation budget — the bound is on zero progress, not on continuations. */
      it("does not fast-fail a guest that keeps publishing while the requested surface is still pending", async () => {
        let continuationCount = 0;
        const total = PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT * 2;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const last = continuationCount >= total;
            return {
              uiPatches: [{ surface: pluginSurfaceRef(7, last ? "puzzle3d-main-perspective" : "puzzle3d-outliner"), revision: 1, baseRevision: 0, ops: [] }],
              effects: [], nextWake: null, status: { tag: last ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "slow-but-progressing-settle#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set([retainedSurfaceId(7, "puzzle3d-main-perspective")]),
            );
            expect(continuationCount).toBe(total);
            expect(result.uiPatches).toHaveLength(total);
          },
        );
      });

      it("does not chase background work during instance-open before a UI surface is requested", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "instance-open-with-background-work#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
            );
            expect(continuationCount).toBe(0);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });

      it("drains until every missing requested surface has published its first patch", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const surface = continuationCount === 1 ? "workflow" : "preview";
            return {
              uiPatches: [{ surface: pluginSurfaceRef(7, surface), revision: 1, baseRevision: 0, ops: [] }],
              effects: [],
              nextWake: null,
              status: { tag: continuationCount === 1 ? "more-work" : "idle" },
            };
          },
          async () => {
            const actorId = "turn-test-multiple-surfaces-actor";
            const result = await settlePluginTurn(
              actorId,
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(7, "workflow"), retainedSurfaceId(7, "preview")]),
            );
            expect(continuationCount).toBe(2);
            expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(["workflow", "preview"]);
          },
        );
      });

      /** 📏️ A large retained surface may still take far more continuations than the former 1 024
       * ceiling — the settle is bounded by PROGRESS, not by round trips. What it may not do is spend
       * them in silence: W-B2 (ticket 26/09/02) made every retirement ladder page-priced, so a guest
       * that publishes, acknowledges and emits nothing for
       * {@link PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT} consecutive turns is stalled, not busy —
       * this guest publishes its section surfaces as it goes and its requested one at the end. */
      it("allows a large retained surface to reconcile beyond the former continuation ceiling while it keeps publishing", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const done = continuationCount === 1_025;
            const progressing = continuationCount % 32 === 0;
            return {
              uiPatches: done
                ? [{ surface: pluginSurfaceRef(9, "large"), revision: 1, baseRevision: 0, ops: [] }]
                : progressing
                  ? [{ surface: pluginSurfaceRef(9, "large-sections"), revision: continuationCount, baseRevision: continuationCount - 1, ops: [] }]
                  : [],
              effects: [],
              nextWake: null,
              status: { tag: done ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-large-surface-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(9, "large")]),
            );
            expect(continuationCount).toBe(1_025);
            expect(continuationCount).toBeGreaterThan(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(result.uiPatches.at(-1)?.surface?.surface).toBe("large");
          },
        );
      });

      it("yields the browser event loop while a retained surface needs several continuation batches", async () => {
        let continuationCount = 0;
        let browserTaskObserved = false;
        const browserTask = yieldPluginUiContinuation().then(() => {
          browserTaskObserved = true;
        });
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return {
              uiPatches: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? [{ surface: pluginSurfaceRef(9, "cooperative"), revision: 1, baseRevision: 0, ops: [] }] : [],
              effects: [],
              nextWake: null,
              status: { tag: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-cooperative-surface-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(9, "cooperative")]),
            );
            expect(browserTaskObserved).toBe(true);
            expect(result.uiPatches).toHaveLength(1);
          },
        );
        await browserTask;
      });

      it("does not poll for an unchanged refresh when every requested surface is already retained", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-retained-surfaces-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set(),
            );
            expect(continuationCount).toBe(0);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });
    });
  //#endregion 🧪️terra-web-plugin-runtime
}
