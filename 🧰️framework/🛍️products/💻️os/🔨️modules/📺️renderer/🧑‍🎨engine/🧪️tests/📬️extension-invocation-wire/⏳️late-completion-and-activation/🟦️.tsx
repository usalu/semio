/** ⏳️ The late completion, activation capture and completion submission tests. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { WireTurnResult } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";
import type { CompletionPublicationHelpers } from "../🛠️requester-helper/🟦️.tsx";

/** 🕰 Registers the framed fault, late completion, activation capture and replacement, lease and single submission tests. */
export function registerLateCompletionAndActivationTests(kit: InvocationWireTestKit, helpers: CompletionPublicationHelpers): void {
  const { encodePackValue, expect, it, retainedWindowByActor, serializeCommandIngressForActor } = kit;
  const { withRequester } = helpers;
    it("rejects a framed completion fault without swallowing its structured fields", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      let instance = 0;
      await withRequester(async () => ({ uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodePackValue(fixture.fault)), report: [] } })) } }], nextWake: null, status: { tag: "idle" } }), async (handle, opened) => {
        instance = opened;
        await expect(handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) })).rejects.toMatchObject({ fault: fixture.fault });
      });
    });
    it("does not publish a late completion after its originating instance is destroyed", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const entered = Promise.withResolvers<void>();
      const result = Promise.withResolvers<WireTurnResult>();
      let turns = 0;
      await withRequester(async () => { turns += 1; entered.resolve(); return result.promise; }, async (handle, instance) => {
        const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
        const observed = expect(completion).rejects.toThrow("no actor");
        await entered.promise;
        const retiring = handle.destroyApp(instance);
        let retired = false;
        void retiring.then(() => { retired = true; });
        await Promise.resolve();
        expect(retired).toBe(false);
        result.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "idle" } });
        await observed;
        await retiring;
        expect(turns).toBe(1);
        expect(retainedWindowByActor.has(`extension-requester#${instance}`)).toBe(false);
      });
    });
    it("captures activation before queued completion and rejects a same-name replacement before dispatch", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      let dispatched = 0;
      await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
        const entered = Promise.withResolvers<void>();
        const release = Promise.withResolvers<void>();
        const held = serializeCommandIngressForActor(`extension-requester#${instance}`, async () => { entered.resolve(); await release.promise; });
        await entered.promise;
        const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
        const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
        const captures = activation.captures();
        activation.replace();
        release.resolve();
        await held;
        await observed;
        expect(captures).toBe(1);
        expect(activation.captures()).toBe(1);
        expect(dispatched).toBe(0);
      });
    });
    it("refuses publication after in-flight activation replacement without running a new continuation", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const entered = Promise.withResolvers<void>();
      const release = Promise.withResolvers<WireTurnResult>();
      let dispatched = 0;
      await withRequester(async () => { dispatched += 1; entered.resolve(); return release.promise; }, async (handle, instance, activation) => {
        const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
        const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
        await entered.promise;
        activation.replace();
        release.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: "stale activation" } }], nextWake: null, status: { tag: "idle" } });
        await observed;
        expect(activation.captures()).toBe(1);
        expect(activation.guardedTurns()).toBe(1);
        expect(dispatched).toBe(1);
        expect(retainedWindowByActor.get(`extension-requester#${instance}`)?.size ?? 0).toBe(0);
      });
    });
    it("keeps the original activation lease through every completion continuation", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      const entered = Promise.withResolvers<void>();
      const release = Promise.withResolvers<WireTurnResult>();
      let dispatched = 0;
      await withRequester(async () => {
        dispatched += 1;
        if (dispatched === 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
        entered.resolve();
        return release.promise;
      }, async (handle, instance, activation) => {
        const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
        const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
        await entered.promise;
        activation.replace();
        release.resolve({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } });
        await observed;
        expect(dispatched).toBe(2);
        expect(activation.captures()).toBe(1);
        expect(activation.guardedTurns()).toBe(2);
      });
    });
    it("rejects a completion captured before evaluation when that activation is later replaced", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      let dispatched = 0;
      await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
        const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
        activation.replace();
        await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("actor-activation.revoked");
        expect(dispatched).toBe(0);
        expect(activation.captures()).toBe(1);
      });
    });
    it("claims one completion submission per captured request", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
      let dispatched = 0;
      await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance) => {
        const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
        expect(Object.isFrozen(completion)).toBe(true);
        expect([completion.instanceId, completion.req]).toEqual([instance, BigInt(fixture.requestId)]);
        await completion.complete({ ok: encodePackValue(fixture.response) });
        await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("extension.completion-already-submitted");
        expect(dispatched).toBe(1);
      });
    });
}
