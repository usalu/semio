/** 🛠️ The inference port harness, lease fields, receipt and page bodies shared by the inference port parts. */
import { gisMapDocumentServiceWireDeclarationV1 } from "@semio-tech/gis-gismap-js/inference/client/io";
import { WORKER_LAW_CAPABILITY, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { BackboneWorkerResponse, DocumentExecutionTargetLeaseFieldsV1, GisInferencePortTestKit, GisMapInferencePortStatusV1, GisMapInferencePreviewV1 } from "../🧰️inference-port-test-kit/🟦️.ts";

/** 🔧 Creates the inference port fixtures shared by the inference port parts. */
export function createInferencePortHelpers(kit: GisInferencePortTestKit) {
  const { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DocumentExecutionTargetLease, artifactState, clearHubSessionCapability, closeArtifact, documentExecutionTargetLeaseMintToken, encodePackValue, hubSessionFetch, installHubSessionCapability, openArtifact, parseDocumentExecutionTargetLeaseFieldsV1, source, testSeams } = kit;
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
  return { SPACE, DOCUMENT, JOB, HASH, PREVIEW, leaseFields, inferenceHarness, receiptBody, pageBody };
}
export type InferencePortHelpers = ReturnType<typeof createInferencePortHelpers>;
