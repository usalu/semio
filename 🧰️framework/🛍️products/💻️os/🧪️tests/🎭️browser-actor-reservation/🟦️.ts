/** 🎭️ Browser document actor reservation: private grant, scope, generation and retirement laws and activation on an exact socket Session. */
import { stubFetch } from "../🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, executionTargetBytes, FakeHubWebSocket, WORKER_LAW_CAPABILITY, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource, ExecutionTargetLeaseFixture } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type PersistenceBinding = import("../../🟦️.ts").PersistenceBinding;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerBrowserActorReservationTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetBodyResponse } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1, closeArtifactRuntime, reserveDocumentBrowserActorChild, hubBinding, dropDocumentExecutionTargetLease, browserActorChildCapacity, decodePackValue, encodePackValue, clearHubSessionCapability, installHubSessionCapability, connectHubOnce, encodeServerFrame } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    it("browser document actor reservation obeys private grant, scope, generation and retirement laws", async () => {
      const fixture = await executionTargetLeaseFixture();
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧵️browser-actor-reservation-v1.json", source.url), "utf8"));
      const originalWorker = globalThis.Worker,
        originalFetch = globalThis.fetch;
      let bodyRequests = 0;
      globalThis.fetch = stubFetch(async () => {
        bodyRequests++;
        throw new Error("fixture forbids body fetch");
      });
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
        openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "untrusted-ui-actor", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        const state = artifactState(documentId)!;
        artifacts.delete(state.runtimeKey);
        const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId } as const;
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
          descriptor = executionTargetBytes(fixture.descriptorHex);
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
          expect(entry.descriptor[0]).toBe(executionTargetBytes(fixture.descriptorHex)[0]);
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
        entry.lease.drop();
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
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧵️browser-actor-session-v1.json", source.url), "utf8"));
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex),
        descriptor = executionTargetBytes(fixture.descriptorHex),
        actor = new TextEncoder().encode("abc");
      const originalFetch = globalThis.fetch,
        originalWorker = globalThis.Worker,
        originalSocket = globalThis.WebSocket;
      /** 🔎️ What the row had reached when a wait ran out — a bare deadline names neither the row nor
       * the stage it stalled at, which is the whole diagnosis for this law. */
      const census = { read: () => "no row in flight" };
      const wait = async (done: () => boolean) => {
        const deadline = Date.now() + 3000;
        while (!done()) {
          if (Date.now() > deadline) throw new Error(`session activation test deadline — ${census.read()}`);
          await new Promise((resolve) => setTimeout(resolve, 2));
        }
      };
      const guest = decodePackValue(descriptor) as Record<string, any>;
      try {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
        for (const row of corpus.cases) {
          let bodies = 0,
            loads = 0,
            describes = 0,
            activated = 0,
            pairRequests = 0;
          const workers: SessionWorker[] = [],
            records: { fixture: ExecutionTargetLeaseFixture; state: ArtifactState; socket: FakeHubWebSocket; connected: Promise<void> }[] = [];
          const streams: { stream: ReadableStream<Uint8Array>; chunk: Uint8Array; cancelled: number }[] = [];
          const requests: { url: string; method: string; body: string }[] = [];
          const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
          const bodyProgress = new Set<string>();
          census.read = () => JSON.stringify({ name: row.name, bodies, loads, describes, activated, statuses: statuses.map((status) => status.diagnostic ?? status.code ?? status.progress?.stage ?? "?") });
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
                    value: { uiPatches: [], effects: [], presence: [], nextWake: null, status: { tag: "idle" }, fuelUsed: 1n, commandIngress: { kind: 0, cursor: { owner: 0n, generation: 0n, commandIndex: 0, commandCount: 0, instance: 0, seq: 0n, kind: 0, pageIndex: 0, pageCount: 0, itemCount: 0, metadata: 0 }, fault: [] }, coldPairIngress: { tag: "idle" }, lifecycleReceipt, uiPatchReceipt: null },
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
          testSeams.documentSocketGrantTestIssue = null;
          testSeams.executionTargetStatusObserver = (status) => {
            statuses.push(status);
            if (status.progress?.stage === "manifest") bodyProgress.delete(status.spaceId);
            if (status.progress?.stage === corpus.progressStage) {
              bodyProgress.add(status.spaceId);
              if (row.name === "client-change-during-progress") records[0]!.state.openClientInstanceId = "replaced-by-progress-observer";
            }
            if (status.code === "renderer-unavailable") bodyProgress.delete(status.spaceId);
          };
          clearHubSessionCapability();
          expect(installHubSessionCapability(WORKER_LAW_CAPABILITY)).toBe(true);
          globalThis.fetch = stubFetch(async (input, init) => {
            const url = String(input);
            requests.push({ url, method: String(init?.method ?? "GET"), body: String(init?.body ?? "") });
            const record = records.find((record) => url.includes("/spaces/" + encodeURIComponent(record.fixture.intent.scope.spaceId) + "/"));
            if (!record) throw new Error("unowned session fixture request");
            const current = record.fixture;
            if (url.endsWith("/open-plan")) return Response.json(current.plan);
            if (url.endsWith("/execution-target/manifest")) return Response.json(current.manifest);
            if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
            if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
            if (url.endsWith("/active-checkpoint/pair")) {
              pairRequests++;
              return new Response("", { status: row.name === "canonical-pair-refused" ? 404 : 503 });
            }
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
                return new Response(retained.stream, { headers: { "content-length": "3" } });
              }
              return executionTargetBodyResponse(row.name === "body-hash-mismatch" ? new Uint8Array([0, 0, 0]) : actor);
            }
            return Response.json(current.socketGrant);
          });
          const setup = async (spaceId: string) => {
            const current = { ...structuredClone(fixture), socketGrant: { ...structuredClone(fixture.socketGrant), expiresAtMs: Date.now() + 25000 } };
            current.intent.scope.spaceId = spaceId;
            current.plan.scope.spaceId = spaceId;
            current.manifest.scope.spaceId = spaceId;
            current.plan.expiresAtUnixMs = Date.now() + 30000;
            openArtifact({ documentId: current.intent.scope.documentId, schema: current.plan.artifact.schema, bindings: [], actor: "untrusted-ui-actor", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
            const state = artifactState(current.intent.scope.documentId)!;
            state.openClientInstanceId = current.intent.clientInstanceId;
            artifacts.delete(state.runtimeKey);
            const binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> = { kind: "hub", dataClass: "persistedShared", baseUrl: current.hubOrigin, spaceId, requestedSurfaceId: current.intent.requestedSurfaceId };
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
            if (row.name === "canonical-pair-busy") {
              await wait(() => pairRequests >= 2);
              expect(first.state.executionTargetLease).toBe(initialLease);
              expect(initialLease.live).toBe(true);
              expect(statuses.some((status) => status.code === "integrity-failed")).toBe(false);
            }
            if (row.name === "canonical-pair-refused") {
              expect(pairRequests).toBe(1);
              expect(statuses.some((status) => status.code === "integrity-failed" && status.diagnostic === "canonical checkpoint pair: unavailable (status 404)")).toBe(true);
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
              if (outcome !== undefined && outcome !== null) retirementOutcomes.push(outcome);
              await record.connected.catch(() => {});
            }
            await Promise.all(retirementOutcomes);
            await wait(() => workers.every((worker) => worker.terminations === 1));
            for (const worker of workers) expect(worker.terminations).toBe(1);
            expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
            testSeams.executionTargetStatusObserver = null;
            clearHubSessionCapability();
          }
        }
        console.log("document-session-activation: socket-path=1 neutral=" + corpus.cases.length + " worker=mocked final-capacity=0");
      } finally {
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
