/** 🔑️ The verified broker session authority and Directory bootstrap tests. */
import { gisMapDocumentServiceWireDeclarationV1 } from "@semio-tech/gis-gismap-js/inference/client/io";
import { WORKER_LAW_ACTOR_IDENTITY_GRANT } from "@semio-tech/framework-os/testing/worker-law-fakes";
import { settleInferenceTurns } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { BackboneWorkerResponse, GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { InferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";

/** 🗝 Registers the verified broker session, physical reopen, bootstrap page correlation and unverified response tests. */
export function registerBrokerSessionAuthorityTests(kit: GisInferencePortTestKit, helpers: InferencePortHelpers): void {
  const { DocumentExecutionTargetLease, artifactState, clearHubSessionCapability, documentExecutionTargetLeaseMintToken, driveInferencePort, encodePackValue, expect, handleTsRequest, hubSessionFetch, installHubSessionCapability, it, openArtifact, source, testSeams } = kit;
  const { DOCUMENT, SPACE, inferenceHarness, leaseFields, receiptBody } = helpers;
  it("binds the verified broker session and retains uncertain inference across authenticated replacement", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const authorities = fixture.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
    const retained = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    const harness = await inferenceHarness({ lease: "editor" });
    const channel = new MessageChannel();
    testSeams.attachHubSessionPort(channel.port1);
    const me = async (value: unknown): Promise<{ status: number; body: string }> => {
      harness.bodies.push(JSON.stringify(value));
      return await new Promise((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error("session authority RPC timed out")), 3000);
        channel.port2.onmessage = (event) => { clearTimeout(timer); resolve(event.data); };
        channel.port2.postMessage({ kind: "request", operation: "me", requestId: crypto.randomUUID() });
      });
    };
    try {
      expect((await me(authorities[0])).status).toBe(200);
      expect(equal(testSeams.browserSessionAuthority, authorities[0])).toBe(true);
      const epoch = testSeams.directorySessionEpoch;
      expect((await me(authorities[0])).status).toBe(200);
      expect(testSeams.directorySessionEpoch).toBe(epoch);
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: retained.operationEpoch + 5, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.bodies.push(receiptBody("accepted", "none", 0));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: retained.operationEpoch + 5, payload: { requestId: retained.requestId } });
      await settleInferenceTurns();
      const original = testSeams.inferencePort;
      if (original === null) throw new Error("inference port retired before the broker replacement");
      const originalRequest = original.request;
      if (originalRequest === null) throw new Error("retained inference port carries no request");
      expect(originalRequest.requestId).toBe(retained.requestId);
      expect((await me(authorities[1])).status).toBe(200);
      expect(testSeams.directorySessionEpoch).toBe(epoch + 1);
      expect(equal(testSeams.browserSessionAuthority, authorities[1])).toBe(true);
      expect(artifactState(DOCUMENT, SPACE)).toBeUndefined();
      expect(testSeams.inferencePort).toBe(original);
      expect(original.status.phase).toBe("indeterminate");
      expect(originalRequest.requestId).toBe(retained.requestId);
      await driveInferencePort(original.operationEpoch);
      expect(harness.requests.filter((entry: string) => entry.endsWith("/jobs"))).toHaveLength(1);
      expect(harness.requests.filter((entry: string) => entry.endsWith("/cancel") || entry.endsWith("/reconcile"))).toHaveLength(0);
    } finally { testSeams.detachHubSessionPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
  });
  it("requires verified session authority to physically reopen a fresh document owner", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const authorityFixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const authorities = authorityFixture.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🔣️.json", import.meta.url), "utf8"));
    
    
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      const original = harness.state;
      const originalClientInstanceId = original.openClientInstanceId;
      harness.bodies.push(JSON.stringify(authorities[1]));
      await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
      const absentBeforeReopen = artifactState(DOCUMENT, SPACE) === undefined;
      const successorClientInstanceId = "12345678-1234-4123-8123-123456789abd";
      openArtifact({ documentId: DOCUMENT, schema: "gis.map", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: SPACE }], actor: "caller", clientInstanceId: successorClientInstanceId, actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
      const successor = artifactState(DOCUMENT, SPACE)!;
      successor.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, leaseFields(true), "http://hub.test", new Uint8Array(1), encodePackValue({ manifest: { topicContributions: [{ topic: "semio.os.document-http-port/v1", payload: gisMapDocumentServiceWireDeclarationV1() }] } }));
      const operationEpoch = fixture.retainedClosing.operationEpoch + 11;
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      const opening = harness.posted.filter((message) => message.kind === "inference-port-opened").at(-1) as Extract<BackboneWorkerResponse, { kind: "inference-port-opened" }>;
      const projection = {
        oldClosed: original.closed,
        absentBeforeReopen,
        differentOwner: successor !== original,
        differentClient: successor.openClientInstanceId !== originalClientInstanceId,
        openingOutcome: opening.outcome,
        successorOwnsFreshClient: testSeams.inferencePort?.clientInstanceId === successorClientInstanceId,
      };
      expect(equal(projection, fixture.retainedClosing.authorityFence.physicalReopen)).toBe(true);
      expect(testSeams.inferencePort?.request).toBeNull();
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "close", operationEpoch, payload: {  } });
      expect(testSeams.inferencePort).toBeNull();
    } finally { harness.release(); }
  });
  it("correlates every Directory bootstrap page with the verified broker session authority", async () => {
    const { readFileSync } = await import("node:fs");
    const { createHash } = await import("node:crypto");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
    const retained = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    for (const scenario of retained.directoryPageBinding) {
    const harness = await inferenceHarness({ lease: "editor" });
      const channel = new MessageChannel();
      testSeams.attachHubSessionPort(channel.port1);
      try {
        harness.bodies.push(JSON.stringify(authority));
        const ready = new Promise<{ status: number }>((resolve, reject) => {
          const timer = setTimeout(() => reject(new Error("session authority RPC timed out")), 3000);
          channel.port2.onmessage = (event) => { clearTimeout(timer); resolve(event.data); };
        });
        channel.port2.postMessage({ kind: "request", operation: "me", requestId: crypto.randomUUID() });
        expect((await ready).status).toBe(200);
        const epoch = testSeams.directorySessionEpoch;
        const page = { schema: "semio.directory.event-page.v1", sessionBindingSha256: scenario === "foreign-binding" ? "b".repeat(64) : authority.sessionBindingSha256, authorizationGeneration: authority.authorizationGeneration + (scenario === "foreign-generation" ? 1 : 0), afterSeqExclusive: 0, throughSeqInclusive: 0, hasMore: false, events: [] };
        const receiptSha256 = createHash("sha256").update(JSON.stringify(page)).digest("hex");
        harness.bodies.push(JSON.stringify({ ...page, receiptSha256 }));
        handleTsRequest({ kind: "directory-bootstrap-open", baseUrl: "http://hub.test", after: 0, bootstrapEpoch: 23 });
        for (let turn = 0; !harness.posted.some((message: BackboneWorkerResponse) => message.kind === "directory-event-page" || message.kind === "directory-bootstrap-failed") && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        const pages = harness.posted.filter((message: BackboneWorkerResponse) => message.kind === "directory-event-page");
        if (scenario === "same-authority") {
          expect(pages).toHaveLength(1);
          expect(equal(testSeams.browserSessionAuthority, authority)).toBe(true);
          expect(testSeams.directorySessionEpoch).toBe(epoch);
          expect(artifactState(DOCUMENT, SPACE)).toBeDefined();
        } else {
          expect(pages).toHaveLength(0);
          expect(testSeams.browserSessionAuthority).toBeNull();
          expect(testSeams.directorySessionEpoch).toBe(epoch + 1);
          expect(artifactState(DOCUMENT, SPACE)).toBeUndefined();
          expect(harness.posted.some((message: BackboneWorkerResponse) => message.kind === "directory-bootstrap-failed" && message.code === "unauthorized" && !message.retryable)).toBe(true);
        }
      } finally { testSeams.detachHubSessionPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
    }
  });
  it("retires unverified session responses without letting old bodies overwrite replacement authority", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const value = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
    const retained = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    for (const scenario of retained.authorityRetirement) {
    const harness = await inferenceHarness({ lease: "editor" });
      const channel = new MessageChannel();
      testSeams.attachHubSessionPort(channel.port1);
      const read = async (body: unknown): Promise<{ status: number; body: string }> => {
        harness.bodies.push(JSON.stringify(body));
        return await new Promise((resolve, reject) => {
          const timer = setTimeout(() => reject(new Error("session authority RPC timed out")), 3000);
          channel.port2.onmessage = (event) => { clearTimeout(timer); resolve(event.data); };
          channel.port2.postMessage({ kind: "request", operation: "me", requestId: crypto.randomUUID() });
        });
      };
      let release: () => void = () => undefined;
      const heldBody: { current: ReadableStream<Uint8Array> | null } = { current: null };
      try {
        expect((await read(value)).status).toBe(200);
        const epoch = testSeams.directorySessionEpoch;
        if (scenario === "http-401") harness.statuses.push(401);
        if (scenario === "non-200") harness.statuses.push(201);
        if (scenario === "old-body-after-proof-replacement") {
          const originalFetch = globalThis.fetch;
          const gate = new Promise<void>((resolve) => { release = resolve; });
          (globalThis as unknown as { fetch: unknown }).fetch = async (...args: Parameters<typeof fetch>) => {
            const response = await originalFetch(...args);
            heldBody.current = new ReadableStream({ async start(controller) { await gate; controller.enqueue(new TextEncoder().encode(JSON.stringify(value))); controller.close(); } });
            return { ...response, body: heldBody.current };
          };
        }
        const pending = read(scenario === "invalid-response" ? {} : value);
        if (scenario === "old-body-after-proof-replacement") {
          for (let turn = 0; !heldBody.current?.locked && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
          expect(harness.requests).toHaveLength(2);
          expect(heldBody.current?.locked).toBe(true);
          clearHubSessionCapability();
          expect(installHubSessionCapability(retained.successorCapability)).toBe(true);
          release();
        }
        expect((await pending).status).toBe(scenario === "http-401" ? 401 : 503);
        const retires = scenario === "http-401" || scenario === "old-body-after-proof-replacement";
        expect(testSeams.browserSessionAuthority === null).toBe(retires);
        expect(testSeams.directorySessionEpoch).toBe(epoch + (retires ? 1 : 0));
        expect(artifactState(DOCUMENT, SPACE) === undefined).toBe(retires);
        if (scenario === "old-body-after-proof-replacement") {
          harness.bodies.push("{}");
          await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
          expect(equal(harness.proofs.at(-1), `Bearer ${retained.successorCapability}`)).toBe(true);
        }
      } finally { release(); testSeams.detachHubSessionPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
    }
  });
}
