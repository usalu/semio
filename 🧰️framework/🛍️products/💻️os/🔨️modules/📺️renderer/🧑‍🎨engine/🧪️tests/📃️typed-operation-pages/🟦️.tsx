/** 📃 submitPluginTurn: typed-operation result pages, parked faults and draining settles. */
import type { AppChannelClient } from "@semio-tech/framework-os";
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { WireTurnResult, WireVariant } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTypedOperationPagesTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { consumeTypedOperationEffects, DIRECTORY_PROJECTION_RECEIPT_SCHEMA, encodePackValue, invocationFromFrames, performInvocation, settlePluginTurn, teardownPluginActor, TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_PARK_EVICTION_FAULT, TYPED_OPERATION_UNATTRIBUTED_FAULT, typedOperationAcknowledgements, TypedOperationRouter, typedOperationResult, wireEffectToFriendly, withTypedOperationCall } = dependencies;
  const { withFakeShardClient } = createPluginRuntimeSupport(dependencies);
    //#region 🧪️terra-web-plugin-runtime
    describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
      it("validates fixed result page authority and preserves document and download effects", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures.json");
        const wire = (lane: number, text: string, receiver = fixture.wire.receiver): WireVariant => {
          const body = Buffer.alloc(30);
          body.writeUInt32LE(fixture.wire.receiver, 0);
          body.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
          body.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
          body.writeUInt32LE(fixture.wire.sequence, 20);
          body[24] = fixture.wire.attempt;
          body[25] = lane;
          const payload = Buffer.from(text);
          body.writeUInt32LE(payload.length, 26);
          return { tag: "send-message", val: { target: { tag: "shell", val: String(receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
        };
        expect(typedOperationResult(wire(10, "x".repeat(4_096)))?.payload.length).toBe(4_096);
        for (const invalid of [wire(15, ""), wire(10, "x".repeat(4_097)), wire(10, "", 8)]) expect(() => typedOperationResult(invalid)).toThrow("authority");
        const truncated = wire(10, "x");
        (truncated.val as { payload: number[] }).payload.pop();
        expect(() => typedOperationResult(truncated)).toThrow("authority");
        expect(wireEffectToFriendly({ tag: "load-document", val: { docPack: fixture.effects.loadDocument.pack, spr: fixture.effects.loadDocument.spr } })).toEqual({ loadDocument: fixture.effects.loadDocument });
        const download = consumeTypedOperationEffects([wire(9, JSON.stringify(fixture.effects.download))]);
        expect(download.map(wireEffectToFriendly)).toEqual([{ downloadMediaExport: { filename: fixture.effects.download[0], mimeType: fixture.effects.download[1], data: fixture.wire.operation, encoding: "semio-segmented-handle-v1:identity" } }]);
        const receipt = {
          schema: DIRECTORY_PROJECTION_RECEIPT_SCHEMA,
          sessionBindingSha256: "a".repeat(64),
          authorizationGeneration: 7,
          throughSeqInclusive: 11,
          receiptSha256: "b".repeat(64),
        };
        const receiptEvent = wire(7, JSON.stringify({ kind: DIRECTORY_PROJECTION_RECEIPT_SCHEMA, payload: receipt }));
        expect(() => invocationFromFrames([], consumeTypedOperationEffects([receiptEvent]), "action")).toThrow("before typed-operation terminal");
        const terminalOutput = consumeTypedOperationEffects([receiptEvent, wire(10, "typed-operation-complete")]);
        expect(invocationFromFrames([], terminalOutput, "action").output).toEqual(receipt);
        const splitOutput = consumeTypedOperationEffects([...consumeTypedOperationEffects([receiptEvent]), ...consumeTypedOperationEffects([wire(10, "typed-operation-complete")])]);
        expect(invocationFromFrames([], splitOutput, "action").output).toEqual(receipt);
        let acknowledged = false;
        await withFakeShardClient(async (_actor, events) => {
          acknowledged = events.length === 1 && events[0]?.kind === "message";
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async () => {
          await expect(settlePluginTurn("faulted-operation#7", { uiPatches: [], effects: [wire(11, fixture.wire.fault)], nextWake: null, status: { tag: "idle" } }, "Interactive", new Set(), undefined, true)).rejects.toThrow(fixture.wire.fault);
          expect(acknowledged).toBe(true);
        });
      });

      it("routes a foreign typed-operation fault page to its own call and never to the observing call", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: routing } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/📬️typed-operation-routing.json");
        expect([TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_UNATTRIBUTED_FAULT, TYPED_OPERATION_PARK_EVICTION_FAULT]).toEqual([routing.capacity, routing.unattributedCode, routing.evictionCode]);
        const page = (operation: string, sequence: number, lane: number, text: string) => {
          const body = Buffer.alloc(30);
          body.writeUInt32LE(routing.receiver, 0);
          body.writeBigUInt64LE(BigInt(operation), 4);
          body.writeBigUInt64LE(1n, 12);
          body.writeUInt32LE(sequence, 20);
          body[24] = 1;
          body[25] = lane;
          const payload = Buffer.from(text);
          body.writeUInt32LE(payload.length, 26);
          return { tag: "send-message", val: { target: { tag: "shell", val: String(routing.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
        };
        const actorId = `routed-operation#${routing.receiver}`;
        const faulted = { uiPatches: [], effects: [page(routing.foreignOperation, routing.faultSequence, 11, routing.fault)], nextWake: null, status: { tag: "idle" } };
        const acknowledged: unknown[][] = [];
        let observerSettled = false;
        await expect(withTypedOperationCall(actorId, "owner", async (owner: any) => {
          expect(consumeTypedOperationEffects([page(routing.foreignOperation, routing.firstSequence, 0, "first")], owner)).toEqual([]);
          await withTypedOperationCall(actorId, "observer", async (observer: any) => {
            await withFakeShardClient(async (_actor, events) => {
              acknowledged.push([...events]);
              return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
            }, async () => {
              const settled = await settlePluginTurn(actorId, faulted, "Interactive", new Set(), undefined, true, undefined, observer);
              expect(settled.effects).toEqual([]);
            });
          });
          observerSettled = true;
        })).rejects.toThrow(routing.fault);
        expect(observerSettled).toBe(true);
        expect(acknowledged).toEqual([typedOperationAcknowledgements(faulted)]);
        teardownPluginActor(actorId);
      });

      it("bounds parked typed-operation faults at a fixed capacity and surfaces the evicted oldest", async () => {
        const { default: routing } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/📬️typed-operation-routing.json");
        const reported: { code: string; message: string }[] = [];
        const router = new TypedOperationRouter((code: string, message: string) => reported.push({ code, message }));
        const owner = router.open("owner");
        const observer = router.open("observer");
        const total = routing.capacity + routing.overflow;
        for (let index = 0; index < total; index += 1) {
          const operation = BigInt(routing.ownedOperation) + BigInt(index);
          expect(router.observe(owner, operation, routing.firstSequence)).toBe(true);
          expect(router.fault(observer, operation, routing.faultSequence, `${routing.fault} ${index}`)).toBe(false);
        }
        expect(reported.map((entry) => entry.code)).toEqual(Array.from({ length: routing.overflow }, () => routing.evictionCode));
        const drained: string[] = [];
        for (let fault = owner.takeFault(); fault !== null; fault = owner.takeFault()) drained.push(fault);
        expect([drained.length, drained[0], drained.at(-1)]).toEqual([routing.capacity, `${routing.fault} ${routing.overflow}`, `${routing.fault} ${total - 1}`]);
        const unowned = BigInt(routing.foreignOperation);
        expect(router.fault(observer, unowned, routing.faultSequence, routing.fault)).toBe(false);
        expect(reported.at(-1)).toEqual({ code: routing.unattributedCode, message: `${routing.fault} (operation ${unowned} sequence ${routing.faultSequence} observed by observer)` });
        owner.close();
        observer.close();
        expect(reported.length).toBe(routing.overflow + 1);
      });

      it("keeps the command reply when publication supplies only an unsolicited UI scope", async () => {
        const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const frames = [
          { Invocation: { in_reply_to: 1, output: bytes({ operationId: fixture.wire.operation }), diagnostics: bytes([]), ui_scope: bytes({ kind: "none" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
          { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: bytes({ kind: "full" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
        ];
        const client = { command: async () => frames } as unknown as AppChannelClient;
        const result = await performInvocation(client, 7, { address: { pluginId: "procedural", appId: "s.procedural.procedural3d@1/*#editor", windowKindId: "procedural-main", actionId: "commitFixture" }, arguments: {} }, "action", { locale: "en", terminology: "native" });
        expect(result.output).toEqual({ operationId: fixture.wire.operation });
        expect(result.uiScope).toEqual({ kind: "full" });
      });

      it("drains an actor's more-work turns until the reconciled UI patch is publishable", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [{ revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "idle" } };
          },
          async () => {
            const actorId = "turn-test-more-work-actor";
            const result = await settlePluginTurn(actorId, { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult, "UserVisible");
            expect(continuationCount).toBe(1);
            expect(result.uiPatches).toHaveLength(1);
          },
        );
      });

      it("ends a draining settle one slice after the operation asked for a refresh, leaving the rest to the yielding drain", async () => {
        const invocation = (inReplyTo: number) => ({ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: inReplyTo, output: [], diagnostics: [], ui_scope: [1], history_patch: [], messages: [], mutations: [], inverse_group: [] } })) } });
        const turn = (effects: unknown[]) => ({ uiPatches: [], effects, nextWake: null, status: { tag: "more-work" } }) as unknown as WireTurnResult;
        const { settleYieldsToRefreshV1, PLUGIN_OPERATION_REFRESH_SLICE_MS } = dependencies;
        expect(settleYieldsToRefreshV1([turn([invocation(0)])], 0, PLUGIN_OPERATION_REFRESH_SLICE_MS, false, false)).toBe(true);
        expect(settleYieldsToRefreshV1([turn([invocation(0)])], 0, PLUGIN_OPERATION_REFRESH_SLICE_MS - 1, false, false)).toBe(false);
        expect(settleYieldsToRefreshV1([turn([invocation(9)])], 0, PLUGIN_OPERATION_REFRESH_SLICE_MS, false, false)).toBe(false);
        expect(settleYieldsToRefreshV1([turn([invocation(0)])], 0, PLUGIN_OPERATION_REFRESH_SLICE_MS, true, false)).toBe(false);
        expect(settleYieldsToRefreshV1([turn([invocation(0)])], 0, PLUGIN_OPERATION_REFRESH_SLICE_MS, false, true)).toBe(false);
        let turns = 0;
        await withFakeShardClient(
          async () => {
            turns += 1;
            await new Promise((resolve) => setTimeout(resolve, 4));
            return { ...turn([invocation(0)]), status: { tag: turns >= 400 ? "idle" : "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn("refresh-slice-actor", turn([]), "UserVisible", new Set(), undefined, true);
            expect((result.status as { tag: string }).tag).toBe("more-work");
            expect(turns).toBeLessThan(40);
          },
        );
      });
    });
  //#endregion 🧪️terra-web-plugin-runtime
}
