type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES, ArtifactBootstrapAssembler, DIRECTORY_COMMAND_TRANSPORT_CAPACITY, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, DirectoryClient, DirectoryEventPageBootstrapV1, DocumentExecutionTargetLease, HUB_RECONNECT_MAX_MS, IDENTITY_CONFIG_SCHEMA, PENDING_MUTATIONS_QUEUE_LIMIT, SANITY_POLL_MIN_MS, SSE_RECONNECT_MAX_MS, SUSTAINED_HEALTHY_MS, VerifiedColdDocumentPair, abortArtifactBootstrap, acceptBrowserSessionAuthority, artifactBootstrapFailure, artifactState, artifacts, bindInferenceApprovalUndoToMountedPair, browserActorChildCapacity, browserBrokerFetch, browserBrokerProofDigest, browserDirectoryRequest, browserExecutionTargetAssetRequest, bytesHex, clearLocalBrowserBrokerProof, closeArtifact, closeArtifactRuntime, closeDirectory, connectHubOnce, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeClientFrame, decodePackPayload, decodePackValue, decodeServerFrame, directoryAdministration, directoryClient, directoryCommandOperations, directoryCommandQueue, directoryCommandSha256, directorySessionEpoch, directoryWorkerEpoch, dispatchBackboneWorkerRequest, documentExecutionOwners, documentExecutionTargetLeaseMintToken, documentExecutionTargetStatusRoleV1, documentOpenPlanAuthority, documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort, dropDocumentExecutionTargetLease, dropVerifiedColdDocumentPair, emitEvent, encodeActorUiPatchReceipt, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentBackboneEnvelopeBatchExact, encodePackValue, encodeServerFrame, executionTargetHex, executionTargetSha256Hex, executionTargetStatusObserver, extractServerCommandsDocumentBackboneBatchExact, flushDirectoryQueue, foldIdentityEvent, fromWireEnvelope, handleHubFrame, handleTsRequest, hexBytes, hubBinding, identityActorConfig, idleGisMapInferencePortStatusV1, inferenceApprovalUndoEpoch, inferenceApprovalUndoOwner, installLocalBrowserBrokerProof, localBrowserBrokerProofExpiresAtMs, localBrowserBrokerQueued, openArtifact, ownedArrayBuffer, parseDocumentBackboneMessage, parseDocumentExecutionTargetLeaseFieldsV1, parseGisMapInferenceApprovalReceiptV1, queueOutbox, readExecutionTargetBody, reissueInferenceApprovalUndoForRebootstrap, relayMutationsToHub, requestDocumentSocketAuthority, reserveDocumentBrowserActorChild, retainInferenceApprovalUndo, revokeDirectoryAdministrationForScope, rollbackEnvelope, sameLeaseFieldsV1, scopedDirectoryStreams, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1, settleDirectoryCommand, socketGrantTestIssue, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations, spaceArtifactCreationTestFetch, stampSession, toWireEnvelope, undoInferenceApproval, verifiedColdDocumentPairMintToken, verifyBrowserActorDescribeV1, workerPostTestSink } = dependencies;
  const testSeams = dependencies.testSeams as {
    directoryAdministration: typeof directoryAdministration;
    directoryClient: typeof directoryClient;
    directorySessionEpoch: typeof directorySessionEpoch;
    executionTargetStatusObserver: typeof executionTargetStatusObserver;
    inferenceApprovalUndoEpoch: typeof inferenceApprovalUndoEpoch;
    inferenceApprovalUndoOwner: typeof inferenceApprovalUndoOwner;
    inferencePort: typeof dependencies.testSeams.inferencePort;
    localBrowserBrokerProofExpiresAtMs: typeof localBrowserBrokerProofExpiresAtMs;
    localBrowserBrokerQueued: typeof localBrowserBrokerQueued;
    readonly browserSessionAuthority: unknown;
    readonly browserSessionOperationFence: object;
    acceptBrowserSessionAuthority(response: unknown, admission: object): Promise<unknown>;
    captureBrowserSessionOperationFence(): unknown;
    attachLocalBrokerPort(port: MessagePort): void;
    detachLocalBrokerPort(): void;
    socketGrantTestIssue: typeof socketGrantTestIssue;
    spaceArtifactCreationTestFetch: null | ((path: string, init: RequestInit, signal: AbortSignal) => Promise<FetchTimeoutResponse>);
    workerPostTestSink: null | ((message: BackboneWorkerResponse) => void);
  };
  const { DOCUMENT_BACKBONE_RETENTION_LIMITS, handleAck } = dependencies;
  vitest.it("retains the preceding inference job when a successor opening is refused", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧬️schema/🔣️.json", source.url), "utf8"));
    vitest.expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    for (const kind of fixture.routes) {
      const routes: string[] = [];
      const request = kind === "inference-open" ? fixture.request
        : kind === "inference-propose" ? { kind, operationEpoch: 21, requestId: "request-a" }
        : kind === "inference-history-undo" ? { kind, historyEpoch: 21, clientInstanceId: "12345678-1234-4123-8123-123456789abc", scope: fixture.request.scope }
        : { kind, operationEpoch: 21 };
      dispatchBackboneWorkerRequest(request, { handleRequestBytes: () => { routes.push("rust"); } }, () => { routes.push("typescript"); });
      vitest.expect(equal(routes, ["typescript"])).toBe(true);
    }
    const seams = dependencies.testSeams;
    const previous = seams.inferencePort;
    const sink = seams.workerPostTestSink;
    const messages: unknown[] = [];
    const retained = { operationEpoch: 20, scope: fixture.request.scope, abort: new AbortController(), status: { ...idleGisMapInferencePortStatusV1(), phase: "running", jobId: "1".repeat(32) }, turns: 0, pollTimer: null, inFlight: false, cancelSent: false, closed: false };
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
  type ActorInstanceLifetime = any;
  vitest.it("binds acknowledged Shell session reads through the actual worker broker", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const { default: equal } = await import("fast-deep-equal");
    const { BrowserBrokerPortClientV1 } = await import("../../🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/🟦️.ts");
    const directory = "./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/";
    const corpus = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL(directory + "🧬️.schema.json", source.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    const authorities = corpus.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
    const originalFetch = globalThis.fetch;
    const channel = new MessageChannel();
    const calls: Array<{ path: string; hasProof: boolean; hasSuccessor: boolean }> = [];
    clearLocalBrowserBrokerProof();
    testSeams.attachLocalBrokerPort(channel.port2);
    const client = new BrowserBrokerPortClientV1(channel.port1, "4".repeat(64));
    try {
      for (const authority of authorities) {
        vitest.expect(validate(authority)).toBe(true);
        const body = JSON.stringify(authority);
        globalThis.fetch = vitest.vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
          const headers = new Headers(init?.headers);
          calls.push({ path: String(input), hasProof: /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker") ?? ""), hasSuccessor: /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker-next") ?? "") });
          return new Response(body, { status: 200, headers: { "x-semio-browser-broker-advanced": "1", "content-length": String(new TextEncoder().encode(body).byteLength) } });
        }) as typeof fetch;
        const response = await client.me();
        vitest.expect(equal(response, { status: 200, body })).toBe(true);
        vitest.expect(equal(testSeams.browserSessionAuthority, authority)).toBe(true);
      }
      vitest.expect(equal(calls, authorities.map(() => ({ path: "/_semio/hub/auth/sessions/me", hasProof: true, hasSuccessor: true })))).toBe(true);
      console.log("[DEBUG] acknowledged Shell session reads installed only canonical worker broker authority");
    } finally {
      client.close();
      testSeams.detachLocalBrokerPort();
      channel.port1.close();
      channel.port2.close();
      clearLocalBrowserBrokerProof();
      globalThis.fetch = originalFetch;
    }
  });
  vitest.it("requires an acknowledged private broker port before Shell session requests", async () => {
    const { readFileSync } = await import("node:fs");
    const { setImmediate: immediate } = await import("node:timers/promises");
    const { default: Ajv } = await import("ajv");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL(directory + "🧬️.schema.json", source.url), "utf8"));
    vitest.expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const { BrowserBrokerPortClientV1, BROWSER_BROKER_CLIENT_TIMEOUT_MS, BROWSER_BROKER_CLIENT_INITIALIZATION_TIMEOUT_MS, BROWSER_BROKER_CLIENT_MAX_PENDING } = await import("../../🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/🟦️.ts");
    vitest.expect(equal([BROWSER_BROKER_CLIENT_TIMEOUT_MS, BROWSER_BROKER_CLIENT_MAX_PENDING], [fixture.timeoutMs, fixture.maximumPending])).toBe(true);
    vitest.expect(BROWSER_BROKER_CLIENT_INITIALIZATION_TIMEOUT_MS).toBe(fixture.initializationTimeoutMs);
    vitest.expect(fixture.initializationTimeoutMs + fixture.timeoutMs * 2).toBeLessThan(fixture.bootstrapTimeoutMs);
    vitest.vi.useFakeTimers();
    try {
      for (const row of fixture.cases) {
        const channel = new MessageChannel();
        const requests: Array<Record<string, any>> = [];
        let initialized!: () => void;
        const initialization = new Promise<void>((resolve) => { initialized = resolve; });
        let requested!: () => void;
        const request = new Promise<void>((resolve) => { requested = resolve; });
        channel.port2.onmessage = (event: MessageEvent<Record<string, any>>) => {
          requests.push(event.data);
          if (event.data.kind === "initialize") initialized();
          if (event.data.kind === "request") {
            requested();
            if (row !== "close-before-response") channel.port2.postMessage({ kind: "response", requestId: event.data.requestId, ...fixture.accepted });
          }
        };
        channel.port2.start();
        const client = new BrowserBrokerPortClientV1(channel.port1, "4".repeat(64));
        const abort = new AbortController();
        const response = client.me(abort.signal).then((value) => ({ kind: "accepted", value }), () => ({ kind: "refused" }));
        try {
          await initialization;
          vitest.expect(requests.filter((value) => value.kind === "request")).toHaveLength(0);
          if (row === "slow-initialization") await vitest.vi.advanceTimersByTimeAsync(fixture.timeoutMs + 1);
          if (row === "cancel-before-ack") abort.abort();
          if (row === "response-before-ack") channel.port2.postMessage({ kind: "response", requestId: "1".repeat(36), ...fixture.accepted });
          else if (row !== "timeout") channel.port2.postMessage(row === "malformed-ack" ? { kind: "initialized", ok: true, authority: "overpost" } : { kind: "initialized", ok: row !== "refused" });
          await immediate();
          if (row === "timeout" || row === "malformed-ack") await vitest.vi.advanceTimersByTimeAsync(fixture.initializationTimeoutMs);
          if (row === "close-before-response") {
            await request;
            client.close();
            const pending = requests.find((value) => value.kind === "request")!;
            channel.port2.postMessage({ kind: "response", requestId: pending.requestId, ...fixture.accepted });
          }
          const result = await response;
          vitest.expect(equal(result, row === "acknowledged" || row === "slow-initialization" ? { kind: "accepted", value: fixture.accepted } : { kind: "refused" })).toBe(true);
          vitest.expect(requests.filter((value) => value.kind === "request")).toHaveLength(Number(row === "acknowledged" || row === "slow-initialization" || row === "close-before-response"));
          console.log("[DEBUG] private broker acknowledgement " + row + " matched the neutral request boundary");
        } finally {
          client.close();
          channel.port1.close();
          channel.port2.close();
        }
      }
    } finally {
      vitest.vi.useRealTimers();
    }
  });
  vitest.it("closes the private broker proof before or during a session-authority read", async () => {
    const { readFileSync } = await import("node:fs");
    const { setImmediate: immediate } = await import("node:timers/promises");
    const { default: Ajv } = await import("ajv");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL(directory + "🧬️.schema.json", source.url), "utf8"));
    vitest.expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const { BrowserBrokerPortClientV1 } = await import("../../🔨️modules/📇️directory/🪪️session-refresh/🌐️broker-port/🟦️.ts");
    const originalFetch = globalThis.fetch;
    try {
      for (const row of fixture.close.cases) {
        clearLocalBrowserBrokerProof();
        const channel = new MessageChannel();
        testSeams.attachLocalBrokerPort(channel.port2);
        let readStarted = false;
        let requestSignal: AbortSignal | undefined;
        let resolveStarted!: () => void;
        const started = new Promise<void>((resolve) => { resolveStarted = resolve; });
        globalThis.fetch = vitest.vi.fn(async (_input: RequestInfo | URL, init?: RequestInit) => {
          readStarted = true;
          requestSignal = init?.signal ?? undefined;
          resolveStarted();
          return new Promise<Response>((_resolve, reject) => requestSignal?.addEventListener("abort", () => reject(new Error("cancelled")), { once: true }));
        }) as typeof fetch;
        const client = new BrowserBrokerPortClientV1(channel.port1, row.id === "before-read" ? "8".repeat(64) : "9".repeat(64));
        try {
          if (row.id === "before-read") {
            client.close();
            await vitest.expect(client.me()).rejects.toThrow("browser broker unavailable");
          } else {
            await immediate();
            const pending = client.me();
            await started;
            client.close();
            await vitest.expect(pending).rejects.toThrow("browser broker closed");
          }
          await immediate();
          await immediate();
          vitest.expect(equal({ id: row.id, readStarted }, row)).toBe(true);
          vitest.expect(testSeams.localBrowserBrokerProofExpiresAtMs).toBe(0);
          if (row.readStarted) vitest.expect(requestSignal?.aborted).toBe(true);
        } finally {
          client.close();
          testSeams.detachLocalBrokerPort();
          channel.port1.close();
          channel.port2.close();
        }
      }
      console.log("[DEBUG] session authority cancellation cleared the exact private broker proof before and during reads");
    } finally {
      clearLocalBrowserBrokerProof();
      globalThis.fetch = originalFetch;
    }
  });
  vitest.it("refreshes Shell session authority serially and cancels stale callbacks", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🪪️session-refresh/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL(directory + "🧬️.schema.json", source.url), "utf8"));
    vitest.expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const { directorySessionAuthorityIsCurrentV1, directorySessionAuthorityTextV1, startDirectorySessionRefreshV1, DIRECTORY_SESSION_REFRESH_INTERVAL_MS, DIRECTORY_SESSION_AUTHORITY_TEXT_V1 } = await import("../../🔨️modules/📇️directory/🪪️session-refresh/🟦️.ts");
    const corpus = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const first = corpus.rows[0].value;
    const second = corpus.rows[1].value;
    for (const row of fixture.actionAdmission) {
      const current = row.id === "missing" ? null : row.id === "different-binding" ? second : row.id === "different-generation" ? { ...first, authorizationGeneration: first.authorizationGeneration + 1 } : row.id === "expired" ? { ...first, expiresAt: Date.now() } : { ...first };
      vitest.expect(directorySessionAuthorityIsCurrentV1(first, current)).toBe(row.accepted);
      vitest.expect(directorySessionAuthorityIsCurrentV1(null, current)).toBe(false);
    }
    vitest.expect(DIRECTORY_SESSION_REFRESH_INTERVAL_MS).toBe(fixture.intervalMs);
    vitest.expect(equal(DIRECTORY_SESSION_AUTHORITY_TEXT_V1, fixture.presentation.text)).toBe(true);
    for (const locale of fixture.presentation.locales) vitest.expect(equal(directorySessionAuthorityTextV1(locale), fixture.presentation.text[locale])).toBe(true);
    vitest.expect(() => directorySessionAuthorityTextV1("fr")).toThrow("directory.session-authority.locale-unsupported");
    vitest.expect(fixture.intervalMs * 2).toBeLessThan(fixture.proofTtlMs);
    vitest.vi.useFakeTimers();
    const owners: Array<{ close(): void }> = [];
    try {
      const seen: unknown[] = [];
      const responses = [first, first, second];
      let calls = 0;
      let active = 0;
      let maximum = 0;
      let release: (() => void) | undefined;
      const firstGate = new Promise<void>((resolve) => { release = resolve; });
      const owner = startDirectorySessionRefreshV1({
        read: async () => {
          active += 1;
          maximum = Math.max(maximum, active);
          const value = responses[calls++];
          if (calls === 1) await firstGate;
          active -= 1;
          return { status: 200, body: JSON.stringify(value) };
        },
        onAuthority: (value) => { seen.push(value); },
        onUnavailable: () => { throw new Error("unexpected authority retirement"); },
      });
      owners.push(owner);
      const joined = owner.refresh();
      const alsoJoined = owner.refresh();
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      vitest.expect(calls).toBe(1);
      release!();
      await Promise.all([joined, alsoJoined]);
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      vitest.expect(equal(seen, responses)).toBe(true);
      vitest.expect(maximum).toBe(fixture.maximumConcurrentRequests);
      owner.close();
      await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
      vitest.expect(calls).toBe(3);
      for (const refusal of fixture.refusals) {
        const accepted: unknown[] = [];
        let refused = 0;
        let reads = 0;
        const failed = startDirectorySessionRefreshV1({
          read: async () => {
            reads += 1;
            if (refusal === "transport") throw new Error("offline");
            const status = refusal === "http-401" ? 401 : refusal === "http-428" ? 428 : refusal === "non-200" ? 201 : 200;
            const body = refusal === "invalid-body" ? "{}" : JSON.stringify(refusal === "expired" ? { ...first, expiresAt: Date.now() } : first);
            return { status, body };
          },
          onAuthority: (value) => { accepted.push(value); },
          onUnavailable: () => { refused += 1; },
        });
        owners.push(failed);
        await failed.refresh();
        await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
        await failed.refresh();
        vitest.expect(equal({ accepted, refused, reads }, { accepted: [], refused: 1, reads: 1 + fixture.unavailableRetries })).toBe(true);
      }
      for (const cancellation of fixture.cancellation) {
        const parent = new AbortController();
        let releaseRead: (() => void) | undefined;
        let readStarted!: () => void;
        const started = new Promise<void>((resolve) => { readStarted = resolve; });
        let requestSignal: AbortSignal | undefined;
        let accepted = 0;
        let refused = 0;
        let calls = 0;
        const gate = new Promise<void>((resolve) => { releaseRead = resolve; });
        const cancelled = startDirectorySessionRefreshV1({
          signal: parent.signal,
          read: async (signal) => {
            calls += 1;
            requestSignal = signal;
            readStarted();
            if (cancellation === "during-request") await gate;
            return { status: 200, body: JSON.stringify(first) };
          },
          onAuthority: () => { accepted += 1; },
          onUnavailable: () => { refused += 1; },
        });
        owners.push(cancelled);
        const pending = cancelled.refresh();
        if (cancellation === "during-request") await started;
        if (cancellation === "during-delay") await pending;
        parent.abort();
        releaseRead!();
        await pending;
        await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
        vitest.expect(requestSignal?.aborted).toBe(cancellation === "before-request" ? undefined : true);
        vitest.expect(equal({ calls, accepted, refused }, { calls: Number(cancellation !== "before-request"), accepted: Number(cancellation === "during-delay"), refused: 0 })).toBe(true);
      }
      console.log("[DEBUG] Shell session refresh matched the neutral serial, refusal and cancellation corpus");
    } finally {
      for (const owner of owners) owner.close();
      vitest.vi.useRealTimers();
    }
  });
  type ArtifactActorConfig = any;
  type ArtifactEvent = any;
  type ArtifactPresencePeer = any;
  type ArtifactState = any;
  type BackboneWorkerRequest = any;
  type BackboneWorkerResponse = any;
  type BrowserActorChildValue = any;
  type BrowserActorUiPatchOfferV1 = any;
  type CanonicalDirectoryEventPageV1 = any;
  type DirectoryCommand = any;
  type DirectoryCommandOutcomeV1 = any;
  type DirectoryCommandRequestV1 = any;
  type DirectoryCommandResultV1 = any;
  type DirectoryCommandTransportOperationV1 = any;
  type DirectoryEventPageAckV1 = any;
  type DirectoryStreamMessage = any;
  type DocumentBrowserActorChild = any;
  type DocumentExecutionTargetLeaseFieldsV1 = any;
  type DocumentOpenIntentV1 = any;
  type DocumentOpenPlanV1 = any;
  type DocumentScope = any;
  type FetchTimeoutResponse = any;
  type GisMapInferencePortStatusV1 = any;
  type GisMapInferencePreviewV1 = any;
  type HubSpaceArtifactCreationStatusV1 = any;
  type Identity = any;
  type InferenceOperationV1 = any;
  type MutationEnvelope = any;
  type PersistenceBinding = any;
  type RustWorkerHost = any;
  type ServerFrame = any;
  type SocketGrantReceiptV1 = any;
  type UiNodeRecord = any;
  type WireArtifactBootstrap = any;
  type WireFrontierSummary = any;

  const { beforeEach, describe, expect, it, vi } = vitest;

  beforeEach(() => {
    for (const operation of spaceArtifactCreationCatalogOperations.values()) operation.abort.abort();
    spaceArtifactCreationCatalogOperations.clear();
    for (const operation of spaceArtifactCreationOperations.values()) operation.abort.abort();
    spaceArtifactCreationOperations.clear();
    testSeams.spaceArtifactCreationTestFetch = null;
    testSeams.workerPostTestSink = null;
    clearLocalBrowserBrokerProof();
    installLocalBrowserBrokerProof("4".repeat(64));
    testSeams.socketGrantTestIssue = async () => ({
      schema: "semio.hub.socket-grant/v1",
      protocol: "semio.socket.v1",
      grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
      actorId: `hub.v1.${"3".repeat(64)}`,
      expiresAtMs: Number.MAX_SAFE_INTEGER,
    });
  });

  describe("space artifact creation owner", () => {
    const requestId = "1".repeat(32);
    const catalogGenerationId = "3".repeat(64);
    const response = (phase: HubSpaceArtifactCreationStatusV1["phase"], ready?: HubSpaceArtifactCreationStatusV1["ready"], generation = catalogGenerationId): FetchTimeoutResponse => {
      const body = JSON.stringify({ schema: "semio.hub.space-artifact-creation-status/v1", requestId, spaceId: "space-a", catalogGenerationId: generation, phase, ...(ready === undefined ? {} : { ready }) });
      return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
    };

    it("publishes only the canonical selected-current catalog for the exact Space", async () => {
      const body = JSON.stringify({
        schema: "semio.hub.space-artifact-creation-catalog/v1",
        spaceId: "space-a",
        catalogGenerationId,
        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
      });
      const replies: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        expect([path, init.method]).toEqual(["/spaces/space-a/artifact-creations", "GET"]);
        return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
      };
      testSeams.workerPostTestSink = (message) => replies.push(message);
      const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
      handleTsRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" });
      await vi.waitFor(() => expect(replies.at(-1)).toEqual({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "ready" }));
      expect(replies).toEqual([
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "loading" },
        { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId: "space-a", catalogGenerationId, kinds: JSON.parse(body).kinds },
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "ready" },
      ]);
      expect(spaceArtifactCreationCatalogOperations).toHaveLength(0);

      testSeams.spaceArtifactCreationTestFetch = async () => new Response(body.replace('"spaceId":"space-a"', '"spaceId":"space-b"'), { status: 200 });
      handleTsRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" });
      await vi.waitFor(() => expect(spaceArtifactCreationCatalogOperations).toHaveLength(0));
      expect(replies.slice(-2)).toEqual([
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "loading" },
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "unavailable" },
      ]);
    });

    it("retains the selected catalog generation through rotated and missing statuses until exact ready", async () => {
      const calls: Array<readonly [string, string]> = [];
      const bodies: string[] = [];
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      const missingGeneration = JSON.stringify({ schema: "semio.hub.space-artifact-creation-status/v1", requestId, spaceId: "space-a", phase: "accepted" });
      const replies: readonly FetchTimeoutResponse[] = [
        response("accepted", undefined, "4".repeat(64)),
        new Response(missingGeneration, { status: 200, headers: { "content-length": String(new TextEncoder().encode(missingGeneration).byteLength) } }),
        response("accepted"),
        response("ready", { documentId: `artifact-${"2".repeat(32)}`, kindId: "s.gis.gismap", artifactSchema: "s.gis.gismap", parentDialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" } }),
      ];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        if (typeof init.body === "string") bodies.push(init.body);
        return replies[calls.length - 1]!;
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("ready"), { timeout: 2_000 });
      expect(calls).toEqual([
        ["/spaces/space-a/artifact-creations", "POST"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
      ]);
      expect(JSON.parse(bodies[0]!)).toEqual({ schema: "semio.hub.space-artifact-create/v1", requestId, expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      expect(statuses.every((status) => status.catalogGenerationId === catalogGenerationId)).toBe(true);
      expect(statuses.filter((status) => status.ready !== undefined)).toHaveLength(1);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("routes cancellation through the exact retained owner without manufacturing a ready tuple", async () => {
      const calls: Array<readonly [string, string]> = [];
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        return response(path.endsWith("/cancel") ? "cancelled" : "preparing");
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.some((status) => status.phase === "preparing")).toBe(true));
      handleTsRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("cancelled"), { timeout: 2_000 });
      expect(calls.at(-1)).toEqual([`/spaces/space-a/artifact-creations/${requestId}/cancel`, "POST"]);
      expect(statuses.every((status) => status.ready === undefined)).toBe(true);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("never fabricates cancelled from a rejected cancel request", async () => {
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      testSeams.spaceArtifactCreationTestFetch = async (path) => {
        if (path.endsWith("/cancel")) return new Response(null, { status: 409 });
        return response("preparing");
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.some((status) => status.phase === "preparing")).toBe(true));
      handleTsRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("failed"), { timeout: 2_000 });
      expect(statuses.some((status) => status.phase === "cancelled")).toBe(false);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("requires a catalog refresh before failing one exact initial POST conflict without resubmission", async () => {
      const calls: Array<readonly [string, string]> = [];
      const statuses: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        return new Response(null, { status: 409 });
      };
      testSeams.workerPostTestSink = (message) => statuses.push(message);
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      await vi.waitFor(() => expect(spaceArtifactCreationOperations).toHaveLength(0));
      expect(calls).toEqual([["/spaces/space-a/artifact-creations", "POST"]]);
      expect(statuses).toEqual([
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "accepted" },
        { kind: "space-artifact-creation-catalog-refresh-required", requestId, spaceId: "space-a", catalogGenerationId },
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "failed" },
      ]);
    });

    it("suppresses a late initial POST conflict after the exact creation owner retires", async () => {
      let resolveResponse!: (response: FetchTimeoutResponse) => void;
      const statuses: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = () => new Promise((resolve) => {
        resolveResponse = resolve;
      });
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status" || message.kind === "space-artifact-creation-catalog-refresh-required") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.gis.gismap", name: "Shared Map" });
      await vi.waitFor(() => expect(spaceArtifactCreationOperations).toHaveLength(1));
      const operation = spaceArtifactCreationOperations.get(requestId)!;
      spaceArtifactCreationOperations.delete(requestId);
      operation.abort.abort(new Error("space artifact creation: replaced"));
      resolveResponse(new Response(null, { status: 409 }));
      await Promise.resolve();
      await Promise.resolve();
      expect(statuses).toEqual([
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "accepted" },
      ]);
    });
  });

  describe("DirectoryEventPageBootstrapV1", () => {
    const page = (afterSeqExclusive: number, throughSeqInclusive: number, hasMore: boolean, receiptSha256: string): CanonicalDirectoryEventPageV1 => ({
      canonicalJson: "{}",
      sessionBindingSha256: "a".repeat(64),
      authorizationGeneration: 9,
      afterSeqExclusive,
      throughSeqInclusive,
      hasMore,
      receiptSha256,
    });
    const ack = (value: CanonicalDirectoryEventPageV1, bootstrapEpoch = 7): DirectoryEventPageAckV1 => ({
      bootstrapEpoch,
      sessionBindingSha256: value.sessionBindingSha256,
      authorizationGeneration: value.authorizationGeneration,
      throughSeqInclusive: value.throughSeqInclusive,
      receiptSha256: value.receiptSha256,
    });

    it("serializes page delivery, exact Home acknowledgement, and live wakeup cursor ownership", () => {
      const machine = new DirectoryEventPageBootstrapV1(7, 3);
      const first = page(3, 5, true, "b".repeat(64));
      const second = page(5, 8, false, "c".repeat(64));
      machine.present(first);
      expect(() => machine.present(second)).toThrow("page ordering mismatch");
      expect(() => machine.acknowledge({ ...ack(first), receiptSha256: "d".repeat(64) })).toThrow("acknowledgement mismatch");
      expect(machine.after()).toBe(3);
      expect(machine.acknowledge(ack(first))).toEqual({ kind: "fetch", after: 5 });
      machine.present(second);
      expect(() => machine.acknowledge(ack(second, 8))).toThrow("acknowledgement mismatch");
      expect(machine.acknowledge(ack(second))).toEqual({ kind: "live", since: 8 });
      expect(machine.wake(false)).toBe(8);
      expect(machine.wake(false)).toBeNull();
    });

    it("round trips exact worker ACK and page envelopes without a raw identity secret", () => {
      const first = page(3, 5, true, "b".repeat(64));
      const request: BackboneWorkerRequest = { kind: "directory-bootstrap-ack", ...ack(first) };
      const response: BackboneWorkerResponse = { kind: "directory-event-page", ...ack(first), canonicalJson: first.canonicalJson, afterSeqExclusive: first.afterSeqExclusive, hasMore: first.hasMore };
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
      expect(JSON.stringify(response)).not.toContain("session.v1.");
    });

    it("allowlists only one canonical safe-decimal event-page route and keeps bootstrap on the TypeScript owner", async () => {
      const originalFetch = globalThis.fetch;
      const urls: string[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string) => {
        urls.push(input);
        return new Response("{}", { status: 200, headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        await browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=3", {}, { timeoutMs: 1_000 });
        await expect(browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=03", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        await expect(browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=9007199254740992", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        expect(urls).toEqual(["/_semio/hub/directory/event-page/v1?after=3"]);
        const typescriptRequests: BackboneWorkerRequest[] = [];
        const rustRequests: BackboneWorkerRequest[] = [];
        const request: BackboneWorkerRequest = { kind: "directory-bootstrap-open", baseUrl: "http://hub.test", after: 3, bootstrapEpoch: 7 };
        dispatchBackboneWorkerRequest(request, { handleRequestBytes: (wire) => rustRequests.push(decodeBackboneWorkerRequest(wire)), postReady() {} }, (value) => typescriptRequests.push(value));
        expect(typescriptRequests).toEqual([request]);
        expect(rustRequests).toEqual([]);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });
  });

  describe("browser broker proof ratchet", () => {
    it("advances only on an explicit acknowledgement and domain-binds the next proof digest", async () => {
      const originalFetch = globalThis.fetch;
      const requests: Headers[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
        requests.push(new Headers(init?.headers));
        return new Response("{}", { status: 200, headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        const firstNextDigest = requests[0]!.get("x-semio-browser-broker-next");
        const secondCurrent = hexBytes(requests[1]!.get("x-semio-browser-broker") ?? "");
        expect(secondCurrent).toBeDefined();
        expect(bytesHex(await browserBrokerProofDigest(secondCurrent!))).toBe(firstNextDigest);
        expect(requests[0]!.get("x-semio-browser-broker")).not.toBe(requests[1]!.get("x-semio-browser-broker"));
      } finally {
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("requires explicit rebootstrap after a lost acknowledgement, 401, or cancel-after-send", async () => {
      const originalFetch = globalThis.fetch;
      let calls = 0;
      try {
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          throw new Error("transport detail must be redacted");
        };
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        expect(calls).toBe(1);

        installLocalBrowserBrokerProof("6".repeat(64));
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          return new Response("unauthorized", { status: 401, headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");

        installLocalBrowserBrokerProof("7".repeat(64));
        const abort = new AbortController();
        (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
          calls += 1;
          await new Promise<void>((_resolve, reject) => init?.signal?.addEventListener("abort", () => reject(new Error("cancelled")), { once: true }));
          return new Response("{}");
        };
        const pending = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000, signal: abort.signal });
        for (let turn = 0; calls < 3 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(calls).toBe(3);
        abort.abort();
        await expect(pending).rejects.toThrow("browser broker rebootstrap required");
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
      } finally {
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("rejects expired, duplicate-initialized, and over-capacity broker work without exposing proof", async () => {
      const originalFetch = globalThis.fetch;
      const originalQueued = testSeams.localBrowserBrokerQueued;
      let observedCurrent = "";
      (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
        observedCurrent = new Headers(init?.headers).get("x-semio-browser-broker") ?? "";
        return new Response("missing acknowledgement", { status: 200 });
      };
      try {
        expect(installLocalBrowserBrokerProof("8".repeat(64))).toBe(false);
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        expect(observedCurrent).toBe("4".repeat(64));

        installLocalBrowserBrokerProof("9".repeat(64));
        testSeams.localBrowserBrokerProofExpiresAtMs = Date.now() - 1;
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");

        installLocalBrowserBrokerProof("a".repeat(64));
        testSeams.localBrowserBrokerQueued = 64;
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker capacity exceeded");
        expect(observedCurrent).not.toContain("a".repeat(64));
      } finally {
        testSeams.localBrowserBrokerQueued = originalQueued;
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("keeps the private port and proof names out of malicious plugin shard source and transfers before activation", async () => {
      const { readFile } = await import("node:fs/promises");
      const pluginShard = await readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts", source.url), "utf8");
      const shellHost = await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx", source.url), "utf8");
      expect(pluginShard).not.toContain("semio-browser-broker-port");
      expect(pluginShard).not.toContain("x-semio-browser-broker");
      expect(shellHost.indexOf("const worker = ensureBackboneWorker();")).toBeLessThan(shellHost.indexOf("void (async () => {\n      const outcome = await installPlugin"));
      expect(shellHost.indexOf("window.history.replaceState")).toBeLessThan(shellHost.indexOf("loadPluginModuleResilient"));
    });
  });

  function sampleEnvelope(): MutationEnvelope {
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

  async function flushSocketGrantTurns(): Promise<void> {
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
  }

  describe("backbone-worker wire bridge", () => {
    it("round-trips an MutationEnvelope through toWireEnvelope/fromWireEnvelope", () => {
      const envelope = sampleEnvelope();
      const wire = toWireEnvelope(envelope, { actor: 1, physical_ms: 2, logical: 3 });
      expect(wire.mutation_id).toBe(envelope.id);
      expect(wire.document_id).toBe(envelope.document);
      expect(wire.actor).toBe(envelope.actor);
      expect(decodePackPayload(wire.diff.payload)).toEqual(envelope.diff.payload);

      const recovered = fromWireEnvelope(wire);
      expect(recovered.id).toBe(envelope.id);
      expect(recovered.document).toBe(envelope.document);
      expect(recovered.diff.payload).toEqual(envelope.diff.payload);
      expect(recovered.inverse.inverseDiff.payload).toEqual(envelope.inverse.inverseDiff.payload);
    });

    it("rollbackEnvelope synthesizes an undo from the original inverse", () => {
      const envelope = sampleEnvelope();
      const rollback = rollbackEnvelope(envelope);
      expect(rollback.deps).toEqual([envelope.id]);
      expect(rollback.diff.payload).toEqual(envelope.inverse.inverseDiff.payload);
      expect(rollback.id).not.toBe(envelope.id);
    });

    it("decodeClientFrame terminally rejects the removed tag-zero Hello carrier", () => {
      expect(() => decodeClientFrame(new Uint8Array([0, 0]))).toThrow(/unknown tag 0/);
    });

    it("preserves a server Commands batch with a maximum-u64 HLC through the raw actor event", async () => {
      const batch = new Uint8Array(Buffer.from("01016d0164016100017301aa016902bbcc03ffffffffffffffffff0105", "hex"));
      const serverFrame = Uint8Array.from([0, 3, ...batch, 6, ...new TextEncoder().encode("remote"), 1, 100, 0, 1, 101, 0, ...new Array(32).fill(0)]);
      const decoded = decodeServerFrame(serverFrame).frame;
      const exactBatch = extractServerCommandsDocumentBackboneBatchExact(serverFrame);
      if (typeof decoded === "string" || !("Commands" in decoded) || exactBatch === null) throw new Error("expected exact server Commands frame");
      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local" };
      const state = { config, actor: "local", openClientInstanceId: "client-1", artifactBootstrap: null, frontier: null, requiredTailFrontier: null, browserActorReservation: null } as unknown as ArtifactState;
      const priorSink = testSeams.workerPostTestSink;
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        await handleHubFrame(state, decoded, null, null, exactBatch);
      } finally {
        testSeams.workerPostTestSink = priorSink;
      }
      expect(posted).toHaveLength(1);
      const response = posted[0]!;
      if (response.kind !== "event" || response.event.kind !== "documentBackbone") throw new Error("expected document backbone event");
      expect(response.event.message).toEqual(encodeBackboneMessage({ kind: "mutations", envelopes: batch }));
      expect(parseDocumentBackboneMessage(response.event.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);
    });

    // 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
    // `stampSession` is the ONE place `peer.color`/`peer.surface` are ever filled — shells never set
    // them themselves. Overwrites whatever the caller handed in, and derives `surface` from the
    // document's own hub binding (`null`/absent for a folder-only document).
    it("stampSession fills color/surface from actor state, overwriting whatever the caller set", () => {
      const installedTarget = parseDocumentExecutionTargetLeaseFieldsV1({
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: { spaceId: "studio-1", documentId: "doc-1" },
        descriptorDigestV1: "5".repeat(64),
        catalog: { generationId: "6".repeat(64) },
        package: { pluginId: "s.test", packageId: "s.test.codec", version: "1", componentSha256: "1".repeat(64), componentBlake3: "2".repeat(64), descriptorByteSha256: "3".repeat(64), executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } },
        component: { sha256: "1".repeat(64), blake3: "2".repeat(64), byteLength: 1 },
        descriptor: { sha256: "3".repeat(64), byteLength: 1 },
        browserActor: { kind: "none" },
        artifact: { kind: "test", schema: "demo/v1", packSchemaHash: "4".repeat(64) },
        parentDialect: { artifactKind: "test", standard: "1", subset: "*" },
        surface: { surfaceId: "s.space.home@1/*#editor", appId: "app.test", windowKindId: "window.document", role: "editor" as const, rendererTarget: "react" as const },
        grant: { read: true, write: true, observe: true },
        checkpoint: {
          checkpointId: "7".repeat(64), descriptorDigestV1: "5".repeat(64), aggregateSha256: "8".repeat(64),
          baselineFrontier: { documentId: "doc-1", headEditOrdinal: 0, headEditId: "", lastCommitSeq: 0, chainHash: Array(32).fill(0) },
        },
        revalidation: { directoryRevision: 1, membershipGeneration: 1, sessionGeneration: 1 },
      });
      const hubConfig: ArtifactActorConfig = { documentId: "doc-1", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1", installedTarget }], actor: "actor-1" };
      const hubState = { config: hubConfig, sessionColor: 7 } as unknown as ArtifactState;
      const peer: ArtifactPresencePeer = { actor: "actor-1", connectedAtMs: 1000, color: 99, surface: "shell-should-never-set-this", views: [] };
      const stamped = stampSession(peer, hubState);
      expect(stamped.color).toBe(7);
      expect(stamped.surface).toBe("s.space.home@1/*#editor");

      const folderConfig: ArtifactActorConfig = { documentId: "doc-2", schema: "demo/v1", bindings: [{ kind: "folder", path: "/tmp/doc-2" }], actor: "actor-1" };
      const folderState = { config: folderConfig, sessionColor: null } as unknown as ArtifactState;
      const stampedFolder = stampSession(peer, folderState);
      expect(stampedFolder.color).toBeUndefined();
      expect(stampedFolder.surface).toBeUndefined();
    });

    it("handleHubFrame stores the hub-assigned session color on a Session frame", () => {
      const config: ArtifactActorConfig = { documentId: "doc-3", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
      const state = { config, actor: "", hubActorReady: false, pendingSocketActorId: "actor-1", outbox: [], sessionColor: null } as unknown as ArtifactState;
      handleHubFrame(state, { Session: { actor: "actor-1", color: 3 } });
      expect(state.sessionColor).toBe(3);
    });
  });

  //#region 🧪️ArtifactBootstrapRestore
  type ArtifactBootstrapFixture = Readonly<{
    artifact: Readonly<{ schema: string; packSchemaHash: string; requiredTailFrontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }> }>;
    payload: Readonly<{ packHex: string; sprHex: string }>;
    wire: Readonly<{ inlineWelcomeHex: string; chunkedWelcomeHex: string; chunkHex: readonly string[]; doneHex: string }>;
  }>;

  function bytesFromHex(hex: string): Uint8Array {
    return Uint8Array.from(hex.match(/../g)?.map((byte) => Number.parseInt(byte, 16)) ?? []);
  }

  async function artifactBootstrapFixture(): Promise<ArtifactBootstrapFixture> {
    const { readFile } = await import("node:fs/promises");
    return JSON.parse(await readFile(new URL("../../🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json", source.url), "utf8")) as ArtifactBootstrapFixture;
  }

  function fixtureConfig(fixture: ArtifactBootstrapFixture): ArtifactActorConfig {
    return {
      documentId: fixture.artifact.requiredTailFrontier.documentId,
      schema: fixture.artifact.schema,
      packSchemaHash: Array.from(bytesFromHex(fixture.artifact.packSchemaHash)),
      bindings: [],
      actor: "actor-bootstrap-test",
      watchExternal: false,
    };
  }

  function decodeFixtureFrame(hex: string): ServerFrame {
    return decodeServerFrame(bytesFromHex(hex)).frame;
  }

  function fixtureRequiredFrontier(fixture: ArtifactBootstrapFixture): WireFrontierSummary {
    const frontier = fixture.artifact.requiredTailFrontier;
    return { document_id: frontier.documentId, head_edit_ordinal: frontier.headEditOrdinal, head_edit_id: frontier.headEditId, last_commit_seq: frontier.lastCommitSeq, chain_hash: Array.from(bytesFromHex(frontier.chainHash)) };
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

  describe("artifact bootstrap atomic restore", () => {
    it("retains the exact bootstrap owner across hash completion and progress callbacks", async () => {
      const { readFile } = await import("node:fs/promises");
      const { default: Ajv } = await import("ajv");
      const { createHash } = await import("node:crypto");
      const corpus = JSON.parse(await readFile(new URL("./🧫️fixtures/📇️directory/🧵️artifact-bootstrap-owner-v1.json", source.url), "utf8"));
      const schema = JSON.parse(await readFile(new URL("./🔨️modules/📇️directory/🧬️schema/🔣️.json", source.url), "utf8")) as { $id: string };
      const validate = new Ajv({ strict: true }).addKeyword("x-semio-note").addSchema(schema).getSchema(`${schema.$id}#/$defs/DirectoryArtifactBootstrapOwnerV1`)!;
      expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
      const fixture = await artifactBootstrapFixture(),
        frame = decodeFixtureFrame(fixture.wire.inlineWelcomeHex);
      if (!("Welcome" in frame) || typeof frame.Welcome.bootstrap !== "object" || !("ArtifactBootstrap" in frame.Welcome.bootstrap)) throw new Error("fixture bootstrap");
      const bootstrap = frame.Welcome.bootstrap.ArtifactBootstrap;
      expect(createHash("sha256").update(bytesFromHex(fixture.payload.packHex)).digest("hex")).toBe(executionTargetHex(new Uint8Array(bootstrap.pack_hash)));
      expect(createHash("sha256").update(bytesFromHex(fixture.payload.sprHex)).digest("hex")).toBe(executionTargetHex(new Uint8Array(bootstrap.spr_hash)));
      const originalFinish = ArtifactBootstrapAssembler.prototype.finish,
        originalPost = testSeams.workerPostTestSink;
      for (const row of corpus.cases) {
        const config = fixtureConfig(fixture);
        openArtifact(config);
        const state = artifactState(config.documentId)!;
        artifacts.delete(state.runtimeKey);
        const binding = { kind: "hub" as const, baseUrl: "http://hub.test", spaceId: "bootstrap-owner-space" };
        state.config = { ...state.config, bindings: [binding] };
        state.runtimeKey = documentRuntimeKeyForConfig(state.config);
        artifacts.set(state.runtimeKey, state);
        state.currentPack = new Uint8Array([9]);
        state.currentSpr = new Uint8Array([8]);
        if (row.name.startsWith("lease-")) {
          const leaseFixture = JSON.parse(await readFile(new URL("../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8")),
            fields = structuredClone(leaseFixture.manifest),
            frontier = bootstrap.baseline_frontier;
          fields.scope = { spaceId: binding.spaceId, documentId: config.documentId };
          fields.browserActor = { kind: "none" };
          fields.surface.rendererTarget = "react";
          fields.descriptorDigestV1 = executionTargetHex(new Uint8Array(bootstrap.descriptor_hash));
          fields.artifact = { kind: bootstrap.artifact_kind, schema: bootstrap.artifact_schema, packSchemaHash: executionTargetHex(new Uint8Array(bootstrap.pack_schema_hash)) };
          fields.parentDialect.artifactKind = fields.artifact.kind;
          fields.checkpoint = {
            checkpointId: "a".repeat(64),
            descriptorDigestV1: fields.descriptorDigestV1,
            aggregateSha256: executionTargetHex(new Uint8Array(bootstrap.aggregate_hash)),
            baselineFrontier: { documentId: frontier.document_id, headEditOrdinal: frontier.head_edit_ordinal, headEditId: frontier.head_edit_id, lastCommitSeq: frontier.last_commit_seq, chainHash: [...frontier.chain_hash] },
          };
          if (row.name === "lease-descriptor") {
            fields.descriptorDigestV1 = "f".repeat(64);
            fields.checkpoint.descriptorDigestV1 = fields.descriptorDigestV1;
          }
          if (row.name === "lease-aggregate") fields.checkpoint.aggregateSha256 = "f".repeat(64);
          if (row.name === "lease-baseline") fields.checkpoint.baselineFrontier.headEditId = "foreign-head";
          if (row.name === "lease-missing-checkpoint") {
            delete fields.checkpoint;
            try {
              expect(() => parseDocumentExecutionTargetLeaseFieldsV1(fields)).toThrow("document-open.invalid-fields");
              expect({ name: row.name, installed: state.currentPack?.length === bytesFromHex(fixture.payload.packHex).length && state.currentPack[0] !== 9 }).toEqual(row);
              expect(state.executionTargetLease).toBeNull();
              expect(state.currentPack).toEqual(new Uint8Array([9]));
              expect(state.currentSpr).toEqual(new Uint8Array([8]));
            } finally {
              closeArtifactRuntime(state.runtimeKey);
            }
            continue;
          }
          if (row.name === "lease-scope") fields.scope.spaceId = "foreign-space";
          if (row.name === "lease-kind") {
            fields.artifact.kind = "foreign-kind";
            fields.parentDialect.artifactKind = fields.artifact.kind;
          }
          state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), binding.baseUrl, new Uint8Array([1]), new Uint8Array([2]));
        }
        let originalCloses = 0,
          replacementCloses = 0;
        const socket = {
          close() {
            originalCloses++;
          },
        } as unknown as WebSocket;
        const replacementSocket = {
          close() {
            replacementCloses++;
          },
        } as unknown as WebSocket;
        state.socket = socket;
        let successor: ArtifactBootstrapAssembler | undefined;
        const returned: { pack: Uint8Array; spr: Uint8Array }[] = [];
        const finish = vi.spyOn(ArtifactBootstrapAssembler.prototype, "finish").mockImplementation(async function (this: ArtifactBootstrapAssembler, done, control) {
          const pair = await originalFinish.call(this, done, control);
          returned.push(pair);
          queueMicrotask(() => {
            if (row.name === "client-after-hash") state.openClientInstanceId = "foreign-client";
            if (row.name === "schema-after-hash") state.config = { ...state.config, schema: "foreign-schema" };
            if (row.name === "space-after-hash") binding.spaceId = "foreign-space";
            if (row.name === "attempt-after-hash") state.executionTargetOpen = Symbol("foreign-open");
            if (row.name === "lease-drop-after-hash") state.executionTargetLease!.drop();
            if (row.name.includes("socket-after-hash")) state.socket = replacementSocket;
            if (row.name.includes("assembler-after-hash")) {
              void handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex), null, socket);
              successor = state.artifactBootstrap ?? undefined;
            }
          });
          return pair;
        });
        testSeams.workerPostTestSink = (message) => {
          if (row.name === "client-during-progress" && message.kind === "artifact-bootstrap-progress") state.openClientInstanceId = "foreign-progress-client";
        };
        try {
          if (row.name.startsWith("chunked-")) {
            await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex), null, socket);
            for (const chunk of fixture.wire.chunkHex) await handleHubFrame(state, decodeFixtureFrame(chunk), null, socket);
            await handleHubFrame(state, decodeFixtureFrame(fixture.wire.doneHex), null, socket);
          } else await handleHubFrame(state, structuredClone(frame), null, socket);
          expect({ name: row.name, installed: state.currentPack?.length === bytesFromHex(fixture.payload.packHex).length && state.currentPack[0] !== 9 }).toEqual(row);
          if (!row.installed) {
            expect(state.currentPack).toEqual(new Uint8Array([9]));
            expect(state.currentSpr).toEqual(new Uint8Array([8]));
          }
          expect(replacementCloses).toBe(0);
          if (row.name.includes("socket-after-hash")) {
            expect(state.artifactBootstrap).toBeNull();
            expect(state.artifactBootstrapOwner).toBeNull();
          }
          if (successor) {
            expect(state.artifactBootstrap, row.name).toBe(successor);
            expect(successor.retainedBytes).toBeGreaterThan(0);
          }
          if (row.installed) expect(originalCloses).toBe(0);
          expect(returned.every((pair) => pair.pack.every((byte) => byte === 0) && pair.spr.every((byte) => byte === 0))).toBe(true);
        } finally {
          finish.mockRestore();
          testSeams.workerPostTestSink = originalPost;
          successor?.abort();
          closeArtifactRuntime(state.runtimeKey);
        }
      }
      console.log("[DEBUG] artifact-bootstrap-owner: AJV=1 node-sha256=2 early-lease-rejection=1 neutral=" + corpus.cases.length + " passed");
    });

    it("installs the exact neutral inline and chunked pair and reaches Live only at the authenticated tail", async () => {
      const fixture = await artifactBootstrapFixture();
      const pack = bytesFromHex(fixture.payload.packHex);
      const spr = bytesFromHex(fixture.payload.sprHex);
      const inline = await installFixture(fixture, false);
      expect(inline.currentPack).toEqual(pack);
      expect(inline.currentSpr).toEqual(spr);
      expect(inline.status.remote.kind).not.toBe("live");
      const inlinePack = inline.currentPack;
      const inlineSpr = inline.currentSpr;
      const inlineProgress = [...inline.artifactBootstrapProgress];
      await handleHubFrame(inline, { Commands: { envelopes: [], origin: inline.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(inline.status.remote.kind).toBe("live");
      expect(inline.resumeToken).toBe("resume-bootstrap-1");
      closeArtifact(inline.config.documentId);

      const chunked = await installFixture(fixture, true);
      expect(chunked.currentPack).toEqual(pack);
      expect(chunked.currentSpr).toEqual(spr);
      expect(chunked.currentPack).toEqual(inlinePack);
      expect(chunked.currentSpr).toEqual(inlineSpr);
      expect(chunked.artifactBootstrapProgress.every((progress, index, all) => index === 0 || (progress.receivedBytes >= all[index - 1]!.receivedBytes && progress.receivedChunks >= all[index - 1]!.receivedChunks))).toBe(true);
      expect(chunked.artifactBootstrapProgress.at(-1)).toMatchObject({ receivedBytes: pack.length + spr.length, receivedChunks: fixture.wire.chunkHex.length });
      expect(inlineProgress.at(-1)).toMatchObject({ receivedBytes: pack.length + spr.length });
      closeArtifact(chunked.config.documentId);
    });

    it("fails a stalled non-inline bootstrap at its exact owner deadline and fences a replaced owner's timer", async () => {
      vi.useFakeTimers();
      vi.setSystemTime(1_000_000);
      const fixture = await artifactBootstrapFixture();
      const config = fixtureConfig(fixture);
      openArtifact(config);
      const state = artifactState(config.documentId)!;
      let firstCloses = 0,
        successorCloses = 0;
      const firstSocket = { close: () => firstCloses++ } as unknown as WebSocket;
      const successorSocket = { close: () => successorCloses++ } as unknown as WebSocket;
      const priorPost = testSeams.workerPostTestSink;
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        state.socket = firstSocket;
        await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex), null, firstSocket);
        const firstAssembler = state.artifactBootstrap;
        const firstOwner = state.artifactBootstrapOwner;
        const firstDeadline = state.artifactBootstrapDeadlineMs;
        const firstTimer = state.artifactBootstrapDeadlineTimer;
        if (!firstAssembler || !firstOwner || firstDeadline === null || firstTimer === null) throw new Error("first bootstrap deadline owner missing");
        const firstHalf = Math.floor((firstDeadline - Date.now()) / 2);
        await vi.advanceTimersByTimeAsync(firstHalf);

        state.socket = successorSocket;
        await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex), null, successorSocket);
        const successorAssembler = state.artifactBootstrap;
        const successorOwner = state.artifactBootstrapOwner;
        const successorDeadline = state.artifactBootstrapDeadlineMs;
        const successorTimer = state.artifactBootstrapDeadlineTimer;
        if (!successorAssembler || !successorOwner || successorDeadline === null || successorTimer === null) throw new Error("successor bootstrap deadline owner missing");
        expect(successorOwner).not.toBe(firstOwner);
        expect(successorAssembler).not.toBe(firstAssembler);
        expect(successorTimer).not.toBe(firstTimer);
        expect(firstAssembler.retainedBytes).toBe(0);

        await vi.advanceTimersByTimeAsync(firstDeadline - Date.now() + 1);
        expect(state.artifactBootstrapOwner).toBe(successorOwner);
        expect(state.artifactBootstrap).toBe(successorAssembler);
        expect(state.artifactBootstrapDeadlineMs).toBe(successorDeadline);
        expect(posted.filter((message) => message.kind === "artifact-bootstrap-failed")).toHaveLength(0);
        expect(firstCloses).toBe(0);
        expect(successorCloses).toBe(0);

        await vi.advanceTimersByTimeAsync(successorDeadline - Date.now());
        expect(posted.filter((message) => message.kind === "artifact-bootstrap-failed")).toEqual([
          expect.objectContaining({ documentId: config.documentId, clientInstanceId: state.openClientInstanceId, code: "deadline-exceeded", retryable: true }),
        ]);
        expect(state.artifactBootstrapOwner).toBeNull();
        expect(state.artifactBootstrap).toBeNull();
        expect(state.artifactBootstrapDeadlineMs).toBeNull();
        expect(state.artifactBootstrapDeadlineTimer).toBeNull();
        expect(successorAssembler.retainedBytes).toBe(0);
        expect(firstCloses).toBe(0);
        expect(successorCloses).toBe(1);
      } finally {
        testSeams.workerPostTestSink = priorPost;
        closeArtifactRuntime(state.runtimeKey);
        vi.useRealTimers();
      }
    });

    it("bounds a required rebootstrap across reconnect and refuses pairless welcome shortcuts", async () => {
      vi.useFakeTimers();
      vi.setSystemTime(2_000_000);
      const fixture = await artifactBootstrapFixture();
      const priorPost = testSeams.workerPostTestSink;
      const rebootstrapControl = (documentId: string): ServerFrame => ({
        RebootstrapRequired: {
          control: {
            space_id: "bootstrap-watchdog-space",
            document_id: documentId,
            checkpoint_id: Array(32).fill(1),
            descriptor_hash: Array(32).fill(2),
            baseline_frontier: { ...fixtureRequiredFrontier(fixture), document_id: documentId },
          },
        },
      });
      const prepare = async (): Promise<ArtifactState> => {
        const state = await installFixture(fixture, false);
        artifacts.delete(state.runtimeKey);
        state.config = { ...state.config, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "bootstrap-watchdog-space" }] };
        state.runtimeKey = documentRuntimeKeyForConfig(state.config);
        artifacts.set(state.runtimeKey, state);
        return state;
      };
      try {
        const stalled = await prepare();
        const posted: BackboneWorkerResponse[] = [];
        testSeams.workerPostTestSink = (message) => posted.push(message);
        let firstCloses = 0,
          replacementCloses = 0,
          currentCloses = 0;
        const firstSocket = { close: () => firstCloses++ } as unknown as WebSocket;
        stalled.socket = firstSocket;
        await handleHubFrame(stalled, rebootstrapControl(stalled.config.documentId), null, firstSocket);
        const firstOwner = stalled.artifactRebootstrapOwner;
        const firstDeadline = stalled.artifactRebootstrapDeadlineMs;
        if (!firstOwner || firstDeadline === null) throw new Error("first rebootstrap watchdog missing");
        await vi.advanceTimersByTimeAsync(Math.floor((firstDeadline - Date.now()) / 2));
        const replacementSocket = { close: () => replacementCloses++ } as unknown as WebSocket;
        stalled.socket = replacementSocket;
        await handleHubFrame(stalled, rebootstrapControl(stalled.config.documentId), null, replacementSocket);
        const replacementOwner = stalled.artifactRebootstrapOwner;
        const replacementDeadline = stalled.artifactRebootstrapDeadlineMs;
        if (!replacementOwner || replacementDeadline === null) throw new Error("replacement rebootstrap watchdog missing");
        expect(replacementOwner).not.toBe(firstOwner);
        const currentSocket = { close: () => currentCloses++ } as unknown as WebSocket;
        stalled.socket = currentSocket;
        await vi.advanceTimersByTimeAsync(firstDeadline - Date.now() + 1);
        expect(stalled.artifactRebootstrapOwner).toBe(replacementOwner);
        expect(stalled.artifactRebootstrapRequired).toBe(true);
        expect(currentCloses).toBe(0);
        await vi.advanceTimersByTimeAsync(replacementDeadline - Date.now());
        expect(posted.filter((message) => message.kind === "artifact-bootstrap-failed")).toEqual([
          expect.objectContaining({ documentId: stalled.config.documentId, code: "deadline-exceeded", retryable: true }),
        ]);
        expect(stalled.artifactRebootstrapOwner).toBeNull();
        expect(stalled.artifactRebootstrapDeadlineMs).toBeNull();
        expect(stalled.artifactRebootstrapDeadlineTimer).toBeNull();
        expect(stalled.artifactRebootstrapRequired).toBe(true);
        expect(stalled.artifactBootstrap).toBeNull();
        expect(firstCloses).toBe(1);
        expect(replacementCloses).toBe(1);
        expect(currentCloses).toBe(1);
        closeArtifactRuntime(stalled.runtimeKey);

        for (const outcome of ["none", "tail", "success"] as const) {
          const state = await prepare();
          const messages: BackboneWorkerResponse[] = [];
          testSeams.workerPostTestSink = (message) => messages.push(message);
          let oldCloses = 0,
            activeCloses = 0;
          const oldSocket = { close: () => oldCloses++ } as unknown as WebSocket;
          state.socket = oldSocket;
          await handleHubFrame(state, rebootstrapControl(state.config.documentId), null, oldSocket);
          const deadline = state.artifactRebootstrapDeadlineMs;
          if (deadline === null) throw new Error("rebootstrap watchdog missing");
          const activeSocket = { close: () => activeCloses++ } as unknown as WebSocket;
          state.socket = activeSocket;
          const welcome = structuredClone(decodeFixtureFrame(fixture.wire.inlineWelcomeHex));
          if (!("Welcome" in welcome)) throw new Error("welcome fixture missing");
          if (outcome === "none") welcome.Welcome.bootstrap = "None";
          if (outcome === "tail") welcome.Welcome.bootstrap = "Tail";
          await handleHubFrame(state, welcome, null, activeSocket);
          if (outcome === "success") {
            expect(state.artifactRebootstrapRequired).toBe(false);
            expect(state.artifactRebootstrapOwner).toBeNull();
            expect(state.artifactRebootstrapDeadlineMs).toBeNull();
            expect(state.artifactRebootstrapDeadlineTimer).toBeNull();
            expect(messages.filter((message) => message.kind === "artifact-bootstrap-failed")).toHaveLength(0);
            await vi.advanceTimersByTimeAsync(Math.max(0, deadline - Date.now() + 1));
            expect(activeCloses).toBe(0);
          } else {
            expect(messages.filter((message) => message.kind === "artifact-bootstrap-failed")).toEqual([
              expect.objectContaining({ documentId: state.config.documentId, code: "invalid-bootstrap", retryable: false }),
            ]);
            expect(state.artifactRebootstrapRequired).toBe(true);
            expect(state.artifactRebootstrapOwner).toBeNull();
            expect(state.artifactRebootstrapDeadlineMs).toBeNull();
            expect(state.artifactRebootstrapDeadlineTimer).toBeNull();
            expect(activeCloses).toBe(1);
          }
          expect(oldCloses).toBe(1);
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = priorPost;
        for (const state of [...artifacts.values()]) {
          if (state.config.documentId === fixture.artifact.requiredTailFrontier.documentId) closeArtifactRuntime(state.runtimeKey);
        }
        vi.useRealTimers();
      }
    });

    it("does not reach Live for a same-ordinal frontier with a different authenticated head or chain", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      const wrong = { ...fixtureRequiredFrontier(fixture), head_edit_id: "edit-wrong", chain_hash: Array(32).fill(0x55) };
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: wrong } });
      expect(state.status.remote.kind).not.toBe("live");
      expect(state.requiredTailFrontier).not.toBeNull();
      expect(state.resumeToken).toBeNull();
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(state.status.remote.kind).toBe("live");
      closeArtifact(state.config.documentId);
    });

    it("invalidates the committed session before rebootstrap and bounds typed failure diagnostics", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      state.config = { ...state.config, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-a" }] };
      state.resumeToken = "stale-resume";
      await handleHubFrame(state, {
        RebootstrapRequired: {
          control: {
            space_id: "space-a",
            document_id: state.config.documentId,
            checkpoint_id: Array(32).fill(1),
            descriptor_hash: Array(32).fill(2),
            baseline_frontier: fixtureRequiredFrontier(fixture),
          },
        },
      });
      expect(state.currentPack).toBeNull();
      expect(state.currentSpr).toBeNull();
      expect(state.frontier).toBeNull();
      expect(state.resumeToken).toBeNull();
      expect(state.status.remote.kind).toBe("connecting");
      const diagnostic = artifactBootstrapFailure(state, new Error("€".repeat(4_096)));
      expect(new TextEncoder().encode(diagnostic.message).byteLength).toBeLessThanOrEqual(ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES);
      closeArtifact(state.config.documentId);
    });

    it("browser document peers refetch the same exact pair after scoped rebootstrap", async () => {
      const { readFile } = await import("node:fs/promises");
      type GisMapPeerRebootstrapFixtureV1 = Readonly<{
        schema: "semio.os.gis-map-peer-rebootstrap/v1";
        scope: DocumentScope;
        clients: readonly Readonly<{ clientInstanceId: string }>[];
        published: Readonly<{
          checkpointId: string;
          descriptorDigestV1: string;
          aggregateSha256: string;
          frontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: readonly number[] }>;
          scene: Readonly<{ revision: number; nodeKind: "tiled-map"; region: Readonly<{ id: string; kind: "inference-bounds" }> }>;
        }>;
        order: readonly string[];
        sourceHostiles: readonly string[];
        nonclaims: readonly string[];
      }>;
      const parsed: unknown = JSON.parse(await readFile(new URL("./🧫️fixtures/🗺️gis-map-peer-rebootstrap-v1/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(await readFile(new URL("./🧬️schema/🔣️.json", source.url), "utf8")) as { $id: string };
      const Ajv = (await import("ajv")).default;
      const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/GisMapPeerRebootstrapV1`)! as unknown as (value: unknown) => value is GisMapPeerRebootstrapFixtureV1;
      expect(validate(parsed)).toBe(true);
      if (!validate(parsed)) throw new Error("GIS Map peer rebootstrap fixture is invalid");
      const corpus = parsed;
      const fixture = await artifactBootstrapFixture();
      const welcome = decodeFixtureFrame(fixture.wire.inlineWelcomeHex);
      if (!("Welcome" in welcome) || typeof welcome.Welcome.bootstrap !== "object" || !("ArtifactBootstrap" in welcome.Welcome.bootstrap)) throw new Error("peer bootstrap fixture");
      const bootstrap = welcome.Welcome.bootstrap.ArtifactBootstrap;
      const { UiDocumentStore } = await import("../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx");
      const priorPost = testSeams.workerPostTestSink;
      const results: { clientInstanceId: string; pack: number[]; spr: number[]; frontier: WireFrontierSummary; node: unknown; closeCount: number }[] = [];
      try {
        for (const client of corpus.clients) {
          const state = await installFixture(fixture, false);
          artifacts.delete(state.runtimeKey);
          const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: "http://hub.test", spaceId: corpus.scope.spaceId, requestedSurfaceId: "s.gis.gismap@1/*/viewer" };
          state.config = { ...state.config, documentId: corpus.scope.documentId, bindings: [binding] };
          state.runtimeKey = documentRuntimeKeyForConfig(state.config);
          state.openClientInstanceId = client.clientInstanceId;
          artifacts.set(state.runtimeKey, state);
          let closeCount = 0;
          const staleSocket = { close: () => closeCount++ } as unknown as WebSocket;
          state.socket = staleSocket;
          const posted: BackboneWorkerResponse[] = [];
          testSeams.workerPostTestSink = (message) => posted.push(message);
          await handleHubFrame(
            state,
            {
              RebootstrapRequired: {
                control: {
                  space_id: corpus.scope.spaceId,
                  document_id: corpus.scope.documentId,
                  checkpoint_id: [...bytesFromHex(corpus.published.checkpointId)],
                  descriptor_hash: [...bootstrap.descriptor_hash],
                  baseline_frontier: { ...bootstrap.baseline_frontier, document_id: corpus.scope.documentId },
                },
              },
            },
            null,
            staleSocket,
          );
          expect(state.currentPack).toBeNull();
          expect(state.currentSpr).toBeNull();
          expect(state.frontier).toBeNull();
          expect(state.resumeToken).toBeNull();
          expect(closeCount).toBe(1);
          expect(posted.filter((message) => message.kind === "artifact-rebootstrap-required")).toEqual([
            { kind: "artifact-rebootstrap-required", documentId: corpus.scope.documentId, clientInstanceId: client.clientInstanceId, scope: corpus.scope, message: "rebootstrap-required", retryable: true },
          ]);
          const freshSocket = { close: () => closeCount++ } as unknown as WebSocket;
          state.socket = freshSocket;
          await handleHubFrame(state, welcome, null, freshSocket);
          const frontier = { ...fixtureRequiredFrontier(fixture), document_id: corpus.scope.documentId };
          await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier } }, null, freshSocket);
          expect(state.status.remote.kind).toBe("live");
          const store = new UiDocumentStore("gis-map-window");
          const node: UiNodeRecord = {
            id: 0,
            key: "map-root",
            component: { type: "surface", kind: corpus.published.scene.nodeKind, docSchema: "tiled-map@1", doc: { bytes: Array.from(encodePackValue({ regions: [corpus.published.scene.region] })) }, bindings: [] },
            layout: { kind: "leaf", width: "fill", height: "fill" },
            style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
            activity: "idle",
            disabled: false,
            transition: null,
            accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
            bindings: [],
            menu: null,
            children: [],
          };
          expect(store.applyPatch({ surface: "gis-map-window", baseRevision: 0, revision: corpus.published.scene.revision, ops: [{ type: "upsert", ...node }, { type: "setRoot", id: 0 }]})).toEqual({ ok: true });
          results.push({ clientInstanceId: client.clientInstanceId, pack: [...state.currentPack!], spr: [...state.currentSpr!], frontier: state.frontier!, node: store.getNodeSnapshot(0), closeCount });
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = priorPost;
      }
      expect(results).toHaveLength(2);
      expect(results[0]!.pack).toEqual(results[1]!.pack);
      expect(results[0]!.spr).toEqual(results[1]!.spr);
      expect(results[0]!.frontier).toEqual(results[1]!.frontier);
      expect(results[0]!.node).toEqual(results[1]!.node);
      expect(results.map(({ clientInstanceId, closeCount }) => ({ clientInstanceId, closeCount }))).toEqual(corpus.clients.map((client) => ({ clientInstanceId: client.clientInstanceId, closeCount: 1 })));
      console.log("gis-map-peer-rebootstrap: clients=2 production-bootstrap=1 scene=controlled simultaneous=0");
    });

    it("discards malformed and disconnected staging, preserves the prior commit, and restarts fresh", async () => {
      const fixture = await artifactBootstrapFixture();
      const config = fixtureConfig(fixture);
      openArtifact(config);
      const state = artifactState(config.documentId)!;
      state.currentPack = Uint8Array.of(9);
      state.currentSpr = Uint8Array.of(8);
      const priorFrontier: WireFrontierSummary = { document_id: state.config.documentId, head_edit_ordinal: 1, head_edit_id: "old", last_commit_seq: 1, chain_hash: Array(32).fill(7) };
      state.frontier = priorFrontier;
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      const malformed = structuredClone(decodeFixtureFrame(fixture.wire.chunkHex[0]!));
      if (!("ArtifactBootstrapChunk" in malformed)) throw new Error("fixture chunk expected");
      const malformedFrame: ServerFrame = { ArtifactBootstrapChunk: { ...malformed.ArtifactBootstrapChunk, descriptor_hash: malformed.ArtifactBootstrapChunk.descriptor_hash.map((byte, index) => (index === 0 ? byte ^ 0xff : byte)) } };
      await handleHubFrame(state, malformedFrame);
      expect(state.currentPack).toEqual(Uint8Array.of(9));
      expect(state.currentSpr).toEqual(Uint8Array.of(8));
      expect(state.frontier).toEqual(priorFrontier);
      expect(state.artifactBootstrap).toBeNull();

      await handleHubFrame(state, { SnapshotDone: { seq_count: 1 } });
      expect(state.currentPack).toEqual(Uint8Array.of(9));
      expect(state.currentSpr).toEqual(Uint8Array.of(8));
      expect(state.frontier).toEqual(priorFrontier);
      expect(state.artifactBootstrap).toBeNull();

      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkHex[0]!));
      abortArtifactBootstrap(state);
      expect(state.frontier).toEqual(priorFrontier);
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      for (const chunk of fixture.wire.chunkHex) await handleHubFrame(state, decodeFixtureFrame(chunk));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.doneHex));
      expect(state.currentPack).toEqual(bytesFromHex(fixture.payload.packHex));
      expect(state.currentSpr).toEqual(bytesFromHex(fixture.payload.sprHex));
      closeArtifact(state.config.documentId);
    });

    it("preserves one pending local edit across replacement and catch-up without duplicate replay", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      const local = { ...sampleEnvelope(), id: "pending-local", document: state.config.documentId, schemaVersion: state.config.schema };
      queueOutbox(state, [local, local]);
      expect(state.outbox.map((envelope) => envelope.id)).toEqual(["pending-local"]);
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(state.outbox.map((envelope) => envelope.id)).toEqual(["pending-local"]);
      expect(state.pendingBatches.size).toBe(0);
      closeArtifact(state.config.documentId);
    });

    it("commits neither pair nor frontier when the atomic folder envelope PUT fails", async () => {
      const fixture = await artifactBootstrapFixture();
      const config = fixtureConfig(fixture);
      openArtifact(config);
      const state = artifactState(config.documentId)!;
      state.config = { ...state.config, bindings: [{ kind: "folder", path: "/tmp/bootstrap-put-failure" }] };
      state.currentPack = Uint8Array.of(1);
      state.currentSpr = Uint8Array.of(2);
      const priorFrontier: WireFrontierSummary = { document_id: state.config.documentId, head_edit_ordinal: 1, head_edit_id: "old", last_commit_seq: 1, chain_hash: Array(32).fill(6) };
      state.frontier = priorFrontier;
      const originalFetch = globalThis.fetch;
      let puts = 0;
      globalThis.fetch = (async (input: string | URL | Request, init?: RequestInit) => {
        if (String(input).includes("/reserve")) {
          return {
            ok: true,
            status: 201,
            headers: { get: () => "application/json" },
            json: async () => ({ schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: 1, capability: "a".repeat(64) }),
            text: async () => "",
          } as unknown as Response;
        }
        if (init?.method === "PUT") puts += 1;
        return { ok: false, status: 500, statusText: "fixture failure", headers: { get: () => null }, json: async () => ({}), text: async () => "" } as unknown as Response;
      }) as typeof fetch;
      try {
        await handleHubFrame(state, decodeFixtureFrame(fixture.wire.inlineWelcomeHex));
        expect(puts).toBe(1);
        expect(state.currentPack).toEqual(Uint8Array.of(1));
        expect(state.currentSpr).toEqual(Uint8Array.of(2));
        expect(state.frontier).toEqual(priorFrontier);
        expect(state.artifactBootstrap).toBeNull();
      } finally {
        globalThis.fetch = originalFetch;
        closeArtifact(state.config.documentId);
      }
    });

    it("retires the exact durable folder epoch when the client owner becomes stale during stage or after publish", async () => {
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/🧑‍💻dev/🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1/🔣️.json", source.url), "utf8"));
      expect(corpus.hostile.filter((row: { name: string }) => row.name.startsWith("client-stale-")).map((row: { name: string }) => row.name)).toEqual(["client-stale-stage", "client-stale-publish"]);
      const fixture = await artifactBootstrapFixture();
      const frame = decodeFixtureFrame(fixture.wire.inlineWelcomeHex);
      const originalFetch = globalThis.fetch;
      try {
        for (const phase of ["stage", "publish"] as const) {
          const config = fixtureConfig(fixture);
          openArtifact(config);
          const state = artifactState(config.documentId)!;
          state.config = { ...state.config, bindings: [{ kind: "folder", path: `/tmp/bootstrap-stale-${phase}` }] };
          state.currentPack = Uint8Array.of(1);
          state.currentSpr = Uint8Array.of(2);
          let publishes = 0,
            retires = 0;
          globalThis.fetch = (async (input: string | URL | Request) => {
            const url = String(input);
            if (url.includes("/reserve")) return { ok: true, status: 201, json: async () => ({ schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: 7, capability: "b".repeat(64) }) } as unknown as Response;
            if (url.includes("/stage")) {
              if (phase === "stage") state.openClientInstanceId = "stale-after-stage";
              return { ok: true, status: 204 } as unknown as Response;
            }
            if (url.includes("/publish")) {
              publishes += 1;
              if (phase === "publish") state.openClientInstanceId = "stale-after-publish";
              return { ok: true, status: 204 } as unknown as Response;
            }
            if (url.includes("/retire")) {
              retires += 1;
              return { ok: true, status: 204 } as unknown as Response;
            }
            throw new Error(`unexpected folder mirror request ${url}`);
          }) as typeof fetch;
          try {
            await handleHubFrame(state, structuredClone(frame));
            expect(retires, phase).toBe(1);
            expect(publishes, phase).toBe(phase === "publish" ? 1 : 0);
            expect(state.currentPack).toEqual(Uint8Array.of(1));
            expect(state.currentSpr).toEqual(Uint8Array.of(2));
            expect(state.canonicalFolderMirror).toBeNull();
          } finally {
            closeArtifactRuntime(state.runtimeKey);
          }
        }
      } finally {
        globalThis.fetch = originalFetch;
      }
    });
  });
  //#endregion 🧪️ArtifactBootstrapRestore

  //#region 🔖️IdentityTests
  describe("identity config facet", () => {
    function sampleIdentity(overrides: Partial<Identity> = {}): Identity {
      return { userId: "u-1", email: "ada@semio.dev", displayName: "Ada", hubBaseUrl: "http://hub.test", issuedAtMs: 1_000, ...overrides };
    }

    it("identityActorConfig binds the folder lane under `${dataDir}/os` when given a dataDir, else local-only", () => {
      expect(identityActorConfig("actor-1", "/tmp/s-user1")).toEqual({
        documentId: IDENTITY_CONFIG_SCHEMA,
        schema: IDENTITY_CONFIG_SCHEMA,
        bindings: [{ kind: "folder", path: "/tmp/s-user1/os" }],
        actor: "actor-1",
      });
      expect(identityActorConfig("actor-1")).toEqual({ documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings: [], actor: "actor-1" });
    });

    it("sign-in -> sign-out -> sign-in round-trips through applyIdentityConfigMutation, and each inverts the last", async () => {
      const { applyIdentityConfigMutation, inverseIdentityConfigMutation, signIn, signOut } = await import("../../🎚️config/🧬️schema/🧬️mutations/🟦️");

      const first = sampleIdentity();
      const afterFirstSignIn = applyIdentityConfigMutation(null, signIn(first));
      expect(afterFirstSignIn).toEqual(first);

      const afterSignOut = applyIdentityConfigMutation(afterFirstSignIn, signOut());
      expect(afterSignOut).toBeNull();

      const second = sampleIdentity({ userId: "u-2", email: "devon@semio.dev", displayName: "Devon", issuedAtMs: 2_000 });
      const afterSecondSignIn = applyIdentityConfigMutation(afterSignOut, signIn(second));
      expect(afterSecondSignIn).toEqual(second);

      // ↩️ sign-out's inverse, from the base it cleared, restores exactly the prior session.
      expect(inverseIdentityConfigMutation(signOut(), afterFirstSignIn)).toEqual([signIn(first)]);
      // ↩️ sign-out's inverse with no prior session is a no-op.
      expect(inverseIdentityConfigMutation(signOut(), null)).toEqual([]);
      // ↩️ sign-in's inverse, from no prior session, is a sign-out.
      expect(inverseIdentityConfigMutation(signIn(first), null)).toEqual([signOut()]);
      // ↩️ sign-in's inverse, from a prior session (switching accounts), restores the prior one.
      expect(inverseIdentityConfigMutation(signIn(second), afterFirstSignIn)).toEqual([signIn(first)]);
    });

    it("foldIdentityEvent folds sign-in -> sign-out -> sign-in as last-envelope-wins, ignoring non-remoteMutations events", () => {
      const first = sampleIdentity();
      const second = sampleIdentity({ userId: "u-2" });
      const decodePayload = (payload: unknown): Identity | null | undefined => {
        if (payload === null) return null;
        if (typeof payload === "object" && payload !== null && "userId" in payload) return payload as Identity;
        return undefined;
      };
      const envelope = (payload: unknown): ArtifactEvent => ({
        kind: "remoteMutations",
        envelopes: [
          {
            id: "e",
            actor: "a",
            document: IDENTITY_CONFIG_SCHEMA,
            schemaVersion: IDENTITY_CONFIG_SCHEMA,
            payloadHash: "",
            diff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload },
            inverse: { targetOperation: "e", inverseDiff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: null }, baseVersion: 0, undoPolicy: "exactBaseOnly" },
          },
        ],
      });

      let state: Identity | null = null;
      state = foldIdentityEvent(state, envelope(first), decodePayload);
      expect(state).toEqual(first);
      state = foldIdentityEvent(state, envelope(null), decodePayload);
      expect(state).toBeNull();
      state = foldIdentityEvent(state, envelope(second), decodePayload);
      expect(state).toEqual(second);
      // 🚧️ A non-`remoteMutations` event (e.g. `status`) passes state through unchanged.
      state = foldIdentityEvent(state, { kind: "status", persisted: true, pendingMutations: 0, remote: { kind: "detached" } }, decodePayload);
      expect(state).toEqual(second);
    });
  });
  //#endregion 🔖️IdentityTests

  //#region 🔖️ConfigMutationTests
  describe("config mutation TypeScript parity", () => {
    it("opening mutations replace one coordinate, preserve siblings, and invert exactly", async () => {
      const { applyOpeningConfigMutation, clearDefaultApp, inverseOpeningConfigMutation, setDefaultApp } = await import("../../🎚️config/🧬️schema/🧬️mutations/🟦️");
      const dialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
      const viewer = { pluginId: "cad", appId: "viewer" };
      const editor = { pluginId: "cad", appId: "editor" };
      const replacement = { pluginId: "draft", appId: "drafting" };
      const base = {
        defaults: [
          { dialect, role: "viewer" as const, app: viewer },
          { dialect, role: "editor" as const, app: editor },
        ],
      };
      const set = setDefaultApp(dialect, "editor", replacement);
      const afterSet = applyOpeningConfigMutation(base, set);
      expect(afterSet).toEqual({
        defaults: [
          { dialect, role: "viewer", app: viewer },
          { dialect, role: "editor", app: replacement },
        ],
      });
      expect(inverseOpeningConfigMutation(set, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      const clear = clearDefaultApp(dialect, "editor");
      expect(applyOpeningConfigMutation(base, clear)).toEqual({ defaults: [{ dialect, role: "viewer", app: viewer }] });
      expect(inverseOpeningConfigMutation(clear, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      expect(inverseOpeningConfigMutation(clearDefaultApp(dialect, "editor"), { defaults: [] })).toEqual([]);
    });

    it("change-merge-policy applies and inverts the prior whole-record setting", async () => {
      const { applyMergePolicyConfigMutation, changeMergePolicy, inverseMergePolicyConfigMutation } = await import("../../🎚️config/🧬️schema/🧬️mutations/🟦️");
      const mutation = changeMergePolicy("Vigilant");
      expect(applyMergePolicyConfigMutation({ policy: "Normal" }, mutation)).toEqual({ policy: "Vigilant" });
      expect(inverseMergePolicyConfigMutation(mutation, { policy: "Normal" })).toEqual([changeMergePolicy("Normal")]);
    });

    it("Nx project inputs track every external OS config and plugin-host source compiled by the targets", async () => {
      const { readFile } = await import("node:fs/promises");
      const tsProject = JSON.parse(await readFile(new URL("./📦️packages/🟦️typescript/📋️project.json", source.url), "utf8")) as { namedInputs: { default: string[] } };
      const hostProject = JSON.parse(await readFile(new URL("./🖥️host/📦️packages/🦀️rust/📋️project.json", source.url), "utf8")) as { namedInputs: { default: string[] } };
      expect(tsProject.namedInputs.default).toContain("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*");
      expect(hostProject.namedInputs.default).toEqual(
        expect.arrayContaining(["{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*.rs", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.rs", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.json"]),
      );
    });
  });
  //#endregion 🔖️ConfigMutationTests

  //#region 🔖️DirectoryLaneTests
  describe("backbone-worker space administration", () => {
    const SPACE = "space-admin-01";

    /** ⏳️ Waits for actual operation completion, including asynchronous page and receipt verification. */
    const settleAdministrationTurns = async (_harness?: { readonly posted: readonly unknown[] }): Promise<void> => {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await vi.waitFor(() => {
        const operation = testSeams.directoryAdministration;
        if (operation === null) return;
        expect(operation.phase).toBe("ready");
        expect(operation.requestId).toBeNull();
        expect(operation.pageRead).toBeNull();
      }, { timeout: 2000, interval: 5 });
      await new Promise((resolve) => setTimeout(resolve, 0));
    };

    async function sealAdministrationPage(members: readonly { userId: string; email: string; role: "author" | "spectator"; owner: boolean }[], invites: readonly { inviteId: string; createdAtMs: number }[], properties: { name?: string; visibility?: "public" | "private"; capabilities?: Readonly<Record<string, boolean>> } = {}): Promise<string> {
      const unsigned = {
        access: "author" as const,
        schema: "semio.directory.space-administration-page.v1" as const,
        sessionBindingSha256: "a".repeat(64),
        authorizationGeneration: 5,
        spaceId: SPACE,
        space: { id: SPACE, name: properties.name ?? "Administered", kind: "studio", visibility: properties.visibility ?? "private", ownerUserId: "user-a", role: "author", memberCount: members.length, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 },
        members: { rows: members.map((row) => ({ userId: row.userId, email: row.email, displayName: row.userId, role: row.role, owner: row.owner })) },
        documents: { rows: [] as unknown[] },
        invites: { rows: invites.map((row) => ({ inviteId: row.inviteId, role: "spectator" as const, createdAtMs: row.createdAtMs, expiresAtMs: 900000, revoked: false, accepted: false })) },
        capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true, ...properties.capabilities },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function sealMemberAdministrationPage(): Promise<string> {
      const unsigned = {
        access: "member" as const,
        schema: "semio.directory.space-administration-page.v1" as const,
        sessionBindingSha256: "a".repeat(64),
        authorizationGeneration: 6,
        spaceId: SPACE,
        space: { id: SPACE, name: "Administered", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "spectator", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 3 },
        members: { rows: [{ userId: "user-b", email: "b@example.invalid", displayName: "user-b", role: "spectator" as const, owner: false }] },
        documents: { rows: [] as unknown[] },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function sealCommandReceipt(requestId: string, command: DirectoryCommand, inviteToken?: string, outcome: "accepted" | "previously-accepted" | "secret-undeliverable" = "accepted"): Promise<string> {
      const commandSha256 = await directorySha256(JSON.stringify(command));
      const unsigned = {
        schema: "semio.directory.command-receipt.v1" as const,
        requestId,
        commandSha256,
        outcome,
        events: [] as unknown[],
        result: inviteToken === undefined ? { kind: "none" as const } : { kind: "invite" as const, inviteToken },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function directorySha256(text: string): Promise<string> {
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
      return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
    }

    function administrationHarness(pageStatuses: readonly number[], receiptStatus = 200) {
      const posted: BackboneWorkerResponse[] = [];
      const original = testSeams.workerPostTestSink;
      testSeams.workerPostTestSink = (message) => posted.push(message);
      const requests: string[] = [];
      let pageIndex = 0;
      const bodies: string[] = [];
      const fetches = vi.fn(async (input: string, init?: { method?: string }) => {
        requests.push(`${init?.method ?? "GET"} ${input}`);
        if ((init?.method ?? "GET") === "POST") {
          return { ok: receiptStatus < 400, status: receiptStatus, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
        }
        const status = pageStatuses[Math.min(pageIndex, pageStatuses.length - 1)] ?? 200;
        pageIndex += 1;
        return { ok: status < 400, status, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
      });
      return {
        posted,
        requests,
        bodies,
        fetches,
        release: () => {
          testSeams.workerPostTestSink = original;
        },
      };
    }

    it("admits administration transport only for the verified page's exact space and command capability", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📇️directory/🧬️schema/";
      const fixture = JSON.parse(readFileSync(new URL(base + "🏛️administration/🧫️fixtures/🛂️command-admission/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL(base + "🏛️administration/🧫️fixtures/🛂️command-admission/🧬️schema/🔣️.json", source.url), "utf8"));
      const directorySchema = JSON.parse(readFileSync(new URL(base + "🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).addSchema(directorySchema).compile(schema)(fixture)).toBe(true);
      const members = [{ userId: "user-a", email: "a@example.invalid", role: "author" as const, owner: true }];
      let epoch = 100;
      for (const row of fixture.allowed as Array<{ capability: string; command: DirectoryCommand }>) {
        for (const refusal of ["foreign-command", "withdrawn-capability", "member-page", "missing-page"]) {
          const harness = administrationHarness([200]);
          try {
            testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
            harness.bodies.push(refusal === "member-page" ? await sealMemberAdministrationPage() : await sealAdministrationPage(members, [], { capabilities: refusal === "withdrawn-capability" ? { [row.capability]: false } : {} }));
            handleTsRequest({ kind: "directory-administration-open", operationEpoch: ++epoch, spaceId: SPACE });
            if (refusal !== "missing-page") await settleAdministrationTurns(harness);
            const command = refusal === "foreign-command" ? { ...row.command, spaceId: "another-space" } : row.command;
            handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId: "1".repeat(32), command });
            await settleAdministrationTurns(harness);
            expect(equal(harness.requests.filter((entry) => entry.startsWith("POST")), []), row.command.kind + ":" + refusal).toBe(true);
            expect(harness.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "submitting")).toBe(false);
            expect(testSeams.directoryAdministration?.requestId).toBeNull();
          } finally {
            harness.release();
            closeDirectory();
          }
        }
      }
      const harness = administrationHarness([200]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(await sealAdministrationPage(members, []));
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: ++epoch, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        for (const command of [...fixture.unrelated, ...fixture.malformed]) {
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId: "1".repeat(32), command });
          await settleAdministrationTurns(harness);
          expect(equal(harness.requests.filter((entry) => entry.startsWith("POST")), [])).toBe(true);
          expect(testSeams.directoryAdministration?.phase).toBe("ready");
        }
        console.log("[DEBUG] actual administration worker refused foreign-space, withdrawn, member, unverified, unrelated and malformed commands before POST");
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("retires older administration page reads and refuses refresh during a sealed command", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📇️directory/🧬️schema/🏛️administration/🧫️fixtures/📄️page-retirement/";
      const fixture = JSON.parse(readFileSync(new URL(base + "🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL(base + "🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const author = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      const member = await sealMemberAdministrationPage();
      for (const row of fixture.cases) {
        const harness = administrationHarness([200]);
        const pending: Array<(value: Response) => void> = [];
        let reads = 0;
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: async () => ++reads === 1 ? new Response(author) : await new Promise<Response>((resolve) => pending.push(resolve)) });
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: 145, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 145 });
          handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 145 });
          expect(pending).toHaveLength(2);
          pending[1]!(new Response(member));
          await settleAdministrationTurns(harness);
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(member);
          const messages = harness.posted.length;
          pending[0]!(row.older === "failure" ? new Response("", { status: 403 }) : new Response(author));
          await settleAdministrationTurns(harness);
          expect(equal(testSeams.directoryAdministration?.page?.access, row.expected), row.id).toBe(true);
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(member);
          expect(harness.posted).toHaveLength(messages);
        } finally {
          harness.release();
          closeDirectory();
        }
      }
      const harness = administrationHarness([200]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(author);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 146, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "rename-space", spaceId: SPACE, name: "Research" };
        harness.bodies.push(await sealCommandReceipt("3".repeat(32), command), author);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 146, requestId: "3".repeat(32), command });
        const requests = harness.requests.length;
        handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 146 });
        expect(harness.requests.length - requests).toBe(fixture.refreshDuringSubmit.additionalRequests);
        expect(harness.posted.at(-1)).toMatchObject({ phase: "submitting", code: fixture.refreshDuringSubmit.code });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        console.log("[DEBUG] administration worker ignored retired page success/failure and blocked refresh during a sealed command");
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("changes space properties only after exact worker receipts and canonical refreshes", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧫️fixtures/⚙️properties/";
      const fixture = JSON.parse(readFileSync(new URL(base + "🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL(base + "🧬️schema/🔣️.json", source.url), "utf8"));
      const directorySchema = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).addSchema(directorySchema).compile(schema)(fixture)).toBe(true);
      const members = [{ userId: "user-a", email: "a@example.invalid", role: "author" as const, owner: true }];
      const properties: { name: string; visibility: "public" | "private" } = { name: "Administered", visibility: "private" };
      const harness = administrationHarness([200]);
      let canonical = await sealAdministrationPage(members, [], properties);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(canonical);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 140, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        let index = 0;
        for (const row of fixture.cases as Array<{ command: Extract<DirectoryCommand, { kind: "rename-space" | "set-visibility" }> }>) {
          const requestId = (++index).toString(16).padStart(32, "0");
          const receipt = await sealCommandReceipt(requestId, row.command);
          if (row.command.kind === "rename-space") properties.name = row.command.name;
          else properties.visibility = row.command.visibility;
          const next = await sealAdministrationPage(members, [], properties);
          harness.bodies.push(receipt, next);
          const start = harness.posted.length;
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 140, requestId, command: row.command });
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(canonical);
          await settleAdministrationTurns(harness);
          await settleAdministrationTurns(harness);
          const states = harness.posted.slice(start).filter((message) => message.kind === "directory-administration-state");
          expect(equal(states.map((message) => message.phase), ["submitting", "receipt", "refreshing", "ready"])).toBe(true);
          expect(states.slice(0, 3).every((message) => message.canonicalJson === canonical)).toBe(true);
          expect(states[1]?.receiptSha256).toBe(JSON.parse(receipt).receiptSha256);
          expect(states[3]?.canonicalJson).toBe(next);
          expect(equal(JSON.parse(next).space.name, properties.name)).toBe(true);
          expect(equal(JSON.parse(next).space.visibility, properties.visibility)).toBe(true);
          canonical = next;
        }
        expect(harness.requests.filter((entry) => entry.startsWith("POST"))).toHaveLength(fixture.cases.length);
        console.log("[DEBUG] actual worker applied four neutral name/visibility results only after independently SHA-256 sealed receipt and page refresh");
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("drives loading → ready → submitting → receipt → refreshing without changing state before the receipt", async () => {
      const harness = administrationHarness([200, 200]);
      const first = await sealAdministrationPage(
        [
          { userId: "user-a", email: "a@example.invalid", role: "author", owner: true },
          { userId: "user-b", email: "b@example.invalid", role: "spectator", owner: false },
        ],
        [{ inviteId: "invite-1", createdAtMs: 20 }],
      );
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(first);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 1, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const loading = harness.posted.filter((message) => message.kind === "directory-administration-state");
        expect(loading.map((message) => (message as { phase: string }).phase)).toEqual(["loading", "ready"]);
        expect((loading.at(-1) as { canonicalJson?: string }).canonicalJson).toBe(first);

        const command: DirectoryCommand = { kind: "remove-member", spaceId: SPACE, userId: "user-b" };
        const receipt = await sealCommandReceipt("0".repeat(31) + "1", command, undefined);
        const second = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], [{ inviteId: "invite-1", createdAtMs: 20 }]);
        harness.bodies.push(receipt, second);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 1, requestId: "0".repeat(31) + "1", command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        const phases = harness.posted.filter((message) => message.kind === "directory-administration-state").map((message) => (message as { phase: string }).phase);
        expect(phases).toEqual(["loading", "ready", "submitting", "receipt", "refreshing", "ready"]);
        const submitting = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "submitting") as { canonicalJson?: string; receiptSha256?: string };
        expect(submitting.canonicalJson).toBe(first);
        expect(submitting.receiptSha256).toBeUndefined();
        const receiptState = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "receipt") as { canonicalJson?: string; receiptSha256?: string };
        expect(receiptState.canonicalJson).toBe(first);
        expect(receiptState.receiptSha256).toMatch(/^[0-9a-f]{64}$/u);
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "ready", canonicalJson: second });
        expect(harness.requests.filter((entry) => entry.startsWith("POST"))).toHaveLength(1);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("settles delete only from an exact accepted receipt and never interprets a page 404 as deletion", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const base = "./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧫️fixtures/🗑️delete-space/";
      const fixture = JSON.parse(readFileSync(new URL(base + "🔣️.json", source.url), "utf8")) as { acceptedOutcomes: Array<"accepted" | "previously-accepted"> };
      const schema = JSON.parse(readFileSync(new URL(base + "🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      let epoch = 160;
      for (const outcome of fixture.acceptedOutcomes) {
        const harness = administrationHarness([200]);
        const command: DirectoryCommand = { kind: "delete-space", spaceId: SPACE };
        const requestId = (++epoch).toString(16).padStart(32, "0");
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
          harness.bodies.push(page, await sealCommandReceipt(requestId, command, undefined, outcome));
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: epoch, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          const requestCount = harness.requests.length;
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId, command });
          await vi.waitFor(() => expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "deleted", receiptSha256: expect.stringMatching(/^[0-9a-f]{64}$/u) }));
          const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>;
          expect(terminal.canonicalJson).toBeUndefined();
          expect(terminal.inviteCapabilityPending).toBeUndefined();
          expect(harness.requests.slice(requestCount).filter((request) => request.startsWith("POST"))).toHaveLength(1);
          expect(harness.requests.slice(requestCount).filter((request) => request.startsWith("GET"))).toHaveLength(0);
          expect(testSeams.directoryAdministration).toBeNull();
        } finally {
          harness.release();
          closeDirectory();
        }
      }
      const refused = administrationHarness([200]);
      try {
        const command: DirectoryCommand = { kind: "delete-space", spaceId: SPACE };
        const requestId = "f".repeat(32);
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: refused.fetches as never });
        refused.bodies.push(page, await sealCommandReceipt(requestId, command, undefined, "secret-undeliverable"));
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 190, spaceId: SPACE });
        await settleAdministrationTurns(refused);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 190, requestId, command });
        await vi.waitFor(() => expect(refused.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "failed", code: "invalid" }));
        expect(refused.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "deleted")).toBe(false);
      } finally {
        refused.release();
        closeDirectory();
      }
      const missing = administrationHarness([404]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: missing.fetches as never });
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 191, spaceId: SPACE });
        await vi.waitFor(() => expect(missing.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "denied", code: "forbidden" }));
        expect(missing.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "deleted")).toBe(false);
      } finally {
        missing.release();
        closeDirectory();
      }
    });

    it("retains the invite capability until exact clipboard success and rejects duplicate results without redisclosure", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 2, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "create-invite", spaceId: SPACE, role: "spectator", ttlSecs: 3600 };
        harness.bodies.push(await sealCommandReceipt("1".repeat(32), command, "invite.v1.secret"), page);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 2, requestId: "1".repeat(32), command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        expect(harness.posted.some((message) => message.kind === "directory-administration-state" && (message as { inviteCapabilityPending?: boolean }).inviteCapabilityPending === true)).toBe(true);
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        let capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(1);
        expect(capabilities[0]).toMatchObject({ operationEpoch: 2, transferEpoch: 1, inviteToken: "invite.v1.secret" });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "capacity" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: false });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({
          inviteCapabilityPending: true,
          inviteCapabilityStatus: "failed",
        });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(2);
        expect(capabilities[1]).toMatchObject({ operationEpoch: 2, transferEpoch: 2, inviteToken: "invite.v1.secret" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "mismatch" });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).not.toMatchObject({ inviteCapabilityPending: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 2, code: "already-settled" });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-capability")).toHaveLength(2);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("erases an invite capability when the canonical refresh no longer grants author access", async () => {
      const harness = administrationHarness([200, 200]);
      const author = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      const member = await sealMemberAdministrationPage();
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(author);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 6, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "create-invite", spaceId: SPACE, role: "spectator", ttlSecs: 3600 };
        harness.bodies.push(await sealCommandReceipt("3".repeat(32), command, "invite.v1.spectator-secret"), member);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 6, requestId: "3".repeat(32), command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "ready", canonicalJson: member });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 6 });
        expect(harness.posted.some((message) => message.kind === "directory-administration-capability")).toBe(false);
        expect(harness.posted.at(-1)).toEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 6, code: "already-settled" });
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("clears the pane on 401 and on 403 and never retains a page after the denial", async () => {
      for (const status of [401, 403]) {
        const harness = administrationHarness([status]);
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: 3, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
          expect(terminal.phase).toBe("denied");
          expect(terminal.canonicalJson).toBeUndefined();
          expect(terminal.code).toBe(status === 401 ? "unauthorized" : "forbidden");
        } finally {
          harness.release();
          closeDirectory();
        }
      }
    });

    it("retires the pane on a scoped 4401 for its own space and ignores another space's revocation", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 5, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope("some-other-space");
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope(SPACE);
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
        expect(terminal.phase).toBe("denied");
        expect(terminal.code).toBe("forbidden");
        expect(terminal.canonicalJson).toBeUndefined();
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("cancels on close and drops every later turn for the retired epoch", async () => {
      const harness = administrationHarness([200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 4, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        if (testSeams.directoryAdministration === null) throw new Error("administration operation missing");
        testSeams.directoryAdministration.inviteToken = "invite.v1.close-secret";
        testSeams.directoryAdministration.inviteCapabilityStatus = "available";
        handleTsRequest({ kind: "directory-administration-close", operationEpoch: 4 });
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string };
        expect(terminal.phase).toBe("cancelled");
        expect(terminal.canonicalJson).toBeUndefined();
        const before = harness.requests.length;
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 4, requestId: "2".repeat(32), command: { kind: "remove-member", spaceId: SPACE, userId: "user-b" } });
        handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 4 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 4 });
        await settleAdministrationTurns(harness);
        expect(harness.requests).toHaveLength(before);
        expect(harness.posted.some((message) => message.kind === "directory-administration-capability")).toBe(false);
        expect(terminal).not.toMatchObject({ inviteCapabilityPending: true });
      } finally {
        harness.release();
        closeDirectory();
      }
    });
  });

  describe("backbone-worker directory lane", () => {
    class FakeDirectoryWebSocket {
      static instances: FakeDirectoryWebSocket[] = [];
      readonly url: string;
      readonly protocol = "semio.socket.v1";
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

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    const commandRequestId = (index: number): string => index.toString(16).padStart(32, "b");
    const sampleCommand: DirectoryCommand = { kind: "create-space", name: "Atelier", spaceKind: "atelier", visibility: "private" };
    const inviteCommand = (spaceId: string): DirectoryCommand => ({ kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 });

    async function commandReceiptBody(request: DirectoryCommandRequestV1, outcome: DirectoryCommandOutcomeV1 = "accepted", result: DirectoryCommandResultV1 = { kind: "none" }): Promise<string> {
      return JSON.stringify(await sealDirectoryCommandReceiptV1(request.requestId, await directoryCommandSha256(request.command), outcome, [], result));
    }

    /** ⏳️ Drains enough macrotask turns for every in-flight command turn (fetch, response text, and
     * two SHA-256 digests) to settle, so a transport assertion never races the receipt parser. */
    async function settleDirectoryTransport(): Promise<void> {
      for (let turn = 0; turn < 8; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
    }

    /** 🧪️ Installs a directory client whose transport is a recorded stub, isolating the command
     * transport owner from the browser broker's own proof ratchet (proved separately above). */
    function installFakeDirectoryClient(respond: (request: DirectoryCommandRequestV1) => Promise<{ status: number; body: string }> | { status: number; body: string }): string[] {
      const bodies: string[] = [];
      testSeams.directoryClient = new DirectoryClient("http://hub.test", {
        request: async (_input: string, init: RequestInit = {}) => {
          const body = String(init.body ?? "");
          bodies.push(body);
          const response = await respond(JSON.parse(body) as DirectoryCommandRequestV1);
          return { ok: response.status >= 200 && response.status < 300, status: response.status, text: async () => response.body } as unknown as FetchTimeoutResponse;
        },
      });
      testSeams.directorySessionEpoch += 1;
      return bodies;
    }

    it("retries only a transient fault with the byte-identical sealed request and never retries a terminal rejection", async () => {
      let status = 503;
      const bodies = installFakeDirectoryClient(async (request) => (status === 202 ? { status, body: await commandReceiptBody(request) } : { status, body: "hub text the client must never echo" }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        status = 202;
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(2);
        expect(bodies[0]).toBe(bodies[1]);
        expect(bodies[0]).toBe(JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: commandRequestId(1), command: sampleCommand }));
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);

        for (const [index, terminal] of [401, 403, 409, 413].entries()) {
          status = terminal;
          bodies.length = 0;
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(10 + index), command: sampleCommand });
          await settleDirectoryTransport();
          expect(bodies).toHaveLength(1);
          expect(directoryCommandQueue).toHaveLength(0);
          expect(directoryCommandOperations.size).toBe(0);
        }
      } finally {
        closeDirectory();
      }
    });

    it("bounds the transport at a fixed capacity, keeps the oldest intent, and terminates a malformed correlation", async () => {
      installFakeDirectoryClient(() => {
        throw new Error("network unreachable");
      });
      try {
        for (let index = 0; index < DIRECTORY_COMMAND_TRANSPORT_CAPACITY; index += 1) {
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(100 + index), command: sampleCommand });
        }
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandQueue).toHaveLength(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(9999), command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(true);
        expect(directoryCommandOperations.has(commandRequestId(9999))).toBe(false);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(100));

        handleTsRequest({ kind: "directory-command", requestId: "r1", command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.has("r1")).toBe(false);

        handleTsRequest({ kind: "directory-command-cancel", requestId: commandRequestId(100) });
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY - 1);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(false);
        closeDirectory();
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("stops the queue at a transient head, then drains it in order without a capability reaching any log", async () => {
      const token = "inv.01920000000070008000000000000001.dGhpcy1jYXBhYmlsaXR5LW5ldmVyLXJlYWNoZXMtYS1sb2c";
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      let head: "transient" | "accepted" = "transient";
      const bodies = installFakeDirectoryClient(async (request) => {
        if (request.requestId === commandRequestId(200) && head === "transient") return { status: 503, body: "" };
        return { status: 202, body: await commandReceiptBody(request, "accepted", request.command.kind === "create-invite" ? { kind: "invite", inviteToken: token } : { kind: "none" }) };
      });
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(200), command: inviteCommand("space-a") });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(201), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue.map((operation) => operation.request.requestId)).toEqual([commandRequestId(200), commandRequestId(201)]);

        head = "accepted";
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(3);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
        expect(JSON.stringify([...directoryCommandOperations.values(), ...directoryCommandQueue])).not.toContain(token);
        expect(
          errorSpy.mock.calls
            .flat()
            .map((entry) => String(entry))
            .join("|"),
        ).not.toContain(token);
      } finally {
        errorSpy.mockRestore();
        closeDirectory();
      }
    });

    it("suppresses delivery for an operation whose session epoch was replaced and settles each operation exactly once", async () => {
      const bodies = installFakeDirectoryClient(async (request) => ({ status: 202, body: await commandReceiptBody(request) }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(300), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.size).toBe(0);

        const stale: DirectoryCommandTransportOperationV1 = {
          request: sealDirectoryCommandRequestV1(commandRequestId(301), sampleCommand),
          abort: new AbortController(),
          sessionEpoch: testSeams.directorySessionEpoch - 1,
          workerEpoch: directoryWorkerEpoch,
          settled: false,
        };
        directoryCommandOperations.set(commandRequestId(301), stale);
        directoryCommandQueue.push(stale);
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.has(commandRequestId(301))).toBe(false);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(stale.settled).toBe(true);

        settleDirectoryCommand(stale, { kind: "directory-command-failed", requestId: commandRequestId(301), code: "closed" });
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("queues a directory command while the hub is unreachable, then flushes it in order on the next live signal", async () => {
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      let reachable = false;
      try {
        handleTsRequest({ kind: "directory-open", baseUrl: "http://hub.test", since: 0 });
        const bodies = installFakeDirectoryClient(async (request) => {
          if (!reachable) throw new Error("network unreachable");
          return { status: 202, body: await commandReceiptBody(request) };
        });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(1));

        // 🟢️ Hub becomes reachable — any live signal on the stream (a heartbeat here) triggers a flush.
        reachable = true;
        FakeDirectoryWebSocket.instances.at(-1)!.triggerMessage({ kind: "heartbeat", headSeq: 0 });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(2);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
      } finally {
        // 🧹️ Restores the real global — an un-restored `FakeDirectoryWebSocket` (no `OPEN` static)
        // previously leaked into every later test's `WebSocket.OPEN` comparisons, silently making
        // `relayMutationsToHub`/`sendWireFrame`'s "is the socket actually open" checks pass when
        // `state.socket` was `null`.
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeDirectory();
      }
    });

    it("backbone worker owns one full scoped stream and retires it terminally on 4401", async () => {
      vi.useFakeTimers();
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const paths: string[] = [];
      testSeams.socketGrantTestIssue = async (_baseUrl, path) => {
        paths.push(path);
        return {
          schema: "semio.hub.socket-grant/v1",
          protocol: "semio.socket.v1",
          grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
          actorId: `hub.v1.${"3".repeat(64)}`,
          expiresAtMs: Number.MAX_SAFE_INTEGER,
        };
      };
      const scope = { spaceId: "space/a", documentId: "document b" };
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        handleTsRequest({ kind: "directory-scope-open", baseUrl: "http://hub.test", scope, since: 7 });
        for (let turn = 0; turn < 16 && FakeDirectoryWebSocket.instances.length === 0; turn += 1) await Promise.resolve();
        const socket = FakeDirectoryWebSocket.instances[0]!;
        expect(paths).toEqual(["/directory/spaces/space%2Fa/documents/document%20b/socket-grants"]);
        expect(socket.url).toBe("ws://hub.test/directory/spaces/space%2Fa/documents/document%20b/socket/v1?since=7");
        socket.triggerOpen();
        const issue = testSeams.socketGrantTestIssue;
        testSeams.socketGrantTestIssue = null;
        openArtifact({ documentId: scope.documentId, schema: "gis.map", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: scope.spaceId }], actor: "caller" });
        testSeams.socketGrantTestIssue = issue;
        const state = artifactState(scope.documentId, scope.spaceId)!;
        testSeams.inferenceApprovalUndoOwner = {
          historyEpoch: ++testSeams.inferenceApprovalUndoEpoch,
          scope,
          clientInstanceId: state.openClientInstanceId,
          sessionEpoch: testSeams.directorySessionEpoch,
          receipt: {
            schema: "semio.hub.inference-approval-receipt/v1",
            jobId: "1".repeat(32),
            mutationId: "2".repeat(32),
            commandHash: "3".repeat(64),
            proposalHash: "4".repeat(64),
            applied: true,
            undo: { targetId: "5".repeat(32), expectedCurrent: { documentId: scope.documentId, headEditOrdinal: 1, headEditId: "edit-1", lastCommitSeq: 1, chainSha256: "6".repeat(64) } },
          },
          idempotencyKey: "7".repeat(32),
          sourceCatalogGenerationId: "8".repeat(64),
          sourceComponentSha256: "9".repeat(64),
          sourceDescriptorSha256: "a".repeat(64),
          sourceBrowserActorSha256: "b".repeat(64),
          sourceDirectoryRevision: 1,
          sourceMembershipGeneration: 1,
          sourceSessionGeneration: 1,
          abort: new AbortController(),
          mount: null,
          phase: "submitting",
          retryable: true,
        };
        const revokedOwner = testSeams.inferenceApprovalUndoOwner;
        socket.triggerClose(4401);
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(scopedDirectoryStreams.size).toBe(0);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        expect(artifacts.has(state.runtimeKey)).toBe(false);
        expect(revokedOwner.abort.signal.aborted).toBe(true);
        expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
        expect(posted.some((message) => message.kind === "inference-history-status" && message.historyEpoch === revokedOwner.historyEpoch && message.status.phase === "unavailable")).toBe(true);
        expect(posted.some((message) => message.kind === "directory-scope-revoked")).toBe(true);
      } finally {
        closeDirectory();
        testSeams.socketGrantTestIssue = null;
        testSeams.workerPostTestSink = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        vi.useRealTimers();
      }
    });
  });
  //#endregion 🔖️DirectoryLaneTests

  //#region 💡️InferencePortTests
  // 💡️ Slice D — the host-owned ephemeral GIS Map inference port, driven through a FAKE, already
  // authenticated transport (an installed browser-broker proof plus a stubbed `fetch`). These are
  // the port's own laws: it refuses to start without a verified live execution-target lease, it
  // publishes no state before an exact server receipt, a Cancel click is never an optimistic
  // terminal, `stale` and `failed` are hard, and no document mutation, outbox entry or artifact
  // event is ever produced.
  describe("gis map inference port", () => {
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
        package: { pluginId: "gis", packageId: "semio:gis", version: "0.1.0", componentSha256: HASH, componentBlake3: HASH, descriptorByteSha256: HASH, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } },
        component: { sha256: HASH, blake3: HASH, byteLength: 1 },
        descriptor: { sha256: HASH, byteLength: 1 },
        browserActor: {
          kind: "closed-browser-actor",
          schema: "semio.os.closed-browser-actor.v1",
          codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1",
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
        proofs.push(init?.headers?.["x-semio-browser-broker"] ?? "");
        const status = statuses.shift() ?? 200;
        const body = bodies.shift() ?? "";
        await gates.shift();
        return {
          ok: status < 400,
          status,
          statusText: "",
          headers: new Headers({ "content-type": "application/json", "content-length": String(new TextEncoder().encode(body).length), "x-semio-browser-broker-advanced": "1" }),
          body: new Response(body).body,
          text: async () => body,
          json: async () => JSON.parse(body),
        };
      };
      clearLocalBrowserBrokerProof();
      installLocalBrowserBrokerProof("b".repeat(64));
      if (options.authority !== "none") {
        const { readFileSync } = await import("node:fs");
        const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
        const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
        bodies.push(JSON.stringify(authority));
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
        requests.length = 0;
        bodies.length = 0;
        statuses.length = 0;
        gates.length = 0;
        proofs.length = 0;
      }
      // 🧷️ No socket-grant issuer and no installed target means `openArtifact` opens no hub socket
      // at all, so this harness exercises the inference port's own four calls and nothing else.
      const originalIssue = testSeams.socketGrantTestIssue;
      testSeams.socketGrantTestIssue = null;
      openArtifact({ documentId: DOCUMENT, schema: "gis.map", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: SPACE }], actor: "caller" });
      const state = artifactState(DOCUMENT, SPACE)!;
      if (options.lease !== "none") state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, leaseFields(options.lease === "editor"), "http://hub.test", new Uint8Array(1), new Uint8Array(1));
      return {
        posted,
        requests,
        bodies,
        statuses,
        gates,
        proofs,
        state,
        ports: () => posted.filter((message) => message.kind === "inference-port-status").map((message) => (message as Extract<BackboneWorkerResponse, { kind: "inference-port-status" }>).status),
        release: () => {
          const retained = testSeams.inferencePort;
          if (retained !== null) {
            retained.closed = true;
            if (retained.pollTimer !== null) clearTimeout(retained.pollTimer);
            retained.abort.abort();
            testSeams.inferencePort = null;
          }
          closeArtifact(DOCUMENT, SPACE);
          clearLocalBrowserBrokerProof();
          testSeams.workerPostTestSink = original;
          testSeams.socketGrantTestIssue = originalIssue;
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
      const { default: Ajv } = await import("ajv");
      const wire = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).wire;
      const schema = JSON.parse(readFileSync(new URL("../../../🌎️hub/💡️inference/🧬️schema/🔣️.json", source.url), "utf8"));
      const production = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
      for (const [name, definition, parse] of [
        ["receipt", "InferenceJobReceiptV1", production.parseGisMapInferenceJobReceiptV1],
        ["page", "InferenceEventPageV1", production.parseGisMapInferenceEventPageV1],
      ] as const) {
        const valid = new Ajv({ strict: true }).addKeyword("x-semio-formats").compile({ $defs: schema.$defs, $ref: `#/$defs/${definition}` });
        expect(valid(wire[name]), JSON.stringify(valid.errors)).toBe(true);
        expect(equal(parse(wire[name]), wire[name])).toBe(true);
        for (const candidate of [{ ...wire[name], schema: "foreign" }, { ...wire[name], proposalHash: undefined }]) expect(() => parse(candidate)).toThrow();
      }
      console.log("[DEBUG] browser inference consumed exact Hub nullable-hash receipt and event page");
    });

    it("requires verified session authority and never adopts a successor proof for retained inference", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const valid = new Ajv({ strict: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/GisMapInferencePortV1" });
      expect(valid(fixture), JSON.stringify(valid.errors)).toBe(true);
      expect(equal(fixture.retainedClosing.authorityFence.cases, ["absent-authority-refused", "sealed-request-proof-replacement"])).toBe(true);

      const absent = await inferenceHarness({ lease: "editor", authority: "none" });
      try {
        const scope = { spaceId: SPACE, documentId: DOCUMENT };
        const fence = testSeams.browserSessionOperationFence;
        handleTsRequest({ kind: "inference-open", operationEpoch: 81, scope });
        expect(absent.posted.at(-1)).toEqual({ kind: "inference-port-opened", operationEpoch: 81, scope, outcome: "refused", code: fixture.retainedClosing.authorityFence.openingCode });
        handleTsRequest({ kind: "inference-propose", operationEpoch: 81, requestId: fixture.retainedClosing.requestId });
        expect(absent.requests).toEqual(fixture.retainedClosing.authorityFence.successorRoutes);
        clearLocalBrowserBrokerProof();
        expect(installLocalBrowserBrokerProof(fixture.retainedClosing.successorProof)).toBe(true);
        expect(testSeams.browserSessionOperationFence).not.toBe(fence);
        expect(absent.requests).toEqual(fixture.retainedClosing.authorityFence.successorRoutes);
      } finally {
        absent.release();
      }

      const retained = await inferenceHarness({ lease: "editor" });
      try {
        const scope = { spaceId: SPACE, documentId: DOCUMENT };
        handleTsRequest({ kind: "inference-open", operationEpoch: 82, scope });
        retained.bodies.push("{");
        handleTsRequest({ kind: "inference-propose", operationEpoch: 82, requestId: fixture.retainedClosing.requestId });
        await settleInferenceTurns();
        const operation = testSeams.inferencePort;
        expect(operation?.status.phase).toBe(fixture.retainedClosing.unknownPhase);
        const requestCount = retained.requests.length;
        clearLocalBrowserBrokerProof();
        expect(installLocalBrowserBrokerProof(fixture.retainedClosing.successorProof)).toBe(true);
        await driveInferencePort(82);
        expect(retained.requests).toHaveLength(requestCount);
        handleTsRequest({ kind: "inference-open", operationEpoch: 83, scope });
        expect(retained.posted.at(-1)).toMatchObject({ kind: "inference-port-opened", outcome: "refused", code: fixture.retainedClosing.authorityFence.blockedOpeningCode });
        expect(testSeams.inferencePort).toBe(operation);
      } finally {
        retained.release();
      }
      console.log("[DEBUG] inference authority fence rejected pre-/me admission and retained an indeterminate original request across proof replacement");
    });

    it("refuses to start at all without a verified live execution-target lease", async () => {
      const harness = await inferenceHarness({ lease: "none" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 1, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        await Promise.resolve();
        expect(harness.ports()).toEqual([]);
        expect(harness.posted.at(-1)).toEqual({ kind: "inference-port-opened", operationEpoch: 1, scope: { spaceId: SPACE, documentId: DOCUMENT }, outcome: "refused", code: "inference.lease-unverified" });
        expect(harness.requests).toEqual([]);
        handleTsRequest({ kind: "inference-propose", operationEpoch: 1, requestId: "2".repeat(32) });
        await Promise.resolve();
        expect(harness.requests).toEqual([]);
      } finally {
        harness.release();
      }
    });

    it("refuses a verified viewer-only lease, because a proposal it could never approve must not start", async () => {
      const harness = await inferenceHarness({ lease: "viewer" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 2, scope: { spaceId: SPACE, documentId: DOCUMENT } });
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
        handleTsRequest({ kind: "inference-open", operationEpoch: 3, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 3, requestId: "3".repeat(32) });
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
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).cancelBeforeReceipt;
      for (const kind of fixture.intents) {
      const harness = await inferenceHarness({ lease: "editor" });
      const projection = () => { const { phase, jobId, cancelRequested } = harness.ports().at(-1)!; return { phase, jobId, cancelRequested }; };
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: fixture.operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200, 200);
        harness.bodies.push(receiptBody("accepted", "none", 0), pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 0, total: 4, cancelRequested: true }));
        handleTsRequest({ kind: "inference-propose", operationEpoch: fixture.operationEpoch, requestId: fixture.requestId });
        handleTsRequest({ kind, operationEpoch: fixture.operationEpoch });
        expect(equal(projection(), fixture.expectedBeforeReceipt)).toBe(true);
        await settleInferenceTurns();
        expect(harness.ports().some(({ phase, jobId, cancelRequested }) => equal({ phase, jobId, cancelRequested }, fixture.expectedAfterReceipt))).toBe(true);
        await driveInferencePort(fixture.operationEpoch);
        expect(harness.ports().at(-1)?.phase).toBe(fixture.expectedTerminal);
        expect(equal(harness.requests, fixture.expectedRequests)).toBe(true);
        console.log("[DEBUG] inference cancel survived pending submit and waited for exact Hub terminal");
      } finally { harness.release(); }
      }
    });

    it("retains the sealed inference owner through uncertain submit and document retirement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      for (const scenario of fixture.cases) {
      const harness = await inferenceHarness({ lease: "editor" });
        const sessionEpoch = testSeams.directorySessionEpoch;
        try {
          const scope = { spaceId: SPACE, documentId: DOCUMENT };
          handleTsRequest({ kind: "inference-open", operationEpoch: fixture.operationEpoch, scope });
          harness.bodies.push(scenario === "retired-document" ? receiptBody("accepted", "none", 0) : "{");
          let releaseReceipt: (() => void) | undefined;
          if (scenario === "retired-document") harness.gates.push(new Promise<void>((resolve) => { releaseReceipt = resolve; }));
          handleTsRequest({ kind: "inference-propose", operationEpoch: fixture.operationEpoch, requestId: fixture.requestId });
          const original = testSeams.inferencePort;
          if (scenario === "retired-document") {
            for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
            expect(harness.requests).toHaveLength(1);
            closeArtifact(DOCUMENT, SPACE);
            releaseReceipt!();
          }
          else handleTsRequest({ kind: "inference-close", operationEpoch: fixture.operationEpoch });
          await settleInferenceTurns();
          expect(testSeams.inferencePort).toBe(original);
          expect(original?.request?.requestId).toBe(fixture.requestId);
          expect(harness.posted.some((message) => message.kind === fixture.closedKind)).toBe(false);
          handleTsRequest({ kind: "inference-open", operationEpoch: fixture.operationEpoch + 1, scope });
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
              schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
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
          expect(equal(closed, [{ kind: fixture.closedKind, operationEpoch: fixture.operationEpoch, scope }])).toBe(true);
          expect(harness.requests.filter((request) => request.endsWith("/jobs"))).toHaveLength(1);
          console.log(`[DEBUG] inference retained owner resolved ${scenario} without resubmission`);
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
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      for (const scenario of fixture.approvalClose) {
      const harness = await inferenceHarness({ lease: "editor" });
        try {
          const operationEpoch = fixture.operationEpoch + 2;
          handleTsRequest({ kind: "inference-open", operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
          harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
          handleTsRequest({ kind: "inference-propose", operationEpoch, requestId: fixture.requestId });
          await settleInferenceTurns();
          harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
          await driveInferencePort(operationEpoch);
          const receipt = { schema: "semio.hub.inference-approval-receipt/v1", jobId: JOB, mutationId: JOB, commandHash: HASH, proposalHash: HASH, applied: true, undo: { targetId: "2".repeat(32), expectedCurrent: { documentId: DOCUMENT, headEditOrdinal: 2, headEditId: "edit-2", lastCommitSeq: 2, chainSha256: "3".repeat(64) } } };
          let release: () => void = () => undefined;
          if (scenario === "approval-response") {
            harness.bodies.push(JSON.stringify(receipt));
            harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
          } else harness.bodies.push("{");
          handleTsRequest({ kind: "inference-approve", operationEpoch });
          for (let turn = 0; harness.requests.length < 3 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
          expect(harness.requests).toHaveLength(3);
          if (scenario === "reconcile-response") await settleInferenceTurns();
          closeArtifact(DOCUMENT, SPACE);
          const statusCount = harness.posted.filter((message) => message.kind === "inference-history-status").length;
          if (scenario === "approval-response") release();
          else {
            harness.bodies.push(JSON.stringify({
              schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
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
          console.log(`[DEBUG] inference late ${scenario} discarded retired document Undo presentation`);
        } finally { harness.release(); }
      }
    });

    it("preserves the successor broker proof when an old inference owner responds", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      const harness = await inferenceHarness({ lease: "editor" });
      const epoch = testSeams.directorySessionEpoch;
      try {
        let release: () => void = () => undefined;
        harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
        harness.bodies.push("{}");
        const pending = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, admit: () => testSeams.directorySessionEpoch === epoch });
        const rejected = expect(pending).rejects.toThrow();
        for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(harness.requests).toHaveLength(1);
        testSeams.directorySessionEpoch += 1;
        clearLocalBrowserBrokerProof();
        expect(installLocalBrowserBrokerProof(fixture.successorProof)).toBe(true);
        release();
        await rejected;
        harness.bodies.push("{}");
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        expect(harness.proofs).toHaveLength(2);
        expect(harness.proofs[0]).toMatch(/^[0-9a-f]{64}$/u);
        expect(equal(harness.proofs[1], fixture.successorProof)).toBe(true);
        console.log("[DEBUG] old inference response preserved successor broker proof");
      } finally { testSeams.directorySessionEpoch = epoch; harness.release(); }
    });

    it("validates the exact canonical authenticated Directory client response", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
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
      console.log("[DEBUG] Directory client consumed only canonical authenticated session authority");
    });

    it("binds the verified broker session and retains uncertain inference across authenticated replacement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authorities = fixture.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
      const retained = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      const harness = await inferenceHarness({ lease: "editor" });
      const channel = new MessageChannel();
      testSeams.attachLocalBrokerPort(channel.port1);
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
        handleTsRequest({ kind: "inference-open", operationEpoch: retained.operationEpoch + 5, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: retained.operationEpoch + 5, requestId: retained.requestId });
        await settleInferenceTurns();
        const original = testSeams.inferencePort;
        expect(original?.request?.requestId).toBe(retained.requestId);
        expect((await me(authorities[1])).status).toBe(200);
        expect(testSeams.directorySessionEpoch).toBe(epoch + 1);
        expect(equal(testSeams.browserSessionAuthority, authorities[1])).toBe(true);
        expect(artifactState(DOCUMENT, SPACE)).toBeUndefined();
        expect(testSeams.inferencePort).toBe(original);
        expect(original.status.phase).toBe("indeterminate");
        expect(original.request.requestId).toBe(retained.requestId);
        await driveInferencePort(original.operationEpoch);
        expect(harness.requests.filter((entry: string) => entry.endsWith("/jobs"))).toHaveLength(1);
        expect(harness.requests.filter((entry: string) => entry.endsWith("/cancel") || entry.endsWith("/reconcile"))).toHaveLength(0);
        console.log("[DEBUG] authenticated broker replacement retained the original request without replay under its successor");
      } finally { testSeams.detachLocalBrokerPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
    });

    it("requires verified session authority to physically reopen a fresh document owner", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const authorityFixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authorities = authorityFixture.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const validate = new Ajv({ strict: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/GisMapInferencePortV1" });
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        const original = harness.state;
        const originalClientInstanceId = original.openClientInstanceId;
        harness.bodies.push(JSON.stringify(authorities[1]));
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
        const absentBeforeReopen = artifactState(DOCUMENT, SPACE) === undefined;
        const successorClientInstanceId = "12345678-1234-4123-8123-123456789abd";
        openArtifact({ documentId: DOCUMENT, schema: "gis.map", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: SPACE }], actor: "caller", clientInstanceId: successorClientInstanceId });
        const successor = artifactState(DOCUMENT, SPACE)!;
        successor.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, leaseFields(true), "http://hub.test", new Uint8Array(1), new Uint8Array(1));
        const operationEpoch = fixture.retainedClosing.operationEpoch + 11;
        handleTsRequest({ kind: "inference-open", operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
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
        handleTsRequest({ kind: "inference-close", operationEpoch });
        expect(testSeams.inferencePort).toBeNull();
        console.log("[DEBUG] authenticated replacement physically reopened one fresh worker document owner");
      } finally { harness.release(); }
    });

    it("correlates every Directory bootstrap page with the verified broker session authority", async () => {
      const { readFileSync } = await import("node:fs");
      const { createHash } = await import("node:crypto");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const retained = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      for (const scenario of retained.directoryPageBinding) {
      const harness = await inferenceHarness({ lease: "editor" });
        const channel = new MessageChannel();
        testSeams.attachLocalBrokerPort(channel.port1);
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
          console.log(`[DEBUG] Directory bootstrap ${scenario} matched the authenticated authority before presentation`);
        } finally { testSeams.detachLocalBrokerPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
      }
    });

    it("retires unverified session responses without letting old bodies overwrite replacement authority", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const value = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const retained = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      for (const scenario of retained.authorityRetirement) {
      const harness = await inferenceHarness({ lease: "editor" });
        const channel = new MessageChannel();
        testSeams.attachLocalBrokerPort(channel.port1);
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
            clearLocalBrowserBrokerProof();
            expect(installLocalBrowserBrokerProof(retained.successorProof)).toBe(true);
            release();
          }
          expect((await pending).status).toBe(428);
          expect(testSeams.browserSessionAuthority).toBeNull();
          expect(testSeams.directorySessionEpoch).toBe(epoch + 1);
          expect(artifactState(DOCUMENT, SPACE)).toBeUndefined();
          if (scenario === "old-body-after-proof-replacement") {
            harness.bodies.push("{}");
            await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
            expect(equal(harness.proofs.at(-1), retained.successorProof)).toBe(true);
          }
          console.log(`[DEBUG] session authority ${scenario} retired only its original owner`);
        } finally { release(); testSeams.detachLocalBrokerPort(); channel.port1.close(); channel.port2.close(); harness.release(); }
      }
    });

    it("retires an attached broker port before its old session body can authorize the successor", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = fixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const retained = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      const harness = await inferenceHarness({ lease: "editor" });
      const prior = new MessageChannel(), successor = new MessageChannel();
      const body = JSON.stringify(authority);
      let release: () => void = () => undefined;
      const gate = new Promise<void>((resolve) => { release = resolve; });
      let cancelled = false;
      const stream = new ReadableStream<Uint8Array>({ async start(controller) { await gate; if (!cancelled) { controller.enqueue(new TextEncoder().encode(body)); controller.close(); } }, cancel() { cancelled = true; } });
      const request = { kind: "request", operation: "me", requestId: crypto.randomUUID() };
      try {
        testSeams.attachLocalBrokerPort(prior.port1);
        (globalThis as unknown as { fetch: unknown }).fetch = async () => ({ ok: true, status: 200, statusText: "", headers: new Headers({ "x-semio-browser-broker-advanced": "1", "content-length": String(new TextEncoder().encode(body).length) }), body: stream });
        prior.port2.postMessage(request);
        for (let turn = 0; !stream.locked && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(stream.locked).toBe(true);
        testSeams.attachLocalBrokerPort(successor.port1);
        for (let turn = 0; !cancelled && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(cancelled).toBe(true);
        release();
        await settleInferenceTurns();
        expect(equal(testSeams.browserSessionAuthority, null), retained.portReplacement).toBe(true);
        expect(cancelled).toBe(true);
        expect(installLocalBrowserBrokerProof(retained.successorProof)).toBe(true);
        console.log("[DEBUG] broker port replacement cancelled old body ownership before successor authorization");
      } finally { release(); testSeams.detachLocalBrokerPort(); prior.port1.close(); prior.port2.close(); successor.port1.close(); successor.port2.close(); harness.release(); }
    });

    it("never dispatches an old queued request under a replacement broker admission", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      const harness = await inferenceHarness({ lease: "editor" });
      let release: () => void = () => undefined;
      try {
        harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
        harness.bodies.push("{}");
        const first = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        const firstRejected = expect(first).rejects.toThrow();
        for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(harness.requests).toHaveLength(1);
        const queued = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        const queuedOutcome = queued.then(() => "dispatched", () => fixture.brokerReplacement);
        clearLocalBrowserBrokerProof();
        expect(installLocalBrowserBrokerProof(fixture.successorProof)).toBe(true);
        release();
        await firstRejected;
        expect(await queuedOutcome).toBe(fixture.brokerReplacement);
        expect(equal(harness.requests.length, 1)).toBe(true);
        harness.bodies.push("{}");
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
        expect(harness.proofs).toHaveLength(2);
        expect(harness.proofs[0]).toMatch(/^[0-9a-f]{64}$/u);
        expect(equal(harness.proofs[1], fixture.successorProof)).toBe(true);
        console.log("[DEBUG] replacement broker admission denied old queued work and preserved its fresh proof");
      } finally { release(); harness.release(); }
    });

    it("does not spend the active broker proof on queued retired inference work", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", source.url), "utf8")).retainedClosing;
      for (const scenario of fixture.queuedRetirement) {
      const harness = await inferenceHarness({ lease: "editor" });
        const epoch = testSeams.directorySessionEpoch;
        try {
          const operationEpoch = fixture.operationEpoch + 3;
          handleTsRequest({ kind: "inference-open", operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
          harness.bodies.push(receiptBody("accepted", "none", 0));
          handleTsRequest({ kind: "inference-propose", operationEpoch, requestId: fixture.requestId });
          await settleInferenceTurns();
          let release: () => void = () => undefined;
          harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
          harness.bodies.push("{}");
          const blocker = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
          for (let turn = 0; harness.requests.length < 2 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
          expect(harness.requests).toHaveLength(2);
          let rejected: Promise<unknown> | undefined;
          if (scenario === "abort") {
            const abort = new AbortController();
            rejected = expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, signal: abort.signal })).rejects.toThrow();
            abort.abort();
          } else {
            handleTsRequest({ kind: "inference-cancel", operationEpoch });
            testSeams.directorySessionEpoch += 1;
          }
          release();
          await blocker;
          await rejected;
          await settleInferenceTurns();
          expect(equal(harness.requests.length, 2)).toBe(true);
          harness.bodies.push("{}");
          await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
          expect(harness.requests).toHaveLength(3);
          console.log(`[DEBUG] queued inference ${scenario} preserved the active broker proof`);
        } finally { testSeams.directorySessionEpoch = epoch; harness.release(); }
      }
    });

    it("records a Cancel click as requested and reaches cancelled only on the server's own answer", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 4, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 4, requestId: "4".repeat(32) });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 1, total: 4, cancelRequested: true }));
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 4 });
        const requested = harness.ports().at(-1)!;
        expect(requested.cancelRequested).toBe(true);
        expect(requested.phase).toBe("running");
        // 🛑️ A second Cancel click never sends a second cancel request.
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 4 });
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
        handleTsRequest({ kind: "inference-open", operationEpoch: 5, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 5, requestId: "5".repeat(32) });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("running", "none", { nextCursor: 1, completed: 1, total: 4, stale: true }));
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 5 });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("stale");
        const before = harness.ports().length;
        handleTsRequest({ kind: "inference-approve", operationEpoch: 5 });
        await settleInferenceTurns();
        expect(harness.ports().length).toBe(before);
      } finally {
        harness.release();
      }
    });

    it("approves exactly the offered hash once and applies only on a committed receipt", async () => {
      const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 6, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 6, requestId: "6".repeat(32) });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("offered");
        handleTsRequest({ kind: "inference-approve", operationEpoch: 6 });
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
        handleTsRequest({ kind: "inference-approve", operationEpoch: 6 });
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
          handleTsRequest({ kind: "inference-open", operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
          harness.statuses.push(200);
          harness.bodies.push(receiptBody("accepted", "none", 0));
          handleTsRequest({ kind: "inference-propose", operationEpoch, requestId: `${operationEpoch}`.repeat(32) });
          await settleInferenceTurns();
          harness.statuses.push(200);
          harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview }));
          await driveInferencePort(operationEpoch);
          expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.transport" });
          handleTsRequest({ kind: "inference-approve", operationEpoch });
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
        handleTsRequest({ kind: "inference-open", operationEpoch: 7, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(503);
        harness.bodies.push(JSON.stringify({ schema: "semio.hub.inference-error/v1", code: "inference.unavailable" }));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 7, requestId: "7".repeat(32) });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.unavailable" });
      } finally {
        harness.release();
      }
    });

    it("keeps explicit English and German text for every phase, code and control with no default language", async () => {
      const { GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, gisMapInferencePortRoleV1 } = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
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

  //#region 🔖️OfflineResilienceTests
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-backbone) findings 1/2/3/5: SSE-primary folder
  // watch with a suppressed sanity-poll fallback, reconnect after a post-open drop, abort-on-close,
  // and the bounded lossless mutation outbox. No real sleeps — `vi.useFakeTimers()` drives every
  // timer-dependent assertion, and every fetch/socket/stream is a controllable local fake.
  describe("backbone-worker offline resilience", () => {
    class FakeEventSource {
      static instances: FakeEventSource[] = [];
      readonly url: string;
      onopen: (() => void) | null = null;
      onmessage: ((event: unknown) => void) | null = null;
      onerror: (() => void) | null = null;
      closed = false;
      constructor(url: string) {
        this.url = url;
        FakeEventSource.instances.push(this);
      }
      close(): void {
        this.closed = true;
      }
    }

    class FakeHubWebSocket {
      static readonly CONNECTING = 0;
      static readonly OPEN = 1;
      static readonly CLOSING = 2;
      static readonly CLOSED = 3;
      static instances: FakeHubWebSocket[] = [];
      readonly url: string;
      readonly protocol = "semio.socket.v1";
      readonly protocols: string | string[] | undefined;
      readyState = FakeHubWebSocket.CONNECTING;
      binaryType = "blob";
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
      open(): void {
        this.readyState = FakeHubWebSocket.OPEN;
        this.onopen?.();
      }
      close(_code?: number, _reason?: string): void {
        this.readyState = FakeHubWebSocket.CLOSED;
        this.onclose?.();
      }
    }

    function folderOnlyConfig(documentId: string): ArtifactActorConfig {
      return { documentId, schema: "demo/v1", bindings: [{ kind: "folder", path: `/tmp/${documentId}` }], actor: "actor-1" };
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
          diff: { schema: envelope.diff.schemaId, payload: diffPayload },
          inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: encodePackValue(envelope.inverse.inverseDiff.payload) },
          timestamp,
        }]),
      });
    }

    function installVerifiedDocumentBackbonePair(state: ArtifactState): WireFrontierSummary {
      const frontier = { document_id: state.config.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0) };
      state.currentPack = new Uint8Array([1]);
      state.currentSpr = new Uint8Array([1]);
      state.frontier = frontier;
      state.verifiedColdPair = { assertCurrent() {}, drop() {} };
      return frontier;
    }

    type BrowserDocumentOpenFixture = {
      nowMs: number;
      intent: DocumentOpenIntentV1;
      installedTarget: NonNullable<Extract<PersistenceBinding, { kind: "hub" }>["installedTarget"]>;
      plan: DocumentOpenPlanV1;
      socketGrant: SocketGrantReceiptV1;
      expected: {
        httpPaths: [string, string];
        webSocketPath: string;
        protocol: string;
        helloSchema: string;
        helloPackSchemaHashByte: number;
        responseMaxBytes: number;
        rustWorkerBypassDenied: true;
        scopeIsolation: { left: { spaceId: string; documentId: string }; right: { spaceId: string; documentId: string }; leftKey: string; rightKey: string; localKey: string };
        forbiddenSocketFragments: string[];
      };
    };

    async function browserDocumentOpenFixture(): Promise<BrowserDocumentOpenFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("./🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", source.url), "utf8")) as BrowserDocumentOpenFixture;
    }

    function currentBrowserDocumentOpenFixture(fixture: BrowserDocumentOpenFixture): { plan: DocumentOpenPlanV1; grant: SocketGrantReceiptV1 } {
      const now = Date.now();
      return {
        plan: { ...structuredClone(fixture.plan), expiresAtUnixMs: now + 30_000 },
        grant: { ...fixture.socketGrant, expiresAtMs: now + 25_000 },
      };
    }

    async function waitForDocumentSocket(): Promise<FakeHubWebSocket> {
      for (let turn = 0; turn < 64; turn += 1) {
        const socket = FakeHubWebSocket.instances.at(-1);
        if (socket) return socket;
        await new Promise<void>((resolve) => setTimeout(resolve, 0));
      }
      throw new Error("browser document-open socket deadline exceeded");
    }

    function notFoundResponse() {
      return { ok: false, status: 404, statusText: "not found", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
    }

    it("document opening attempt retires A before B and makes stale send close and duplicate open inert", async () => {
      const fixture = await browserDocumentOpenFixture();
      const documentId = `${fixture.intent.scope.documentId}-attempt-owner`;
      const spaceId = fixture.intent.scope.spaceId;
      const attemptA = "11111111-1111-4111-8111-111111111111";
      const attemptB = "22222222-2222-4222-8222-222222222222";
      const requests: BackboneWorkerRequest[] = [];
      const dispatch = (request: BackboneWorkerRequest): void => dispatchBackboneWorkerRequest(request, null, (value) => requests.push(value));
      const open = (clientInstanceId: string): BackboneWorkerRequest => ({
        kind: "open",
        clientInstanceId,
        documentId,
        schema: fixture.plan.artifact.schema,
        bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId, installedTarget: fixture.installedTarget }],
        actor: "caller-selected-actor",
      });
      try {
        dispatch(open(attemptA));
        dispatch(open(attemptB));
        dispatch(open(attemptB));
        dispatch({ kind: "send", documentId, spaceId, clientInstanceId: attemptA, message: { kind: "externalChanged" } });
        dispatch({ kind: "close", documentId, spaceId, clientInstanceId: attemptA });
        dispatch({ kind: "send", documentId, spaceId, clientInstanceId: attemptB, message: { kind: "externalChanged" } });
        dispatch({ kind: "close", documentId, spaceId, clientInstanceId: attemptB });
        expect(requests.map((request) => [request.kind, "clientInstanceId" in request ? request.clientInstanceId : undefined])).toEqual([
          ["open", attemptA],
          ["close", attemptA],
          ["open", attemptB],
          ["send", attemptB],
          ["close", attemptB],
        ]);
        expect(documentExecutionOwners.has(documentRuntimeKeyV1({ kind: "hub", spaceId, documentId }))).toBe(false);
      } finally {
        documentExecutionOwners.delete(documentRuntimeKeyV1({ kind: "hub", spaceId, documentId }));
      }
    });

    it("document opening attempt remains D1-owned when the Rust worker resolves", async () => {
      const fixture = await browserDocumentOpenFixture();
      const documentId = `${fixture.intent.scope.documentId}-resolved-rust`;
      const clientInstanceId = "33333333-3333-4333-8333-333333333333";
      const typescriptRequests: BackboneWorkerRequest[] = [];
      const rustRequests: BackboneWorkerRequest[] = [];
      const host: RustWorkerHost = {
        handleRequestBytes: (bytes) => rustRequests.push(decodeBackboneWorkerRequest(bytes)),
        postReady: () => {},
      };
      const dispatch = (request: BackboneWorkerRequest): void => dispatchBackboneWorkerRequest(request, host, (value) => typescriptRequests.push(value));
      dispatch({
        kind: "open",
        clientInstanceId,
        documentId,
        schema: fixture.plan.artifact.schema,
        bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
        actor: "caller-selected-actor",
      });
      dispatch({ kind: "send", documentId, spaceId: fixture.intent.scope.spaceId, clientInstanceId, message: { kind: "detach" } });
      dispatch({ kind: "close", documentId, spaceId: fixture.intent.scope.spaceId, clientInstanceId });
      expect(typescriptRequests.map(({ kind }) => kind)).toEqual(["open", "send", "close"]);
      expect(rustRequests).toHaveLength(0);
    });

    it("browser document open runtime ownership isolates the same document id across two hub spaces", async () => {
      const fixture = await browserDocumentOpenFixture();
      const originalWebSocket = globalThis.WebSocket;
      const current = currentBrowserDocumentOpenFixture(fixture);
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      testSeams.socketGrantTestIssue = async () => current.grant;
      const left = fixture.expected.scopeIsolation.left;
      const right = fixture.expected.scopeIsolation.right;
      try {
        openArtifact({
          documentId: left.documentId,
          schema: fixture.installedTarget.artifact.schema,
          bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: left.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        openArtifact({
          documentId: right.documentId,
          schema: fixture.installedTarget.artifact.schema,
          bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: right.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        expect(documentRuntimeKeyV1({ kind: "hub", ...left })).toBe(fixture.expected.scopeIsolation.leftKey);
        expect(documentRuntimeKeyV1({ kind: "hub", ...right })).toBe(fixture.expected.scopeIsolation.rightKey);
        expect(documentRuntimeKeyV1({ kind: "local", documentId: left.documentId })).toBe(fixture.expected.scopeIsolation.localKey);
        expect(fixture.expected.scopeIsolation.leftKey).not.toBe(fixture.expected.scopeIsolation.rightKey);
        expect(fixture.expected.scopeIsolation.localKey).not.toBe(fixture.expected.scopeIsolation.leftKey);
        expect(artifacts.has(fixture.expected.scopeIsolation.leftKey)).toBe(true);
        expect(artifacts.has(fixture.expected.scopeIsolation.rightKey)).toBe(true);
        expect(artifacts.get(fixture.expected.scopeIsolation.leftKey)!.channel.name).toBe(`semio-doc-${fixture.expected.scopeIsolation.leftKey}`);
        expect(artifacts.get(fixture.expected.scopeIsolation.rightKey)!.channel.name).toBe(`semio-doc-${fixture.expected.scopeIsolation.rightKey}`);
        expect(artifactState(left.documentId)).toBeUndefined();
        closeArtifact(left.documentId, left.spaceId);
        expect(artifacts.has(fixture.expected.scopeIsolation.leftKey)).toBe(false);
        expect(artifacts.has(fixture.expected.scopeIsolation.rightKey)).toBe(true);
      } finally {
        closeArtifact(left.documentId, left.spaceId);
        closeArtifact(right.documentId, right.spaceId);
        testSeams.socketGrantTestIssue = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open requires exact installed package artifact and surface authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const config: ArtifactActorConfig = {
        documentId: fixture.intent.scope.documentId,
        schema: fixture.installedTarget.artifact.schema,
        bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
        actor: "caller-selected-actor",
        packSchemaHash: new Array(32).fill(fixture.expected.helloPackSchemaHashByte),
      };
      expect(documentOpenPlanAuthority(fixture.plan, fixture.intent, config, fixture.installedTarget).surfaceId).toBe(fixture.installedTarget.surface.surfaceId);
      const replacements: readonly [string, string, unknown][] = [
        ["package", "pluginId", "s.foreign"],
        ["package", "packageId", "s.foreign.codec"],
        ["package", "version", "9.9.9"],
        ["package", "componentSha256", "a".repeat(64)],
        ["package", "componentBlake3", "a".repeat(64)],
        ["package", "descriptorByteSha256", "a".repeat(64)],
        ["artifact", "kind", "s.foreign"],
        ["artifact", "packSchemaHash", "a".repeat(64)],
        ["parentDialect", "artifactKind", "s.foreign"],
        ["parentDialect", "standard", "2"],
        ["parentDialect", "subset", "preview"],
        ["surface", "appId", "app.foreign"],
        ["surface", "windowKindId", "window.foreign"],
        ["surface", "role", "viewer"],
        ["surface", "rendererTarget", "wgpu"],
      ];
      for (const [section, field, value] of replacements) {
        const candidate = structuredClone(fixture.plan) as unknown as Record<string, Record<string, unknown>>;
        candidate[section]![field] = value;
        expect(() => documentOpenPlanAuthority(candidate as unknown as DocumentOpenPlanV1, fixture.intent, config, fixture.installedTarget)).toThrow("document open: authority mismatch");
      }
    });

    it("browser document open uses the authenticated D1 plan and receipt exchange before its credential-free socket", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      const originalWebSocket = globalThis.WebSocket;
      const requests: { url: string; headers: Headers; body: string }[] = [];
      FakeHubWebSocket.instances = [];
      testSeams.socketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: RequestInit) => {
        const url = String(input);
        const body = String(init?.body ?? "");
        requests.push({ url, headers: new Headers(init?.headers), body });
        const response = requests.length === 1 ? current.plan : current.grant;
        return Response.json(response, { headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        openArtifact({
          documentId: fixture.intent.scope.documentId,
          schema: fixture.plan.artifact.schema,
          bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        const socket = await waitForDocumentSocket();
        expect(requests).toHaveLength(2);
        expect(requests.map(({ url }) => url)).toEqual(fixture.expected.httpPaths.map((path) => `/_semio/hub${path}`));
        const intent = JSON.parse(requests[0]!.body) as Record<string, unknown>;
        expect(Object.keys(intent).sort()).toEqual(["clientInstanceId", "requestedSurfaceId", "schema", "scope", "version"]);
        expect(intent.scope).toEqual(fixture.intent.scope);
        expect(intent.requestedSurfaceId).toBe(fixture.intent.requestedSurfaceId);
        expect(intent.clientInstanceId).toMatch(/^[0-9a-f-]{36}$/u);
        expect(requests[0]!.body).not.toContain(current.plan.receipt);
        expect(JSON.parse(requests[1]!.body)).toEqual({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: current.plan.receipt });
        expect(requests[0]!.headers.get("x-semio-browser-broker")).not.toBe(requests[1]!.headers.get("x-semio-browser-broker"));
        expect(requests.every(({ headers }) => /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker") ?? "") && /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker-next") ?? ""))).toBe(true);
        expect(socket.url).toBe(`ws://hub.test${fixture.expected.webSocketPath}`);
        expect(socket.protocols).toEqual([fixture.expected.protocol, current.grant.grant]);
        for (const forbidden of fixture.expected.forbiddenSocketFragments) expect(socket.url).not.toContain(forbidden);
        socket.open();
        expect(socket.sent).toHaveLength(1);
        const hello = decodeClientFrame(socket.sent[0]!).frame;
        if (typeof hello === "string" || !("SocketHelloV1" in hello)) throw new Error("expected SocketHelloV1");
        expect(hello.SocketHelloV1.schema).toBe(fixture.expected.helloSchema);
        expect(hello.SocketHelloV1.pack_schema_hash).toEqual(new Array(32).fill(fixture.expected.helloPackSchemaHashByte));
        expect(JSON.stringify(hello)).not.toContain("open.v1.");
        expect(JSON.stringify(hello)).not.toContain("socket.v1.");
        const state = artifactState(fixture.intent.scope.documentId, fixture.intent.scope.spaceId)!;
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBe(current.grant.actorId);
        await handleHubFrame(state, { Session: { actor: current.grant.actorId, color: 7 } });
        expect(state.hubActorReady).toBe(true);
        expect(state.actor).toBe(current.grant.actorId);
        expect(state.pendingSocketActorId).toBeNull();
      } finally {
        closeArtifact(fixture.intent.scope.documentId, fixture.intent.scope.spaceId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open withholds activation until an exact authenticated Session and terminally clears a mismatched actor", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      const originalWebSocket = globalThis.WebSocket;
      FakeHubWebSocket.instances = [];
      testSeams.socketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      let effects = 0;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        effects += 1;
        return Response.json(effects === 1 ? current.plan : current.grant, { headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        openArtifact({
          documentId: fixture.intent.scope.documentId,
          schema: fixture.plan.artifact.schema,
          bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        const socket = await waitForDocumentSocket();
        socket.open();
        const state = artifactState(fixture.intent.scope.documentId, fixture.intent.scope.spaceId)!;
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBe(current.grant.actorId);
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"f".repeat(64)}`, color: 9 } });
        expect(socket.readyState).toBe(FakeHubWebSocket.CLOSED);
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBeNull();
      } finally {
        closeArtifact(fixture.intent.scope.documentId, fixture.intent.scope.spaceId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    type ExecutionTargetLeaseFixture = {
      nowMs: number;
      hubOrigin: string;
      intent: DocumentOpenIntentV1;
      plan: DocumentOpenPlanV1;
      manifest: DocumentExecutionTargetLeaseFieldsV1;
      socketGrant: SocketGrantReceiptV1;
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

    async function executionTargetLeaseFixture(): Promise<ExecutionTargetLeaseFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8")) as ExecutionTargetLeaseFixture;
    }

    function executionTargetBytes(hexText: string): Uint8Array {
      return Uint8Array.from({ length: hexText.length / 2 }, (_unused, index) => Number.parseInt(hexText.slice(index * 2, index * 2 + 2), 16));
    }

    function executionTargetMutate(source: unknown, path: string, value: unknown): Record<string, unknown> {
      const candidate = structuredClone(source) as Record<string, unknown>;
      const segments = path.split(".");
      let cursor = candidate as Record<string, unknown>;
      for (const segment of segments.slice(0, -1)) cursor = cursor[segment] as Record<string, unknown>;
      cursor[segments.at(-1)!] = value;
      return candidate;
    }

    function executionTargetBodyResponse(bytes: Uint8Array, declaredLength = bytes.byteLength): Response {
      return new Response(ownedArrayBuffer(bytes), { headers: { "content-length": String(declaredLength), "x-semio-browser-broker-advanced": "1" } });
    }

    type ExecutionTargetHarness = {
      state: ArtifactState;
      binding: Extract<PersistenceBinding, { kind: "hub" }>;
      requests: { url: string; method: string; body: string }[];
      statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[];
      release: () => void;
    };

    async function acceptCurrentTestBrowserSessionAuthority(proof: string): Promise<void> {
      const authorityFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = authorityFixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const body = JSON.stringify(authority);
      const priorFetch = globalThis.fetch;
      clearLocalBrowserBrokerProof();
      expect(installLocalBrowserBrokerProof(proof)).toBe(true);
      globalThis.fetch = async () => new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength), "x-semio-browser-broker-advanced": "1" } });
      try {
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000, accept: testSeams.acceptBrowserSessionAuthority });
      } finally {
        globalThis.fetch = priorFetch;
      }
    }

    async function executionTargetHarness(fixture: ExecutionTargetLeaseFixture, respond: (url: string, requests: { url: string; method: string; body: string }[]) => Response | Promise<Response>): Promise<ExecutionTargetHarness> {
      const requests: { url: string; method: string; body: string }[] = [];
      const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
      const originalFetch = globalThis.fetch;
      testSeams.socketGrantTestIssue = null;
      testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
      await acceptCurrentTestBrowserSessionAuthority("a".repeat(64));
      openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor" });
      const state = artifactState(fixture.intent.scope.documentId)!;
      state.openClientInstanceId = fixture.intent.clientInstanceId;
      // 🪪️ The caller declares only which surface it wants: nothing forgeable is supplied, so the
      // verified lease is the sole local comparison input for this wasm target.
      const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: fixture.hubOrigin, spaceId: fixture.intent.scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId };
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
          clearLocalBrowserBrokerProof();
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        },
      };
    }

    it("browser document first open verifies server assets without a prior installed target", async () => {
      const fixture = await executionTargetLeaseFixture();
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧬️schema/🔣️.json", source.url), "utf8")) as { $id: string };
      const Ajv = (await import("ajv")).default;
      const validators = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note").addSchema(schema);
      const validateScope = validators.getSchema(`${schema.$id}#/$defs/DocumentOpeningScopeResolutionV1`)!;
      const validateFirstOpen = validators.getSchema(`${schema.$id}#/$defs/DocumentFirstOpenV1`)!;
      expect(corpus.cases.every((row: unknown) => validateScope(row)) && validateFirstOpen(corpus.firstOpen), JSON.stringify([...validateScope.errors ?? [], ...validateFirstOpen.errors ?? []])).toBe(true);
      const originalFetch = globalThis.fetch,
        originalSocket = globalThis.WebSocket,
        originalPost = testSeams.workerPostTestSink;
      const requests: { stage: string; body: Record<string, unknown> }[] = [];
      const posted: BackboneWorkerResponse[] = [];
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...structuredClone(fixture.socketGrant), expiresAtMs: Date.now() + 25_000 };
      const scope = fixture.intent.scope;
      const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...scope });
      testSeams.socketGrantTestIssue = null;
      FakeHubWebSocket.instances = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      globalThis.fetch = async (input, init) => {
        const stage = String(input).split("/").at(-1)!;
        const body = JSON.parse(String(init?.body));
        requests.push({ stage, body });
        expect(init?.method).toBe("POST");
        if (stage === "open-plan") return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (stage === "manifest") return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (stage === "component") return executionTargetBodyResponse(executionTargetBytes(fixture.componentHex));
        if (stage === "descriptor") return executionTargetBodyResponse(executionTargetBytes(fixture.descriptorHex));
        expect(stage).toBe("socket-grants");
        return Response.json(grant, { headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        handleTsRequest({
          kind: "open",
          documentId: scope.documentId,
          schema: plan.artifact.schema,
          actor: "caller-is-not-authority",
          bindings: [{ kind: "hub", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId }],
        });
        const deadline = Date.now() + 5_000;
        while (FakeHubWebSocket.instances.length === 0 && !posted.some((message) => message.kind === "socket-actor-failed") && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(requests.map(({ stage }) => stage)).toEqual(corpus.firstOpen.requestStages);
        expect(FakeHubWebSocket.instances.length).toBe(corpus.firstOpen.socketCount);
        const state = artifacts.get(runtimeKey)!;
        expect(hubBinding(state.config)!.installedTarget).toBeUndefined();
        expect(state.executionTargetLease?.live).toBe(true);
        expect(sameLeaseFieldsV1(state.executionTargetLease!.fields(), fixture.manifest)).toBe(true);
        const intent = { ...fixture.intent, clientInstanceId: state.openClientInstanceId };
        for (const request of requests.slice(0, 4)) expect(request.body).toEqual(intent);
        expect(requests.at(-1)!.body).toEqual({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
        expect(state.actor).toBe("");
        expect(state.hubActorReady).toBe(false);
        expect(state.pendingSocketActorId).toBe(grant.actorId);
        expect(state.outbox).toHaveLength(0);
        const socket = FakeHubWebSocket.instances[0]!;
        socket.open();
        expect(socket.sent).toHaveLength(1);
        expect(decodeClientFrame(socket.sent[0]!).frame).toHaveProperty("SocketHelloV1.schema", plan.artifact.schema);
        const localId = "local-first-opening";
        handleTsRequest({ kind: "open", documentId: localId, schema: plan.artifact.schema, actor: "local", bindings: [] });
        expect(posted.filter((message) => message.kind === "socket-actor-failed" && message.documentId === localId)).toHaveLength(corpus.firstOpen.localSocketFailures);
        closeArtifact(localId);
        const unselectedId = "unselected-first-opening";
        handleTsRequest({ kind: "open", documentId: unselectedId, schema: plan.artifact.schema, actor: "untrusted", bindings: [{ kind: "hub", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId }] });
        expect(posted.filter((message) => message.kind === "socket-actor-failed" && message.documentId === unselectedId)).toHaveLength(corpus.firstOpen.unselectedSocketFailures);
        closeArtifact(unselectedId, scope.spaceId);
        expect(requests.map(({ stage }) => stage)).toEqual(corpus.firstOpen.requestStages);
        console.log("[DEBUG] document-first-open requested-surface-only=1 verified-assets=3 socket=1 hello=1 authenticated-session=0 local-failures=0 unselected-refusal=1 writes=0");
      } finally {
        closeArtifactRuntime(runtimeKey);
        closeArtifact("local-first-opening");
        closeArtifact("unselected-first-opening", scope.spaceId);
        clearLocalBrowserBrokerProof();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
        testSeams.workerPostTestSink = originalPost;
      }
    });

    it("browser document first open rejects hostile assets and retired owners before socket authority", async () => {
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope/🔣️.json", source.url), "utf8"));
      const originalFetch = globalThis.fetch,
        originalSocket = globalThis.WebSocket,
        originalPost = testSeams.workerPostTestSink;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      testSeams.socketGrantTestIssue = null;
      try {
        for (const row of corpus.firstOpen.hostile) {
          const fixture = await executionTargetLeaseFixture();
          const scope = fixture.intent.scope;
          const runtimeKey = documentRuntimeKeyV1({ kind: "hub", ...scope });
          const stages: string[] = [];
          const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
          const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
          if (row.id === "foreign-plan-scope") plan.scope = { ...plan.scope, spaceId: "foreign-space" };
          FakeHubWebSocket.instances = [];
          clearLocalBrowserBrokerProof();
          installLocalBrowserBrokerProof("a".repeat(64));
          testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
          testSeams.workerPostTestSink = () => {};
          globalThis.fetch = async (input) => {
            const stage = String(input).split("/").at(-1)!;
            stages.push(stage);
            if (stage === "open-plan") return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
            if (stage === "manifest") {
              if (row.id === "cancel-at-manifest") artifacts.get(runtimeKey)!.docAbort.abort();
              return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
            }
            if (stage === "component") {
              const bytes = executionTargetBytes(fixture.componentHex);
              if (row.id === "corrupt-component") bytes[0] = bytes[0]! ^ 255;
              if (row.id === "retire-at-component") closeArtifactRuntime(runtimeKey);
              return executionTargetBodyResponse(bytes);
            }
            expect(stage).toBe("descriptor");
            return executionTargetBodyResponse(executionTargetBytes(fixture.descriptorHex));
          };
          try {
            handleTsRequest({
              kind: "open",
              documentId: scope.documentId,
              schema: fixture.plan.artifact.schema,
              actor: "untrusted",
              bindings: [{ kind: "hub", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId }],
            });
            const state = artifacts.get(runtimeKey);
            const deadline = Date.now() + 5_000;
            while (!statuses.some(({ code }) => code === "integrity-failed" || code === "cancelled") && !state?.docAbort.signal.aborted && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 0));
            expect(stages, row.id).toEqual(row.requestStages);
            expect(FakeHubWebSocket.instances, row.id).toHaveLength(0);
            expect(state?.executionTargetLease ?? null, row.id).toBeNull();
            expect(state?.outbox ?? [], row.id).toHaveLength(0);
            expect(stages.includes("socket-grants"), row.id).toBe(false);
            expect(JSON.stringify(statuses)).not.toContain(plan.receipt);
            console.log("[DEBUG] document-first-open-hostile", row.id, "requests=" + stages.length, "socket=0 grant=0 writes=0");
          } finally {
            closeArtifactRuntime(runtimeKey);
            await new Promise((resolve) => setTimeout(resolve, 0));
            testSeams.executionTargetStatusObserver = null;
            clearLocalBrowserBrokerProof();
          }
        }
      } finally {
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
        testSeams.workerPostTestSink = originalPost;
      }
    });

    it("browser execution target lease verifies GIS wasm bytes before plan exchange", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json(grant, { headers: { "x-semio-browser-broker-advanced": "1" } });
      });
      try {
        expect(fixture.plan.surface.rendererTarget).toBe("wasm");
        expect(fixture.manifest.grant.write).toBe(false);
        const authority = await requestDocumentSocketAuthority(harness.state, harness.binding);
        expect(authority.surfaceId).toBe(fixture.manifest.surface.surfaceId);
        expect(harness.requests.map(({ url }) => url)).toEqual([`/_semio/hub${fixture.expected.openPlanPath}`, ...fixture.expected.assetPaths.map((path) => `/_semio/hub${path}`), `/_semio/hub${fixture.expected.socketGrantPath}`]);
        for (const index of [1, 2, 3]) {
          expect(harness.requests[index]!.method).toBe("POST");
          expect(JSON.parse(harness.requests[index]!.body)).toEqual(fixture.intent);
          expect(harness.requests[index]!.body).not.toContain(plan.receipt);
        }
        expect(JSON.parse(harness.requests.at(-1)!.body).planReceipt).toBe(plan.receipt);
        expect(harness.state.executionTargetLease).not.toBeNull();
        expect(harness.state.executionTargetLease!.live).toBe(true);
        expect(sameLeaseFieldsV1(harness.state.executionTargetLease!.fields(), fixture.manifest)).toBe(true);
        const observed = harness.statuses.map(({ code, progress }) => `${code}:${progress?.stage ?? "-"}`);
        expect(observed.at(-1)).toBe(`${fixture.expected.rendererState}:-`);
        expect(new Set(harness.statuses.flatMap(({ progress }) => (progress ? [progress.stage] : [])))).toEqual(new Set(fixture.expected.progressStages));
        const payload = JSON.stringify(harness.statuses);
        for (const fragment of fixture.expected.forbiddenStatusFragments) expect(payload).not.toContain(fragment);
      } finally {
        harness.release();
      }
    });

    it("browser execution target lease rejects every single-field substitution without publication", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      FakeHubWebSocket.instances = [];
      const rejected: string[] = [];
      for (const vector of fixture.hostile) {
        let plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
        if (vector.kind === "stale-plan" || vector.kind === "mixed-generation") plan.catalog = { generationId: fixture.expected.rotation.generationA };
        let manifest =
          vector.kind === "manifest-field"
            ? executionTargetMutate(fixture.manifest, vector.path!, vector.value)
            : vector.kind === "mixed-generation"
              ? (structuredClone(fixture.manifest) as unknown as Record<string, unknown>)
              : (structuredClone(fixture.manifest) as unknown as Record<string, unknown>);
        let servedDescriptor = descriptor;
        if (vector.kind === "descriptor-protocol") {
          servedDescriptor = encodePackValue({ ...(decodePackValue(descriptor) as Record<string, unknown>), executionProtocol: { appChannelVersion: vector.value } });
          const descriptorSha256 = await executionTargetSha256Hex(servedDescriptor);
          if (plan.browserActor.kind !== "closed-browser-actor") throw new Error("execution target fixture lost its closed browser actor");
          plan = {
            ...plan,
            package: { ...plan.package, descriptorByteSha256: descriptorSha256 },
            browserActor: { ...plan.browserActor, sourceDescriptorByteSha256: descriptorSha256 },
          };
          manifest = {
            ...manifest,
            package: { ...(manifest.package as Record<string, unknown>), descriptorByteSha256: descriptorSha256 },
            descriptor: { ...(manifest.descriptor as Record<string, unknown>), sha256: descriptorSha256, byteLength: servedDescriptor.byteLength },
            browserActor: { ...(manifest.browserActor as Record<string, unknown>), sourceDescriptorByteSha256: descriptorSha256 },
          };
        }
        const harness = await executionTargetHarness(fixture, (url, requests) => {
          if (url.endsWith("/open-plan")) return Response.json(vector.kind === "stale-plan" ? { ...plan, catalog: { generationId: fixture.expected.rotation.generationB } } : plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
          if (url.endsWith("/execution-target/manifest")) {
            if (vector.kind === "cancel" && vector.stage === "manifest") requests.length > 0 && harnessAbort();
            return Response.json(manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
          }
          if (url.endsWith("/execution-target/component")) {
            if (vector.kind === "cancel" && (vector.stage === "component" || vector.stage === "verify")) harnessAbort();
            if (vector.kind === "missing-body" && vector.stage === "component") return new Response(null, { status: 204, headers: { "x-semio-browser-broker-advanced": "1" } });
            if (vector.kind === "component-max-plus-one") return executionTargetBodyResponse(component, fixture.expected.componentMaxBytes + 1);
            if (vector.kind === "component-truncated") return executionTargetBodyResponse(component.subarray(0, component.length - 1), component.length);
            if (vector.kind === "component-extra-byte") return executionTargetBodyResponse(new Uint8Array([...component, 7]), component.length);
            if (vector.kind === "component-bytes" || vector.kind === "mixed-generation") {
              const substituted = new Uint8Array(component);
              substituted[0] = substituted[0]! ^ 0xff;
              return executionTargetBodyResponse(substituted);
            }
            if (vector.kind === "deadline") return new Promise<Response>(() => undefined);
            return executionTargetBodyResponse(component);
          }
          if (url.endsWith("/execution-target/descriptor")) {
            if (vector.kind === "cancel" && vector.stage === "descriptor") harnessAbort();
            if (vector.kind === "missing-body" && vector.stage === "descriptor") return new Response(null, { status: 204, headers: { "x-semio-browser-broker-advanced": "1" } });
            if (vector.kind === "descriptor-max-plus-one") return executionTargetBodyResponse(descriptor, fixture.expected.descriptorMaxBytes + 1);
            if (vector.kind === "descriptor-trailing-byte") return executionTargetBodyResponse(new Uint8Array([...descriptor, 0]));
            if (vector.kind === "descriptor-noncanonical") return executionTargetBodyResponse(encodePackValue({ descriptorVersion: 1, packageId: "semio:gis" }));
            if (vector.kind === "descriptor-self-hash")
              return executionTargetBodyResponse(encodePackValue({ ...(decodePackValue(descriptor) as Record<string, unknown>), hashes: { wasmSha256: "a".repeat(64), coreWasmSha256: "9".repeat(64), descriptorSha256: "8".repeat(64) } }));
            if (vector.kind === "descriptor-bytes") {
              const substituted = new Uint8Array(descriptor);
              substituted[substituted.length - 1] = substituted[substituted.length - 1]! ^ 0xff;
              return executionTargetBodyResponse(substituted);
            }
            return executionTargetBodyResponse(servedDescriptor);
          }
          return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 }, { headers: { "x-semio-browser-broker-advanced": "1" } });
        });
        function harnessAbort(): void {
          harness.state.docAbort.abort();
        }
        try {
          if (vector.kind === "caller-url" || vector.kind === "caller-path" || vector.kind === "caller-module") {
            const denied = await browserExecutionTargetAssetRequest(harness.binding, `${harness.state.config.documentId}/../${String(vector.value)}`, "component", fixture.intent, { timeoutMs: 1_000, signal: harness.state.docAbort.signal }).catch(
              (error: unknown) => error as Error,
            );
            expect((denied as Error).message).toBe("document execution target: operation denied");
            expect(harness.requests).toHaveLength(0);
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "viewer-write") {
            await requestDocumentSocketAuthority(harness.state, harness.binding);
            expect(harness.state.executionTargetLease!.fields().grant.write).toBe(false);
            harness.state.hubActorReady = true;
            relayMutationsToHub(harness.state, [sampleEnvelope()]);
            expect(harness.state.outbox).toHaveLength(0);
            expect(harness.state.pendingBatches.size).toBe(0);
            expect(FakeHubWebSocket.instances).toHaveLength(0);
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "reconnect-after-invalidation") {
            const authority = await requestDocumentSocketAuthority(harness.state, harness.binding).catch((error: unknown) => error as Error);
            expect(authority).not.toBeInstanceOf(Error);
            harness.state.executionTargetLease!.drop();
            expect(() => harness.state.executionTargetLease!.fields()).toThrow("document execution target lease: dropped");
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "deadline") harness.state.docAbort.abort();
          const outcome = await requestDocumentSocketAuthority(harness.state, harness.binding).catch((error: unknown) => error as Error);
          expect(outcome).toBeInstanceOf(Error);
          expect(harness.state.executionTargetLease).toBeNull();
          expect(harness.requests.some(({ url }) => url.endsWith("/socket-grants"))).toBe(false);
          expect(FakeHubWebSocket.instances).toHaveLength(0);
          expect(vector.expected).toBe("unpublished");
          rejected.push(vector.name);
        } finally {
          harness.release();
        }
      }
      expect(rejected).toHaveLength(fixture.hostile.length);
      expect(new Set(rejected).size).toBe(fixture.hostile.length);
    });

    it("execution target body reader bounds cancellation, retirement and late publication", async () => {
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json", source.url), "utf8"));
      for (const row of corpus.cases) {
        const abort = new AbortController(),
          chunk = new Uint8Array([7, 8, 9]);
        let current = true,
          source: ReadableStreamDefaultController<Uint8Array> | undefined,
          cancelCalls = 0;
        let pending!: () => void;
        const pulled = new Promise<void>((resolve) => {
          pending = resolve;
        });
        const body = new ReadableStream<Uint8Array>({
          start(controller) {
            source = controller;
            controller.enqueue(chunk);
            if (["success", "short", "oversize"].includes(row.name)) controller.close();
          },
          pull(controller) {
            pending();
            if (row.name === "stale-after-read") {
              current = false;
              controller.close();
            }
            if (row.name === "abort-at-eof") {
              abort.abort();
              try {
                controller.close();
              } catch {}
            }
          },
          cancel() {
            cancelCalls++;
            return new Promise<void>(() => {});
          },
        });
        const length = row.name === "short" ? 4 : row.name === "oversize" ? 2 : 3;
        const response = new Response(body, { headers: { "content-length": String(length) } });
        const control = {
          signal: abort.signal,
          deadlineAtMs: Date.now() + corpus.deadlineMs,
          assertCurrent() {
            if (!current) throw new Error("fixture stale body owner");
          },
        };
        const reading = readExecutionTargetBody(response, length, 8, "component", control, () => {}).then(
          (bytes) => {
            const result = { kind: "fulfilled", bytes };
            return result;
          },
          () => ({ kind: "rejected", bytes: null }),
        );
        let timer: ReturnType<typeof setTimeout> | undefined;
        try {
          if (row.name === "pending-abort") {
            await pulled;
            abort.abort();
          }
          const result = await Promise.race([
            reading,
            new Promise<{ kind: string; bytes: null }>((resolve) => {
              timer = setTimeout(() => resolve({ kind: "stranded", bytes: null }), corpus.completionMs);
            }),
          ]);
          expect({ name: row.name, outcome: result.kind, locked: body.locked, chunkWiped: chunk.every((byte) => byte === 0) }).toEqual(row);
          if (result.bytes) {
            expect(Array.from(result.bytes)).toEqual([7, 8, 9]);
            result.bytes.fill(0);
          }
          expect(cancelCalls).toBeLessThanOrEqual(1);
        } finally {
          clearTimeout(timer);
          abort.abort();
          try {
            source?.close();
          } catch {}
          await reading;
        }
      }
    });

    it("execution target body reader prevents stale owners publishing a lease or grant", async () => {
      const fixture = await executionTargetLeaseFixture();
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json", source.url), "utf8"));
      const component = executionTargetBytes(fixture.componentHex),
        descriptor = executionTargetBytes(fixture.descriptorHex);
      for (const row of corpus.ownership) {
        const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
        const grant = { ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 };
        const originalDigest = crypto.subtle.digest;
        let changed = false,
          pendingBody: ReadableStream<Uint8Array> | undefined,
          pendingChunk: Uint8Array | undefined;
        const mutate = (): void => {
          if (changed) return;
          changed = true;
          const state = harness.state,
            binding = harness.binding;
          if (row.name.endsWith("-abort")) state.docAbort.abort();
          else if (row.name === "grant-eof-stale") artifacts.delete(state.runtimeKey);
          else if (row.name === "superseded-open") state.executionTargetOpen = Symbol("replacement-open");
          else if (row.name === "component-client") state.openClientInstanceId = "replaced-client";
          else if (row.name === "component-schema") state.config = { ...state.config, schema: "replaced-schema" };
          else {
            const changedBinding = { ...binding };
            if (row.name === "component-scope") changedBinding.spaceId = "replaced-space";
            else if (row.name === "component-origin") changedBinding.baseUrl = "https://replaced.test";
            else changedBinding.requestedSurfaceId = "replaced-surface";
            state.config = { ...state.config, bindings: [changedBinding] };
          }
        };
        const harness = await executionTargetHarness(fixture, (url) => {
          const headers: Record<string, string> = { "x-semio-browser-broker-advanced": "1" };
          const pendingStage = row.name.replace("-pending-abort", "");
          if (row.name.includes("-pending-abort") && url.endsWith("/" + pendingStage)) {
            pendingChunk =
              pendingStage === "component"
                ? new Uint8Array(component)
                : pendingStage === "descriptor"
                  ? new Uint8Array(descriptor)
                  : new TextEncoder().encode(JSON.stringify(pendingStage === "open-plan" ? plan : pendingStage === "manifest" ? fixture.manifest : grant));
            if (pendingStage === "component" || pendingStage === "descriptor") headers["content-length"] = String(pendingChunk.byteLength);
            pendingBody = new ReadableStream<Uint8Array>({
              start(controller) {
                controller.enqueue(pendingChunk!);
              },
              pull() {
                mutate();
              },
              cancel() {
                return new Promise<void>(() => {});
              },
            });
            return new Response(pendingBody, { headers });
          }
          if (url.endsWith("/open-plan")) return Response.json(plan, { headers });
          if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest, { headers });
          if (url.endsWith("/execution-target/component")) {
            if (row.name.startsWith("component-") || row.name === "superseded-open") mutate();
            return executionTargetBodyResponse(component);
          }
          if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
          if (row.name.startsWith("grant-eof-"))
            return new Response(
              new ReadableStream<Uint8Array>({
                start(controller) {
                  controller.enqueue(new TextEncoder().encode(JSON.stringify(grant)));
                },
                pull(controller) {
                  mutate();
                  controller.close();
                },
              }),
              { headers },
            );
          return Response.json(grant, { headers });
        });
        if (row.name === "status-owner-change")
          testSeams.executionTargetStatusObserver = (status) => {
            if (status.code === "renderer-unavailable") mutate();
          };
        crypto.subtle.digest = async function (algorithm, bytes) {
          const result = await originalDigest.call(this, algorithm, bytes);
          if (row.name.startsWith("hash-") && bytes.byteLength === component.byteLength) mutate();
          return result;
        };
        try {
          const result = await requestDocumentSocketAuthority(harness.state, harness.binding).then(
            () => "fulfilled",
            () => "rejected",
          );
          expect({ name: row.name, outcome: result, lease: harness.state.executionTargetLease !== null, reservation: harness.state.browserActorReservation !== null }).toEqual(row);
          expect(changed).toBe(true);
          if (pendingBody) {
            expect(pendingBody.locked).toBe(false);
            expect(pendingChunk!.every((byte) => byte === 0)).toBe(true);
          }
        } finally {
          crypto.subtle.digest = originalDigest;
          dropDocumentExecutionTargetLease(harness.state);
          closeArtifactRuntime(harness.state.runtimeKey);
          harness.release();
        }
      }
    });

    it("browser document actor reservation obeys private grant, scope, generation and retirement laws", async () => {
      const fixture = await executionTargetLeaseFixture();
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧫️fixtures/📇️directory/🧵️browser-actor-reservation-v1.json", source.url), "utf8"));
      const originalWorker = globalThis.Worker,
        originalFetch = globalThis.fetch;
      let bodyRequests = 0;
      globalThis.fetch = async () => {
        bodyRequests++;
        throw new Error("fixture forbids body fetch");
      };
      const instances: FakeReservationWorker[] = [];
      let delayed = false,
        failFactory = false,
        loads = 0;
      let beforeTerminate: (() => void) | undefined;
      class FakeReservationWorker {
        port: MessagePort | null = null;
        initialization: Record<string, any> | null = null;
        terminated = false;
        constructor() {
          if (failFactory) throw new Error("fixture worker failure");
          instances.push(this);
        }
        postMessage(message: Record<string, any>): void {
          this.initialization = message;
          this.port = message.port;
          this.port!.onmessage = (event) => {
            if (event.data.kind === "load") loads++;
          };
          this.port!.start();
          if (!delayed) this.ready();
        }
        ready(): void {
          const message = this.initialization!;
          this.port!.postMessage({ schema: message.schema, nonce: message.nonce, generation: message.generation, kind: "ready" });
        }
        terminate(): void {
          beforeTerminate?.();
          this.terminated = true;
          this.port?.close();
        }
      }
      (globalThis as unknown as { Worker: unknown }).Worker = FakeReservationWorker;
      const setup = (name: string, spaceId = "reservation-space") => {
        const documentId = "reservation-" + name;
        openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "untrusted-ui-actor" });
        const state = artifactState(documentId)!;
        artifacts.delete(state.runtimeKey);
        const binding = { kind: "hub", baseUrl: fixture.hubOrigin, spaceId } as const;
        state.config = { ...state.config, bindings: [binding] };
        if (name === "foreign-origin") state.config = { ...state.config, bindings: [{ ...binding, baseUrl: "http://foreign.test" }] };
        if (name === "foreign-schema") state.config = { ...state.config, schema: "foreign-schema" };
        if (name === "foreign-surface") state.config = { ...state.config, bindings: [{ ...binding, requestedSurfaceId: "foreign-surface" }] };
        state.runtimeKey = documentRuntimeKeyForConfig(state.config);
        artifacts.set(state.runtimeKey, state);
        const fields = structuredClone(fixture.manifest);
        fields.scope = { spaceId, documentId };
        if (fields.checkpoint) fields.checkpoint = { ...fields.checkpoint, baselineFrontier: { ...fields.checkpoint.baselineFrontier, documentId } };
        if (name === "none") {
          fields.browserActor = { kind: "none" };
          fields.surface = { ...fields.surface, rendererTarget: "wgpu" };
        }
        if (name === "foreign-scope") fields.scope.spaceId = "foreign";
        const component = new Uint8Array([1]),
          descriptor = new Uint8Array([2]);
        const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, component, descriptor);
        state.executionTargetLease = name === "no-lease" ? null : lease;
        if (name !== "ungranted-lease")
          lease.admitBrowserActor(
            documentExecutionTargetLeaseMintToken,
            { ...fixture.socketGrant, expiresAtMs: name === "expired-grant" ? Date.now() - 1 : Date.now() + (name === "grant-expired-during-reserve" ? 50 : name === "expiry" ? 500 : 25000) },
            Date.now() + (name === "expiry" ? 500 : 30000),
            { binding, intent: { ...fixture.intent, scope: { spaceId, documentId } }, assertCurrent() {} },
          );
        if (name === "dropped-lease") lease.drop();
        if (name === "stale-state") artifacts.delete(state.runtimeKey);
        return { state, lease, component, descriptor };
      };
      const created: ReturnType<typeof setup>[] = [];
      const cleanup = () => {
        beforeTerminate = undefined;
        for (const entry of created.splice(0)) {
          artifacts.set(entry.state.runtimeKey, entry.state);
          closeArtifactRuntime(entry.state.runtimeKey);
          entry.lease.drop();
        }
      };
      try {
        for (const row of corpus.cases) {
          const count = instances.length;
          let outcome = "denied";
          delayed = ["close-during-reserve", "replace-during-reserve", "grant-expired-during-reserve"].includes(row.name);
          failFactory = row.name === "factory-failure";
          try {
            const entry = setup(row.name);
            created.push(entry);
            const reserving = reserveDocumentBrowserActorChild(entry.state);
            if (row.name === "close-during-reserve") closeArtifactRuntime(entry.state.runtimeKey);
            if (row.name === "replace-during-reserve") entry.state.executionTargetLease = null;
            if (row.name === "grant-expired-during-reserve") await new Promise((resolve) => setTimeout(resolve, 80));
            if (delayed) instances.at(-1)?.ready();
            const owner = await reserving;
            outcome = owner === null ? "none" : "reserved";
            if (row.name === "duplicate") {
              await expect(reserveDocumentBrowserActorChild(entry.state)).rejects.toThrow();
              outcome = "denied";
            }
            if (owner !== null) expect(owner.generation > 0n).toBe(true);
          } catch {
            outcome = "denied";
          } finally {
            cleanup();
            delayed = false;
            failFactory = false;
          }
          expect({ name: row.name, expected: outcome, created: instances.length - count }).toEqual(row);
          expect(instances.every((worker) => worker.terminated)).toBe(true);
        }
        let entry = setup("lifecycle");
        created.push(entry);
        const first = await reserveDocumentBrowserActorChild(entry.state);
        expect(instances.at(-1)!.initialization!.actorId).toBe(fixture.socketGrant.actorId);
        expect(instances.at(-1)!.initialization!.actorId).not.toBe(entry.state.config.actor);
        expect(() => entry.lease.admitBrowserActor(Symbol("foreign"), fixture.socketGrant, Date.now() + 30000, { binding: hubBinding(entry.state.config)!, intent: fixture.intent, assertCurrent() {} })).toThrow();
        expect(() => entry.lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, fixture.socketGrant, Date.now() + 30000, { binding: hubBinding(entry.state.config)!, intent: fixture.intent, assertCurrent() {} })).toThrow();
        beforeTerminate = () => {
          expect(entry.component[0]).toBe(1);
          expect(entry.descriptor[0]).toBe(2);
        };
        dropDocumentExecutionTargetLease(entry.state);
        expect(entry.component[0]).toBe(0);
        expect(entry.state.browserActorReservation).toBeNull();
        beforeTerminate = undefined;
        cleanup();
        entry = setup("lifecycle");
        created.push(entry);
        const reopened = await reserveDocumentBrowserActorChild(entry.state);
        expect(reopened!.generation).toBeGreaterThan(first!.generation);
        const peer = setup("lifecycle", "second-space");
        created.push(peer);
        const peerOwner = await reserveDocumentBrowserActorChild(peer.state);
        expect(peerOwner).not.toBe(reopened);
        entry.lease.drop();
        expect(entry.state.browserActorReservation).toBeNull();
        expect(peer.state.browserActorReservation).toBe(peerOwner);
        cleanup();
        entry = setup("expiry");
        created.push(entry);
        const retired = new Promise<void>((resolve) => {
          beforeTerminate = resolve;
        });
        await reserveDocumentBrowserActorChild(entry.state);
        await retired;
        expect(entry.state.browserActorReservation).toBeNull();
        expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
        expect(loads).toBe(corpus.loadedActors);
        expect(bodyRequests).toBe(corpus.bodyRequests);
        expect(corpus.lifecycle).toHaveLength(7);
      } finally {
        cleanup();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
      }
    });

    it("browser document actor reservation activates only after an exact current socket Session", async () => {
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧫️fixtures/📇️directory/🧵️browser-actor-session-v1.json", source.url), "utf8"));
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex),
        descriptor = executionTargetBytes(fixture.descriptorHex),
        actor = new TextEncoder().encode("abc");
      const originalFetch = globalThis.fetch,
        originalWorker = globalThis.Worker,
        originalSocket = globalThis.WebSocket;
      const wait = async (done: () => boolean) => {
        const deadline = Date.now() + 3000;
        while (!done()) {
          if (Date.now() > deadline) throw new Error("session activation test deadline");
          await new Promise((resolve) => setTimeout(resolve, 2));
        }
      };
      const guest = decodePackValue(descriptor) as Record<string, any>;
      for (const key of Object.keys(guest.hashes)) guest.hashes[key] = "";
      try {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
        for (const row of corpus.cases) {
          let bodies = 0,
            loads = 0,
            describes = 0,
            activated = 0;
          const workers: SessionWorker[] = [],
            records: { fixture: ExecutionTargetLeaseFixture; state: ArtifactState; socket: FakeHubWebSocket; connected: Promise<void> }[] = [];
          const streams: { stream: ReadableStream<Uint8Array>; chunk: Uint8Array; cancelled: number }[] = [];
          const requests: { url: string; method: string; body: string }[] = [];
          const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
          const bodyProgress = new Set<string>();
          const describeApi = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts");
          const originalVerify = describeApi.verifyBrowserActorDescribeV1;
          const verifySpy = ["client-change-after-describe", "socket-close-after-describe"].includes(row.name)
            ? vi.spyOn(describeApi, "verifyBrowserActorDescribeV1").mockImplementation((guest, staged, codec) => {
                originalVerify(guest, staged, codec);
                queueMicrotask(() => {
                  if (row.name === "socket-close-after-describe") records[0]!.socket.close();
                  else records[0]!.state.openClientInstanceId = "replaced-after-describe";
                });
              })
            : undefined;
          class SessionWorker {
            port!: MessagePort;
            binding!: { schema: string; nonce: string; generation: string };
            held: Record<string, any> | null = null;
            terminations = 0;
            postMessage(init: Record<string, any>): void {
              this.port = init.port;
              this.binding = { schema: init.schema, nonce: init.nonce, generation: init.generation };
              this.port.onmessage = (event) => {
                const message = event.data;
                if (message.kind === "load") {
                  loads++;
                  expect(new Uint8Array(message.bytes)).toEqual(actor);
                  if (["socket-replaced-during-load", "duplicate-session-during-load"].includes(row.name) && workers[0] === this) this.held = message;
                  else {
                    if (row.name === "client-replaced-during-load") records[0]!.state.openClientInstanceId = "replacement-client";
                    this.loaded(message);
                  }
                }
                if (message.kind === "invoke") {
                  if (message.path[0] === "describe") {
                    describes++;
                    expect(message.path).toEqual(["describe", "describe"]);
                    expect(message.args).toEqual([]);
                    const value = structuredClone(guest);
                    if (row.name === "descriptor-mismatch") value.manifest.pluginId = "foreign";
                    const encoded = encodePackValue(value);
                    const bytes = row.name === "descriptor-noncanonical" ? new Uint8Array([...encoded, 0]) : encoded;
                    this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: row.name === "descriptor-result-shape" ? [bytes] : bytes }, [bytes.buffer]);
                    expect(bytes.byteLength).toBe(0);
                    this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 1 });
                    return;
                  }
                  expect(message.path).toEqual(["reactor", "poll"]);
                  const events = message.args[0] as Record<string, any>[];
                  const open = events[0]?.tag === "instance-open" ? events[0].val : null;
                  const close = events[0]?.tag === "instance-close" ? events[0].val : null;
                  const lifecycleAck = events[0]?.tag === "instance-lifecycle-ack" ? events[0].val : null;
                  const lifecycleReceipt = open
                    ? { tag: "captured", val: { lifetime: { activationGeneration: open.activationGeneration, instanceId: open.instance, guestLifetime: 1n }, requestSequence: open.requestSequence } }
                    : close
                      ? { tag: "accepted", val: { lifetime: close.lifetime, requestSequence: close.requestSequence, closeGeneration: 1n } }
                      : lifecycleAck?.tag === "accepted"
                        ? { tag: "retired", val: { lifetime: lifecycleAck.val.lifetime, requestSequence: lifecycleAck.val.requestSequence, closeGeneration: lifecycleAck.val.closeGeneration } }
                        : null;
                  if (lifecycleAck?.tag === "captured") activated++;
                  this.port.postMessage({
                    ...this.binding,
                    kind: "result",
                    sequence: message.sequence,
                    value: { uiPatches: [], effects: [], presence: [], nextWake: null, status: { tag: "idle" }, fuelUsed: 1n, commandIngress: { tag: "idle" }, coldPairIngress: { tag: "idle" }, lifecycleReceipt, uiPatchReceipt: null },
                  });
                  this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 0 });
                }
              };
              this.port.start();
              this.port.postMessage({ ...this.binding, kind: "ready" });
            }
            constructor() {
              workers.push(this);
            }
            loaded(message: Record<string, any>): void {
              this.port.postMessage({ ...this.binding, kind: "loaded", byteLength: message.bytes.byteLength, sha256: message.sha256 });
              new Uint8Array(message.bytes).fill(0);
            }
            terminate(): void {
              this.terminations++;
              if (this.held) new Uint8Array(this.held.bytes).fill(0);
              this.port?.close();
            }
          }
          (globalThis as unknown as { Worker: unknown }).Worker = SessionWorker;
          testSeams.socketGrantTestIssue = null;
          testSeams.executionTargetStatusObserver = (status) => {
            statuses.push(status);
            if (status.progress?.stage === "manifest") bodyProgress.delete(status.spaceId);
            if (status.progress?.stage === corpus.progressStage) {
              bodyProgress.add(status.spaceId);
              if (row.name === "client-change-during-progress") records[0]!.state.openClientInstanceId = "replaced-by-progress-observer";
            }
            if (status.code === "renderer-unavailable") bodyProgress.delete(status.spaceId);
          };
          clearLocalBrowserBrokerProof();
          installLocalBrowserBrokerProof("a".repeat(64));
          globalThis.fetch = async (input, init) => {
            const url = String(input);
            requests.push({ url, method: String(init?.method ?? "GET"), body: String(init?.body ?? "") });
            const record = records.find((record) => url.includes("/spaces/" + encodeURIComponent(record.fixture.intent.scope.spaceId) + "/"));
            if (!record) throw new Error("unowned session fixture request");
            const current = record.fixture;
            if (url.endsWith("/open-plan")) return Response.json(current.plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
            if (url.endsWith("/execution-target/manifest")) return Response.json(current.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
            if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
            if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
            if (url.endsWith("/execution-target/browser-actor")) {
              bodies++;
              expect(init?.method).toBe("POST");
              expect(JSON.parse(String(init?.body))).toEqual(current.intent);
              if (row.name === "socket-close-during-body" || row.name === "scope-change-at-headers") {
                const retained = { stream: null as unknown as ReadableStream<Uint8Array>, chunk: new Uint8Array([97]), cancelled: 0 };
                retained.stream = new ReadableStream({
                  start(controller) {
                    if (row.name === "socket-close-during-body") controller.enqueue(retained.chunk);
                  },
                  cancel() {
                    retained.cancelled++;
                    return new Promise(() => {});
                  },
                });
                streams.push(retained);
                if (row.name === "scope-change-at-headers") record.state.config = { ...record.state.config, schema: "foreign-schema" };
                return new Response(retained.stream, { headers: { "content-length": "3", "x-semio-browser-broker-advanced": "1" } });
              }
              return executionTargetBodyResponse(row.name === "body-hash-mismatch" ? new Uint8Array([0, 0, 0]) : actor);
            }
            return Response.json(current.socketGrant, { headers: { "x-semio-browser-broker-advanced": "1" } });
          };
          const setup = async (spaceId: string) => {
            const current = { ...structuredClone(fixture), socketGrant: { ...structuredClone(fixture.socketGrant), expiresAtMs: Date.now() + 25000 } };
            current.intent.scope.spaceId = spaceId;
            current.plan.scope.spaceId = spaceId;
            current.manifest.scope.spaceId = spaceId;
            current.plan.expiresAtUnixMs = Date.now() + 30000;
            openArtifact({ documentId: current.intent.scope.documentId, schema: current.plan.artifact.schema, bindings: [], actor: "untrusted-ui-actor" });
            const state = artifactState(current.intent.scope.documentId)!;
            state.openClientInstanceId = current.intent.clientInstanceId;
            artifacts.delete(state.runtimeKey);
            const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: current.hubOrigin, spaceId, requestedSurfaceId: current.intent.requestedSurfaceId };
            state.config = { ...state.config, bindings: [binding] };
            state.runtimeKey = documentRuntimeKeyForConfig(state.config);
            artifacts.set(state.runtimeKey, state);
            const record = { fixture: current, state, socket: null as unknown as FakeHubWebSocket, connected: null as unknown as Promise<void> };
            records.push(record);
            const count = FakeHubWebSocket.instances.length;
            record.connected = connectHubOnce(state, binding);
            void record.connected.catch(() => {});
            await wait(() => FakeHubWebSocket.instances.length > count);
            record.socket = FakeHubWebSocket.instances.at(-1)!;
            record.socket.open();
            return record;
          };
          const session = async (record: (typeof records)[number], actorId = record.fixture.socketGrant.actorId) => {
            record.socket.onmessage?.({ data: encodeServerFrame({ Session: { actor: actorId, color: 3 } }, "command").buffer as ArrayBuffer });
            await record.state.hubFrameChain;
          };
          try {
            const first = await setup("session-" + row.name);
            const initialLease = first.state.executionTargetLease!;
            expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
            expect(bodies).toBe(0);
            if (row.name !== "before-session") await session(first, row.name === "wrong-session" ? "hub.v1." + "f".repeat(64) : first.fixture.socketGrant.actorId);
            if (row.name === "socket-close-during-body") {
              await wait(() => bodies === 1 && streams[0]?.stream.locked);
              first.socket.close();
              await wait(() => !streams[0]!.stream.locked);
              expect(streams[0]!.cancelled).toBe(1);
              expect(streams[0]!.chunk.every((byte) => byte === 0)).toBe(true);
            } else if (row.name === "duplicate-session-during-load") {
              await wait(() => loads === 1);
              await session(first);
              await wait(() => !initialLease.live);
              expect(workers).toHaveLength(1);
            } else if (row.name === "socket-replaced-during-load") {
              await wait(() => loads === 1);
              const staleSocket = first.socket;
              staleSocket.close();
              const binding = hubBinding(first.state.config)!;
              first.connected = connectHubOnce(first.state, binding);
              void first.connected.catch(() => {});
              await wait(() => FakeHubWebSocket.instances.at(-1) !== staleSocket);
              first.socket = FakeHubWebSocket.instances.at(-1)!;
              first.socket.open();
              const nextLease = first.state.executionTargetLease!;
              await session(first);
              staleSocket.onmessage?.({ data: encodeServerFrame({ Session: { actor: first.fixture.socketGrant.actorId, color: 9 } }, "command").buffer as ArrayBuffer });
              await wait(() => activated === 1);
              expect(initialLease.live).toBe(false);
              expect(first.state.executionTargetLease).toBe(nextLease);
              expect(nextLease.live).toBe(true);
            } else if (row.name === "same-document-other-space") {
              await wait(() => activated === 1);
              const peer = await setup(first.fixture.intent.scope.spaceId + "-peer");
              await session(peer);
              await wait(() => activated === 2);
              const peerOwner = peer.state.browserActorReservation;
              first.socket.close();
              expect(peer.state.browserActorReservation).toBe(peerOwner);
              expect(peer.state.executionTargetLease?.live).toBe(true);
              expect(peer.socket.readyState).toBe(FakeHubWebSocket.OPEN);
            } else if (row.activated > 0) await wait(() => activated === row.activated);
            else if (row.name !== "before-session" && row.name !== "socket-close-during-body") await wait(() => !initialLease.live);
            if (row.name === "scope-change-at-headers") {
              expect(streams[0]!.cancelled).toBe(1);
              expect(streams[0]!.stream.locked).toBe(false);
            }
            expect({ name: row.name, bodies, loads, describes, activated }).toEqual(row);
            if (row.name !== "before-session" && row.activated === 0) expect(first.state.executionTargetLease).toBeNull();
            expect(requests.filter((request) => request.url.endsWith("/browser-actor")).every((request) => !request.body.includes("open.v1.") && !request.body.includes("socket.v1."))).toBe(true);
            expect(records.every((record) => record.state.outbox.length === 0)).toBe(true);
            expect(statuses.every((status) => !JSON.stringify(status).includes("open.v1.") && !JSON.stringify(status).includes("socket.v1."))).toBe(true);
          } finally {
            verifySpy?.mockRestore();
            const retirementOutcomes: Promise<unknown>[] = [];
            for (const record of records) {
              const outcome = record.state.browserActorReservation?.retirementOutcome;
              closeArtifactRuntime(record.state.runtimeKey);
              if (outcome !== undefined) retirementOutcomes.push(outcome);
              await record.connected.catch(() => {});
            }
            await Promise.all(retirementOutcomes);
            await wait(() => workers.every((worker) => worker.terminations === 1));
            for (const worker of workers) expect(worker.terminations).toBe(1);
            expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
            testSeams.executionTargetStatusObserver = null;
            clearLocalBrowserBrokerProof();
          }
        }
        console.log("document-session-activation: socket-path=1 neutral=" + corpus.cases.length + " worker=mocked final-capacity=0");
      } finally {
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
      }
    });

    it("browser document actor transfers one verified cold pair only after lifecycle ACK and exact page receipts", async () => {
      const fixture = await executionTargetLeaseFixture();
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const actionFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const Ajv = (await import("ajv")).default;
      const schema = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);
      const { applyPatch } = await import("fast-json-patch");
      const rendererOracle = applyPatch({ revision: 0, nodeKind: null as string | null }, [
        { op: "replace", path: "/revision", value: corpus.browserRender.revision },
        { op: "replace", path: "/nodeKind", value: corpus.browserRender.nodeKind },
      ]).newDocument;
      const renderEvents: string[] = [];
      let renderWakes = 0;
      let activeLifetime: ActorInstanceLifetime | null = null;
      const exact = corpus.exact as {
        packLength: number;
        sprLength: number;
        pageCount: number;
        packPattern: { multiplier: number; addend: number };
        sprPattern: { multiplier: number; addend: number };
        packSha256: string;
        sprSha256: string;
        aggregateSha256: string;
      };
      const pattern = (length: number, multiplier: number, addend: number): Uint8Array => Uint8Array.from({ length }, (_unused, index) => (index * multiplier + addend) & 255);
      const pack = pattern(exact.packLength, exact.packPattern.multiplier, exact.packPattern.addend),
        spr = pattern(exact.sprLength, exact.sprPattern.multiplier, exact.sprPattern.addend),
        publishedPack = Uint8Array.from(pack),
        publishedSpr = Uint8Array.from(spr),
        actorBytes = new TextEncoder().encode("abc"),
        sourceDescriptor = executionTargetBytes(fixture.descriptorHex),
        guest = decodePackValue(sourceDescriptor) as Record<string, any>,
        originalWorker = globalThis.Worker,
        originalFetch = globalThis.fetch;
      const descriptorApp = guest.manifest.apps.find((app: Record<string, unknown>) => app.id === fixture.manifest.surface.appId);
      if (descriptorApp === undefined) throw new Error("direct browser actor fixture: descriptor app absent");
      descriptorApp.role = "editor";
      const descriptor = encodePackValue(guest),
        descriptorByteSha256 = await executionTargetSha256Hex(descriptor);
      const { UiDocumentStore } = await import("../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx");
      const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
      const { decodeAppCommand, encodeAppFrame } = await import("../../🟦️.ts");
      const { readActorBytePage } = await import("../../../../🔨️modules/🎭️actor/📃️page/🟦️.ts");
      const { createBrowserActorAppCommandRequestV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command/🟦️.ts");
      const { createBrowserActorUiIntentRequestV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent/🟦️.ts");
      const { decodeBrowserActorHostEffectsV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts");
      const windowKindId = fixture.manifest.surface.windowKindId;
      const hostFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧫️fixtures/🪟️host-opening-context/🔣️.json", source.url), "utf8"));
      const hostView = { ...hostFixture.valid.viewState, windowId: windowKindId, activeWindowKindId: windowKindId, activeUtilityId: "pan", activeUtilityByWindowId: { [windowKindId]: "pan" }, windowInstances: [{ id: windowKindId, windowKindId }] };
      const bodyKey = leaseBodyKey(guest);
      function leaseBodyKey(descriptor: Record<string, any>): string {
        return descriptor.manifest.apps.find((app: Record<string, any>) => app.id === fixture.manifest.surface.appId).windowKinds.find((window: Record<string, any>) => window.id === windowKindId).bodyKey;
      }
      const visibleViews: Record<string, unknown>[] = [];
      for (const key of Object.keys(guest.hashes)) guest.hashes[key] = "";
      const received = new Uint8Array(pack.byteLength + spr.byteLength),
        pageIndexes: number[] = [],
        statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [],
        historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[] = [],
        workerResponses: BackboneWorkerResponse[] = [],
        patchOffers: BrowserActorUiPatchOfferV1[] = [],
        documentControls: string[] = [],
        lifecycleClose: string[] = [],
        actionSequences: bigint[] = [],
        commandSequences: number[] = [],
        commandInvocations: Record<string, unknown>[] = [],
        commandViews: Record<string, unknown>[] = [],
        backboneIngress: Uint8Array[] = [],
        uiStore = new UiDocumentStore(windowKindId);
      let lifecycleAcknowledged = false,
        patchAcknowledged = false,
        patchRejected = false,
        actionPatchAcknowledged = false,
        heldActionPatch: BrowserActorUiPatchOfferV1 | null = null,
        loaded = 0,
        described = 0,
        terminated = 0;
      class ColdPairWorker {
        port!: MessagePort;
        binding!: { schema: string; nonce: string; generation: string };
        postMessage(init: Record<string, any>): void {
          this.port = init.port;
          this.binding = { schema: init.schema, nonce: init.nonce, generation: init.generation };
          this.port.onmessage = (event) => {
            const message = event.data;
            if (message.kind === "load") {
              loaded++;
              expect(new Uint8Array(message.bytes)).toEqual(actorBytes);
              const byteLength = message.bytes.byteLength;
              new Uint8Array(message.bytes).fill(0);
              this.port.postMessage({ ...this.binding, kind: "loaded", byteLength, sha256: message.sha256 });
              return;
            }
            if (message.kind !== "invoke") return;
            if (message.path[0] === "describe") {
              described++;
              const bytes = encodePackValue(guest);
              this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: bytes }, [bytes.buffer]);
              this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 1 });
              return;
            }
            expect(message.path).toEqual(["reactor", "poll"]);
            const events = message.args[0] as Record<string, any>[];
            const open = events[0]?.tag === "instance-open" ? events[0].val : null;
            const close = events[0]?.tag === "instance-close" ? events[0].val : null;
            const lifecycleAck = events[0]?.tag === "instance-lifecycle-ack" ? events[0].val : null;
            const acknowledged = lifecycleAck?.tag === "captured";
            const patchAck = events[0]?.tag === "patch-ack" ? events[0].val : null;
            const patchRejection = events[0]?.tag === "patch-rejected" ? events[0].val : null;
            const visible = events[0]?.tag === "surface-visible" ? events[0].val : null;
            const wake = events[0]?.tag === "wake";
            const shellMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "shell" ? events[0].val : null;
            const backboneMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "backbone" ? events[0].val : null;
            const uiIntent = events[0]?.tag === "ui-intent" ? events[0].val : null;
            const commandPage = message.args[1] as Record<string, any> | null;
            if (uiIntent !== null) {
              expect(uiIntent.instance).toBe(0);
              const decoded = decodePackValue(uiIntent.intent) as Record<string, any>;
              expect(decoded.surface).toBe(`0:${windowKindId}`);
              actionSequences.push(decoded.seq.value);
            }
            let commandSequence: number | null = null;
            if (commandPage !== null) {
              expect(commandPage.cursor).toMatchObject({ owner: 0n, generation: reservation!.generation, commandIndex: 0, commandCount: 1, instance: 0, pageIndex: 0, pageCount: 1 });
              const command = decodeAppCommand(readActorBytePage(commandPage.page));
              if (!("Command" in command)) throw new Error("expected browser actor AppCommand::Command");
              commandSequence = command.Command.seq;
              expect(commandPage.cursor.seq).toBe(BigInt(commandSequence));
              commandSequences.push(commandSequence);
              commandInvocations.push(decodePackValue(Uint8Array.from(command.Command.command)) as Record<string, unknown>);
              commandViews.push(decodePackValue(Uint8Array.from(command.Command.view_state)) as Record<string, unknown>);
            }
            if (backboneMessage !== null) backboneIngress.push(Uint8Array.from(backboneMessage.payload));
            if (visible) {
              expect(lifecycleAcknowledged).toBe(true);
              expect(pageIndexes).toHaveLength(exact.pageCount);
              expect(visible.surface).toEqual({ instance: 0, surface: windowKindId });
              expect(visible.bodyKey).toBe(bodyKey);
              const visibleView = decodePackValue(visible.viewState) as Record<string, unknown>;
              expect(visibleView).toEqual(state.browserActorViewState);
              visibleViews.push(visibleView);
            }
            if (wake) {
              expect(renderEvents[0]).toBe("surface-visible");
              renderWakes++;
            }
            if (visible || wake || patchAck || patchRejection) renderEvents.push(events[0].tag);
            const page = message.args[2] as Record<string, any> | null;
            let coldPairIngress: Record<string, any> = { tag: "idle" };
            if (acknowledged) lifecycleAcknowledged = true;
            if (patchAck) {
              expect(patchAck.surface).toEqual({ instance: 0, surface: windowKindId });
              if (patchAck.receipt.patchSequence === 1n) {
                expect(patchAck.revision).toBe(1n);
                patchAcknowledged = true;
              } else {
                expect(patchAck.receipt.patchSequence).toBe(3n);
                expect(patchAck.revision).toBe(2n);
                actionPatchAcknowledged = true;
              }
            }
            if (patchRejection) {
              expect(patchRejection.surface).toEqual({ instance: 0, surface: windowKindId });
              expect(patchRejection.revision).toBe(1n);
              expect(patchRejection.receipt.patchSequence).toBe(2n);
              expect(patchRejection.reason).toBe("revisionMismatch");
              patchRejected = true;
            }
            if (page) {
              expect(lifecycleAcknowledged).toBe(true);
              const header = page.header as Record<string, any>,
                pageIndex = page.pageIndex as number,
                bytes = page.bytes as Uint8Array,
                start = pageIndex * corpus.limits.pageBytes;
              expect(pageIndexes).toHaveLength(pageIndex);
              expect(bytes.byteLength).toBe(Math.min(corpus.limits.pageBytes, received.byteLength - start));
              received.set(bytes, start);
              bytes.fill(0);
              pageIndexes.push(pageIndex);
              activeLifetime = header.lifetime;
              const cursor = { lifetime: header.lifetime, transferGeneration: header.transferGeneration, pageIndex, pageCount: header.pageCount };
              coldPairIngress =
                pageIndex + 1 === header.pageCount
                  ? { tag: "applied", val: { lifetime: header.lifetime, transferGeneration: header.transferGeneration, baselineFrontier: header.baselineFrontier, aggregateSha256: header.aggregateSha256 } }
                  : { tag: "page-accepted", val: cursor };
            }
            if (close) lifecycleClose.push("close");
            if (lifecycleAck?.tag === "accepted") lifecycleClose.push("accepted-ack");
            if (lifecycleAck?.tag === "retired") lifecycleClose.push("retired-ack");
            const lifecycleReceipt = open
              ? { tag: "captured", val: { lifetime: { activationGeneration: open.activationGeneration, instanceId: open.instance, guestLifetime: 1n }, requestSequence: open.requestSequence } }
              : close
                ? { tag: "accepted", val: { lifetime: close.lifetime, requestSequence: close.requestSequence, closeGeneration: 1n } }
                : lifecycleAck?.tag === "accepted"
                  ? { tag: "retired", val: { lifetime: lifecycleAck.val.lifetime, requestSequence: lifecycleAck.val.requestSequence, closeGeneration: lifecycleAck.val.closeGeneration } }
                  : null;
            const initialScene = wake && renderWakes === corpus.browserRender.lazyWakeTurns;
            const lifetime = activeLifetime;
            const node = {
              id: 0,
              key: "map-root",
              component: { type: "surface", kind: "tiled-map", docSchema: "tiled-map@1", doc: { bytes: [1, 3, 5] }, bindings: [] },
              layout: { kind: "leaf", width: "fill", height: "fill" },
              style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
              activity: "idle",
              disabled: false,
              transition: null,
              accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
              bindings: [],
              menu: null,
              children: [],
            };
            const emitsPatch = initialScene || uiIntent !== null || patchAck?.receipt.patchSequence === 1n;
            const nodeBytes = initialScene ? encodePackValue(node) : null;
            const patchReceiptBytes = emitsPatch ? encodeActorUiPatchReceipt({ lifetime: lifetime!, patchSequence: initialScene ? 1n : uiIntent !== null ? 3n : 2n }) : null;
            const effects: Record<string, unknown>[] = [];
            let controlReceipt: Uint8Array | null = null;
            if (shellMessage !== null) {
              const command = decodeDocumentBackboneControlV1(shellMessage.payload);
              documentControls.push(command.operation);
              controlReceipt = encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" });
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: controlReceipt } });
            }
            let commandBackbone: Uint8Array | null = null;
            const commandMutation = commandSequence === 3
              ? (() => {
                  const mutationId = "direct-command-mutation",
                    invocationId = "direct-command:0:3",
                    forward = encodePackValue({ sequenceNumber: 3 }),
                    inverse = encodePackValue(null),
                    inverseVector = Uint8Array.of(1, inverse.byteLength, ...inverse),
                    mutation = {
                      id: mutationId,
                      document: "0",
                      baseVersion: 0,
                      invocationId,
                      diff: { schema: `${fields.artifact.schema}.operation`, payload: Array.from(forward) },
                      inverse: { targetMutation: mutationId, inverseDiff: { schema: `${fields.artifact.schema}.operation.inverse`, payload: Array.from(inverseVector) }, baseVersion: 0, dependencies: [], undoPolicy: "ExactBaseOnly" },
                      dependencies: [],
                      author: "actor-1",
                      timestamp: { actor: 7, physical_ms: 8, logical: 9 },
                    },
                    envelope: MutationEnvelope = {
                      id: mutationId,
                      actor: "actor-1",
                      document: documentId,
                      schemaVersion: fields.artifact.schema,
                      deps: [],
                      payloadHash: "unused",
                      diff: { schemaId: fields.artifact.schema, payload: { sequenceNumber: 3 } },
                      inverse: { targetOperation: mutationId, inverseDiff: { schemaId: fields.artifact.schema, payload: null }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
                    };
                  commandBackbone = exactDocumentBackboneMessage(envelope, forward, { actor: 7n, physical_ms: 8n, logical: 9n });
                  return { mutations: Array.from(encodePackValue([mutation])), inverseGroup: Array.from(encodePackValue({ invocationId, mutations: [mutationId], inverseMutations: [mutation.inverse] })) };
                })()
              : null;
            const actionPublication = uiIntent !== null
              ? encodeAppFrame(actionFixture.publication.emit)
              : commandSequence !== null
                ? encodeAppFrame({ Invocation: { in_reply_to: commandSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: commandMutation?.mutations ?? [], inverse_group: commandMutation?.inverseGroup ?? [] } })
                : null;
            if (actionPublication !== null) {
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: actionPublication } });
              effects.push(actionFixture.publication.hostEffect);
            }
            if (commandBackbone !== null) effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: `actor://${state.runtimeKey}` }, payload: commandBackbone } });
            const resultTransfers: ArrayBuffer[] = initialScene ? [nodeBytes!.buffer, patchReceiptBytes!.buffer] : patchReceiptBytes ? [patchReceiptBytes.buffer] : [];
            if (controlReceipt !== null) resultTransfers.push(controlReceipt.buffer);
            if (actionPublication !== null) resultTransfers.push(actionPublication.buffer);
            if (commandBackbone !== null) resultTransfers.push(commandBackbone.buffer);
            this.port.postMessage(
              {
                ...this.binding,
                kind: "result",
                sequence: message.sequence,
                value: {
                  uiPatches: initialScene
                    ? [
                        {
                          surface: { instance: 0, surface: windowKindId },
                          baseRevision: 0n,
                          revision: 1n,
                          ops: [
                            { tag: "upsert", val: { node: nodeBytes } },
                            { tag: "set-root", val: 0n },
                          ],
                        },
                      ]
                    : uiIntent !== null
                      ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 1n, revision: 2n, ops: [] }]
                    : patchAck?.receipt.patchSequence === 1n
                      ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 0n, revision: 2n, ops: [] }]
                      : [],
                  effects,
                  presence: [],
                  nextWake: null,
                  status: { tag: (visible && visibleViews.length === 1) || (wake && !initialScene) ? "more-work" : "idle" },
                  fuelUsed: 1n,
                  commandIngress: { tag: commandSequence === null ? "idle" : "command-complete" },
                  coldPairIngress,
                  lifecycleReceipt,
                  uiPatchReceipt: patchReceiptBytes,
                },
              },
              resultTransfers,
            );
            this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: resultTransfers.length });
          };
          this.port.start();
          this.port.postMessage({ ...this.binding, kind: "ready" });
        }
        terminate(): void {
          terminated++;
          this.port?.close();
        }
      }
      clearLocalBrowserBrokerProof();
      installLocalBrowserBrokerProof("d".repeat(64));
      const authorityFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = authorityFixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const authorityBody = JSON.stringify(authority);
      globalThis.fetch = async () => new Response(authorityBody, { status: 200, headers: { "content-length": String(new TextEncoder().encode(authorityBody).byteLength), "x-semio-browser-broker-advanced": "1" } });
      await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
      const sessionFence = testSeams.captureBrowserSessionOperationFence();
      expect(sessionFence).not.toBeNull();
      globalThis.fetch = async (input) => {
        expect(String(input).endsWith("/execution-target/browser-actor")).toBe(true);
        return executionTargetBodyResponse(actorBytes);
      };
      const documentId = "cold-browser-pair";
      openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "untrusted-ui-actor" });
      const state = artifactState(documentId)!;
      artifacts.delete(state.runtimeKey);
      const binding = { kind: "hub", baseUrl: fixture.hubOrigin, spaceId: "cold-browser-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
      state.config = { ...state.config, documentId, schema: fixture.manifest.artifact.schema, bindings: [binding] };
      state.runtimeKey = documentRuntimeKeyForConfig(state.config);
      artifacts.set(state.runtimeKey, state);
      const hostRequest = { kind: "browser-actor-view-state", clientInstanceId: state.openClientInstanceId, scope: { spaceId: binding.spaceId, documentId }, viewState: hostView } as const;
      handleTsRequest({ ...hostRequest, clientInstanceId: "00000000-0000-4000-8000-000000000000" });
      expect(state.browserActorViewState).toBeNull();
      handleTsRequest(hostRequest);
      expect(state.browserActorViewState).toEqual(hostView);
      const fields = structuredClone(fixture.manifest);
      fields.scope = { spaceId: binding.spaceId, documentId };
      fields.grant = { ...fields.grant, write: true };
      fields.surface = { ...fields.surface, role: "editor" };
      fields.package = { ...fields.package, descriptorByteSha256 };
      fields.descriptor = { sha256: descriptorByteSha256, byteLength: descriptor.byteLength };
      fields.browserActor = { ...fields.browserActor, sourceDescriptorByteSha256: descriptorByteSha256 };
      fields.checkpoint = {
        ...fields.checkpoint!,
        aggregateSha256: exact.aggregateSha256,
        baselineFrontier: { ...fields.checkpoint!.baselineFrontier, documentId },
      };
      const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), descriptor);
      state.executionTargetLease = lease;
      state.currentPack = publishedPack;
      state.currentSpr = publishedSpr;
      state.frontier = {
        document_id: documentId,
        head_edit_ordinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
        head_edit_id: fields.checkpoint.baselineFrontier.headEditId,
        last_commit_seq: fields.checkpoint.baselineFrontier.lastCommitSeq,
        chain_hash: fields.checkpoint.baselineFrontier.chainHash,
      };
      const intent = { ...structuredClone(fixture.intent), scope: { spaceId: binding.spaceId, documentId } },
        actorFixtureExpiresAtMs = Date.now() + 600_000,
        receipt = { ...structuredClone(fixture.socketGrant), expiresAtMs: actorFixtureExpiresAtMs };
      lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, actorFixtureExpiresAtMs + 5_000, { binding, intent, assertCurrent() {} });
      state.actor = receipt.actorId;
      state.hubActorReady = true;
      state.pendingSocketActorId = null;
      const socket = new FakeHubWebSocket("ws://hub.test/cold");
      state.socket = socket as unknown as WebSocket;
      socket.open();
      testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
      testSeams.workerPostTestSink = (message) => {
        workerResponses.push(message);
        if (message.kind === "inference-history-status") {
          historyStatuses.push(message);
          return;
        }
        if (message.kind === "browser-actor-ui-mounted") {
          expect(patchAcknowledged).toBe(true);
          return;
        }
        if (message.kind !== "browser-actor-ui-patch") return;
        patchOffers.push(message);
        if (message.patch.baseRevision === 1 && message.patch.revision === 2) {
          heldActionPatch = message;
          return;
        }
        const before = uiStore.getState();
        const applied = uiStore.applyPatch(message.patch);
        if (applied.ok) {
          handleTsRequest({
            kind: "browser-actor-ui-patch-result",
            clientInstanceId: message.clientInstanceId,
            scope: message.scope,
            verifiedSurfaceId: message.verifiedSurfaceId,
            activationGeneration: message.activationGeneration,
            instanceId: message.instanceId,
            receipt: message.receipt,
            outcome: "acknowledged",
            revision: uiStore.getRevisionSnapshot(),
          });
          return;
        }
        expect(uiStore.getState()).toBe(before);
        handleTsRequest({
          kind: "browser-actor-ui-patch-result",
          clientInstanceId: message.clientInstanceId,
          scope: message.scope,
          verifiedSurfaceId: message.verifiedSurfaceId,
          activationGeneration: message.activationGeneration,
          instanceId: message.instanceId,
          receipt: message.receipt,
          outcome: "rejected",
          revision: uiStore.getRevisionSnapshot(),
          reason: applied.rejection.type,
        });
      };
      (globalThis as unknown as { Worker: unknown }).Worker = ColdPairWorker;
      let owner: VerifiedColdDocumentPair | null = null;
      let reservation: Awaited<ReturnType<typeof reserveDocumentBrowserActorChild>> = null;
      try {
        reservation = await reserveDocumentBrowserActorChild(state);
        expect(reservation).not.toBeNull();
        await reservation!.activate(state.socket);
        const bootstrap: WireArtifactBootstrap = {
          format_version: 1,
          descriptor_hash: Array.from(executionTargetBytes(fields.descriptorDigestV1)),
          artifact_schema: fields.artifact.schema,
          artifact_kind: fields.artifact.kind,
          pack_schema_hash: Array.from(executionTargetBytes(fields.artifact.packSchemaHash)),
          baseline_frontier: state.frontier,
          pack_hash: Array.from(executionTargetBytes(exact.packSha256)),
          spr_hash: Array.from(executionTargetBytes(exact.sprSha256)),
          pack_length: pack.byteLength,
          spr_length: spr.byteLength,
          chunk_count: 2,
          aggregate_hash: Array.from(executionTargetBytes(exact.aggregateSha256)),
          required_tail_frontier: state.frontier,
          inline: null,
        };
        owner = new VerifiedColdDocumentPair(verifiedColdDocumentPairMintToken, state, lease, bootstrap, { pack, spr }, { pack: publishedPack, spr: publishedSpr });
        state.verifiedColdPair = owner;
        const approvalReceipt = parseGisMapInferenceApprovalReceiptV1({
          schema: "semio.hub.inference-approval-receipt/v1",
          jobId: "1".repeat(32),
          mutationId: "2".repeat(32),
          commandHash: "3".repeat(64),
          proposalHash: "4".repeat(64),
          applied: true,
          undo: {
            targetId: "5".repeat(32),
            expectedCurrent: {
              documentId,
              headEditOrdinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
              headEditId: fields.checkpoint.baselineFrontier.headEditId,
              lastCommitSeq: fields.checkpoint.baselineFrontier.lastCommitSeq,
              chainSha256: executionTargetHex(Uint8Array.from(fields.checkpoint.baselineFrontier.chainHash)),
            },
          },
        });
        const operation: InferenceOperationV1 = {
          operationEpoch: 1,
          scope: { spaceId: binding.spaceId, documentId },
          abort: new AbortController(),
          sessionEpoch: testSeams.directorySessionEpoch,
          sessionFence,
          clientInstanceId: state.openClientInstanceId,
          leaseFields: lease.fields(),
          request: null,
          closeRequested: false,
          reconcileRequired: false,
          status: idleGisMapInferencePortStatusV1(),
          turns: 0,
          pollTimer: null,
          inFlight: false,
          cancelSent: false,
          closed: false,
        };
        retainInferenceApprovalUndo(operation, approvalReceipt);
        expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "awaiting-mount", clientInstanceId: state.openClientInstanceId });
        expect(historyStatuses).toEqual([]);
        expect(owner.pageCount).toBe(exact.pageCount);
        await reservation!.installColdPair(owner);
        expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "available", clientInstanceId: state.openClientInstanceId });
        expect(historyStatuses.at(-1)?.status).toEqual({ phase: "available", canUndo: true, code: null });
        expect(renderEvents).toEqual(corpus.browserRender.events);
        await reservation!.installColdPair(owner);
        expect(renderEvents).toEqual(corpus.browserRender.events);
        expect(pageIndexes).toEqual(Array.from({ length: exact.pageCount }, (_unused, index) => index));
        expect(Buffer.from(received.subarray(0, pack.byteLength)).equals(Buffer.from(pack))).toBe(true);
        expect(Buffer.from(received.subarray(pack.byteLength)).equals(Buffer.from(spr))).toBe(true);
        expect(statuses.filter((status) => status.code === "renderer-unavailable")).toHaveLength(0);
        expect(patchOffers).toHaveLength(2);
        expect(patchAcknowledged).toBe(true);
        expect(patchRejected).toBe(true);
        expect(documentControls).toEqual(["bind"]);
        const mounted = workerResponses.find((message) => message.kind === "browser-actor-ui-mounted") as Extract<BackboneWorkerResponse, { kind: "browser-actor-ui-mounted" }>;
        expect(mounted).toMatchObject({
          scope: fields.scope,
          activeCheckpointId: fields.checkpoint.checkpointId,
          descriptorDigestV1: fields.checkpoint.descriptorDigestV1,
          frontier: fields.checkpoint.baselineFrontier,
          uiRevision: 1,
        });
        expect(uiStore.getRevisionSnapshot()).toBe(rendererOracle.revision);
        expect(uiStore.getState().nodes.get(uiStore.getState().root!)?.component).toMatchObject({ type: "surface", kind: rendererOracle.nodeKind });

        const beforeConfig = JSON.stringify(state.config);
        const beforeFrontier = JSON.stringify(state.frontier);
        handleTsRequest({ ...hostRequest, viewState: { ...hostView, locale: "en", terminology: "native" } });
        await vi.waitFor(() => expect(visibleViews.at(-1)?.locale).toBe("en"));
        expect(visibleViews.at(-1)?.terminology).toBe("native");
        expect(JSON.stringify(state.config)).toBe(beforeConfig);
        expect(JSON.stringify(state.frontier)).toBe(beforeFrontier);
        await vi.waitFor(() => expect((reservation as unknown as { viewRefresh: Promise<void> | null }).viewRefresh).toBeNull());
        console.log("[DEBUG] browser-host-context live-preference-refresh=1 artifact-config-unchanged=1 stale-opening-rejected=1");

        const actionRequest = createBrowserActorUiIntentRequestV1(
          {
            scope: { spaceId: binding.spaceId, documentId },
            verifiedSurfaceId: fields.surface.surfaceId,
            appChannelVersion: actionFixture.request.appChannelVersion,
            activationGeneration: reservation!.generation.toString(),
            instanceId: 0,
            surfaceRevision: 1,
            actionSequence: 2,
          },
          windowKindId,
          { ...actionFixture.uiIntent, surface: windowKindId, revision: 1, seq: 9_007_199_254_740_993n },
        );
        handleTsRequest({ ...actionRequest, clientInstanceId: state.openClientInstanceId });
        await vi.waitFor(() => expect(heldActionPatch !== null || workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
        expect(workerResponses.find((message) => message.kind === "browser-actor-action-result")).toBeUndefined();
        const remoteEnvelope: MutationEnvelope = {
          id: "remote-direct-turn",
          actor: "remote-actor",
          document: documentId,
          schemaVersion: fixture.manifest.artifact.schema,
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "remote" } },
          inverse: { targetOperation: "remote-direct-turn", inverseDiff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "before" } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        const remoteMessage = exactDocumentBackboneMessage(remoteEnvelope),
          remoteDelivery = reservation!.receiveBackbone(remoteMessage);
        await Promise.resolve();
        expect(backboneIngress).toHaveLength(0);
        const actionPatch = heldActionPatch!;
        const actionApplied = uiStore.applyPatch(actionPatch.patch);
        expect(actionApplied.ok).toBe(true);
        handleTsRequest({
          kind: "browser-actor-ui-patch-result",
          clientInstanceId: actionPatch.clientInstanceId,
          scope: actionPatch.scope,
          verifiedSurfaceId: actionPatch.verifiedSurfaceId,
          activationGeneration: actionPatch.activationGeneration,
          instanceId: actionPatch.instanceId,
          receipt: actionPatch.receipt,
          outcome: "acknowledged",
          revision: uiStore.getRevisionSnapshot(),
        });
        await vi.waitFor(() => expect(workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
        await remoteDelivery;
        const actionResult = workerResponses.find((message) => message.kind === "browser-actor-action-result") as Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }>;
        expect(actionResult).toMatchObject({
          clientInstanceId: state.openClientInstanceId,
          actionSequence: 2,
          surfaceRevision: 1,
          outcome: "guest-applied",
          mutationCount: 0,
        });
        expect(decodeBrowserActorHostEffectsV1(actionResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
        expect(actionSequences).toEqual([9_007_199_254_740_993n]);
        expect(actionPatchAcknowledged).toBe(true);
        expect(backboneIngress).toHaveLength(1);
        expect(backboneIngress[0]).toEqual(remoteMessage);
        expect(workerResponses.some((message) => message.kind === "event" && message.event.kind === "documentBackbone")).toBe(false);
        console.log("[DEBUG] direct-browser-actor-action: skipped-sequence=1 u64-preserved=1 shell-frame=1 host-effect=1 full-turn-serialized=1 remote-echo=0");

        const directActionInvocation = {
            ...actionFixture.actionInvocation,
            address: { ...actionFixture.actionInvocation.address, pluginId: fields.package.pluginId, appId: fields.surface.appId, windowKindId, windowInstanceId: windowKindId },
          },
          directCommandInvocation = {
            ...actionFixture.commandInvocation,
            address: { ...actionFixture.commandInvocation.address, owner: { app: { pluginId: fields.package.pluginId, appId: fields.surface.appId } } },
          },
          directCommandView = { ...actionFixture.commandViewState, activeWindowKindId: windowKindId, windowId: windowKindId };
        for (const [index, invocation] of [directActionInvocation, directCommandInvocation].entries()) {
          const commandRequest = createBrowserActorAppCommandRequestV1(
            {
              scope: { spaceId: binding.spaceId, documentId },
              verifiedSurfaceId: fields.surface.surfaceId,
              appChannelVersion: actionFixture.request.appChannelVersion,
              activationGeneration: reservation!.generation.toString(),
              instanceId: 0,
              surfaceRevision: 2,
              actionSequence: index + 3,
            },
            invocation,
            directCommandView,
          );
          handleTsRequest({ ...commandRequest, clientInstanceId: state.openClientInstanceId });
          await vi.waitFor(() => expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(index + 2));
          const commandResult = workerResponses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }> => message.kind === "browser-actor-action-result").at(-1)!;
          expect(commandResult).toMatchObject({ actionSequence: index + 3, surfaceRevision: 2, outcome: "guest-applied", mutationCount: index === 0 ? 1 : 0 });
          expect(decodeBrowserActorHostEffectsV1(commandResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
        }
        expect(commandSequences).toEqual([3, 4]);
        expect(commandInvocations).toEqual([directActionInvocation, directCommandInvocation]);
        expect(commandViews).toEqual([directCommandView, directCommandView]);
        console.log("[DEBUG] direct-browser-actor-command: action-invocation=1 command-invocation=1 canonical-page=1 shell-publication=1 raw-backbone-projection=1 singleton-host-effect=1");

        retainInferenceApprovalUndo(operation, approvalReceipt);
        reissueInferenceApprovalUndoForRebootstrap(state);
        const ownerAwaitingA = testSeams.inferenceApprovalUndoOwner!;
        const stateB: ArtifactState = {
          ...state,
          runtimeKey: documentRuntimeKeyV1({ kind: "hub", spaceId: "unrelated-space", documentId: "unrelated-document" }),
          config: { ...state.config, documentId: "unrelated-document", bindings: [{ kind: "hub", baseUrl: fixture.hubOrigin, spaceId: "unrelated-space" }] },
          openClientInstanceId: "unrelated-client",
          browserActorReservation: reservation,
          verifiedColdPair: owner,
        };
        bindInferenceApprovalUndoToMountedPair(stateB, reservation!, owner);
        expect(testSeams.inferenceApprovalUndoOwner).toBe(ownerAwaitingA);
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");
        reservation!.bindApprovalUndoIfMounted();
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("available");

        for (const field of ["directoryRevision", "membershipGeneration", "sessionGeneration"] as const) {
          retainInferenceApprovalUndo(operation, approvalReceipt);
          const originalOwner = testSeams.inferenceApprovalUndoOwner!;
          reissueInferenceApprovalUndoForRebootstrap(state);
          const reissued = testSeams.inferenceApprovalUndoOwner!;
          expect(originalOwner.abort.signal.aborted).toBe(true);
          expect(reissued).toMatchObject({ phase: "awaiting-mount", idempotencyKey: originalOwner.idempotencyKey });
          const rotatedFields = structuredClone(fields);
          rotatedFields.revalidation[field] = (rotatedFields.revalidation[field] ?? 0) + 1;
          const rotatedLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
          state.executionTargetLease = rotatedLease;
          reservation!.bindApprovalUndoIfMounted();
          expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
          expect(historyStatuses.at(-1)?.status).toEqual({ phase: "unavailable", canUndo: false, code: null });
          rotatedLease.drop();
          state.executionTargetLease = lease;
        }

        const shareFields = structuredClone(fields);
        delete shareFields.revalidation.sessionGeneration;
        shareFields.revalidation.shareGeneration = 1;
        const shareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(shareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
        state.executionTargetLease = shareLease;
        retainInferenceApprovalUndo(operation, approvalReceipt);
        reissueInferenceApprovalUndoForRebootstrap(state);
        const rotatedShareFields = structuredClone(shareFields);
        rotatedShareFields.revalidation.shareGeneration = (rotatedShareFields.revalidation.shareGeneration ?? 0) + 1;
        const rotatedShareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedShareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
        state.executionTargetLease = rotatedShareLease;
        reservation!.bindApprovalUndoIfMounted();
        expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
        rotatedShareLease.drop();
        shareLease.drop();
        state.executionTargetLease = lease;

        retainInferenceApprovalUndo(operation, approvalReceipt);
        reservation!.bindApprovalUndoIfMounted();
        const submittingOwner = testSeams.inferenceApprovalUndoOwner!;
        expect(submittingOwner.phase).toBe("available");
        const answerUndo: { resolve: ((response: Response) => void) | null } = { resolve: null };
        globalThis.fetch = async (input) => {
          expect(String(input).endsWith("/inference/gis-map/approval-undos")).toBe(true);
          return await new Promise<Response>((resolve) => {
            answerUndo.resolve = resolve;
          });
        };
        const pendingUndo = undoInferenceApproval(submittingOwner.historyEpoch, submittingOwner.clientInstanceId, submittingOwner.scope);
        await vi.waitFor(() => {
          expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("submitting");
          expect(answerUndo.resolve).not.toBeNull();
        });
        reissueInferenceApprovalUndoForRebootstrap(state);
        const reissuedOwner = testSeams.inferenceApprovalUndoOwner!;
        expect(reissuedOwner.idempotencyKey).toBe(submittingOwner.idempotencyKey);
        expect(submittingOwner.abort.signal.aborted).toBe(true);
        const respondUndo = answerUndo.resolve;
        if (respondUndo === null) throw new Error("approval undo fetch did not enter");
        respondUndo(
          Response.json(
            {
              schema: "semio.hub.gis-map-approval-undo-receipt/v1",
              targetId: approvalReceipt.undo.targetId,
              originalJobId: approvalReceipt.jobId,
              mutationId: "6".repeat(32),
              commandHash: "7".repeat(64),
              applied: true,
              replayed: false,
              frontier: { ...approvalReceipt.undo.expectedCurrent, headEditOrdinal: approvalReceipt.undo.expectedCurrent.headEditOrdinal + 1, headEditId: "undo-edit" },
            },
            { headers: { "x-semio-browser-broker-advanced": "1" } },
          ),
        );
        await pendingUndo;
        expect(testSeams.inferenceApprovalUndoOwner).toBe(reissuedOwner);
        expect(historyStatuses.some((message) => message.historyEpoch === submittingOwner.historyEpoch && message.status.phase === "applied")).toBe(false);
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");

        const renderDriver = reservation as unknown as { child: DocumentBrowserActorChild; renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> };
        const retainedChild = renderDriver.child;
        for (const row of corpus.browserRender.hostile) {
          let turns = 0,
            current = row.name !== "stale-before-turn";
          const child = {
            async invoke(path: readonly string[], args: BrowserActorChildValue[]): Promise<BrowserActorChildValue> {
              expect(path).toEqual(["reactor", "poll"]);
              const events = args[0] as Record<string, any>[];
              expect(events).toEqual([turns === 0 ? { tag: "surface-visible", val: { surface: { instance: 0, surface: windowKindId }, bodyKey, viewState: encodePackValue(state.browserActorViewState!) } } : { tag: "wake" }]);
              turns++;
              if (row.name === "stale-after-turn") current = false;
              return {
                uiPatches: [],
                uiPatchReceipt: null,
                lifecycleReceipt: null,
                status: { tag: row.name === "faulted-turn" ? "faulted" : "more-work" },
                coldPairIngress: row.name === "cold-ingress" ? { tag: "page-accepted", val: { lifetime: activeLifetime!, transferGeneration: owner!.transferGeneration, pageIndex: 0, pageCount: owner!.pageCount } } : { tag: "idle" },
              };
            },
          } as unknown as DocumentBrowserActorChild;
          renderDriver.child = child;
          try {
            await expect(
              renderDriver.renderSurface(child, () => {
                if (!current) throw new Error("fixture-stale-render");
              }),
            ).rejects.toThrow(row.error);
          } finally {
            renderDriver.child = retainedChild;
          }
          expect(turns).toBe(row.turns);
          console.log("[DEBUG] browser-render-refusal: " + row.name + " turns=" + turns);
        }
        dropVerifiedColdDocumentPair(state);
        expect(() => owner!.page({ activationGeneration: reservation!.generation, instanceId: 0, guestLifetime: 1n }, 0)).toThrow("stale owner");
        expect({ loaded, described, lifecycleAcknowledged, pages: pageIndexes.length, networkChunks: bootstrap.chunk_count, terminated }).toEqual({
          loaded: 1,
          described: 1,
          lifecycleAcknowledged: true,
          pages: exact.pageCount,
          networkChunks: 2,
          terminated: 0,
        });
        console.log(
          `[DEBUG] browser-cold-pair-transfer: lazy-render-events=${renderEvents.join(",")} pages=${pageIndexes.length} bytes=${received.byteLength} lifecycle-ack=1 applied=1 patch-ack=1 patch-rejected=1 tiled-map=1 network-chunks=${bootstrap.chunk_count}`,
        );
      } finally {
        closeArtifactRuntime(state.runtimeKey);
        expect(await reservation?.retirementOutcome).toBe("retired");
        expect(state.browserActorViewState).toBeNull();
        expect(terminated).toBe(1);
        expect(documentControls).toEqual(["bind", "retire"]);
        expect(lifecycleClose).toEqual(["close", "accepted-ack", "retired-ack"]);
        expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
        testSeams.executionTargetStatusObserver = null;
        testSeams.workerPostTestSink = null;
        clearLocalBrowserBrokerProof();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
      }
    });

    it("browser GIS viewer exposes localized renderer-unavailable after verified lease", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 }, { headers: { "x-semio-browser-broker-advanced": "1" } });
      });
      const events: ArtifactEvent[] = [];
      const originalPostMessage = (globalThis as unknown as { postMessage?: unknown }).postMessage;
      try {
        await requestDocumentSocketAuthority(harness.state, harness.binding);
        const terminal = harness.statuses.at(-1)!;
        expect(terminal.code).toBe(fixture.expected.rendererState);
        for (const [code, text] of Object.entries(fixture.expected.status)) {
          expect(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1]).toEqual(text);
          expect(documentExecutionTargetStatusRoleV1(code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1)).toBe(fixture.expected.statusRoles[code]);
        }
        expect(Object.keys(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1).sort()).toEqual(Object.keys(fixture.expected.status).sort());
        const workerSource = await (await import("node:fs/promises")).readFile(new URL("../🔨️modules/🏪️store/👷️worker/🟦️.ts", source.url), "utf8");
        const leaseRegion = workerSource.slice(workerSource.indexOf("//#region 🪪️ExecutionTargetLease"), workerSource.indexOf("//#endregion 🪪️ExecutionTargetLease"));
        expect(leaseRegion).not.toContain("loadPluginModule");
        expect(leaseRegion).not.toContain("ActivationRegistry");
        expect(leaseRegion).not.toContain("load_wasm_plugins");
        expect(leaseRegion).not.toContain("attach_backbone");
        expect(Object.values(fixture.expected.rendererClaims).every((claim) => claim === false)).toBe(true);
        harness.state.hubActorReady = true;
        const originalEmit = FakeHubWebSocket.instances.length;
        relayMutationsToHub(harness.state, [sampleEnvelope()]);
        expect(harness.state.outbox).toHaveLength(0);
        expect(FakeHubWebSocket.instances).toHaveLength(originalEmit);
        expect(events).toHaveLength(0);
      } finally {
        (globalThis as unknown as { postMessage?: unknown }).postMessage = originalPostMessage;
        harness.release();
      }
    });

    it("browser document open rejects mismatched and max-plus-one plans and cancels before receipt exchange without leaking authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      FakeHubWebSocket.instances = [];
      testSeams.socketGrantTestIssue = null;
      const openOwner = async (proof: string): Promise<Readonly<{ state: ArtifactState; binding: Extract<PersistenceBinding, { kind: "hub" }> }>> => {
        await acceptCurrentTestBrowserSessionAuthority(proof);
        openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor" });
        const state = artifactState(fixture.intent.scope.documentId)!;
        state.openClientInstanceId = fixture.intent.clientInstanceId;
        const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget };
        artifacts.delete(state.runtimeKey);
        state.config = { ...state.config, bindings: [binding] };
        state.runtimeKey = documentRuntimeKeyForConfig(state.config);
        artifacts.set(state.runtimeKey, state);
        return { state, binding };
      };
      let { state, binding } = await openOwner("a".repeat(64));
      try {
        const rejection = <T>(promise: Promise<T>): Promise<Error> =>
          promise.then(
            () => {
              throw new Error("expected document socket authority rejection");
            },
            (error: unknown) => {
              if (!(error instanceof Error)) throw new Error("document socket authority rejected with a non-Error value");
              return error;
            },
          );
        const hostileReceipt = current.plan.receipt;
        let effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return Response.json(current.plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const unavailable = await rejection(requestDocumentSocketAuthority(state, { kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId }));
        expect(unavailable.message).toBe("document open: installed target unavailable");
        expect(effects).toBe(0);

        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return Response.json({ ...structuredClone(current.plan), scope: { ...current.plan.scope, spaceId: "foreign" } }, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const mismatch = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(mismatch.message).toBe("document open: invalid plan");
        expect(mismatch.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);

        ({ state, binding } = await openOwner("b".repeat(64)));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return new Response("{}", { headers: { "content-length": String(fixture.expected.responseMaxBytes + 1), "x-semio-browser-broker-advanced": "1" } });
        };
        const oversized = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(oversized.message).toBe("document open: invalid plan");
        expect(effects).toBe(1);

        ({ state, binding } = await openOwner("c".repeat(64)));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          state.docAbort.abort();
          return Response.json(current.plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const cancelled = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(cancelled.message).toBe("document open: cancelled");
        expect(cancelled.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);
        expect(FakeHubWebSocket.instances).toHaveLength(0);
      } finally {
        closeArtifact(fixture.intent.scope.documentId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("keeps two concurrent document grant actors isolated and rewrites caller envelopes at the wire boundary", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      const originalBroadcastChannel = globalThis.BroadcastChannel;
      const broadcasts: unknown[] = [];
      class BoundPortBroadcastChannel {
        onmessage: ((event: MessageEvent) => void) | null = null;
        constructor(readonly name: string) {}
        postMessage(value: unknown): void { broadcasts.push(value); }
        close(): void {}
      }
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = BoundPortBroadcastChannel;
      const actorA = `hub.v1.${"a".repeat(64)}`;
      const actorB = `hub.v1.${"b".repeat(64)}`;
      const opaqueNoncanonicalPack = new Uint8Array(Buffer.from("00010111048000", "hex"));
      testSeams.socketGrantTestIssue = async (_baseUrl, path) => ({
        schema: "semio.hub.socket-grant/v1",
        protocol: "semio.socket.v1",
        grant: `socket.v1.${path.includes("doc-a") ? "1".repeat(32) : "2".repeat(32)}.${"3".repeat(64)}`,
        actorId: path.includes("doc-a") ? actorA : actorB,
        expiresAtMs: Number.MAX_SAFE_INTEGER,
      });
      const envelope = (document: string): MutationEnvelope => ({
        id: `edit-${document}`,
        actor: "caller-selected-actor",
        document,
        schemaVersion: "demo/v1",
        deps: [],
        payloadHash: "unused",
        diff: { schemaId: "demo/v1", payload: { document } },
        inverse: { targetOperation: `edit-${document}`, inverseDiff: { schemaId: "demo/v1", payload: {} }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
      });
      const documentBackbone = (value: MutationEnvelope) => ({
        kind: "documentBackbone" as const,
        message: encodeBackboneMessage({
          kind: "mutations",
          envelopes: encodeDocumentBackboneEnvelopeBatchExact([{
            mutation_id: value.id,
            document_id: value.document,
            actor: value.actor,
            dependencies: value.deps ?? [],
            diff: { schema: value.diff.schemaId, payload: value.document === "doc-a" ? opaqueNoncanonicalPack : encodePackValue(value.diff.payload) },
            inverse: { schema: value.inverse.inverseDiff.schemaId, payload: encodePackValue(value.inverse.inverseDiff.payload) },
            timestamp: { actor: 7n, physical_ms: 9_007_199_254_740_992n, logical: 11n },
          }]),
        }),
      });
      try {
        for (const documentId of ["doc-a", "doc-b"]) {
          openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller-selected-actor" });
          installVerifiedDocumentBackbonePair(artifactState(documentId, "space-1")!);
        }
        await flushSocketGrantTurns();
        const [socketA, socketB] = FakeHubWebSocket.instances;
        socketA!.open();
        socketB!.open();
        await handleHubFrame(artifactState("doc-a", "space-1")!, { Session: { actor: actorA, color: 1 } });
        await handleHubFrame(artifactState("doc-b", "space-1")!, { Session: { actor: actorB, color: 2 } });
        handleTsRequest({ kind: "send", documentId: "doc-a", clientInstanceId: artifactState("doc-a", "space-1")!.openClientInstanceId, message: documentBackbone(envelope("doc-a")) });
        handleTsRequest({ kind: "send", documentId: "doc-b", clientInstanceId: artifactState("doc-b", "space-1")!.openClientInstanceId, message: documentBackbone(envelope("doc-b")) });
        const commandEnvelope = (socket: FakeHubWebSocket) => {
          const frame = decodeClientFrame(socket.sent[1]!).frame;
          if (typeof frame === "string" || !("Commands" in frame)) throw new Error("expected commands");
          return frame.Commands.envelopes[0]!;
        };
        expect(commandEnvelope(socketA!).actor).toBe(actorA);
        expect(commandEnvelope(socketB!).actor).toBe(actorB);
        expect(commandEnvelope(socketA!).actor).not.toBe(commandEnvelope(socketB!).actor);
        expect(commandEnvelope(socketA!).diff.payload).toEqual(Array.from(opaqueNoncanonicalPack));
        expect(broadcasts).toEqual([]);
      } finally {
        closeArtifact("doc-a");
        closeArtifact("doc-b");
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

    it("bounds retained document backbone bytes until terminal Ack and drains them on disposal", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      const originalBroadcastChannel = globalThis.BroadcastChannel;
      class BoundPortBroadcastChannel {
        onmessage: ((event: MessageEvent) => void) | null = null;
        postMessage(): void { throw new Error("bound document backbone must not echo before server authority"); }
        close(): void {}
      }
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = BoundPortBroadcastChannel;
      testSeams.socketGrantTestIssue = async () => ({
        schema: "semio.hub.socket-grant/v1",
        protocol: "semio.socket.v1",
        grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
        actorId: `hub.v1.${"3".repeat(64)}`,
        expiresAtMs: Number.MAX_SAFE_INTEGER,
      });
      const documentId = "doc-retained-bytes";
      const actor = `hub.v1.${"3".repeat(64)}`;
      const messageOfSize = (id: string, target: number): Uint8Array => {
        let textBytes = Math.max(0, target - 128);
        for (let turn = 0; turn < 8; turn += 1) {
          const message = encodeBackboneMessage({
            kind: "mutations",
            envelopes: encodeDocumentBackboneEnvelopeBatchExact([{
              mutation_id: id,
              document_id: documentId,
              actor: "caller",
              dependencies: [],
              diff: { schema: "demo/v1", payload: encodePackValue("x".repeat(textBytes)) },
              inverse: { schema: "demo/v1", payload: encodePackValue(null) },
              timestamp: { actor: 1n, physical_ms: 2n, logical: 3n },
            }]),
          });
          if (message.byteLength === target) return message;
          textBytes += target - message.byteLength;
          if (textBytes < 0) break;
        }
        throw new Error(`unable to construct ${target}-byte document backbone message`);
      };
      const outcomes: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => outcomes.push(message);
      try {
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller" });
        await flushSocketGrantTurns();
        const socket = FakeHubWebSocket.instances[0]!;
        socket.open();
        const state = artifactState(documentId, "space-1")!;
        installVerifiedDocumentBackbonePair(state);
        await handleHubFrame(state, { Session: { actor, color: 1 } });
        const targets = [262_144, 262_144, 262_144, 262_143] as const;
        targets.forEach((target, index) => handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: messageOfSize(`edit-${index}`, target) } }));
        expect(state.pendingDocumentBackboneBytes).toBe(DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - 1);
        expect(state.pendingDocumentBackboneMessages).toBe(4);
        const refused = messageOfSize("edit-refused", 128);
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: refused } });
        expect(state.pendingDocumentBackboneBytes).toBe(DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - 1);
        expect(outcomes.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", reason: "document backbone pending capacity" } } });
        handleAck(state, 0, [{ Applied: { outcome: "Accepted" } }]);
        expect(state.pendingDocumentBackboneBytes).toBe(DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - 1 - targets[0]);
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: refused } });
        expect(state.pendingDocumentBackboneBytes).toBe(DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - 1 - targets[0] + refused.byteLength);
        closeArtifact(documentId, "space-1", state.openClientInstanceId);
        expect(state.pendingDocumentBackboneBytes).toBe(0);
        expect(state.pendingDocumentBackboneMessages).toBe(0);
      } finally {
        closeArtifact(documentId, "space-1");
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

    it("fences rebootstrap before mirror retirement and replays only the retained preexisting raw batch after exact catchup", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      const originalBroadcastChannel = globalThis.BroadcastChannel;
      const originalFetch = globalThis.fetch;
      class BoundPortBroadcastChannel {
        onmessage: ((event: MessageEvent) => void) | null = null;
        postMessage(): void { throw new Error("bound document backbone must not echo before server authority"); }
        close(): void {}
      }
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = BoundPortBroadcastChannel;
      testSeams.socketGrantTestIssue = async () => ({
        schema: "semio.hub.socket-grant/v1",
        protocol: "semio.socket.v1",
        grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
        actorId: `hub.v1.${"3".repeat(64)}`,
        expiresAtMs: Number.MAX_SAFE_INTEGER,
      });
      const documentId = "doc-rebootstrap-raw";
      const first: MutationEnvelope = {
        id: "edit-before-rebootstrap",
        actor: "caller",
        document: documentId,
        schemaVersion: "demo/v1",
        deps: [],
        payloadHash: "unused",
        diff: { schemaId: "demo/v1", payload: { n: 1 } },
        inverse: { targetOperation: "edit-before-rebootstrap", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
      };
      const second = { ...first, id: "edit-during-rebootstrap" };
      const duringBootstrap = { ...first, id: "edit-during-bootstrap" };
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      let releaseMirrorRetirement: (() => void) | null = null;
      try {
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller" });
        await flushSocketGrantTurns();
        const state = artifactState(documentId, "space-1")!;
        const frontier = installVerifiedDocumentBackbonePair(state);
        const oldSocket = FakeHubWebSocket.instances.at(-1)!;
        oldSocket.open();
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"3".repeat(64)}`, color: 1 } });
        state.artifactBootstrap = {} as ArtifactState["artifactBootstrap"];
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: exactDocumentBackboneMessage(duringBootstrap) } });
        expect(state.pendingDocumentBackboneBytes).toBe(0);
        expect(state.pendingMutations).toHaveLength(0);
        expect(state.outbox).toHaveLength(0);
        expect(oldSocket.sent).toHaveLength(1);
        state.artifactBootstrap = null;
        const firstMessage = exactDocumentBackboneMessage(first);
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: firstMessage } });
        expect(state.pendingBatches.size).toBe(1);
        expect(state.pendingDocumentBackboneBytes).toBe(firstMessage.byteLength);
        state.canonicalFolderMirror = { binding: { kind: "folder", path: "/tmp/rebootstrap-raw" }, documentId, epoch: 1, capability: "a".repeat(64) };
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          await new Promise<void>((resolve) => { releaseMirrorRetirement = resolve; });
          return new Response(null, { status: 204 });
        };
        const rebootstrap = handleHubFrame(
          state,
          { RebootstrapRequired: { control: { space_id: "space-1", document_id: documentId, checkpoint_id: Array(32).fill(1), descriptor_hash: Array(32).fill(2), baseline_frontier: frontier } } },
          null,
          oldSocket,
        );
        await vi.waitFor(() => expect(state.artifactRebootstrapRequired).toBe(true));
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);

        const sentBeforeRefusal = oldSocket.sent.length;
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: exactDocumentBackboneMessage(second) } });
        expect(state.pendingDocumentBackboneBytes).toBe(firstMessage.byteLength);
        expect(state.pendingMutations.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(oldSocket.sent).toHaveLength(sentBeforeRefusal);
        expect(posted.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", reason: "document backbone canonical pair unavailable" } } });
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "localMutations", envelopes: [second] } });
        expect(state.pendingDocumentBackboneBytes).toBe(firstMessage.byteLength);
        expect(state.pendingMutations.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(oldSocket.sent).toHaveLength(sentBeforeRefusal);
        expect(posted.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", reason: "document backbone canonical pair unavailable", messages: [0] } } });

        await handleHubFrame(state, { Ack: { batch_id: 0, stages: [{ Applied: { outcome: "Accepted" } }], frontier } });
        expect(state.pendingDocumentBackboneBytes).toBe(firstMessage.byteLength);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);
        const oldSocketFrontier = {
          ...frontier,
          head_edit_id: "old-socket-command",
          head_edit_ordinal: frontier.head_edit_ordinal + 1,
          last_commit_seq: frontier.last_commit_seq + 1,
          chain_hash: Array(32).fill(9),
        };
        await handleHubFrame(state, { Commands: { envelopes: [], origin: state.actor, frontier: oldSocketFrontier } }, null, oldSocket);
        expect(state.frontier).toEqual(frontier);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);

        releaseMirrorRetirement?.();
        await rebootstrap;
        expect(state.currentPack).toBeNull();
        expect(state.currentSpr).toBeNull();
        const freshSocket = new FakeHubWebSocket("ws://fresh");
        freshSocket.open();
        state.socket = freshSocket;
        state.hubActorReady = false;
        state.pendingSocketActorId = `hub.v1.${"3".repeat(64)}`;
        await handleHubFrame(state, { Session: { actor: state.pendingSocketActorId, color: 2 } }, null, freshSocket);
        expect(freshSocket.sent).toHaveLength(0);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);
        state.artifactRebootstrapRequired = false;
        state.artifactRebootstrapOwner = null;
        const freshFrontier = installVerifiedDocumentBackbonePair(state);
        state.requiredTailFrontier = freshFrontier;
        await handleHubFrame(state, { Commands: { envelopes: [], origin: state.actor, frontier: freshFrontier } }, null, freshSocket);
        expect(state.outbox).toHaveLength(0);
        expect(state.pendingBatches.size).toBe(1);
        expect(freshSocket.sent).toHaveLength(1);
        const replay = decodeClientFrame(freshSocket.sent[0]!).frame;
        if (typeof replay === "string" || !("Commands" in replay)) throw new Error("expected replayed Commands frame");
        expect(replay.Commands.envelopes.map((envelope) => envelope.mutation_id)).toEqual([first.id]);
      } finally {
        releaseMirrorRetirement?.();
        closeArtifact(documentId, "space-1");
        testSeams.workerPostTestSink = null;
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    it("poll never overlaps itself: concurrent revalidateFolder() calls collapse into one coalesced follow-up", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      let fetchCalls = 0;
      // 🚪️ Gated rather than immediately resolved — the whole point of this test is to fire more
      // calls WHILE one is still in flight, so the fetch must stay pending until we say so.
      const gates: Array<() => void> = [];
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        await new Promise<void>((resolve) => gates.push(resolve));
        return notFoundResponse();
      };

      try {
        openArtifact(folderOnlyConfig("doc-overlap"));
        const state = artifactState("doc-overlap")!;
        await flushMicrotasks(); // let watchFolder's bootstrap read actually START (not settle — it's gated).
        expect(fetchCalls).toBe(1);

        // 🥇️ Three callers race a revalidate while the bootstrap read is still stuck at its gate —
        // `latestWins` must coalesce all three into exactly ONE queued follow-up, never three reruns.
        const call2 = state.revalidateFolder();
        const call3 = state.revalidateFolder();
        const call4 = state.revalidateFolder();
        await flushMicrotasks();
        expect(fetchCalls).toBe(1); // nothing new launched synchronously — still just the bootstrap.

        gates.shift()!(); // let the bootstrap call resolve, which launches the coalesced follow-up.
        await flushMicrotasks();
        expect(fetchCalls).toBe(2); // exactly one follow-up — never a separate rerun per caller.

        gates.shift()!(); // let the follow-up resolve so call2/call3/call4 all settle.
        await Promise.all([call2, call3, call4]);
        expect(fetchCalls).toBe(2);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-overlap");
      }
    });

    it("poll is suppressed while SSE is healthy and resumes once it drops", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      let fetchCalls = 0;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        return notFoundResponse();
      };
      vi.useFakeTimers();
      // 🎯 Deterministic jitter (every delay collapses to its minimum): this test advances fake time
      // across THREE phases in sequence, and leftover jitter slack from an earlier phase could
      // otherwise let two sanity ticks land inside one later advance window — pinning `Math.random`
      // removes that risk instead of just hoping the window is wide enough.
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0);

      try {
        openArtifact(folderOnlyConfig("doc-sanity"));
        const state = artifactState("doc-sanity")!;
        await vi.advanceTimersByTimeAsync(0); // bootstrap read.
        expect(fetchCalls).toBe(1);

        // 🛟️ SSE never opened yet (`sseHealthy` still false) — the sanity fallback must still fire.
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2);

        // 📡️ SSE opens — the very next sanity tick must be a no-op while it stays healthy.
        const source = FakeEventSource.instances.at(-1)!;
        source.onopen?.();
        expect(state.sseHealthy).toBe(true);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2); // suppressed — no new fetch while SSE is healthy.

        // 📴️ SSE drops — the fallback must resume on the next tick.
        source.onerror?.();
        expect(state.sseHealthy).toBe(false);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sanity");
      }
    });

    it("a post-open SSE drop reconnects with jittered backoff", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => notFoundResponse();
      vi.useFakeTimers();

      try {
        openArtifact(folderOnlyConfig("doc-sse-reconnect"));
        await vi.advanceTimersByTimeAsync(0);
        expect(FakeEventSource.instances).toHaveLength(1);

        const first = FakeEventSource.instances[0]!;
        first.onopen?.();
        first.onerror?.(); // drops AFTER a successful open — the bug finding 2 is about.
        expect(first.closed).toBe(true);

        // 🔁️ Reconnect is jittered within [SSE_RECONNECT_MIN_MS, SSE_RECONNECT_MAX_MS] — advancing
        // past the max guarantees the next attempt has fired regardless of the random draw.
        await vi.advanceTimersByTimeAsync(SSE_RECONNECT_MAX_MS + 1);
        expect(FakeEventSource.instances.length).toBeGreaterThan(1);
      } finally {
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sse-reconnect");
      }
    });

    it("abort on close cancels an in-flight folder fetch", async () => {
      let capturedSignal: AbortSignal | undefined;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = (_url: string, init?: RequestInit) => {
        capturedSignal = init?.signal ?? undefined;
        return new Promise(() => {}); // never settles — only `closeArtifact` can end this.
      };
      const originalEventSource = (globalThis as unknown as { EventSource: unknown }).EventSource;
      (globalThis as unknown as { EventSource: unknown }).EventSource = class {
        constructor() {
          throw new Error("no SSE in this test");
        }
      };

      try {
        openArtifact(folderOnlyConfig("doc-abort"));
        await Promise.resolve();
        expect(capturedSignal).toBeDefined();
        expect(capturedSignal?.aborted).toBe(false);

        closeArtifact("doc-abort");
        expect(capturedSignal?.aborted).toBe(true);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { EventSource: unknown }).EventSource = originalEventSource;
      }
    });

    it("queue overflow rejects and reports rather than dropping silently", () => {
      const config: ArtifactActorConfig = { documentId: "doc-overflow", schema: "demo/v1", bindings: [], actor: "actor-1" };
      openArtifact(config);
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

      try {
        const state = artifactState("doc-overflow")!;
        const makeEnvelope = (index: number): MutationEnvelope => ({
          id: `edit-${index}`,
          actor: "actor-1",
          document: "doc-overflow",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: index } },
          inverse: { targetOperation: `edit-${index}`, inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        });
        const overSized = Array.from({ length: PENDING_MUTATIONS_QUEUE_LIMIT + 1 }, (_unused, index) => makeEnvelope(index));

        handleTsRequest({ kind: "send", documentId: "doc-overflow", clientInstanceId: state.openClientInstanceId, message: { kind: "localMutations", envelopes: overSized } });

        // 🚨️ Rejected wholesale, never partially accepted or silently dropped — the queue is
        // untouched, and the rejection is explicitly logged (the shell-facing signal is the same
        // `commandOutcome`/`rejected` vocabulary a real hub rejection uses — see
        // `rejectMutationQueueOverflow`'s doc comment).
        expect(state.pendingMutations).toHaveLength(0);
        expect(errorSpy).toHaveBeenCalledWith("[backbone-worker] pending mutation queue full, rejecting batch", "doc-overflow", overSized.length);

        // ✅ A batch that fits is still accepted normally — overflow doesn't wedge the queue shut.
        handleTsRequest({ kind: "send", documentId: "doc-overflow", clientInstanceId: state.openClientInstanceId, message: { kind: "localMutations", envelopes: [makeEnvelope(0)] } });
        expect(state.pendingMutations).toHaveLength(1);
      } finally {
        errorSpy.mockRestore();
        closeArtifact("doc-overflow");
      }
    });

    it("a mutation made while offline is queued, then flushed once the hub reconnects (Welcome flushes the outbox)", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-flush", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-flush")!;
        installVerifiedDocumentBackbonePair(state);
        const socket = FakeHubWebSocket.instances.at(-1)!;
        expect(socket.readyState).toBe(FakeHubWebSocket.CONNECTING);

        const envelope: MutationEnvelope = {
          id: "edit-offline-1",
          actor: "actor-1",
          document: "doc-hub-flush",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: 1, sequenceNumber: 1 } },
          inverse: { targetOperation: "edit-offline-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        const opaqueNoncanonicalPack = new Uint8Array(Buffer.from("00010111048000", "hex"));
        const rawMessage = exactDocumentBackboneMessage(envelope, opaqueNoncanonicalPack);
        handleTsRequest({ kind: "send", documentId: "doc-hub-flush", clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: rawMessage } });

        // 📴️ Socket isn't open yet — the mutation is queued in the outbox, never silently dropped.
        expect(state.outbox).toHaveLength(1);
        expect(socket.sent).toHaveLength(0);
        expect(state.pendingMutations).toHaveLength(1);
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);

        // 🔌️ Hub (re)connects: `Hello` goes out on open, then the hub answers with `Welcome`.
        socket.open();
        expect(socket.sent).toHaveLength(1); // Hello

        const welcome: ServerFrame = {
          Welcome: {
            session_id: "s1",
            resume_token: "resume-1",
            server_frontier: { document_id: "doc-hub-flush", head_edit_ordinal: 0, head_edit_id: "e0", last_commit_seq: 0, chain_hash: new Array(32).fill(0) },
            bootstrap: "None",
          },
        };
        socket.onmessage?.({ data: encodeServerFrame(welcome, "command").buffer as ArrayBuffer });
        await state.hubFrameChain;

        expect(state.outbox).toHaveLength(1);
        expect(socket.sent).toHaveLength(1);
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"3".repeat(64)}`, color: 4 } });

        // ♻️ Exact Session authority, not Welcome alone, flushes the outbox.
        expect(state.outbox).toHaveLength(0);
        expect(socket.sent).toHaveLength(2); // Hello + the flushed Commands batch.
        const commandsFrame = decodeClientFrame(socket.sent[1]!).frame;
        if (typeof commandsFrame === "string" || !("Commands" in commandsFrame)) throw new Error("expected a Commands frame");
        expect(commandsFrame.Commands.envelopes).toHaveLength(1);
        expect(commandsFrame.Commands.envelopes[0]!.mutation_id).toBe("edit-offline-1");
        expect(commandsFrame.Commands.envelopes[0]!.diff.payload).toEqual(Array.from(opaqueNoncanonicalPack));
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);
        expect(state.pendingBatches.size).toBe(1); // now awaiting `Ack` — no longer in the outbox.
      } finally {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-flush");
      }
    });

    it("a batch whose socket dies before Ack moves back into the outbox instead of being stranded", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-stranded", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-stranded")!;
        installVerifiedDocumentBackbonePair(state);
        const socket = FakeHubWebSocket.instances.at(-1)!;
        socket.open();
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"3".repeat(64)}`, color: 5 } });

        const envelope: MutationEnvelope = {
          id: "edit-inflight-1",
          actor: "actor-1",
          document: "doc-hub-stranded",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: 1, sequenceNumber: 1 } },
          inverse: { targetOperation: "edit-inflight-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        const rawMessage = exactDocumentBackboneMessage(envelope);
        handleTsRequest({ kind: "send", documentId: "doc-hub-stranded", clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: rawMessage } });
        expect(state.pendingBatches.size).toBe(1); // socket was open — sent immediately, awaiting Ack.
        expect(state.outbox).toHaveLength(0);
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);

        // 💥️ The socket dies before the hub ever acks — the batch must not be lost.
        socket.close();
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox).toHaveLength(1);
        expect(state.outbox[0]!.id).toBe("edit-inflight-1");
        expect(state.pendingMutations).toHaveLength(1); // status-visible pending count is unaffected.
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);
      } finally {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-stranded");
      }
    });

    // 🧬️ Coordinator follow-up (finding 4b): `retryWithJitteredBackoff`'s attempt counter grows for
    // the life of one call and never resets after success — `connectHub`/`connectSseOnce` now loop
    // fresh calls via `reconnectForever`, resetting only after SUSTAINED health, never on "socket
    // opened" alone. `Math.random` is pinned throughout (not to 0 — that would collapse every
    // jittered delay to its floor and hide growth entirely) so the exact backoff value at every
    // attempt is a known, computable number, making "did it actually reset" a precise assertion
    // rather than a coincidence of timing windows.
    it("a hub drop after sustained health resets the backoff, unlike continued accumulation", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-reset", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();

        // 💥️ Two quick failures BEFORE any sustained health — attempt 1 → 750ms, attempt 2 → 1250ms
        // (both exact with `Math.random` pinned at 0.5: `minMs + 0.5*(min(maxMs,minMs*2**attempt)-minMs)`).
        FakeHubWebSocket.instances[0]!.close();
        await vi.advanceTimersByTimeAsync(751);
        expect(FakeHubWebSocket.instances).toHaveLength(2);
        FakeHubWebSocket.instances[1]!.close();
        await vi.advanceTimersByTimeAsync(1251);
        expect(FakeHubWebSocket.instances).toHaveLength(3);

        // ✅️ Third attempt opens and stays up long enough to count as sustainedly healthy.
        const healthy = FakeHubWebSocket.instances[2]!;
        healthy.open();
        await vi.advanceTimersByTimeAsync(SUSTAINED_HEALTHY_MS + 1);

        // 📴️ NOW it drops. If the attempt counter had kept accumulating, the next attempt (attempt 3)
        // would wait 2250ms. A reset instead starts a brand-new call — its first failure is attempt 1,
        // waiting only 750ms.
        healthy.close();
        await vi.advanceTimersByTimeAsync(0); // let the resolved promise's fresh retryWithJitteredBackoff call fire its immediate first attempt.
        expect(FakeHubWebSocket.instances).toHaveLength(4); // the fresh call's immediate (0-delay) first attempt.
        FakeHubWebSocket.instances[3]!.close(); // that immediate attempt also fails fast.

        // 🎯 The decisive check: 800ms is enough for the RESET value (750ms) but not enough for what
        // continued accumulation would have required (2250ms).
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(5);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-reset");
      }
    });

    it("rapid accept-then-drop cycling does NOT reset the hub backoff — it keeps climbing", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-no-reset", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();

        // 🔁️ Each cycle opens (well under `SUSTAINED_HEALTHY_MS`) then drops immediately — never
        // healthy long enough to reset. Attempt 1 → 750ms, attempt 2 → 1250ms, attempt 3 → 2250ms.
        FakeHubWebSocket.instances[0]!.open();
        FakeHubWebSocket.instances[0]!.close();
        // ⏱️ 800ms is past attempt 1's 750ms floor but nowhere near attempt-2-sized delays — confirms
        // the wait is climbing on schedule, not staying flat at the floor.
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2);

        FakeHubWebSocket.instances[1]!.open();
        FakeHubWebSocket.instances[1]!.close();
        // 🎯 The decisive check: 800ms was enough after attempt 1 (750ms) but must NOT be enough here —
        // if this fired, the counter would have wrongly reset back down near the floor.
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2); // still 2 — attempt 2's 1250ms hasn't elapsed.
        await vi.advanceTimersByTimeAsync(451); // 800 + 451 = 1251 total, past attempt 2's 1250ms.
        expect(FakeHubWebSocket.instances).toHaveLength(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-no-reset");
      }
    });

    it("abort cancels the hub reconnect loop promptly, with no leaked timer", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-abort", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        FakeHubWebSocket.instances[0]!.open(); // sustained-health timer now pending too.

        closeArtifact("doc-hub-abort");
        await vi.advanceTimersByTimeAsync(0);

        // 🧹️ Nothing left pending — not the sustained-health timer, not a reconnect backoff delay.
        expect(vi.getTimerCount()).toBe(0);

        // 🚫️ …and no further reconnect attempt ever happens, however long we wait.
        await vi.advanceTimersByTimeAsync(60_000);
        expect(FakeHubWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("an SSE drop after sustained health resets ITS backoff too (the same fix applied to connectSseOnce)", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => notFoundResponse();
      vi.useFakeTimers();
      // 🎯 SSE's own formula: `minMs=1000, maxMs=30000` → attempt 1 = 1000+0.5*(2000-1000)=1500ms.
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        openArtifact(folderOnlyConfig("doc-sse-reset"));
        await vi.advanceTimersByTimeAsync(0); // bootstrap read + first SSE connect attempt.
        expect(FakeEventSource.instances).toHaveLength(1);

        const healthy = FakeEventSource.instances[0]!;
        healthy.onopen?.();
        await vi.advanceTimersByTimeAsync(SUSTAINED_HEALTHY_MS + 1);
        healthy.onerror?.(); // drops AFTER sustained health.
        await vi.advanceTimersByTimeAsync(0); // fresh reconnectForever cycle's immediate first attempt.
        expect(FakeEventSource.instances).toHaveLength(2);

        // 🎯 That fresh attempt also fails fast — the wait before the NEXT one must be the reset
        // (attempt 1 ≈ 1500ms), not a continuation of any prior accumulation (there was none yet in
        // this call, so this mirrors the hub test's decisive-window shape at SSE's own numbers).
        FakeEventSource.instances[1]!.onerror?.();
        await vi.advanceTimersByTimeAsync(1600);
        expect(FakeEventSource.instances).toHaveLength(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sse-reset");
      }
    });
  });
  //#endregion 🔖️OfflineResilienceTests

  describe("backbone-worker scope-safe presence", () => {
    it("binds verified surface authority only after the exact socket Session and isolates equal document ids", async () => {
      const responses: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => responses.push(message);
      const socketA = { close: vi.fn() } as unknown as WebSocket;
      const socketB = { close: vi.fn() } as unknown as WebSocket;
      const makeState = (spaceId: string, socket: WebSocket): ArtifactState =>
        ({
          config: { documentId: "same-document", schema: "demo/v1", actor: "requested", bindings: [{ kind: "hub", baseUrl: "https://hub.example", spaceId }] },
          socket,
          actor: "",
          hubActorReady: false,
          pendingSocketActorId: `hub.v1.${spaceId === "space-a" ? "a" : "b"}`.padEnd(71, spaceId === "space-a" ? "a" : "b"),
          presenceAuthority: null,
          outbox: [],
          sessionColor: null,
        }) as unknown as ArtifactState;
      const stateA = makeState("space-a", socketA);
      const stateB = makeState("space-b", socketB);
      const actorA = stateA.pendingSocketActorId!;
      const actorB = stateB.pendingSocketActorId!;
      try {
        await handleHubFrame(stateA, { Session: { actor: actorA, color: 2 } }, { socket: socketA, scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor" });
        await handleHubFrame(stateB, { Session: { actor: actorB, color: 5 } }, { socket: socketB, scope: { spaceId: "space-b", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#viewer" });
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, label: "Ada", connectedAtMs: 101, color: 2, surface: "map@1/*#editor", views: [] }] });
        emitEvent(stateB, { kind: "presence", peers: [{ actor: actorB, label: "Berta", connectedAtMs: 202, color: 5, surface: "map@1/*#viewer", views: [] }] });
        const presence = responses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "event" }> => message.kind === "event" && message.event.kind === "presence");
        expect(presence.map((message) => [message.scope?.spaceId, message.verifiedSurfaceId, message.event.kind === "presence" ? message.event.peers[0]?.label : undefined])).toEqual([
          ["space-a", "map@1/*#editor", "Ada"],
          ["space-b", "map@1/*#viewer", "Berta"],
        ]);
        emitEvent(stateA, { kind: "presence", peers: [] });
        stateA.presenceAuthority = null;
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, connectedAtMs: 303, surface: "map@1/*#editor", views: [] }] });
        const tail = responses.slice(-2) as Extract<BackboneWorkerResponse, { kind: "event" }>[];
        expect(tail[0]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor", event: { kind: "presence", peers: [] } });
        expect(tail[1]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, event: { kind: "presence", peers: [] } });
        expect(tail[1]?.verifiedSurfaceId).toBeUndefined();
      } finally {
        testSeams.workerPostTestSink = null;
      }
    });
  });

}
