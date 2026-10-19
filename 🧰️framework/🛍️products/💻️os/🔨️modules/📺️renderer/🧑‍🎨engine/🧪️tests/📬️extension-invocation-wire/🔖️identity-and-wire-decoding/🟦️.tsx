/** 🔖️ The extension invocation WIT identity, icon export and host effect address decoding tests. */
import type { WireVariant } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";

/** 🆔 Registers the WIT request identity, icon export wire decoding and host effect address decoding tests. */
export function registerIdentityAndWireDecodingTests(kit: InvocationWireTestKit): void {
  const { SemioFaultError, assertAddressedInvocation, decodePackValue, decodePackWire, describe, encodePackValue, expect, it, wireEffectToFriendly, wireExtensionInvocation } = kit;
    describe("extension invocation WIT request identity", () => {
      it("rejects narrowed, exhausted or malformed request identities", () => {
        for (const req of [0n, -1n, 0x10000000000000000n, 1, "1", undefined]) {
          expect(() => wireExtensionInvocation({ tag: "invoke-extension", val: { req, params: { extensionId: "text", capability: "evaluate", payload: [] } } })).toThrow("extension.request-id-invalid");
        }
      });
  
      it("preserves nested UTF-8 payloads and exact u64 ids in both renderer decoders", async () => {
        const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const { wireEffectToFriendly: sharedDecode } = await import("@semio-tech/framework-actor/wire-turn");
        const requestJson = JSON.stringify({ ...fixture.request, label: "Hölzer 日本語" });
        for (const id of fixture.requestIds) {
          const req = BigInt(id);
          const wire = { tag: "invoke-extension", val: { req, params: { extensionId: fixture.extensionId, capability: fixture.capability, payload: new TextEncoder().encode(requestJson) } } };
          const expected = { invokeExtension: { req, extensionId: fixture.extensionId, capability: fixture.capability, requestJson } };
          expect(wireEffectToFriendly(wire)).toEqual(expected);
          expect(sharedDecode(wire, decodePackValue)).toEqual(expected);
        }
      });
    });
    describe("icon export wire decoding", () => {
      it("preserves every packed export item in both renderer doors and rejects malformed envelopes", async () => {
        const { default: fixture } = await import("../../../🧱️elements/🖼️IconRenderHost/🧪️tests/🧫️fixtures/📡️export-wire/🔣️.json");
        const { default: schema } = await import("../../../🧱️elements/🖼️IconRenderHost/🧬️schema/📡️export-wire/🔣️.json");
        const { default: Ajv2020 } = await import("ajv/dist/2020");
            const { wireEffectToFriendly: sharedDecode } = await import("@semio-tech/framework-actor/wire-turn");
        const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
        for (const sample of fixture.cases) {
          expect(validate(sample.items), sample.name).toBe(sample.valid);
          const wire = { tag: "icon-render-export", val: { items: encodePackValue(sample.items) } };
          for (const decode of [wireEffectToFriendly, (effect: WireVariant) => sharedDecode(effect, decodePackWire)]) {
            if (sample.valid) expect(decode(wire), sample.name).toEqual({ iconRenderExport: { items: sample.items } });
            else expect(() => decode(wire), sample.name).toThrow("icon-export.items-invalid");
          }
        }
      });
    });
    describe("host effect address decoding", () => {
      it("reads every request-carrying effect out of its nested WIT params record", async () => {
        const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        const nested = (tag: string, req: number, params: Record<string, unknown>) => ({ tag, val: { req, params } });
        const { req: dispatchReq, ...dispatchParams } = fixture.dispatchAction;
        expect(wireEffectToFriendly(nested("dispatch-action", dispatchReq, dispatchParams))).toEqual({ dispatchAction: { req: dispatchReq, action: fixture.dispatchAction.action, args: undefined, delayMs: fixture.dispatchAction.delayMs } });
        const { req: windowReq, ...windowParams } = fixture.openWindow;
        expect(wireEffectToFriendly(nested("open-window", windowReq, windowParams))).toEqual({ openWindow: { req: windowReq, kind: fixture.openWindow.kind, params: undefined } });
        const { req: dialogReq, ...dialogParams } = fixture.openDialog;
        expect(wireEffectToFriendly(nested("open-dialog", dialogReq, dialogParams))).toEqual({ openDialog: { req: dialogReq, dialogId: fixture.openDialog.dialogId, args: undefined } });
        const { req: spawnReq, ...spawnParams } = fixture.spawnPluginInstance;
        expect(wireEffectToFriendly(nested("spawn-plugin-instance", spawnReq, spawnParams))).toEqual({ spawnPluginInstance: { req: spawnReq, ...spawnParams } });
      });

      it("drops a dispatch-action whose action id never survived the boundary", async () => {
        const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        expect(wireEffectToFriendly({ tag: "dispatch-action", val: { ...fixture.dispatchAction } })).toBeNull();
        expect(wireEffectToFriendly({ tag: "dispatch-action", val: { req: fixture.dispatchAction.req, params: { action: "", delayMs: 0 } } })).toBeNull();
      });

      it("refuses an unaddressed invocation with a typed renderer fault naming its window kind", async () => {
        const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        const address = { pluginId: "procedural", appId: fixture.spawnPluginInstance.appId, windowKindId: fixture.windowKindId, actionId: "" };
        let thrown: unknown;
        try {
          assertAddressedInvocation({ address, arguments: {} }, "action", fixture.instanceId);
        } catch (error) {
          thrown = error;
        }
        expect(thrown).toBeInstanceOf(SemioFaultError);
        const fault = (thrown as InstanceType<typeof SemioFaultError>).fault;
        expect(fault.code).toBe(fixture.unaddressedFaultCode);
        expect(fault.origin).toBe(fixture.unaddressedFaultOrigin);
        expect(fault.message).toContain(fixture.windowKindId);
        expect(fault.scope.instanceId).toBe(String(fixture.instanceId));
        expect(() => assertAddressedInvocation({ address: { ...address, actionId: fixture.dispatchAction.action } }, "action", fixture.instanceId)).not.toThrow();
        expect(() => assertAddressedInvocation({ address: { commandId: "" } }, "command", fixture.instanceId)).toThrow(SemioFaultError);
        expect(() => assertAddressedInvocation({ address: { commandId: "open" } }, "command", fixture.instanceId)).not.toThrow();
      });
    });
}
