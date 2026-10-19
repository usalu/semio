import { Buffer } from "node:buffer";
/** 📃 submitPluginTurn: typed-operation result pages, parked faults and draining settles. */
import type { AppChannelClient } from "@semio-tech/framework-os";
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { WireTurnResult, WireVariant } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTypedOperationPagesTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { TYPED_OPERATION_PAGE_MAGIC, consumeTypedOperationEffects, DIRECTORY_PROJECTION_RECEIPT_SCHEMA, encodePackValue, invocationFromFrames, performInvocation, settlePluginTurn, teardownPluginActor, TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_PARK_EVICTION_FAULT, TYPED_OPERATION_UNATTRIBUTED_FAULT, typedOperationAcknowledgements, TypedOperationRouter, typedOperationResult, wireEffectToFriendly, withTypedOperationCall } = dependencies;
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
          expect(router.observe(owner, {receiver:routing.receiver,operation,generation:1n,sequence:routing.firstSequence,attempt:1})).toBe(true);
          expect(router.fault(observer, {receiver:routing.receiver,operation,generation:1n,sequence:routing.faultSequence,attempt:1}, `${routing.fault} ${index}`)).toBe(false);
        }
        expect(reported.map((entry) => entry.code)).toEqual(Array.from({ length: routing.overflow }, () => routing.evictionCode));
        const drained: string[] = [];
        for (let fault = owner.takeFault(); fault !== null; fault = owner.takeFault()) drained.push(fault);
        expect([drained.length, drained[0], drained.at(-1)]).toEqual([routing.capacity, `${routing.fault} ${routing.overflow}`, `${routing.fault} ${total - 1}`]);
        const unowned = BigInt(routing.foreignOperation);
        expect(router.fault(observer, {receiver:routing.receiver,operation:unowned,generation:1n,sequence:routing.faultSequence,attempt:1}, routing.fault)).toBe(false);
        expect(reported.at(-1)).toEqual({ code: routing.unattributedCode, message: `${routing.fault} (operation ${unowned} sequence ${routing.faultSequence} observed by observer)` });
        owner.close();
        observer.close();
        expect(reported.length).toBe(routing.overflow + 1);
      });


      it("typed-operation diagnostic reception preserves the original command across generations before ACK", async () => {
        const reported: string[] = [];
        const router = new TypedOperationRouter((code: string, message: string) => reported.push(code + ": " + message));
        const owner = router.open("original"), observer = router.open("other-generation");
        const token = { receiver: 1, operation: 7n, generation: 2n, sequence: 0, attempt: 0 };
        expect(owner.observe(token)).toBe(true);
        expect(observer.observe({ ...token, generation: 3n })).toBe(true);
        const expected = { code: "interaction.target-missing", severity: "warning", span: { line: 1, column: 3, length: 5 }, message: "Target 🧩 is unavailable.", scope: { pluginId: "fem", bodyKey: "nodes" }, expected: { tokens: ["node"], keywords: [], keys: ["id"] } };
        const encoded = Buffer.from(JSON.stringify(expected));
        const page = (sequence: number, payload: Uint8Array, complete: boolean) => {
          const body = Buffer.alloc(35 + payload.length);
          body.writeUInt32LE(token.receiver, 0); body.writeBigUInt64LE(token.operation, 4); body.writeBigUInt64LE(token.generation, 12); body.writeUInt32LE(sequence, 20); body[24] = 0; body[25] = 15; body.writeUInt32LE(payload.length + 5, 26); body.writeUInt32LE(0, 30); body[34] = Number(complete); body.set(payload, 35);
          return { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: Buffer.concat([Buffer.from(TYPED_OPERATION_PAGE_MAGIC), body]) } };
        };
        const initial = page(1, encoded.subarray(0, 3), false), final = page(2, encoded.subarray(3), true);
        await observer.accept([initial]);
        expect(owner.takeDiagnosticEffects()).toEqual([]);
        await observer.accept([final]);
        await observer.accept([final]);
        expect(observer.takeDiagnosticEffects()).toEqual([]);
        const response = invocationFromFrames([], owner.takeDiagnosticEffects(), "command");
        expect(response.diagnostics).toEqual([expected]);
        expect(response.requestedEffects).toEqual([]);
        expect(owner.diagnosticProgress).toEqual({ bytes: encoded.length, nodes: expect.any(Number) });
        expect(owner.diagnosticOwnedBytes).toBeGreaterThan(0);
        owner.close(); observer.close(); expect(reported).toEqual([]);
        console.log("[DEBUG] mounted host diagnostic owner: same operation/different generation isolated, original command receives warning/expected/span/scope once before ACK, actual decode progress retained");
      });


      it("typed-operation diagnostic delivery isolates observers before the original guest ACK", async () => {
        const { default: fixture } = await import("../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/⚠️diagnostic/🧫️fixtures/🔣️.json");
        const { operationDiagnosticFanouts, OperationDiagnosticObservers } = dependencies;
        const token = { receiver: fixture.host.observers.receiver, operation: 19n, generation: 4n, sequence: 1, attempt: 0 };
        const actorId = "diagnostic-observer#" + token.receiver;
        const diagnostic = { code: "interaction.target-missing", severity: "warning", span: { line: 0, column: 0, length: 0 }, message: "Original target is unavailable.", scope: {} };
        const received: unknown[] = [], failures: unknown[] = [];
        const listeners = new OperationDiagnosticObservers<import("../../🧱️elements/🔌️PluginRuntime/🟦️.tsx").PluginOperationDiagnostic>(fixture.host.observers.maximumListeners, error => failures.push(error));
        listeners.subscribe(() => { throw new Error("observer refused"); });
        listeners.subscribe(receipt => received.push(receipt));
        operationDiagnosticFanouts.set(token.receiver, listeners);
        const payload = Buffer.from(JSON.stringify(diagnostic)), body = Buffer.alloc(35 + payload.length);
        body.writeUInt32LE(token.receiver, 0); body.writeBigUInt64LE(token.operation, 4); body.writeBigUInt64LE(token.generation, 12); body.writeUInt32LE(token.sequence, 20); body[25] = 15; body.writeUInt32LE(payload.length + 5, 26); body[34] = 1; body.set(payload, 35);
        const page = { tag: "send-message", val: { target: { tag: "shell", val: String(token.receiver) }, payload: Buffer.concat([Buffer.from(TYPED_OPERATION_PAGE_MAGIC), body]) } };
        let crossings = 0;
        try {
          await withTypedOperationCall(actorId, "original", async (owner: any) => {
            expect(owner.observe({ ...token, sequence: 0 })).toBe(true);
            const settled = await withFakeShardClient(async (_actor, events) => { crossings++; expect(events).toEqual(typedOperationAcknowledgements({ effects: [page] })); return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async () => {
              return settlePluginTurn(actorId, { uiPatches: [], effects: [page], nextWake: null, status: { tag: "idle" } }, "Interactive", new Set(), undefined, true, undefined, owner);
            });
            expect(invocationFromFrames([], settled.effects, "command").diagnostics).toEqual([diagnostic]);
            expect(owner.takeDiagnosticEffects()).toEqual([]);
          });
          expect(crossings).toBe(1); expect(failures).toHaveLength(1); expect(received).toEqual([{ token, diagnostic }]);
          console.log("[DEBUG] actual mounted diagnostic delivery: refused UI listener isolated, later original listener retained, exact command row and guest acknowledgement delivered once");
        } finally { operationDiagnosticFanouts.delete(token.receiver); teardownPluginActor(actorId); }
      });

      it("typed-operation diagnostic reception refuses a truncated terminal before guest ACK", async () => {
        const actorId = "diagnostic-truncated#1";
        let crossings = 0;
        const token = { receiver: 1, operation: 9n, generation: 4n, sequence: 0, attempt: 0 };
        const page = (sequence: number, lane: number, payload: Uint8Array) => {
          const body = Buffer.alloc(30 + payload.length); body.writeUInt32LE(token.receiver, 0); body.writeBigUInt64LE(token.operation, 4); body.writeBigUInt64LE(token.generation, 12); body.writeUInt32LE(sequence, 20); body[25] = lane; body.writeUInt32LE(payload.length, 26); body.set(payload, 30);
          return { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: Buffer.concat([Buffer.from(TYPED_OPERATION_PAGE_MAGIC), body]) } };
        };
        await withTypedOperationCall(actorId, "owner", async (owner: any) => {
          await owner.accept([page(0, 15, new Uint8Array([0, 0, 0, 0, 0, 123]))]);
          await withFakeShardClient(async () => { crossings++; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async () => {
            await expect(settlePluginTurn(actorId, { uiPatches: [], effects: [page(1, 10, new Uint8Array())], nextWake: null, status: { tag: "idle" } }, "Interactive", new Set(), undefined, true, undefined, owner)).rejects.toThrow("terminal precedes complete diagnostic custody");
          });
          expect(crossings).toBe(0);
        });
        teardownPluginActor(actorId);
        console.log("[DEBUG] mounted host diagnostic terminal: partial original row refuses before any acknowledgement crossing");
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
