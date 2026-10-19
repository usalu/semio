/** 🔌️ The attached broker port retirement and queued request tests. */
import { WORKER_LAW_CAPABILITY } from "@semio-tech/framework-os/testing/worker-law-fakes";
import { settleInferenceTurns } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { InferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";

/** 🚪 Registers the attached broker port retirement, old queued request and retired inference work tests. */
export function registerBrokerPortRetirementTests(kit: GisInferencePortTestKit, helpers: InferencePortHelpers): void {
  const { clearHubSessionCapability, expect, handleTsRequest, hubSessionFetch, installHubSessionCapability, it, source, testSeams } = kit;
  const { DOCUMENT, SPACE, inferenceHarness, receiptBody } = helpers;
  it("retires an attached broker port before its old session body can authorize the successor", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
    const retained = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    const harness = await inferenceHarness({ lease: "editor" });
    const prior = new MessageChannel(), successor = new MessageChannel();
    const body = JSON.stringify(authority);
    let release: () => void = () => undefined;
    const gate = new Promise<void>((resolve) => { release = resolve; });
    let cancelled = false;
    const stream = new ReadableStream<Uint8Array>({ async start(controller) { await gate; if (!cancelled) { controller.enqueue(new TextEncoder().encode(body)); controller.close(); } }, cancel() { cancelled = true; } });
    const request = { kind: "request", operation: "me", requestId: crypto.randomUUID() };
    try {
      testSeams.attachHubSessionPort(prior.port1);
      (globalThis as unknown as { fetch: unknown }).fetch = async () => ({ ok: true, status: 200, statusText: "", headers: new Headers({ "content-length": String(new TextEncoder().encode(body).length) }), body: stream });
      prior.port2.postMessage(request);
      for (let turn = 0; !stream.locked && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
      expect(stream.locked).toBe(true);
      testSeams.attachHubSessionPort(successor.port1);
      for (let turn = 0; !cancelled && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
      expect(cancelled).toBe(true);
      release();
      await settleInferenceTurns();
      expect(equal(testSeams.browserSessionAuthority, null), retained.portReplacement).toBe(true);
      expect(cancelled).toBe(true);
      expect(installHubSessionCapability(retained.successorCapability)).toBe(true);
    } finally { release(); testSeams.detachHubSessionPort(); prior.port1.close(); prior.port2.close(); successor.port1.close(); successor.port2.close(); harness.release(); }
  });
  it("never dispatches an old queued request under a replacement broker admission", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    const harness = await inferenceHarness({ lease: "editor" });
    let release: () => void = () => undefined;
    try {
      harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
      harness.bodies.push("{}");
      const first = hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
      const firstRejected = expect(first).rejects.toThrow();
      for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
      expect(harness.requests).toHaveLength(1);
      const queued = hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
      const queuedOutcome = queued.then(() => "dispatched", () => fixture.brokerReplacement);
      clearHubSessionCapability();
      expect(installHubSessionCapability(fixture.successorCapability)).toBe(true);
      release();
      await firstRejected;
      expect(await queuedOutcome).toBe(fixture.brokerReplacement);
      expect(equal(harness.requests.length, 1)).toBe(true);
      harness.bodies.push("{}");
      await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
      expect(harness.proofs).toHaveLength(2);
      expect(harness.proofs[0]).toBe(`Bearer ${WORKER_LAW_CAPABILITY}`);
      expect(equal(harness.proofs[1], `Bearer ${fixture.successorCapability}`)).toBe(true);
    } finally { release(); harness.release(); }
  });
  it("does not spend the active broker proof on queued retired inference work", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    for (const scenario of fixture.queuedRetirement) {
    const harness = await inferenceHarness({ lease: "editor" });
      const epoch = testSeams.directorySessionEpoch;
      try {
        const operationEpoch = fixture.operationEpoch + 3;
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch, payload: { requestId: fixture.requestId } });
        await settleInferenceTurns();
        let release: () => void = () => undefined;
        harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
        harness.bodies.push("{}");
        const blocker = hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        for (let turn = 0; harness.requests.length < 2 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(harness.requests).toHaveLength(2);
        let rejected: Promise<unknown> | undefined;
        if (scenario === "abort") {
          const abort = new AbortController();
          rejected = expect(hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, signal: abort.signal })).rejects.toThrow();
          abort.abort();
        } else {
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch, payload: {  } });
          testSeams.directorySessionEpoch += 1;
        }
        release();
        await blocker;
        await rejected;
        await settleInferenceTurns();
        expect(equal(harness.requests.length, 2)).toBe(true);
        harness.bodies.push("{}");
        await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        expect(harness.requests).toHaveLength(3);
      } finally { testSeams.directorySessionEpoch = epoch; harness.release(); }
    }
  });
}
