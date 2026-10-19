/** 🔗️ The actual actor binding, document port retirement and disposal tests. */
import type { WireVariant } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";
import type { CompletionPublicationHelpers } from "../🛠️requester-helper/🟦️.tsx";

/** 🪢 Registers the failed binding reconciliation, document port retirement, handle disposal and pending creation tests. */
export function registerActorBindingAndDisposalTests(kit: InvocationWireTestKit, helpers: CompletionPublicationHelpers): void {
  const { expect, it } = kit;
  const { withRequester } = helpers;
    it("reconciles failed actual actor bindings without giving a successor stale control", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
      const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("@semio-tech/framework/backbone/binding");
      for (const row of fixture.bindingRetirement) {
        const commands: string[] = [];
        let instance = 0, current = true;
        await withRequester(async (_actor, events) => {
          const effects: WireVariant[] = [];
          for (const event of events) {
            if (event.kind !== "message") continue;
            const value = event.payload as { source: { tag: string }; payload: number[] };
            if (value.source.tag !== "shell") continue;
            const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
            commands.push(command.operation);
            const firstBind = command.bindingGeneration === 1n && command.operation === "bind";
            const refused = firstBind && row.result === "refused";
            if (firstBind && row.result === "transport") throw new Error("transport lost");
            if (firstBind && row.result === "replaced") current = false;
            const effect: WireVariant = { tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: refused ? "refused" : command.operation === "bind" ? "bound" : "retired", ...(refused ? { code: "binding-refused" } : {}) }) } };
            effects.push(effect);
            if (firstBind && row.result === "duplicate") effects.push(effect);
          }
          return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
        }, async (handle, opened) => {
          instance = opened;
          const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null, current: () => current, send: () => {} };
          await expect(handle.bindDocumentPort!(instance, source)).rejects.toThrow();
          expect(commands, row.result).toEqual(row.sequence);
          current = true;
          const successor = await handle.bindDocumentPort!(instance, { ...source, clientInstanceId: "client-b" });
          await successor.retire();
          expect(commands, row.result).toEqual([...row.sequence, "bind", "retire"]);
        });
      }
    });
    it("binds the actual actor document port and retires it before guest disposal", async () => {
      const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("@semio-tech/framework/backbone/binding");
      const { encodeBackboneMessage } = await import("@semio-tech/framework-os");
      const outgoing = encodeBackboneMessage({ kind: "ack", opIds: ["operation"] });
      let outgoingUri = "actor://map";
      const commands: string[] = [], delivered: number[][] = [], sent: number[][] = [];
      let instance = 0;
      await withRequester(async (_actor, events) => {
        const effects: WireVariant[] = [];
        for (const event of events) {
          if (event.kind !== "message") continue;
          const value = event.payload as { source: { tag: string; val: string }; payload: number[] };
          if (value.source.tag === "shell") {
            const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
            commands.push(command.operation);
            effects.push({ tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
          } else {
            delivered.push([...value.payload]);
            effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: outgoingUri }, payload: outgoing } });
          }
        }
        return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
      }, async (handle, opened, activation) => {
        instance = opened;
        const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null };
        const port = await handle.bindDocumentPort!(instance, { ...source, current: () => true, send: payload => { sent.push([...payload]); } });
        expect(commands).toEqual(["bind"]);
        expect(await port.receive(source, outgoing)).toBe(true);
        expect(delivered).toEqual([[...outgoing]]);
        expect(sent).toEqual([[...outgoing]]);
        outgoingUri = "actor://foreign";
        await expect(port.receive(source, outgoing)).rejects.toThrow("actor-document-port.foreign-uri");
        expect(activation.guardedTurns()).toBeGreaterThanOrEqual(2);
        await handle.destroyApp(instance);
        expect(commands).toEqual(["bind", "retire"]);
        expect(await port.receive(source, outgoing)).toBe(false);
      });
    });
    it("awaits actual actor handle disposal until its exact document retirement settles", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
      const { default: deepEqual } = await import("fast-deep-equal");
      const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("@semio-tech/framework/backbone/binding");
      let release!: () => void, entered!: () => void;
      const gate = new Promise<void>(resolve => { release = resolve; });
      const retiring = new Promise<void>(resolve => { entered = resolve; });
      const events: string[] = [];
      await withRequester(async (_actor, input) => {
        const effects: WireVariant[] = [];
        for (const event of input) {
          if (event.kind !== "message") continue;
          const value = event.payload as { source: { tag: string }; payload: number[] };
          if (value.source.tag !== "shell") continue;
          const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
          events.push(command.operation);
          if (command.operation === "retire") { entered(); await gate; events.push("retired"); }
          effects.push({ tag: "send-message", val: { target: { tag: "shell", val: command.instanceId }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
        }
        return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
      }, async (handle, instance) => {
        const port = await handle.bindDocumentPort!(instance, { runtimeKey: "map", clientInstanceId: "client", scope: null, current: () => true, send: () => {} });
        const closing = handle.dispose();
        try {
          expect(closing).toBeInstanceOf(Promise);
          expect(handle.dispose()).toBe(closing);
          let settled = false;
          void closing.then(() => { settled = true; });
          await retiring;
          expect(settled).toBe(false);
          expect(events).toEqual(fixture.disposal.bound.slice(0, 2));
          expect(await port.receive({ runtimeKey: "map", clientInstanceId: "client", scope: null }, Uint8Array.of(1))).toBe(false);
          await expect(handle.createApp("late")).rejects.toThrow("plugin-handle.closed");
        } finally { release(); await closing; }
        expect(deepEqual(events, fixture.disposal.bound)).toBe(true);
      }, { dispose: () => { events.push("dispose"); } });
    });
    it("awaits actual actor pending creation without opening an orphan after disposal", async () => {
      const { default: fixture } = await import("../../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
      const { default: deepEqual } = await import("fast-deep-equal");
      for (const row of fixture.disposal.opening) {
        let release!: () => void, entered!: () => void, target = "";
        const gate = new Promise<void>(resolve => { release = resolve; });
        const pending = new Promise<void>(resolve => { entered = resolve; });
        const events: string[] = [];
        await withRequester(async () => ({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }), async (handle, instance) => {
          const original = `extension-requester#${instance}`;
          target = "pending";
          const opening = handle.createApp("pending");
          const observed = opening.then(value => ({ value }), error => ({ error }));
          await pending;
          const closing = handle.dispose();
          try {
            expect(closing).toBeInstanceOf(Promise);
            let settled = false;
            void closing.then(() => { settled = true; }, () => {});
            await Promise.resolve();
            expect(settled).toBe(false);
            expect(target).not.toBe(original);
            expect(events).not.toContain("dispose");
          } finally { release(); await closing; }
          expect(await observed).toMatchObject({ error: { message: "plugin-handle.closed" } });
          expect(deepEqual(events, row.events), row.phase).toBe(true);
        }, {
          activate: async actor => {
            if (!target) return;
            target = actor; events.push("activate");
            if (row.phase === "activation") { entered(); await gate; }
            events.push("activated");
          },
          open: async actor => {
            if (actor !== target) return;
            events.push("open");
            if (row.phase === "open") { entered(); await gate; }
            events.push("opened");
          },
          dispose: actor => { if (actor === target) events.push("dispose"); },
        });
      }
    });
}
