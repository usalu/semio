/** 🎯️ Execution target lease: server asset verification, hostile asset refusal, single-field substitution and bounded body readers. */
import { semioSchemaAjvV1 } from "../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { stubFetch } from "../🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, FakeHubWebSocket, executionTargetBytes, WORKER_LAW_CAPABILITY, sampleEnvelope } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerExecutionTargetLeaseTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetBodyResponse, executionTargetHarness } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, documentRuntimeKeyV1, handleTsRequest, artifacts, hubBinding, sameLeaseFieldsV1, decodeClientFrame, closeArtifact, closeArtifactRuntime, clearHubSessionCapability, installHubSessionCapability, encodePackValue, decodePackValue, executionTargetSha256Hex, browserExecutionTargetAssetRequest, requestDocumentSocketAuthority, relayMutationsToHub, readExecutionTargetBody, dropDocumentExecutionTargetLease, documentExecutionTargetStatusRoleV1 } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    function executionTargetMutate(source: unknown, path: string, value: unknown): Record<string, unknown> {
      const candidate = structuredClone(source) as Record<string, unknown>;
      const segments = path.split(".");
      let cursor = candidate as Record<string, unknown>;
      for (const segment of segments.slice(0, -1)) cursor = cursor[segment] as Record<string, unknown>;
      cursor[segments.at(-1)!] = value;
      return candidate;
    }

    it("browser document first open verifies server assets without a prior installed target", async () => {
      const fixture = await executionTargetLeaseFixture();
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧪️tests/🧫️fixtures/📍️scope/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧬️schema/🔣️.json", source.url), "utf8")) as { $id: string };
      const validators = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(schema);
      
      
      
      const originalFetch = globalThis.fetch,
        originalSocket = globalThis.WebSocket,
        originalPost = testSeams.workerPostTestSink;
      const requests: { stage: string; body: Record<string, unknown> }[] = [];
      const posted: BackboneWorkerResponse[] = [];
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...structuredClone(fixture.socketGrant), expiresAtMs: Date.now() + 25_000 };
      const scope = fixture.intent.scope;
      const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
      testSeams.documentSocketGrantTestIssue = null;
      FakeHubWebSocket.instances = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      globalThis.fetch = stubFetch(async (input, init) => {
        const stage = String(input).split("/").at(-1)!;
        const body = JSON.parse(String(init?.body));
        requests.push({ stage, body });
        expect(init?.method).toBe("POST");
        if (stage === "open-plan") return Response.json(plan);
        if (stage === "manifest") return Response.json(fixture.manifest);
        if (stage === "component") return executionTargetBodyResponse(executionTargetBytes(fixture.componentHex));
        if (stage === "descriptor") return executionTargetBodyResponse(executionTargetBytes(fixture.descriptorHex));
        expect(stage).toBe("socket-grants");
        return Response.json(grant);
      });
      try {
        handleTsRequest({
          kind: "open",
          documentId: scope.documentId,
          schema: plan.artifact.schema,
          actor: "caller-is-not-authority",
          bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId }],
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
        handleTsRequest({ kind: "open", documentId: unselectedId, schema: plan.artifact.schema, actor: "untrusted", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId }] });
        expect(posted.filter((message) => message.kind === "socket-actor-failed" && message.documentId === unselectedId)).toHaveLength(corpus.firstOpen.unselectedSocketFailures);
        closeArtifact(unselectedId, scope.spaceId);
        expect(requests.map(({ stage }) => stage)).toEqual(corpus.firstOpen.requestStages);
      } finally {
        closeArtifactRuntime(runtimeKey);
        closeArtifact("local-first-opening");
        closeArtifact("unselected-first-opening", scope.spaceId);
        clearHubSessionCapability();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
        testSeams.workerPostTestSink = originalPost;
      }
    });

    it("browser document first open rejects hostile assets and retired owners before socket authority", async () => {
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧪️tests/🧫️fixtures/📍️scope/🔣️.json", source.url), "utf8"));
      const originalFetch = globalThis.fetch,
        originalSocket = globalThis.WebSocket,
        originalPost = testSeams.workerPostTestSink;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      testSeams.documentSocketGrantTestIssue = null;
      try {
        for (const row of corpus.firstOpen.hostile) {
          const fixture = await executionTargetLeaseFixture();
          const scope = fixture.intent.scope;
          const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
          const stages: string[] = [];
          const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
          const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
          if (row.id === "foreign-plan-scope") plan.scope = { ...plan.scope, spaceId: "foreign-space" };
          FakeHubWebSocket.instances = [];
          clearHubSessionCapability();
          expect(installHubSessionCapability(WORKER_LAW_CAPABILITY)).toBe(true);
          testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
          testSeams.workerPostTestSink = () => {};
          globalThis.fetch = stubFetch(async (input) => {
            const stage = String(input).split("/").at(-1)!;
            stages.push(stage);
            if (stage === "open-plan") return Response.json(plan);
            if (stage === "manifest") {
              if (row.id === "cancel-at-manifest") artifacts.get(runtimeKey)!.docAbort.abort();
              return Response.json(fixture.manifest);
            }
            if (stage === "component") {
              const bytes = executionTargetBytes(fixture.componentHex);
              if (row.id === "corrupt-component") bytes[0] = bytes[0]! ^ 255;
              if (row.id === "retire-at-component") closeArtifactRuntime(runtimeKey);
              return executionTargetBodyResponse(bytes);
            }
            expect(stage).toBe("descriptor");
            return executionTargetBodyResponse(executionTargetBytes(fixture.descriptorHex));
          });
          try {
            handleTsRequest({
              kind: "open",
              documentId: scope.documentId,
              schema: fixture.plan.artifact.schema,
              actor: "untrusted",
              bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId }],
            });
            const state = artifacts.get(runtimeKey);
            const deadline = Date.now() + 5_000;
            while (!statuses.some(({ code }) => code === "integrity-failed" || code === "cancelled") && !state?.docAbort.signal.aborted && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 0));
            expect(stages, row.id).toEqual(row.requestStages);
            expect(testSeams.hubSessionCapabilityHeld, `${row.id}: a cancelled or retired open is no evidence against the hub and keeps the session; a hostile asset revokes it`).toBe(row.sessionKept);
            expect(FakeHubWebSocket.instances, row.id).toHaveLength(0);
            expect(state?.executionTargetLease ?? null, row.id).toBeNull();
            expect(state?.outbox ?? [], row.id).toHaveLength(0);
            expect(stages.includes("socket-grants"), row.id).toBe(false);
            expect(JSON.stringify(statuses)).not.toContain(plan.receipt);
          } finally {
            closeArtifactRuntime(runtimeKey);
            await new Promise((resolve) => setTimeout(resolve, 0));
            testSeams.executionTargetStatusObserver = null;
            clearHubSessionCapability();
          }
        }
      } finally {
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalSocket;
        testSeams.workerPostTestSink = originalPost;
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
          if (url.endsWith("/open-plan")) return Response.json(vector.kind === "stale-plan" ? { ...plan, catalog: { generationId: fixture.expected.rotation.generationB } } : plan);
          if (url.endsWith("/execution-target/manifest")) {
            if (vector.kind === "cancel" && vector.stage === "manifest") requests.length > 0 && harnessAbort();
            return Response.json(manifest);
          }
          if (url.endsWith("/execution-target/component")) {
            if (vector.kind === "cancel" && (vector.stage === "component" || vector.stage === "verify")) harnessAbort();
            if (vector.kind === "missing-body" && vector.stage === "component") return new Response(null, { status: 204 });
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
            if (vector.kind === "missing-body" && vector.stage === "descriptor") return new Response(null, { status: 204 });
            if (vector.kind === "descriptor-max-plus-one") return executionTargetBodyResponse(descriptor, fixture.expected.descriptorMaxBytes + 1);
            if (vector.kind === "descriptor-trailing-byte") return executionTargetBodyResponse(new Uint8Array([...descriptor, 0]));
            if (vector.kind === "descriptor-noncanonical") return executionTargetBodyResponse(encodePackValue({ descriptorVersion: 1, packageId: "semio:demo" }));
            if (vector.kind === "descriptor-self-hash")
              return executionTargetBodyResponse(encodePackValue({ ...(decodePackValue(descriptor) as Record<string, unknown>), hashes: { wasmSha256: "a".repeat(64), coreWasmSha256: "9".repeat(64), descriptorSha256: "8".repeat(64) } }));
            if (vector.kind === "descriptor-bytes") {
              const substituted = new Uint8Array(descriptor);
              substituted[substituted.length - 1] = substituted[substituted.length - 1]! ^ 0xff;
              return executionTargetBodyResponse(substituted);
            }
            return executionTargetBodyResponse(servedDescriptor);
          }
          return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 });
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
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json", source.url), "utf8"));
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
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json", source.url), "utf8"));
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
          const headers: Record<string, string> = {};
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

    it("asks the hub again for an execution-target asset it declared temporarily unavailable, announces each retry, and still verifies the lease", async () => {
      const fixture = await executionTargetLeaseFixture();
      const policy = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🏪️store/👷️worker/🔁️execution-target-retry/🔣️.json", source.url), "utf8")) as { transientStatuses: number[]; maxAttempts: number };
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      for (const [transient, failures, expected] of [[503, 1, "installed"], [502, policy.maxAttempts, "refused"], [404, 1, "refused"]] as const) {
        const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
        let componentAnswers = 0;
        const harness = await executionTargetHarness(fixture, (url) => {
          if (url.endsWith("/open-plan")) return Response.json(plan);
          if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest);
          if (url.endsWith("/execution-target/component")) {
            componentAnswers += 1;
            return componentAnswers <= failures ? new Response(`{"schema":"semio.hub.document-open-plan-error/v1","code":"deadline-exceeded"}`, { status: transient }) : executionTargetBodyResponse(component);
          }
          if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
          return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 });
        });
        try {
          let reason = "";
          const outcome = await requestDocumentSocketAuthority(harness.state, harness.binding).then(
            () => "installed",
            (error: unknown) => ((reason = String(error)), "refused"),
          );
          const retries = harness.statuses.filter((status) => status.code === "retrying").map((status) => status.progress);
          const declared = policy.transientStatuses.includes(transient);
          expect(outcome, `${transient} ×${failures}: ${reason} ${JSON.stringify(harness.statuses.map((status) => [status.code, status.progress?.stage, status.diagnostic]))}`).toBe(expected);
          expect(componentAnswers, `${transient} ×${failures}: component requests`).toBe(declared ? Math.min(failures + 1, policy.maxAttempts) : 1);
          expect(retries, `${transient} ×${failures}: announced retries`).toEqual(declared ? Array.from({ length: Math.min(failures, policy.maxAttempts - 1) }, (_unused, index) => ({ stage: "component", completedBytes: index + 2, totalBytes: policy.maxAttempts })) : []);
          expect(documentExecutionTargetStatusRoleV1("retrying")).toBe("status");
          if (expected === "installed") expect(harness.statuses.at(-1)!.code).toBe(fixture.expected.rendererState);
          else expect(harness.statuses.at(-1)!.code).not.toBe("retrying");
        } finally {
          harness.release();
        }
      }
    }, 60_000);

  });
  //#endregion 🔖️OfflineResilienceTests
}
