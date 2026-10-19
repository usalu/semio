/** 🧰️ Shared fakes and fixtures of the worker-law suites: the fake hub socket and stream endpoint, the verified cold pair installer, the document-open and execution-target fixtures and the common reset hook. One memoized instance per dependency bag. */
import { stubFetch } from "../🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";

export type TestSource = { readonly directory: string; readonly url: string };

/** 🔭️ Rebases the worker's own test source onto the OS root every fixture path is spelled against. */
export function workerLawSourceV1(registrar: TestSource): TestSource {
  return { directory: registrar.directory, url: new URL("../../../🟦️.ts", registrar.url).href };
}
type Vitest = NonNullable<ImportMeta["vitest"]>;
type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🧾️document-state/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type DirectoryStreamMessage = import("@semio-tech/framework/directory/schema").DirectoryStreamMessage;
type DocumentExecutionTargetLeaseFieldsV1 = import("@semio-tech/framework/directory/schema").DocumentExecutionTargetLeaseFieldsV1;
type DocumentOpenIntentV1 = import("@semio-tech/framework/directory/schema").DocumentOpenIntentV1;
type DocumentOpenPlanV1 = import("@semio-tech/framework/directory/schema").DocumentOpenPlanV1;
type MutationEnvelope = import("@semio-tech/framework-replication").MutationEnvelope;
type PersistenceBinding = import("../../🟦️.ts").PersistenceBinding;
type ServerFrame = import("@semio-tech/framework-replication").ServerFrame;
type DocumentSocketGrantReceiptV1 = import("../../🟦️.ts").DocumentSocketGrantReceiptV1;
type WireArtifactBootstrap = import("@semio-tech/framework-replication").WireArtifactBootstrap;
type WireFrontierSummary = import("@semio-tech/framework-replication").WireFrontierSummary;

export type ArtifactBootstrapFixture = Readonly<{
  artifact: Readonly<{ schema: string; packSchemaHash: string; requiredTailFrontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }> }>;
  payload: Readonly<{ packHex: string; sprHex: string }>;
  wire: Readonly<{ inlineWelcomeHex: string; chunkedWelcomeHex: string; chunkHex: readonly string[]; doneHex: string }>;
}>;
export type ExecutionTargetLeaseFixture = {
  nowMs: number;
  hubOrigin: string;
  intent: DocumentOpenIntentV1;
  plan: DocumentOpenPlanV1;
  manifest: DocumentExecutionTargetLeaseFieldsV1;
  socketGrant: DocumentSocketGrantReceiptV1;
  componentHex: string;
  descriptorHex: string;
  expected: {
    assetPaths: [string, string, string];
    openPlanPath: string;
    socketGrantPath: string;
    componentMaxBytes: number;
    descriptorMaxBytes: number;
    manifestMaxBytes: number;
    progressUnitBytes: number;
    progressStages: string[];
    rendererState: string;
    rendererClaims: Record<string, boolean>;
    viewerWriteRejectedLocally: true;
    forbiddenStatusFragments: string[];
    status: Record<string, { en: string; de: string }>;
    statusRoles: Record<string, "status" | "alert">;
    rotation: { generationA: string; generationB: string };
  };
  hostile: { name: string; stage: string; kind: string; path?: string; value?: unknown; expected: "unpublished" }[];
};
export type ExecutionTargetHarness = {
  state: ArtifactState;
  binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>;
  requests: { url: string; method: string; body: string }[];
  statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[];
  release: () => void;
};
/** 🎫️ One well-formed session capability, exactly the shape `POST /auth/sessions` mints. */
export const WORKER_LAW_CAPABILITY = `session.v1.${"c".repeat(32)}.${"4".repeat(64)}`;
export const WORKER_LAW_CAPABILITY_TWO = `session.v1.${"d".repeat(32)}.${"5".repeat(64)}`;
/** 🪪️ The retained-clone grant every test actor identity opens its document under. */
export const WORKER_LAW_ACTOR_IDENTITY_GRANT: ArtifactActorConfig["actorIdentityGrant"] = { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 };
export function sampleEnvelope(): MutationEnvelope {
  return {
    id: "edit-1",
    actor: "actor-1",
    document: "doc-1",
    schemaVersion: "demo/v1",
    deps: [],
    payloadHash: "unused-in-this-fallback",
    diff: { schemaId: "demo/v1", payload: { n: 5, sequenceNumber: 1 } },
    inverse: { targetOperation: "edit-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
  };
}
export async function flushSocketGrantTurns(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}
export function bytesFromHex(hex: string): Uint8Array {
  return Uint8Array.from(hex.match(/../g)?.map((byte) => Number.parseInt(byte, 16)) ?? []);
}
export function fixtureConfig(fixture: ArtifactBootstrapFixture): ArtifactActorConfig {
  return {
    documentId: fixture.artifact.requiredTailFrontier.documentId,
    schema: fixture.artifact.schema,
    packSchemaHash: Array.from(bytesFromHex(fixture.artifact.packSchemaHash)),
    bindings: [],
    actor: "actor-bootstrap-test",
    watchExternal: false,
    actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT,
  };
}
export function fixtureRequiredFrontier(fixture: ArtifactBootstrapFixture): WireFrontierSummary {
  const frontier = fixture.artifact.requiredTailFrontier;
  return { document_id: frontier.documentId, head_edit_ordinal: frontier.headEditOrdinal, head_edit_id: frontier.headEditId, last_commit_seq: frontier.lastCommitSeq, chain_hash: Array.from(bytesFromHex(frontier.chainHash)) };
}
export class FakeDirectoryWebSocket {
  static instances: FakeDirectoryWebSocket[] = [];
  readonly url: string;
  readonly protocol = "semio.session.v1";
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: ((event: { code: number }) => void) | null = null;
  onerror: (() => void) | null = null;
  constructor(
    url: string,
    readonly protocols?: string | string[],
  ) {
    this.url = url;
    FakeDirectoryWebSocket.instances.push(this);
  }
  send(): void {}
  close(): void {}
  triggerOpen(): void {
    this.onopen?.();
  }
  triggerClose(code: number): void {
    this.onclose?.({ code });
  }
  triggerMessage(message: DirectoryStreamMessage): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}
/** 🕸️ `implements WebSocket` on purpose: the worker types `state.socket` and every `handleHubFrame`
 * argument as the DOM `WebSocket`, so a fake that merely resembles one has to be forced in at each
 * call site. Declaring the interface makes the compiler name a member the worker could reach and
 * this fake has not implemented, which is the whole point of the double. The members below the
 * `sent` ledger are the ones the worker never calls; they exist so the type is honest. */
export class FakeHubWebSocket implements WebSocket {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;
  static instances: FakeHubWebSocket[] = [];
  readonly CONNECTING = 0 as const;
  readonly OPEN = 1 as const;
  readonly CLOSING = 2 as const;
  readonly CLOSED = 3 as const;
  readonly url: string;
  readonly protocol = "semio.session.v1";
  readonly protocols: string | string[] | undefined;
  readonly extensions = "";
  readonly bufferedAmount = 0;
  readyState = FakeHubWebSocket.CONNECTING;
  binaryType: BinaryType = "blob";
  readonly sent: Uint8Array[] = [];
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: (() => void) | null = null;
  constructor(url: string, protocols?: string | string[]) {
    this.url = url;
    this.protocols = protocols;
    FakeHubWebSocket.instances.push(this);
  }
  send(data: Uint8Array): void {
    this.sent.push(data);
  }
  addEventListener(): void {
    throw new Error("fake hub socket: the worker dials through the on* handlers, never addEventListener");
  }
  removeEventListener(): void {
    throw new Error("fake hub socket: the worker dials through the on* handlers, never removeEventListener");
  }
  dispatchEvent(): boolean {
    throw new Error("fake hub socket: the worker never dispatches synthetic events");
  }
  open(): void {
    this.readyState = FakeHubWebSocket.OPEN;
    this.onopen?.();
  }
  close(_code?: number, _reason?: string): void {
    this.readyState = FakeHubWebSocket.CLOSED;
    this.onclose?.();
  }
}
export function executionTargetBytes(hexText: string): Uint8Array {
  return Uint8Array.from({ length: hexText.length / 2 }, (_unused, index) => Number.parseInt(hexText.slice(index * 2, index * 2 + 2), 16));
}

const memo = new WeakMap<object, unknown>();

export function workerLawFakesV1(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource) {
  const cached = memo.get(dependencies);
  if (cached) return cached as ReturnType<typeof create>;
  const created = create(vitest, dependencies, source);
  memo.set(dependencies, created);
  return created;
}

function create(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource) {
  const { expect } = vitest;
  const { testSeams, decodeServerFrame, openArtifact, artifactState, handleHubFrame, encodePackValue, encodeBackboneMessage, encodeDocumentBackboneEnvelopeBatchExact, hubBinding, parseDocumentExecutionTargetLeaseFieldsV1, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, VerifiedColdDocumentPair, verifiedColdDocumentPairMintToken, ownedArrayBuffer, clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, artifacts, documentRuntimeKeyForConfig, closeArtifact, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations } = dependencies;
  async function artifactBootstrapFixture(): Promise<ArtifactBootstrapFixture> {
    const { readFile } = await import("node:fs/promises");
    return JSON.parse(await readFile(new URL("../../🔨️modules/📡️replication/🧪️tests/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json", source.url), "utf8")) as ArtifactBootstrapFixture;
  }
  function decodeFixtureFrame(hex: string): ServerFrame {
    return decodeServerFrame(bytesFromHex(hex)).frame;
  }
  async function installFixture(fixture: ArtifactBootstrapFixture, chunked: boolean): Promise<ArtifactState> {
    const config = fixtureConfig(fixture);
    openArtifact(config);
    const state = artifactState(config.documentId)!;
    await handleHubFrame(state, decodeFixtureFrame(chunked ? fixture.wire.chunkedWelcomeHex : fixture.wire.inlineWelcomeHex));
    if (chunked) {
      for (const chunk of fixture.wire.chunkHex) await handleHubFrame(state, decodeFixtureFrame(chunk));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.doneHex));
    }
    return state;
  }
    function exactDocumentBackboneMessage(
      envelope: MutationEnvelope,
      diffPayload = encodePackValue(envelope.diff.payload),
      timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }> = { actor: 1n, physical_ms: 9_007_199_254_740_992n, logical: 3n },
    ): Uint8Array {
      return encodeBackboneMessage({
        kind: "mutations",
        envelopes: encodeDocumentBackboneEnvelopeBatchExact([{
          mutation_id: envelope.id,
          document_id: envelope.document,
          actor: envelope.actor,
          dependencies: envelope.deps ?? [],
          observed: null,
          target: [],
          diff: { schema: envelope.diff.schemaId, payload: diffPayload },
          inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: encodePackValue(envelope.inverse.inverseDiff.payload) },
          timestamp,
          transaction: null,
          verb: null, line: null,
        }]),
      });
    }
    /** 🧊️ Puts one hub-bound document into the state production reaches after a verified cold
     * transfer: the execution-target lease it was admitted under, the published pack/spr pair, the
     * baseline frontier, and the real `VerifiedColdDocumentPair` minted over all three. The owner is
     * a nominal class guarded by a private mint token, so no structural stand-in can carry its
     * retention or page cursor; every law below measures `documentBackboneAdmissionReady`, which
     * calls `assertCurrent()` on exactly this object. */
    function installVerifiedDocumentBackbonePair(state: ArtifactState, baseline?: Partial<WireFrontierSummary>): WireFrontierSummary {
      const binding = hubBinding(state.config);
      if (!binding) throw new Error("a verified cold pair exists only on a hub-bound document");
      const digest = "9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130";
      const aggregate = "1".repeat(64);
      const frontier = { document_id: state.config.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0), ...baseline };
      const fields = parseDocumentExecutionTargetLeaseFieldsV1({
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: { spaceId: binding.spaceId, documentId: state.config.documentId },
        descriptorDigestV1: digest,
        catalog: { generationId: digest },
        package: { pluginId: "demo", packageId: "semio:demo", version: "0.1.0", componentSha256: digest, componentBlake3: digest, descriptorByteSha256: digest, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 }, componentReceipt: { schema: "semio.plugin.component-receipt/v1", pluginId: "demo", packageId: "semio:demo", profile: "release", target: "wasm32-wasip2", wasmSha256: digest, wasmBytes: 4096, semanticDescriptorSha256: digest, toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "a".repeat(64), provenanceSha256: "b".repeat(64) } },
        component: { sha256: digest, blake3: digest, byteLength: 4096 },
        descriptor: { sha256: digest, byteLength: 1 },
        browserActor: {
          kind: "closed-browser-actor",
          schema: "semio.os.closed-browser-actor.v1",
          codegenPolicy: "semio.os.browser-jco-1.34.0-jspi.v1",
          byteLength: 3,
          sha256: digest,
          sourceComponentSha256: digest,
          sourceDescriptorByteSha256: digest,
          policySha256: "4".repeat(64),
          importInterfaces: ["semio:framework/host-async@1.0.0", "semio:framework/pure@1.0.0"],
        },
        artifact: { kind: "s.demo.demo", schema: state.config.schema, packSchemaHash: digest },
        parentDialect: { artifactKind: "s.demo.demo", standard: "1", subset: "*" },
        surface: { surfaceId: "main", appId: "demo", windowKindId: "demo-main", role: "editor", rendererTarget: "wasm" },
        grant: { read: true, write: true, observe: true },
        checkpoint: {
          checkpointId: digest, descriptorDigestV1: digest, aggregateSha256: aggregate,
          baselineFrontier: { documentId: state.config.documentId, headEditOrdinal: 0, headEditId: "", lastCommitSeq: 0, chainHash: Array(32).fill(0) },
        },
        revalidation: { directoryRevision: 1, membershipGeneration: 1, sessionGeneration: 1 },
      });
      const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, fields, binding.baseUrl, new Uint8Array([1]), new Uint8Array([2]));
      const published = { pack: new Uint8Array([1]), spr: new Uint8Array([1]) };
      const transferred = { pack: new Uint8Array([1]), spr: new Uint8Array([1]) };
      const bootstrap: WireArtifactBootstrap = {
        format_version: 1,
        descriptor_hash: Array.from(executionTargetBytes(digest)),
        artifact_schema: fields.artifact.schema,
        artifact_kind: fields.artifact.kind,
        pack_schema_hash: Array.from(executionTargetBytes(digest)),
        baseline_frontier: frontier,
        pack_hash: Array.from(executionTargetBytes(digest)),
        spr_hash: Array.from(executionTargetBytes(digest)),
        pack_length: transferred.pack.byteLength,
        spr_length: transferred.spr.byteLength,
        chunk_count: 2,
        aggregate_hash: Array.from(executionTargetBytes(aggregate)),
        required_tail_frontier: frontier,
        inline: null,
      };
      state.executionTargetLease = lease;
      state.currentPack = published.pack;
      state.currentSpr = published.spr;
      state.frontier = frontier;
      state.verifiedColdPair = new VerifiedColdDocumentPair(verifiedColdDocumentPairMintToken, state, lease, bootstrap, transferred, published);
      return frontier;
    }
    async function executionTargetLeaseFixture(): Promise<ExecutionTargetLeaseFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8")) as ExecutionTargetLeaseFixture;
    }
    function executionTargetBodyResponse(bytes: Uint8Array, declaredLength = bytes.byteLength): Response {
      return new Response(ownedArrayBuffer(bytes), { headers: { "content-length": String(declaredLength) } });
    }
    async function acceptCurrentTestBrowserSessionAuthority(proof: string): Promise<void> {
      const authorityFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = authorityFixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const body = JSON.stringify(authority);
      const priorFetch = globalThis.fetch;
      clearHubSessionCapability();
      expect(installHubSessionCapability(`session.v1.${"1".repeat(32)}.${proof}`)).toBe(true);
      globalThis.fetch = stubFetch(async () => new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } }));
      try {
        await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000, accept: testSeams.acceptBrowserSessionAuthority });
      } finally {
        globalThis.fetch = priorFetch;
      }
    }
    async function executionTargetHarness(fixture: ExecutionTargetLeaseFixture, respond: (url: string, requests: { url: string; method: string; body: string }[]) => Response | Promise<Response>): Promise<ExecutionTargetHarness> {
      const requests: { url: string; method: string; body: string }[] = [];
      const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
      const originalFetch = globalThis.fetch;
      testSeams.documentSocketGrantTestIssue = null;
      testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
      await acceptCurrentTestBrowserSessionAuthority("a".repeat(64));
      openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
      const state = artifactState(fixture.intent.scope.documentId)!;
      state.openClientInstanceId = fixture.intent.clientInstanceId;
      const binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: fixture.intent.scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId };
      artifacts.delete(state.runtimeKey);
      state.config = { ...state.config, bindings: [binding] };
      state.runtimeKey = documentRuntimeKeyForConfig(state.config);
      artifacts.set(state.runtimeKey, state);
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: RequestInit) => {
        const url = String(input);
        requests.push({ url, method: String(init?.method ?? "GET"), body: String(init?.body ?? "") });
        return respond(url, requests);
      };
      return {
        state,
        binding,
        requests,
        statuses,
        release: () => {
          testSeams.executionTargetStatusObserver = null;
          closeArtifact(fixture.intent.scope.documentId);
          clearHubSessionCapability();
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        },
      };
    }
  function resetWorkerLaw(): void {
    for (const operation of spaceArtifactCreationCatalogOperations.values()) operation.abort.abort();
    spaceArtifactCreationCatalogOperations.clear();
    for (const operation of spaceArtifactCreationOperations.values()) operation.abort.abort();
    spaceArtifactCreationOperations.clear();
    testSeams.spaceArtifactCreationTestFetch = null;
    testSeams.workerPostTestSink = null;
    clearHubSessionCapability();
    installHubSessionCapability(WORKER_LAW_CAPABILITY);
    testSeams.socketGrantTestIssue = async () => ({
      schema: "semio.hub.socket-grant/v1",
      protocol: "semio.socket.v1",
      grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
      actorId: `hub.v1.${"3".repeat(64)}`,
      expiresAtMs: Number.MAX_SAFE_INTEGER,
    });
    testSeams.documentSocketGrantTestIssue = async () => ({
      schema: "semio.hub.document-socket-grant/v1",
      protocol: "semio.session.v1",
      actorId: `hub.v1.${"3".repeat(64)}`,
      expiresAtMs: Number.MAX_SAFE_INTEGER,
    });
  }
  return { resetWorkerLaw, artifactBootstrapFixture, decodeFixtureFrame, installFixture, exactDocumentBackboneMessage, installVerifiedDocumentBackbonePair, executionTargetLeaseFixture, executionTargetBodyResponse, acceptCurrentTestBrowserSessionAuthority, executionTargetHarness };
}
