/** 🪪️ The exact receipt wire, session authority and live lease admission tests. */
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { settleInferenceTurns } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { InferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";

/** 📜 Registers the exact receipt and events wire, session authority, lease admission and cancellation retention tests. */
export function registerReceiptAndLeaseAdmissionTests(kit: GisInferencePortTestKit, helpers: InferencePortHelpers): void {
  const { beforeEach, clearHubSessionCapability, driveInferencePort, expect, handleTsRequest, installHubSessionCapability, it, resetWorkerLaw, source, testSeams } = kit;
  const { DOCUMENT, JOB, SPACE, inferenceHarness, pageBody, receiptBody } = helpers;
  beforeEach(resetWorkerLaw);
  it("accepts the exact current Hub inference receipt and events wire", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const wire = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).wire;
    const schema = JSON.parse(readFileSync(new URL("../../../🌎️hub/💡️inference/🧬️schema/🔣️.json", source.url), "utf8"));
    const production = await import("@semio-tech/gis-gismap-js/inference/schema");
    for (const [name, definition, parse] of [
      ["receipt", "InferenceJobReceiptV1", production.parseGisMapInferenceJobReceiptV1],
      ["page", "InferenceEventPageV1", production.parseGisMapInferenceEventPageV1],
    ] as const) {
      const valid = semioSchemaAjvV1({ strict: true }).compile({ $defs: schema.$defs, $ref: `#/$defs/${definition}` });
      expect(valid(wire[name]), JSON.stringify(valid.errors)).toBe(true);
      expect(equal(parse(wire[name]), wire[name])).toBe(true);
      for (const candidate of [{ ...wire[name], schema: "foreign" }, { ...wire[name], proposalHash: undefined }]) expect(() => parse(candidate)).toThrow();
    }
  });
  it("requires verified session authority and never adopts a successor proof for retained inference", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🔣️.json", import.meta.url), "utf8"));
    
    
    expect(equal(fixture.retainedClosing.authorityFence.cases, ["absent-authority-refused", "sealed-request-proof-replacement"])).toBe(true);

    const absent = await inferenceHarness({ lease: "editor", authority: "none" });
    try {
      const scope = { spaceId: SPACE, documentId: DOCUMENT };
      const fence = testSeams.browserSessionOperationFence;
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 81, payload: { scope } });
      expect(absent.posted.at(-1)).toEqual({ owner: "gis", serviceId: "s.gis.gismap.inference", kind: "inference-port-opened", operationEpoch: 81, scope, outcome: "refused", code: fixture.retainedClosing.authorityFence.openingCode });
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 81, payload: { requestId: fixture.retainedClosing.requestId } });
      expect(absent.requests).toEqual(fixture.retainedClosing.authorityFence.successorRoutes);
      clearHubSessionCapability();
      expect(installHubSessionCapability(fixture.retainedClosing.successorCapability)).toBe(true);
      expect(testSeams.browserSessionOperationFence).not.toBe(fence);
      expect(absent.requests).toEqual(fixture.retainedClosing.authorityFence.successorRoutes);
    } finally {
      absent.release();
    }

    const retained = await inferenceHarness({ lease: "editor" });
    try {
      const scope = { spaceId: SPACE, documentId: DOCUMENT };
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 82, payload: { scope } });
      retained.bodies.push("{");
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 82, payload: { requestId: fixture.retainedClosing.requestId } });
      await settleInferenceTurns();
      const operation = testSeams.inferencePort;
      expect(operation?.status.phase).toBe(fixture.retainedClosing.unknownPhase);
      const requestCount = retained.requests.length;
      clearHubSessionCapability();
      expect(installHubSessionCapability(fixture.retainedClosing.successorCapability)).toBe(true);
      await driveInferencePort(82);
      expect(retained.requests).toHaveLength(requestCount);
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 83, payload: { scope } });
      expect(retained.posted.at(-1)).toMatchObject({ kind: "inference-port-opened", outcome: "refused", code: fixture.retainedClosing.authorityFence.blockedOpeningCode });
      expect(testSeams.inferencePort).toBe(operation);
    } finally {
      retained.release();
    }
  });
  it("refuses to start at all without a verified live execution-target lease", async () => {
    const harness = await inferenceHarness({ lease: "none" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 1, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      await Promise.resolve();
      expect(harness.ports()).toEqual([]);
      expect(harness.posted.at(-1)).toEqual({ owner: "gis", serviceId: "s.gis.gismap.inference", kind: "inference-port-opened", operationEpoch: 1, scope: { spaceId: SPACE, documentId: DOCUMENT }, outcome: "refused", code: "inference.lease-unverified" });
      expect(harness.requests).toEqual([]);
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 1, payload: { requestId: "2".repeat(32) } });
      await Promise.resolve();
      expect(harness.requests).toEqual([]);
    } finally {
      harness.release();
    }
  });
  it("refuses a verified viewer-only lease, because a proposal it could never approve must not start", async () => {
    const harness = await inferenceHarness({ lease: "viewer" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 2, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      await Promise.resolve();
      expect(harness.ports()).toEqual([]);
      expect(harness.posted.at(-1)).toMatchObject({ kind: "inference-port-opened", operationEpoch: 2, outcome: "refused", code: "inference.lease-unverified" });
      expect(harness.requests).toEqual([]);
    } finally {
      harness.release();
    }
  });
  it("publishes no phase beyond submitting until an exact server receipt lands, and never mutates the document", async () => {
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 3, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(200);
      harness.bodies.push(receiptBody("accepted", "none", 0));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 3, payload: { requestId: "3".repeat(32) } });
      expect(harness.ports().map((status) => status.phase)).toEqual(["idle", "submitting"]);
      await settleInferenceTurns();
      expect(harness.ports().map((status) => status.phase)).toEqual(["idle", "submitting", "running"]);
      expect(harness.ports().at(-1)?.jobId).toBe(JOB);
      expect(harness.requests).toEqual([`POST /_semio/hub/spaces/${SPACE}/documents/${DOCUMENT}/inference/gis-map/jobs`]);
      expect(harness.state.outbox).toEqual([]);
      expect(harness.state.pendingMutations).toEqual([]);
      expect(harness.posted.some((message) => message.kind === "event" && "event" in message && (message as { event: { kind: string } }).event.kind === "remoteMutations")).toBe(false);
    } finally {
      harness.release();
    }
  });
  it("retains cancellation requested while inference submission is pending", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).cancelBeforeReceipt;
    for (const kind of fixture.intents) {
    const harness = await inferenceHarness({ lease: "editor" });
    const projection = () => { const { phase, jobId, cancelRequested } = harness.ports().at(-1)!; return { phase, jobId, cancelRequested }; };
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: fixture.operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(200, 200);
      harness.bodies.push(receiptBody("accepted", "none", 0), pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 0, total: 4, cancelRequested: true }));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: fixture.operationEpoch, payload: { requestId: fixture.requestId } });
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: kind, operationEpoch: fixture.operationEpoch, payload: {} });
      expect(equal(projection(), fixture.expectedBeforeReceipt)).toBe(true);
      await settleInferenceTurns();
      expect(harness.ports().some(({ phase, jobId, cancelRequested }) => equal({ phase, jobId, cancelRequested }, fixture.expectedAfterReceipt))).toBe(true);
      await driveInferencePort(fixture.operationEpoch);
      expect(harness.ports().at(-1)?.phase).toBe(fixture.expectedTerminal);
      expect(equal(harness.requests, fixture.expectedRequests)).toBe(true);
    } finally { harness.release(); }
    }
  });
}
