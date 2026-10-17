/** 💡️ The host-owned GIS Map inference port driven through a fake authenticated transport, and the retained inference job across a refused successor opening. */
import { idleGisMapInferencePortStatusV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import { gisMapDocumentServiceWireDeclarationV1 } from "@semio-tech/gis-gismap-js/inference/client/io";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { workerLawFakesV1, WORKER_LAW_CAPABILITY, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1, InferenceOperationV1 } from "../🧰️dependencies/🟦️.ts";

type BackboneWorkerResponse = import("@semio-tech/framework-os").BackboneWorkerResponse;
type DocumentExecutionTargetLeaseFieldsV1 = import("@semio-tech/framework/directory/schema").DocumentExecutionTargetLeaseFieldsV1;
type GisMapInferencePortStatusV1 = import("@semio-tech/gis-gismap-js/inference/schema").GisMapInferencePortStatusV1;
type GisMapInferencePreviewV1 = import("@semio-tech/gis-gismap-js/inference/schema").GisMapInferencePreviewV1;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerGisInferencePortTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, dispatchBackboneWorkerRequest, parseDocumentExecutionTargetLeaseFieldsV1, idleGisMapInferencePortStatusV1, handleTsRequest, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, openArtifact, artifactState, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, encodePackValue, closeArtifact, driveInferencePort, DirectoryClient } = dependencies;
  //#region 💡️InferencePortTests
  vitest.it("retains the preceding inference job when a successor opening is refused", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    for (const field of ["closed", "request", "opened", "refused", "authorityRefused", "indeterminate"]) Object.assign(fixture[field], { owner: "gis", serviceId: "s.gis.gismap.inference" });
    for (const kind of fixture.routes) {
      const routes: string[] = [];
      const request = fixture.request;
      dispatchBackboneWorkerRequest(request, { handleRequestBytes: () => { routes.push("rust"); }, postReady: () => { routes.push("rust-ready"); } }, () => { routes.push("typescript"); });
      vitest.expect(equal(routes, ["typescript"])).toBe(true);
    }
    const seams = dependencies.testSeams;
    const previous = seams.inferencePort;
    const sink = seams.workerPostTestSink;
    const messages: unknown[] = [];
    // 🧭️ A real `InferenceOperationV1`, not a partial double: its lease fields come from the committed
    // browser-document-open corpus through the production parser, so the retained port this test holds
    // is the same shape the worker mints.
    const leaseFields = parseDocumentExecutionTargetLeaseFieldsV1(
      (JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", source.url), "utf8")) as { installedTarget: unknown }).installedTarget,
    );
    const retained: InferenceOperationV1 = {
      transport: { call: async () => { throw new Error("capacity law must not perform transport"); } },
      operationEpoch: 20,
      scope: fixture.request.payload.scope,
      abort: new AbortController(),
      sessionEpoch: testSeams.directorySessionEpoch,
      sessionFence: { token: {}, sessionBindingSha256: "0".repeat(64), authorizationGeneration: 1 },
      clientInstanceId: "12345678-1234-4123-8123-123456789abc",
      leaseFields,
      request: null,
      closeRequested: false,
      reconcileRequired: false,
      status: { ...idleGisMapInferencePortStatusV1(), phase: "running", jobId: "1".repeat(32) },
      turns: 0,
      pollTimer: null,
      inFlight: false,
      cancelSent: false,
      closed: false,
    };
    try {
      seams.inferencePort = retained;
      seams.workerPostTestSink = (message: unknown) => { messages.push(message); };
      await handleTsRequest(fixture.request);
      vitest.expect(seams.inferencePort).toBe(retained);
      vitest.expect(retained.abort.signal.aborted).toBe(false);
      vitest.expect(equal(messages, [fixture.refused])).toBe(true);
    } finally {
      seams.inferencePort = previous;
      seams.workerPostTestSink = sink;
    }
  });

  // 💡️ Slice D — the host-owned ephemeral GIS Map inference port, driven through a FAKE, already
  // authenticated transport (an installed browser-broker proof plus a stubbed `fetch`). These are
  // the port's own laws: it refuses to start without a verified live execution-target lease, it
  // publishes no state before an exact server receipt, a Cancel click is never an optimistic
  // terminal, `stale` and `failed` are hard, and no document mutation, outbox entry or artifact
  // event is ever produced.
  describe("gis map inference port", () => {
    beforeEach(resetWorkerLaw);
    const SPACE = "sp-inference";
    const DOCUMENT = "doc-inference";
    const JOB = "1".repeat(32);
    const HASH = "9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130";
    const PREVIEW: GisMapInferencePreviewV1 = {
      schema: "semio.hub.gis-map-inference-preview/v1",
      jobId: JOB,
      proposalHash: HASH,
      regionId: `inference-${JOB}`,
      ring: [
        [7, 46],
        [9, 46],
        [9, 48],
        [7, 48],
        [7, 46],
      ],
    };

    function leaseFields(write: boolean): DocumentExecutionTargetLeaseFieldsV1 {
      return parseDocumentExecutionTargetLeaseFieldsV1({
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: { spaceId: SPACE, documentId: DOCUMENT },
        descriptorDigestV1: HASH,
        catalog: { generationId: HASH },
        package: { pluginId: "gis", packageId: "semio:gis", version: "0.1.0", componentSha256: HASH, componentBlake3: HASH, descriptorByteSha256: HASH, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 }, componentReceipt: { schema: "semio.plugin.component-receipt/v1", pluginId: "gis", packageId: "semio:gis", profile: "release", target: "wasm32-wasip2", wasmSha256: HASH, wasmBytes: 4096, semanticDescriptorSha256: HASH, toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "a".repeat(64), provenanceSha256: "b".repeat(64) } },
        component: { sha256: HASH, blake3: HASH, byteLength: 4096 },
        descriptor: { sha256: HASH, byteLength: 1 },
        browserActor: {
          kind: "closed-browser-actor",
          schema: "semio.os.closed-browser-actor.v1",
          codegenPolicy: "semio.os.browser-jco-1.34.0-jspi.v1",
          byteLength: 3,
          sha256: HASH,
          sourceComponentSha256: HASH,
          sourceDescriptorByteSha256: HASH,
          policySha256: "4".repeat(64),
          importInterfaces: ["semio:framework/host-async@1.0.0", "semio:framework/pure@1.0.0"],
        },
        artifact: { kind: "s.gis.gismap", schema: "gis.map", packSchemaHash: HASH },
        parentDialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "*" },
        surface: { surfaceId: "inference", appId: "gis", windowKindId: "gis-main", role: write ? "editor" : "viewer", rendererTarget: "wasm" },
        grant: { read: true, write, observe: true },
        checkpoint: {
          checkpointId: HASH, descriptorDigestV1: HASH, aggregateSha256: HASH,
          baselineFrontier: { documentId: DOCUMENT, headEditOrdinal: 0, headEditId: "", lastCommitSeq: 0, chainHash: Array(32).fill(0) },
        },
        revalidation: { directoryRevision: 1, membershipGeneration: 1, sessionGeneration: 1 },
      });
    }

    async function inferenceHarness(options: { readonly lease: "none" | "viewer" | "editor"; readonly authority?: "verified" | "none" }) {
      const posted: BackboneWorkerResponse[] = [];
      const original = testSeams.workerPostTestSink;
      testSeams.workerPostTestSink = (message) => posted.push(message);
      const originalFetch = globalThis.fetch;
      const requests: string[] = [];
      const bodies: string[] = [];
      const statuses: number[] = [];
      const gates: Promise<void>[] = [];
      const proofs: string[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: { method?: string; headers?: Record<string, string> }) => {
        requests.push(`${init?.method ?? "GET"} ${input}`);
        proofs.push(init?.headers?.["authorization"] ?? "");
        const status = statuses.shift() ?? 200;
        const body = bodies.shift() ?? "";
        await gates.shift();
        return {
          ok: status < 400,
          status,
          statusText: "",
          headers: new Headers({ "content-type": "application/json", "content-length": String(new TextEncoder().encode(body).length) }),
          body: new Response(body).body,
          text: async () => body,
          json: async () => JSON.parse(body),
        };
      };
      clearHubSessionCapability();
      installHubSessionCapability(WORKER_LAW_CAPABILITY);
      if (options.authority !== "none") {
        const { readFileSync } = await import("node:fs");
        const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
        const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
        bodies.push(JSON.stringify(authority));
        await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
        requests.length = 0;
        bodies.length = 0;
        statuses.length = 0;
        gates.length = 0;
        proofs.length = 0;
      }
      // 🧷️ No socket-grant issuer and no installed target means `openArtifact` opens no hub socket
      // at all, so this harness exercises the inference port's own four calls and nothing else.
      const originalIssue = testSeams.documentSocketGrantTestIssue;
      testSeams.documentSocketGrantTestIssue = null;
      openArtifact({ documentId: DOCUMENT, schema: "gis.map", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: SPACE }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
      const state = artifactState(DOCUMENT, SPACE)!;
      if (options.lease !== "none") state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, leaseFields(options.lease === "editor"), "http://hub.test", new Uint8Array(1), encodePackValue({ manifest: { topicContributions: [{ topic: "semio.os.document-http-port/v1", payload: gisMapDocumentServiceWireDeclarationV1() }] } }));
      return {
        posted,
        requests,
        bodies,
        statuses,
        gates,
        proofs,
        state,
        ports: () => posted.filter((message) => message.kind === "inference-port-status").map((message) => (message as Extract<BackboneWorkerResponse, { kind: "inference-port-status" }>).status.payload as GisMapInferencePortStatusV1),
        release: () => {
          const retained = testSeams.inferencePort;
          if (retained !== null) {
            retained.closed = true;
            if (retained.pollTimer !== null) clearTimeout(retained.pollTimer);
            retained.abort.abort();
            testSeams.inferencePort = null;
          }
          closeArtifact(DOCUMENT, SPACE);
          clearHubSessionCapability();
          testSeams.workerPostTestSink = original;
          testSeams.documentSocketGrantTestIssue = originalIssue;
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        },
      };
    }

    const receiptBody = (state: string, proposalState: string, cursor: number, proposalHash?: string): string =>
      JSON.stringify({ schema: "semio.hub.inference-job-receipt/v1", jobId: JOB, state, proposalState, proposalHash: proposalHash ?? null, cursor, expiresAtMs: 1_700_000_060_000 });

    const pageBody = (
      state: string,
      proposalState: string,
      options: { readonly nextCursor: number; readonly completed: number; readonly total: number; readonly proposalHash?: string; readonly preview?: GisMapInferencePreviewV1; readonly cancelRequested?: boolean; readonly stale?: boolean },
    ): string =>
      JSON.stringify({
        schema: "semio.hub.inference-job-events/v1",
        jobId: JOB,
        state,
        proposalState,
        cancelRequested: options.cancelRequested ?? false,
        stale: options.stale ?? false,
        proposalHash: options.proposalHash ?? null,
        ...(options.preview === undefined ? {} : { preview: options.preview }),
        events: [],
        progress: [{ cursor: options.nextCursor, runEpoch: 1, completed: options.completed, total: options.total, atMs: 1_700_000_001_000 }],
        nextCursor: options.nextCursor,
      });

    it("accepts the exact current Hub inference receipt and events wire", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const wire = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).wire;
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
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🔣️.json", import.meta.url), "utf8"));
      
      
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
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).cancelBeforeReceipt;
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

    it("retains the sealed inference owner through uncertain submit and document retirement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
      for (const scenario of fixture.cases) {
      const harness = await inferenceHarness({ lease: "editor" });
        const sessionEpoch = testSeams.directorySessionEpoch;
        try {
          const scope = { spaceId: SPACE, documentId: DOCUMENT };
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: fixture.operationEpoch, payload: { scope } });
          harness.bodies.push(scenario === "retired-document" ? receiptBody("accepted", "none", 0) : "{");
          let releaseReceipt: (() => void) | undefined;
          if (scenario === "retired-document") harness.gates.push(new Promise<void>((resolve) => { releaseReceipt = resolve; }));
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: fixture.operationEpoch, payload: { requestId: fixture.requestId } });
          const original = testSeams.inferencePort;
          if (scenario === "retired-document") {
            for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
            expect(harness.requests).toHaveLength(1);
            closeArtifact(DOCUMENT, SPACE);
            releaseReceipt!();
          }
          else handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "close", operationEpoch: fixture.operationEpoch, payload: {  } });
          await settleInferenceTurns();
          expect(testSeams.inferencePort).toBe(original);
          expect(original?.request?.requestId).toBe(fixture.requestId);
          expect(harness.posted.some((message) => message.kind === fixture.closedKind)).toBe(false);
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: fixture.operationEpoch + 1, payload: { scope } });
          expect(harness.posted.at(-1)).toMatchObject({ kind: "inference-port-opened", outcome: "refused", code: "inference.capacity" });
          if (scenario === "rotated-session") {
            testSeams.directorySessionEpoch += 1;
            const count = harness.requests.length;
            await driveInferencePort(fixture.operationEpoch);
            expect(harness.requests).toHaveLength(count);
            expect(harness.ports().at(-1)?.phase).toBe(fixture.unknownPhase);
            expect(testSeams.inferencePort).toBe(original);
            continue;
          }
          if (scenario === "lost-submit") {
            expect(harness.ports().at(-1)?.phase).toBe(fixture.unknownPhase);
            harness.bodies.push(JSON.stringify({
              schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
              job: {
                receipt: { schema: "semio.hub.inference-job-receipt/v1", jobId: JOB, state: "running", proposalState: "none", proposalHash: null, cursor: 0, expiresAtMs: 1_700_000_060_000 },
                page: { jobId: JOB, state: "running", proposalState: "none", cancelRequested: false, expired: false, proposalHash: null, events: [], progress: [], nextCursor: 0 },
                approval: null,
              },
            }));
            await driveInferencePort(fixture.operationEpoch);
            expect(harness.requests.at(-1)).toBe(fixture.reconcilePath);
            expect(original?.status.jobId).toBe(JOB);
          }
          harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 0, total: 4, cancelRequested: true }));
          await driveInferencePort(fixture.operationEpoch);
          expect(harness.ports().at(-1)?.phase).toBe(fixture.terminalPhase);
          expect(testSeams.inferencePort).toBeNull();
          const closed = harness.posted.filter((message) => message.kind === fixture.closedKind);
          expect(equal(closed, [{ kind: fixture.closedKind, owner:"gis", serviceId:"s.gis.gismap.inference", operationEpoch: fixture.operationEpoch, scope }])).toBe(true);
          expect(harness.requests.filter((request) => request.endsWith("/jobs"))).toHaveLength(1);
        } finally {
          testSeams.directorySessionEpoch = sessionEpoch;
          const retained = testSeams.inferencePort;
          if (retained?.pollTimer) clearTimeout(retained.pollTimer);
          retained?.abort.abort();
          testSeams.inferencePort = null;
          harness.release();
        }
      }
    });

    it("never restores an inference Undo owner after the document closes", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
      for (const scenario of fixture.approvalClose) {
      const harness = await inferenceHarness({ lease: "editor" });
        try {
          const operationEpoch = fixture.operationEpoch + 2;
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
          harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch, payload: { requestId: fixture.requestId } });
          await settleInferenceTurns();
          harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
          await driveInferencePort(operationEpoch);
          const receipt = { schema: "semio.hub.inference-approval-receipt/v1", jobId: JOB, mutationId: JOB, commandHash: HASH, proposalHash: HASH, applied: true, undo: { targetId: "2".repeat(32), expectedCurrent: { documentId: DOCUMENT, headEditOrdinal: 2, headEditId: "edit-2", lastCommitSeq: 2, chainSha256: "3".repeat(64) } } };
          let release: () => void = () => undefined;
          if (scenario === "approval-response") {
            harness.bodies.push(JSON.stringify(receipt));
            harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
          } else harness.bodies.push("{");
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch, payload: {  } });
          for (let turn = 0; harness.requests.length < 3 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
          expect(harness.requests).toHaveLength(3);
          if (scenario === "reconcile-response") await settleInferenceTurns();
          closeArtifact(DOCUMENT, SPACE);
          const statusCount = harness.posted.filter((message) => message.kind === "inference-history-status").length;
          if (scenario === "approval-response") release();
          else {
            harness.bodies.push(JSON.stringify({
              schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
              job: {
                receipt: { schema: "semio.hub.inference-job-receipt/v1", jobId: JOB, state: "succeeded", proposalState: "approved", proposalHash: HASH, cursor: 1, expiresAtMs: 1_700_000_060_000 },
                page: { jobId: JOB, state: "succeeded", proposalState: "approved", cancelRequested: false, expired: true, proposalHash: HASH, events: [], progress: [], nextCursor: 1 },
                approval: { state: "available", receipt },
              },
            }));
          }
          await settleInferenceTurns();
          if (scenario === "reconcile-response") await driveInferencePort(operationEpoch);
          expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
          expect(harness.posted.filter((message) => message.kind === "inference-history-status")).toHaveLength(statusCount);
          expect(equal(harness.ports().at(-1)?.phase, "applied")).toBe(true);
        } finally { harness.release(); }
      }
    });

    it("preserves the successor broker proof when an old inference owner responds", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
      const harness = await inferenceHarness({ lease: "editor" });
      const epoch = testSeams.directorySessionEpoch;
      try {
        let release: () => void = () => undefined;
        harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
        harness.bodies.push("{}");
        const pending = hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, admit: () => testSeams.directorySessionEpoch === epoch });
        const rejected = expect(pending).rejects.toThrow();
        for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(harness.requests).toHaveLength(1);
        testSeams.directorySessionEpoch += 1;
        clearHubSessionCapability();
        expect(installHubSessionCapability(fixture.successorCapability)).toBe(true);
        release();
        await rejected;
        harness.bodies.push("{}");
        await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        expect(harness.proofs).toHaveLength(2);
        expect(harness.proofs[0]).toBe(`Bearer ${WORKER_LAW_CAPABILITY}`);
        expect(equal(harness.proofs[1], `Bearer ${fixture.successorCapability}`)).toBe(true);
      } finally { testSeams.directorySessionEpoch = epoch; harness.release(); }
    });

    it("validates the exact canonical authenticated Directory client response", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      for (const row of fixture.rows) {
        const client = new DirectoryClient("http://hub.test", { request: async () => new Response(JSON.stringify(row.value), { status: 200 }) });
        if (row.accepted) expect(equal(await client.me(), row.value)).toBe(true);
        else await expect(client.me()).rejects.toThrow();
      }
      for (const row of fixture.raw) {
        const client = new DirectoryClient("http://hub.test", { request: async () => new Response(row.source, { status: 200 }) });
        if (row.accepted) expect(equal(await client.me(), JSON.parse(row.source))).toBe(true);
        else await expect(client.me()).rejects.toThrow();
      }
    });

    it("binds the verified broker session and retains uncertain inference across authenticated replacement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authorities = fixture.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
      const retained = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🔣️.json", import.meta.url), "utf8"));
      
      
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
      const retained = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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
      const retained = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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
          // 🚪️ The session port's own two refusal statuses (`👷️worker/🟦️.ts:838`): `401` when the hub
          // itself said the capability is gone, `503` for every other refused revalidation. The
          // broker-era `428` this asserted has no producer left anywhere in the worker.
          expect((await pending).status).toBe(scenario === "http-401" ? 401 : 503);
          // 🪪️ Retirement follows the HUB's authority over the session, not the shape of one answer.
          // `401` is the hub saying this capability is gone, and a capability replaced mid-flight is
          // the human's own switch — both retire every owner, bump the session epoch and drop the
          // open artifacts. A `201` or an unparsable body is the link failing to CONFIRM the session,
          // which says nothing about it: the read is refused (`503`, so the caller retries) and the
          // authority the worker already accepted is kept. This is the same law the shell's own
          // revalidation loop follows one layer up (`🪪️session-refresh/🟦️.ts`, ticket 26/09/18 C1c
          // §S5.2) — before that slice both layers signed a shell out for ever on one hiccup.
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

    it("retires an attached broker port before its old session body can authorize the successor", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const retained = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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
      const fixture = JSON.parse(readFileSync(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
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

    it("records a Cancel click as requested and reaches cancelled only on the server's own answer", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 4, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 4, payload: { requestId: "4".repeat(32) } });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 1, total: 4, cancelRequested: true }));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 4, payload: {  } });
        const requested = harness.ports().at(-1)!;
        expect(requested.cancelRequested).toBe(true);
        expect(requested.phase).toBe("running");
        // 🛑️ A second Cancel click never sends a second cancel request.
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 4, payload: {  } });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("cancelled");
        expect(harness.requests.filter((entry) => entry.endsWith("/cancel"))).toEqual([`POST /_semio/hub/spaces/${SPACE}/documents/${DOCUMENT}/inference/gis-map/jobs/${JOB}/cancel`]);
      } finally {
        harness.release();
      }
    });

    it("reports a stale base as a hard terminal that no later answer can move", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 5, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 5, payload: { requestId: "5".repeat(32) } });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("running", "none", { nextCursor: 1, completed: 1, total: 4, stale: true }));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 5, payload: {  } });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("stale");
        const before = harness.ports().length;
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 5, payload: {  } });
        await settleInferenceTurns();
        expect(harness.ports().length).toBe(before);
      } finally {
        harness.release();
      }
    });

    it("approves exactly the offered hash once and applies only on a committed receipt", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 6, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 6, payload: { requestId: "6".repeat(32) } });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("offered");
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 6, payload: {  } });
        expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
        harness.statuses.push(200);
        harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 2, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
        await driveInferencePort(6);
        expect(harness.ports().at(-1)?.preview).toEqual(PREVIEW);
        harness.statuses.push(200);
        harness.bodies.push(JSON.stringify({
          schema: "semio.hub.inference-approval-receipt/v1",
          jobId: JOB,
          mutationId: JOB,
          commandHash: HASH,
          proposalHash: HASH,
          applied: true,
          undo: { targetId: "2".repeat(32), expectedCurrent: { documentId: DOCUMENT, headEditOrdinal: 2, headEditId: "edit-2", lastCommitSeq: 2, chainSha256: "3".repeat(64) } },
        }));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 6, payload: {  } });
        expect(harness.ports().at(-1)?.phase).toBe("approving");
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("applied");
        const approvalRequests = harness.requests.filter((entry) => entry.endsWith("/approval"));
        expect(approvalRequests).toHaveLength(1);
        expect(harness.state.outbox).toEqual([]);
      } finally {
        harness.release();
      }
    });

    it("retains an indeterminate owner on cross-job or cross-hash previews without sending approval", async () => {
      const substituted = [
        { ...PREVIEW, jobId: "8".repeat(32), regionId: `inference-${"8".repeat(32)}` },
        { ...PREVIEW, proposalHash: "9".repeat(64) },
      ];
      for (const [index, preview] of substituted.entries()) {
        const operationEpoch = 8 + index;
      const harness = await inferenceHarness({ lease: "editor" });
        try {
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
          harness.statuses.push(200);
          harness.bodies.push(receiptBody("accepted", "none", 0));
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch, payload: { requestId: `${operationEpoch}`.repeat(32) } });
          await settleInferenceTurns();
          harness.statuses.push(200);
          harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview }));
          await driveInferencePort(operationEpoch);
          expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.transport" });
          handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch, payload: {  } });
          await settleInferenceTurns();
          expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
        } finally {
          harness.release();
        }
      }
    });

    it("maps a published route rejection onto the closed failure vocabulary", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 7, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.statuses.push(503);
        harness.bodies.push(JSON.stringify({ schema: "semio.hub.inference-error/v1", code: "inference.unavailable" }));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 7, payload: { requestId: "7".repeat(32) } });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.unavailable" });
      } finally {
        harness.release();
      }
    });

    it("keeps explicit English and German text for every phase, code and control with no default language", async () => {
      const { GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, gisMapInferencePortRoleV1 } = await import("@semio-tech/gis-gismap-js/inference/schema");
      const phases: readonly GisMapInferencePortStatusV1["phase"][] = ["idle", "submitting", "running", "offered", "approving", "indeterminate", "applied", "cancelled", "stale", "failed"];
      for (const phase of phases) {
        const row = GIS_MAP_INFERENCE_PORT_TEXT_V1[phase];
        expect(Object.keys(row).sort()).toEqual(["de", "en"]);
        expect(row.en.length).toBeGreaterThan(0);
        expect(row.de.length).toBeGreaterThan(0);
        expect(row.de).not.toBe(row.en);
      }
      for (const [code, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1)) {
        expect(Object.keys(row).sort(), code).toEqual(["de", "en"]);
        expect(row.de, code).not.toBe(row.en);
      }
      for (const [control, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1)) {
        expect(Object.keys(row).sort(), control).toEqual(["de", "en"]);
        expect(row.de, control).not.toBe(row.en);
      }
      expect(gisMapInferencePortRoleV1("running")).toBe("status");
      expect(gisMapInferencePortRoleV1("failed")).toBe("alert");
    });
  });

  /** ⏳️ Settles every turn one bounded request/response leg needs, then four quiet ones. */
  async function settleInferenceTurns(): Promise<void> {
    for (let turn = 0; turn < 40; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
  }
  //#endregion 💡️InferencePortTests
}
