type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { AllowAllCapabilities, EffectBackbone, MESSAGE_ENDPOINT_VARIANT_FIELDS, RecordingBackboneOverflowReporter, backboneMessageEndpoint, bridgeBackboneWorkerInbound, createBackboneWorkerTransport, decodeBackboneGuestMessage, encodeBackboneGuestMessage } = dependencies;
  type BackboneTransport = any;
  type BackboneWorkerLike = any;
  type BackboneWorkerWireMessage = any;
  type CapabilityChecker = any;
  type EventMessage = any;

  const { describe, expect, it } = vitest;

  function fakeTransport(): { readonly transport: BackboneTransport; readonly received: Array<{ readonly uri: string; readonly payload: readonly number[] }> } {
    const received: Array<{ readonly uri: string; readonly payload: readonly number[] }> = [];
    return { transport: { send: (uri, payload) => received.push({ uri, payload }) }, received };
  }

  class DenyAll implements CapabilityChecker {
    isGranted(): boolean {
      return false;
    }
  }

  //#region 🔑️CapabilityTests
  describe("EffectBackbone capability gating", () => {
    it("mirrors backbone_send_is_rejected_without_the_capability: send is rejected without the messaging.backbone:<uri> capability", () => {
      const backbone = new EffectBackbone({ capabilities: new DenyAll() });
      const { transport } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const outcome = backbone.send("actor-1", "studio-42", [1, 2, 3]);
      expect(outcome).toEqual({ ok: false, error: { kind: "capabilityDenied", uri: "studio-42" } });
    });

    it("mirrors backbone_send_reaches_the_registered_transport_once_granted: send reaches the registered transport once granted", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport, received } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const outcome = backbone.send("actor-1", "studio-42", [1, 2, 3]);
      expect(outcome).toEqual({ ok: true });
      expect(received).toEqual([{ uri: "studio-42", payload: [1, 2, 3] }]);
    });

    it("send against an unregistered uri fails noSuchEndpoint even when granted", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const outcome = backbone.send("actor-1", "nowhere", [1]);
      expect(outcome).toEqual({ ok: false, error: { kind: "noSuchEndpoint", uri: "nowhere" } });
    });

    it("dispatchSendMessage routes a Backbone target through send, and reports a non-Backbone target as the documented no-op gap", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport, received } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const sent = backbone.dispatchSendMessage("actor-1", { sendMessage: { target: backboneMessageEndpoint("studio-42"), payload: [9] } });
      expect(sent).toEqual({ kind: "sent" });
      expect(received).toEqual([{ uri: "studio-42", payload: [9] }]);
      const skipped = backbone.dispatchSendMessage("actor-1", { sendMessage: { target: { topic: { name: "whatever" } }, payload: [9] } });
      expect(skipped).toEqual({ kind: "notBackboneTarget" });
    });
  });
  //#endregion 🔑️CapabilityTests

  //#region 📥️DeltaFanoutTests
  describe("EffectBackbone delta fan-out", () => {
    it("mirrors backbone_delta_fanout_coalesces_a_burst_for_the_same_uri: a burst for the same uri collapses to the latest, not queued", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      const first = backbone.fanoutDelta("studio-42", [1]);
      const second = backbone.fanoutDelta("studio-42", [2]);
      expect(first.get("actor-1")).toEqual({ kind: "delivered" });
      expect(second.get("actor-1")).toEqual({ kind: "collapsed" });
      const drained = backbone.drain("actor-1");
      expect(drained).toEqual([{ message: { source: { backbone: { uri: "studio-42" } }, payload: [2] } }]);
    });

    it("fanoutDelta only reaches actors subscribed to that specific uri", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      backbone.subscribe("actor-2", "studio-99");
      const outcomes = backbone.fanoutDelta("studio-42", [7]);
      expect([...outcomes.keys()]).toEqual(["actor-1"]);
      expect(backbone.drain("actor-2")).toEqual([]);
    });

    it("fanoutDelta against a uri with no subscribers is an empty, error-free no-op", () => {
      const backbone = new EffectBackbone();
      expect(backbone.fanoutDelta("nobody-here", [1]).size).toBe(0);
    });

    it("nextRevision is monotonic per uri and independent across uris", () => {
      const backbone = new EffectBackbone();
      expect(backbone.nextRevision("a")).toBe(1);
      expect(backbone.nextRevision("a")).toBe(2);
      expect(backbone.nextRevision("b")).toBe(1);
    });
  });
  //#endregion 📥️DeltaFanoutTests

  //#region 🚨️OverflowTests
  describe("EffectBackbone queue overflow", () => {
    it("a lossless direct-send queue rejects-and-reports once full, rather than silently dropping (consistent with backbone-worker's outbox contract)", () => {
      const reporter = new RecordingBackboneOverflowReporter();
      const backbone = new EffectBackbone({ sendQueueCapacity: 2, reporter });
      backbone.subscribe("actor-1", "studio-42");
      expect(backbone.deliverMessage("actor-1", "studio-42", [1])).toEqual({ kind: "delivered" });
      expect(backbone.deliverMessage("actor-1", "studio-42", [2])).toEqual({ kind: "delivered" });
      const third = backbone.deliverMessage("actor-1", "studio-42", [3]);
      expect(third).toEqual({ kind: "rejectedFull" });
      expect(reporter.recorded()).toHaveLength(1);
      expect(reporter.recorded()[0]).toMatchObject({ actor: "actor-1", uri: "studio-42", channel: "send", outcome: { kind: "rejectedFull" } });
      // 🧾️ Nothing was silently dropped: both accepted messages are still there, in order.
      expect(backbone.drain("actor-1")).toEqual([
        { message: { source: { backbone: { uri: "studio-42" } }, payload: [1] } },
        { message: { source: { backbone: { uri: "studio-42" } }, payload: [2] } },
      ]);
    });

    it("deliverMessage against a uri actor never subscribed to is noSuchSubscriber, not a crash", () => {
      const backbone = new EffectBackbone();
      expect(backbone.deliverMessage("actor-1", "studio-42", [1])).toEqual({ kind: "noSuchSubscriber" });
    });
  });
  //#endregion 🚨️OverflowTests

  //#region 🪪️IsolationTests
  describe("EffectBackbone per-instance isolation", () => {
    it("an inbound Event::Message reaches only the subscribed actor, not other actors", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      backbone.subscribe("actor-2", "studio-42");
      backbone.deliverMessage("actor-1", "studio-42", [123]);
      expect(backbone.drain("actor-1")).toHaveLength(1);
      expect(backbone.drain("actor-2")).toHaveLength(0);
    });

    it("two EffectBackbone instances for the SAME plugin never share endpoints or subscriptions", () => {
      const instanceA = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const instanceB = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport } = fakeTransport();
      instanceA.registerEndpoint("studio-42", transport);
      instanceA.subscribe("actor-1", "studio-42");
      instanceA.deliverMessage("actor-1", "studio-42", [1]);

      // instanceB has no endpoint registered for studio-42 at all — a send through it fails noSuchEndpoint.
      expect(instanceB.send("actor-1", "studio-42", [1])).toEqual({ ok: false, error: { kind: "noSuchEndpoint", uri: "studio-42" } });
      // instanceB was never subscribed — nothing instanceA delivered leaks across.
      expect(instanceB.drain("actor-1")).toEqual([]);
      // instanceA's own state is untouched by instanceB's independent existence.
      expect(instanceA.drain("actor-1")).toHaveLength(1);
    });
  });
  //#endregion 🪪️IsolationTests

  //#region 🌉️GuestWireTests
  describe("BackboneGuestMessage wire shape", () => {
    it("round-trips a send message through base64, matching the Rust doc's {kind,uri,payload} shape", () => {
      const event: EventMessage = { message: { source: backboneMessageEndpoint("studio-42"), payload: [1, 2, 3, 255] } };
      const wire = encodeBackboneGuestMessage(event);
      expect(wire.kind).toBe("send");
      expect(Object.keys(wire).sort()).toEqual(["kind", "payload", "uri"]);
      expect(decodeBackboneGuestMessage(wire)).toEqual(event);
    });

    it("round-trips a delta message with its revision, matching the Rust doc's {kind,uri,payload,revision} shape", () => {
      const event: EventMessage = { message: { source: backboneMessageEndpoint("studio-42"), payload: [9] } };
      const wire = encodeBackboneGuestMessage(event, 7);
      expect(wire).toEqual({ kind: "delta", uri: "studio-42", payload: btoa(String.fromCharCode(9)), revision: 7 });
      expect(decodeBackboneGuestMessage(wire)).toEqual(event);
    });

    it("refuses to encode a non-Backbone source", () => {
      const event: EventMessage = { message: { source: { topic: { name: "x" } }, payload: [1] } };
      expect(() => encodeBackboneGuestMessage(event)).toThrow();
    });
  });
  //#endregion 🌉️GuestWireTests

  //#region 🌉️WorkerTransportTests
  describe("createBackboneWorkerTransport / bridgeBackboneWorkerInbound", () => {
    function fakeWorker(): { readonly worker: BackboneWorkerLike; readonly posted: BackboneWorkerWireMessage[] } {
      const posted: BackboneWorkerWireMessage[] = [];
      return { worker: { postMessage: (message) => posted.push(message), onmessage: null }, posted };
    }

    it("document opening attempt transport stamps one retained owner on its lazy open and every send", async () => {
      const { decodeBackboneWorkerRequest } = await import("../../🟦️.ts");
      const { worker, posted } = fakeWorker();
      const transport = createBackboneWorkerTransport(worker, "studio-42", { actor: "actor-1", hub: { kind: "hub", baseUrl: "https://hub.example", spaceId: "space-1" } });
      transport.send("studio-42", [1, 2]);
      transport.send("studio-42", [3, 4]);
      expect(posted).toHaveLength(3); // one "open" + two "send"
      const decoded = posted.map((message) => decodeBackboneWorkerRequest(message.wire));
      expect(decoded[0]).toMatchObject({ kind: "open", documentId: "studio-42", actor: "actor-1" });
      expect(decoded[1]).toMatchObject({ kind: "send", documentId: "studio-42", message: { kind: "publishPreview", key: "studio-42", seq: 1, payload: [1, 2] } });
      expect(decoded[2]).toMatchObject({ kind: "send", documentId: "studio-42", message: { kind: "publishPreview", key: "studio-42", seq: 2, payload: [3, 4] } });
      const clientInstanceIds = decoded.map((request) => ("clientInstanceId" in request ? request.clientInstanceId : undefined));
      expect(clientInstanceIds[0]).toMatch(/^[0-9a-f-]{36}$/u);
      expect(clientInstanceIds).toEqual([clientInstanceIds[0], clientInstanceIds[0], clientInstanceIds[0]]);
    });

    it("send throws if used for a different uri than it was bound to", () => {
      const { worker } = fakeWorker();
      const transport = createBackboneWorkerTransport(worker, "studio-42", { actor: "actor-1", hub: { kind: "hub", baseUrl: "https://hub.example", spaceId: "space-1" } });
      expect(() => transport.send("other-uri", [1])).toThrow();
    });

    it("bridgeBackboneWorkerInbound turns an inbound preview event into a fanoutDelta reaching a subscribed actor, chaining any prior onmessage", async () => {
      const { encodeBackboneWorkerResponse } = await import("../../🟦️.ts");
      const { worker } = fakeWorker();
      const priorCalls: unknown[] = [];
      const priorHandler = (event: { readonly data: unknown }): void => {
        priorCalls.push(event.data);
      };
      worker.onmessage = priorHandler;
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      const dispose = bridgeBackboneWorkerInbound(backbone, worker);
      const wire = { wire: encodeBackboneWorkerResponse({ kind: "event", documentId: "studio-42", clientInstanceId: "33333333-3333-4333-8333-333333333333", event: { kind: "preview", actor: "peer", key: "studio-42", seq: 1, payload: [42] } }) };
      worker.onmessage?.({ data: wire });
      expect(priorCalls).toEqual([wire]); // chained, not replaced
      expect(backbone.drain("actor-1")).toEqual([{ message: { source: { backbone: { uri: "studio-42" } }, payload: [42] } }]);
      dispose();
      expect(worker.onmessage).toBe(priorHandler); // restored, not nulled
    });

    it("bridgeBackboneWorkerInbound ignores non-event and non-preview responses without throwing", async () => {
      const { encodeBackboneWorkerResponse } = await import("../../🟦️.ts");
      const { worker } = fakeWorker();
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      bridgeBackboneWorkerInbound(backbone, worker);
      worker.onmessage?.({ data: { wire: encodeBackboneWorkerResponse({ kind: "ready" }) } });
      worker.onmessage?.({ data: "not a wire message at all" });
      expect(backbone.drain("actor-1")).toEqual([]);
    });
  });
  //#endregion 🌉️WorkerTransportTests

  //#region 🧬️ParityTests
  describe("EffectBackbone Rust↔TS wire parity", () => {
    function parseRustVariants(body: string): Array<{ readonly name: string; readonly fields: readonly string[] | null }> {
      const stripped = body.replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const variants: Array<{ readonly name: string; readonly fields: readonly string[] | null }> = [];
      let match: RegExpExecArray | null;
      while ((match = variantPattern.exec(stripped)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields
            .split(",")
            .map((part) => part.trim())
            .filter((part) => part.length > 0)
            .map((part) => part.split(":")[0]!.trim());
          variants.push({ name: name!, fields });
        } else if (tupleType !== undefined) {
          variants.push({ name: name!, fields: null });
        }
      }
      return variants;
    }

    function parseFieldList(fields: string): readonly string[] {
      return fields
        .split(",")
        .map((part) => part.trim())
        .filter((part) => part.length > 0)
        .map((part) => part.split(":")[0]!.trim());
    }

    it("MessageEndpoint variant/field names match the live Rust enum in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../../../../🔨️modules/🎠️kernel/🦀️.rs", source.url);
      const testSource = readFileSync(kernelUrl, "utf8");
      const enumMatch = testSource.match(/pub enum MessageEndpoint \{([\s\S]*?)\n\}/);
      expect(enumMatch).not.toBeNull(); // [DEBUG] `pub enum MessageEndpoint { ... }` shape not found — Rust source changed, update this test's regex
      const rustVariants = parseRustVariants(enumMatch![1]!);
      expect(rustVariants.map((variant) => variant.name)).toEqual(MESSAGE_ENDPOINT_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null) continue;
        const tsVariant = MESSAGE_ENDPOINT_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name)!;
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });

    it("Effect::SendMessage fields match the live Rust variant in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../../../../🔨️modules/🎠️kernel/🦀️.rs", source.url);
      const testSource = readFileSync(kernelUrl, "utf8");
      const variantMatch = testSource.match(/\bSendMessage\s*\{([^{}]*)\}/);
      expect(variantMatch).not.toBeNull(); // [DEBUG] `SendMessage { ... }` not found — Rust `Effect::SendMessage` changed, update this test
      expect(parseFieldList(variantMatch![1]!)).toEqual(["target", "payload"]);
    });

    it("Event::Message fields match the live Rust variant in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../../../../🔨️modules/🎠️kernel/🦀️.rs", source.url);
      const testSource = readFileSync(kernelUrl, "utf8");
      const variantMatch = testSource.match(/\bMessage\s*\{([^{}]*)\}/);
      expect(variantMatch).not.toBeNull(); // [DEBUG] `Message { ... }` not found — Rust `Event::Message` changed, update this test
      expect(parseFieldList(variantMatch![1]!)).toEqual(["source", "payload"]);
    });
  });
  //#endregion 🧬️ParityTests

}
