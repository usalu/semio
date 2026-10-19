/** 📴️ The bounded lossless mutation outbox and the hub reconnect backoff: offline queueing, flush on Welcome and abort of the reconnect loop. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, FakeHubWebSocket, flushSocketGrantTurns, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
type MutationEnvelope = import("@semio-tech/framework-replication").MutationEnvelope;
type ServerFrame = import("@semio-tech/framework-replication").ServerFrame;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerHubReconnectOutboxTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw, installVerifiedDocumentBackbonePair, exactDocumentBackboneMessage } = workerLawFakesV1(vitest, dependencies, source);
  const { openArtifact, artifactState, PENDING_MUTATIONS_QUEUE_LIMIT, handleTsRequest, closeArtifact, encodeServerFrame, handleHubFrame, decodeClientFrame, SUSTAINED_HEALTHY_MS } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    it("queue overflow rejects and reports rather than dropping silently", () => {
      const config: ArtifactActorConfig = { documentId: "doc-overflow", schema: "demo/v1", bindings: [], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
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

        expect(state.pendingMutations).toHaveLength(0);
        expect(errorSpy).toHaveBeenCalledWith("[backbone-worker] refused a local batch", "doc-overflow", "local.queue-full", { envelopes: overSized.length, limit: PENDING_MUTATIONS_QUEUE_LIMIT });

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
        const config: ArtifactActorConfig = { documentId: "doc-hub-flush", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-flush")!;
        installVerifiedDocumentBackbonePair(state, { head_edit_id: "e0" });
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

        expect(state.outbox).toHaveLength(1);
        expect(socket.sent).toHaveLength(0);
        expect(state.pendingMutations).toHaveLength(1);
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);

        socket.open();
        expect(socket.sent).toHaveLength(1);

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

        expect(state.outbox).toHaveLength(0);
        expect(socket.sent).toHaveLength(2);
        const commandsFrame = decodeClientFrame(socket.sent[1]!).frame;
        if (typeof commandsFrame === "string" || !("Commands" in commandsFrame)) throw new Error("expected a Commands frame");
        expect(commandsFrame.Commands.envelopes).toHaveLength(1);
        expect(commandsFrame.Commands.envelopes[0]!.mutation_id).toBe("edit-offline-1");
        expect(commandsFrame.Commands.envelopes[0]!.diff.payload).toEqual(Array.from(opaqueNoncanonicalPack));
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);
        expect(state.pendingBatches.size).toBe(1);
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
        const config: ArtifactActorConfig = { documentId: "doc-hub-stranded", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
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
        expect(state.pendingBatches.size).toBe(1);
        expect(state.outbox).toHaveLength(0);
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);

        socket.close();
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox).toHaveLength(1);
        expect(state.outbox[0]!.id).toBe("edit-inflight-1");
        expect(state.pendingMutations).toHaveLength(1);
        expect(state.pendingDocumentBackboneBytes).toBe(rawMessage.byteLength);
      } finally {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-stranded");
      }
    });

    it("a hub drop after sustained health resets the backoff, unlike continued accumulation", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-reset", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
        openArtifact(config);
        await flushSocketGrantTurns();

        FakeHubWebSocket.instances[0]!.close();
        await vi.advanceTimersByTimeAsync(751);
        expect(FakeHubWebSocket.instances).toHaveLength(2);
        FakeHubWebSocket.instances[1]!.close();
        await vi.advanceTimersByTimeAsync(1251);
        expect(FakeHubWebSocket.instances).toHaveLength(3);

        const healthy = FakeHubWebSocket.instances[2]!;
        healthy.open();
        await vi.advanceTimersByTimeAsync(SUSTAINED_HEALTHY_MS + 1);

        healthy.close();
        await vi.advanceTimersByTimeAsync(0);
        expect(FakeHubWebSocket.instances).toHaveLength(4);
        FakeHubWebSocket.instances[3]!.close();

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
        const config: ArtifactActorConfig = { documentId: "doc-hub-no-reset", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
        openArtifact(config);
        await flushSocketGrantTurns();

        FakeHubWebSocket.instances[0]!.open();
        FakeHubWebSocket.instances[0]!.close();
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2);

        FakeHubWebSocket.instances[1]!.open();
        FakeHubWebSocket.instances[1]!.close();
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2);
        await vi.advanceTimersByTimeAsync(451);
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
        const config: ArtifactActorConfig = { documentId: "doc-hub-abort", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
        openArtifact(config);
        await flushSocketGrantTurns();
        FakeHubWebSocket.instances[0]!.open();

        closeArtifact("doc-hub-abort");
        await vi.advanceTimersByTimeAsync(0);

        expect(vi.getTimerCount()).toBe(0);

        await vi.advanceTimersByTimeAsync(60_000);
        expect(FakeHubWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
