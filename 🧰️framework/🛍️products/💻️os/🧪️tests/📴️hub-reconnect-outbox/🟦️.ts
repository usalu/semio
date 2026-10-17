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
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-backbone) findings 1/2/3/5: stream-primary folder
  // watch (`backbone.folder` on the page's stream channel) with a suppressed sanity-poll fallback, abort-on-close,
  // and the bounded lossless mutation outbox. No real sleeps — `vi.useFakeTimers()` drives every
  // timer-dependent assertion, and every fetch/socket/stream is a controllable local fake.
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

        // 🚨️ Rejected wholesale, never partially accepted or silently dropped — the queue is
        // untouched, and the rejection is explicitly logged (the shell-facing signal is the same
        // `commandOutcome`/`rejected` vocabulary a real hub rejection uses — see
        // `rejectMutationQueueOverflow`'s doc comment).
        expect(state.pendingMutations).toHaveLength(0);
        expect(errorSpy).toHaveBeenCalledWith("[backbone-worker] refused a local batch", "doc-overflow", "local.queue-full", { envelopes: overSized.length, limit: PENDING_MUTATIONS_QUEUE_LIMIT });

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
        const config: ArtifactActorConfig = { documentId: "doc-hub-flush", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-flush")!;
        // 🧊️ Minted at the baseline this law's `Welcome` confirms below: `assertCurrent()`
        // (`🏪️store/👷️worker/🟦️.ts:978`) refuses an owner whose frontier the state has moved past.
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
    // the life of one call and never resets after success — `connectHub` now loops
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
        const config: ArtifactActorConfig = { documentId: "doc-hub-reset", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
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
        const config: ArtifactActorConfig = { documentId: "doc-hub-no-reset", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
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
        const config: ArtifactActorConfig = { documentId: "doc-hub-abort", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
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

  });
  //#endregion 🔖️OfflineResilienceTests
}
