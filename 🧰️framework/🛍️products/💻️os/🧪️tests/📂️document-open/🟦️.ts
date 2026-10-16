/** 📂️ Browser document open: attempt ownership, installed-artifact authority and the authenticated plan and receipt exchange. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, FakeHubWebSocket, WORKER_LAW_CAPABILITY } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerRequest = import("../../🟦️.ts").BackboneWorkerRequest;
type DocumentOpenIntentV1 = import("@semio-tech/framework/directory/schema").DocumentOpenIntentV1;
type DocumentOpenPlanV1 = import("@semio-tech/framework/directory/schema").DocumentOpenPlanV1;
type PersistenceBinding = import("../../🟦️.ts").PersistenceBinding;
type RustWorkerHost = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").RustWorkerHost;
type DocumentSocketGrantReceiptV1 = import("../../🟦️.ts").DocumentSocketGrantReceiptV1;
type BrowserDocumentOpenFixture = {
  nowMs: number;
  intent: DocumentOpenIntentV1;
  installedTarget: NonNullable<Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>["installedTarget"]>;
  plan: DocumentOpenPlanV1;
  socketGrant: DocumentSocketGrantReceiptV1;
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
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDocumentOpenTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw, acceptCurrentTestBrowserSessionAuthority } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, dispatchBackboneWorkerRequest, documentExecutionOwners, documentRuntimeKeyV1, decodeBackboneWorkerRequest, openArtifact, artifacts, artifactState, closeArtifact, documentOpenPlanAuthority, decodeClientFrame, handleHubFrame, clearHubSessionCapability, documentRuntimeKeyForConfig, requestDocumentSocketAuthority } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    async function browserDocumentOpenFixture(): Promise<BrowserDocumentOpenFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", source.url), "utf8")) as BrowserDocumentOpenFixture;
    }

    function currentBrowserDocumentOpenFixture(fixture: BrowserDocumentOpenFixture): { plan: DocumentOpenPlanV1; grant: DocumentSocketGrantReceiptV1 } {
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
        bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId, installedTarget: fixture.installedTarget }],
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
        expect(documentExecutionOwners.has(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId }))).toBe(false);
      } finally {
        documentExecutionOwners.delete(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId }));
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
        bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
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
      testSeams.documentSocketGrantTestIssue = async () => current.grant;
      const left = fixture.expected.scopeIsolation.left;
      const right = fixture.expected.scopeIsolation.right;
      try {
        openArtifact({
          documentId: left.documentId,
          schema: fixture.installedTarget.artifact.schema,
          bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: left.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        openArtifact({
          documentId: right.documentId,
          schema: fixture.installedTarget.artifact.schema,
          bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: right.spaceId, installedTarget: fixture.installedTarget }],
          actor: "caller-selected-actor",
        });
        expect(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...left })).toBe(fixture.expected.scopeIsolation.leftKey);
        expect(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...right })).toBe(fixture.expected.scopeIsolation.rightKey);
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
        testSeams.documentSocketGrantTestIssue = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open requires exact installed package artifact and surface authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const config: ArtifactActorConfig = {
        documentId: fixture.intent.scope.documentId,
        schema: fixture.installedTarget.artifact.schema,
        bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
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
      testSeams.documentSocketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: RequestInit) => {
        const url = String(input);
        const body = String(init?.body ?? "");
        requests.push({ url, headers: new Headers(init?.headers), body });
        const response = requests.length === 1 ? current.plan : current.grant;
        return Response.json(response);
      };
      try {
        openArtifact({
          documentId: fixture.intent.scope.documentId,
          schema: fixture.plan.artifact.schema,
          bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
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
        expect(requests.every(({ headers }) => headers.get("authorization") === `Bearer ${WORKER_LAW_CAPABILITY}`)).toBe(true);
        expect(socket.url).toBe(`ws://hub.test${fixture.expected.webSocketPath}`);
        expect(socket.protocols).toEqual([fixture.expected.protocol, WORKER_LAW_CAPABILITY]);
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
        clearHubSessionCapability();
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
      testSeams.documentSocketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      let effects = 0;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        effects += 1;
        return Response.json(effects === 1 ? current.plan : current.grant);
      };
      try {
        openArtifact({
          documentId: fixture.intent.scope.documentId,
          schema: fixture.plan.artifact.schema,
          bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
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
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open rejects mismatched and max-plus-one plans and cancels before receipt exchange without leaking authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      FakeHubWebSocket.instances = [];
      testSeams.documentSocketGrantTestIssue = null;
      const openOwner = async (proof: string): Promise<Readonly<{ state: ArtifactState; binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> }>> => {
        await acceptCurrentTestBrowserSessionAuthority(proof);
        openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor" });
        const state = artifactState(fixture.intent.scope.documentId)!;
        state.openClientInstanceId = fixture.intent.clientInstanceId;
        const binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> = { kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget };
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
          return Response.json(current.plan);
        };
        const unavailable = await rejection(requestDocumentSocketAuthority(state, { kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId }));
        expect(unavailable.message).toBe("document open: installed target unavailable");
        expect(effects).toBe(0);

        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return Response.json({ ...structuredClone(current.plan), scope: { ...current.plan.scope, spaceId: "foreign" } });
        };
        const mismatch = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(mismatch.message).toBe("document open: invalid plan");
        expect(mismatch.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);

        ({ state, binding } = await openOwner("b".repeat(64)));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return new Response("{}", { headers: { "content-length": String(fixture.expected.responseMaxBytes + 1) } });
        };
        const oversized = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(oversized.message).toBe("document open: invalid plan");
        expect(effects).toBe(1);

        ({ state, binding } = await openOwner("c".repeat(64)));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          state.docAbort.abort();
          return Response.json(current.plan);
        };
        const cancelled = await rejection(requestDocumentSocketAuthority(state, binding));
        expect(cancelled.message).toBe("document open: cancelled");
        expect(cancelled.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);
        expect(FakeHubWebSocket.instances).toHaveLength(0);
      } finally {
        closeArtifact(fixture.intent.scope.documentId);
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
