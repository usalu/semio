/** 📨️ Document backbone batches: grant actor isolation, retained bytes, refused and transient batches and rebootstrap fencing. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, FakeHubWebSocket, flushSocketGrantTurns, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🧾️document-state/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type MutationEnvelope = import("@semio-tech/framework-replication").MutationEnvelope;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDocumentBackboneBatchTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw, installVerifiedDocumentBackbonePair, exactDocumentBackboneMessage } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, encodeBackboneMessage, encodeDocumentBackboneEnvelopeBatchExact, encodePackValue, openArtifact, artifactState, handleHubFrame, handleTsRequest, decodeClientFrame, closeArtifact, DOCUMENT_BACKBONE_RETENTION_LIMITS, handleAck, parseDocumentExecutionTargetLeaseFieldsV1, sameLeaseFieldsV1, HUB_RECONNECT_MAX_MS } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

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
      testSeams.documentSocketGrantTestIssue = async (_baseUrl, path) => ({
        schema: "semio.hub.document-socket-grant/v1",
        protocol: "semio.session.v1",
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
            observed: null,
            target: [],
            diff: { schema: value.diff.schemaId, payload: value.document === "doc-a" ? opaqueNoncanonicalPack : encodePackValue(value.diff.payload) },
            inverse: { schema: value.inverse.inverseDiff.schemaId, payload: encodePackValue(value.inverse.inverseDiff.payload) },
            timestamp: { actor: 7n, physical_ms: 9_007_199_254_740_992n, logical: 11n },
            transaction: null,
            verb: null, line: null,
          }]),
        }),
      });
      try {
        for (const documentId of ["doc-a", "doc-b"]) openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller-selected-actor", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        await flushSocketGrantTurns();
        const [socketA, socketB] = FakeHubWebSocket.instances;
        socketA!.open();
        socketB!.open();
        for (const documentId of ["doc-a", "doc-b"]) installVerifiedDocumentBackbonePair(artifactState(documentId, "space-1")!);
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
      testSeams.documentSocketGrantTestIssue = async () => ({
        schema: "semio.hub.document-socket-grant/v1",
        protocol: "semio.session.v1",
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
              observed: null,
              target: [],
              diff: { schema: "demo/v1", payload: encodePackValue("x".repeat(textBytes)) },
              inverse: { schema: "demo/v1", payload: encodePackValue(null) },
              timestamp: { actor: 1n, physical_ms: 2n, logical: 3n },
              transaction: null,
              verb: null, line: null,
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
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
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
        expect(outcomes.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", code: "local.backbone-capacity", reason: "document backbone pending capacity", messages: [], detail: { envelopes: 1, limit: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes } } } });
        await handleAck(state, 0, [{ Applied: { outcome: "Accepted" } }]);
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
    }, 30_000);

    it("rebootstraps a browser actor whose batch the hub refused or transformed, never handing its correction to the Shell", async () => {
      const originalWebSocket = globalThis.WebSocket;
      const originalBroadcastChannel = globalThis.BroadcastChannel;
      class BoundPortBroadcastChannel {
        onmessage: ((event: MessageEvent) => void) | null = null;
        postMessage(): void { throw new Error("bound document backbone must not echo before server authority"); }
        close(): void {}
      }
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = BoundPortBroadcastChannel;
      testSeams.documentSocketGrantTestIssue = async () => ({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: `hub.v1.${"4".repeat(64)}`, expiresAtMs: Number.MAX_SAFE_INTEGER });
      const replacement = (documentId: string) => ({ mutation_id: "hub-replacement", document_id: documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: Array.from(encodePackValue("hub")) }, inverse: { schema: "demo/v1", payload: Array.from(encodePackValue(null)) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 4n }, transaction: null });
      const cases = [
        { documentId: "doc-ack-refused", outcome: { Rejected: { reason: "stale-base", messages: [] } }, expected: { kind: "rejected", code: "hub.refused", reason: "stale-base", messages: [] } },
        { documentId: "doc-ack-transformed", outcome: { Transformed: { envelope: replacement("doc-ack-transformed") } }, expected: { kind: "transformed" } },
      ] as const;
      try {
        for (const row of cases) {
          FakeHubWebSocket.instances = [];
          const edit = (id: string) => encodeBackboneMessage({
            kind: "mutations",
            envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: id, document_id: row.documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue(id) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 3n }, transaction: null, verb: null, line: null }]),
          });
          const outcomes: BackboneWorkerResponse[] = [];
          testSeams.workerPostTestSink = (message) => outcomes.push(message);
          openArtifact({ documentId: row.documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
          await flushSocketGrantTurns();
          const socket = FakeHubWebSocket.instances[0]!;
          socket.open();
          const state = artifactState(row.documentId, "space-1")!;
          installVerifiedDocumentBackbonePair(state);
          await handleHubFrame(state, { Session: { actor: `hub.v1.${"4".repeat(64)}`, color: 1 } });
          for (const id of ["authored-edit", "queued-edit"]) handleTsRequest({ kind: "send", documentId: row.documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: edit(id) } });
          const delivered: Uint8Array[] = [];
          state.browserActorReservation = { receiveBackbone: async (bytes: Uint8Array) => { delivered.push(bytes); }, close: () => {} } as unknown as ArtifactState["browserActorReservation"];
          await handleAck(state, 0, [{ Applied: { outcome: row.outcome } }] as unknown as Parameters<typeof handleAck>[2]);
          expect(delivered, row.documentId).toEqual([]);
          expect(outcomes.filter((message) => message.kind === "event" && message.event.kind === "documentBackbone"), row.documentId).toEqual([]);
          expect(outcomes.some((message) => message.kind === "artifact-rebootstrap-required"), row.documentId).toBe(true);
          expect(state.artifactRebootstrapRequired, row.documentId).toBe(true);
          expect(state.outbox.map((envelope) => envelope.id), row.documentId).toEqual(["queued-edit"]);
          expect(outcomes.filter((message) => message.kind === "event" && message.event.kind === "commandOutcome").map((message) => (message as Extract<BackboneWorkerResponse, { kind: "event" }>).event), row.documentId).toEqual([expect.objectContaining({ outcome: row.expected })]);
          state.browserActorReservation = null;
          closeArtifact(row.documentId, "space-1");
        }
      } finally {
        for (const row of cases) closeArtifact(row.documentId, "space-1");
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

    it("keeps and resends a batch the hub refused for a transient reason, drains the outbox ack-clocked in bounded batches, never rebuilds", async () => {
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
      const actor = `hub.v1.${"5".repeat(64)}`;
      testSeams.documentSocketGrantTestIssue = async () => ({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: actor, expiresAtMs: Number.MAX_SAFE_INTEGER });
      const documentId = "doc-ack-transient";
      const edit = (id: string, padding = 0) => encodeBackboneMessage({
        kind: "mutations",
        envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: id, document_id: documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue(id + "x".repeat(padding)) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 3n }, transaction: null, verb: null, line: null }]),
      });
      const sentIds = (socket: FakeHubWebSocket) => socket.sent.flatMap((bytes) => {
        const decoded = decodeClientFrame(bytes).frame;
        return typeof decoded === "object" && decoded !== null && "Commands" in decoded ? [decoded.Commands.envelopes.map((envelope: { mutation_id: string }) => envelope.mutation_id)] : [];
      });
      const transient = Array.from(new TextEncoder().encode(JSON.stringify([{ level: "warning", code: "hub.unavailable", message: "DB I/O aggregate admission exhausted" }])));
      const refuse = (state: ArtifactState, batchId: number) => handleAck(state, batchId, [{ Applied: { outcome: { Rejected: { reason: "unavailable: DB I/O aggregate admission exhausted", messages: transient } } } }] as unknown as Parameters<typeof handleAck>[2]);
      const outcomes: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => outcomes.push(message);
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);
      try {
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        await flushSocketGrantTurns();
        const socket = FakeHubWebSocket.instances[0]!;
        socket.open();
        const state = artifactState(documentId, "space-1")!;
        installVerifiedDocumentBackbonePair(state);
        await handleHubFrame(state, { Session: { actor, color: 1 } });
        vi.useFakeTimers();
        const send = (id: string, padding = 0) => handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: edit(id, padding) } });
        for (const id of ["first-edit", "second-edit"]) send(id);
        expect(sentIds(socket), "one batch in flight: the next waits for its Ack").toEqual([["first-edit"]]);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(["second-edit"]);
        await refuse(state, 0);
        expect(state.artifactRebootstrapRequired).toBe(false);
        expect(outcomes.some((message) => message.kind === "artifact-rebootstrap-required")).toBe(false);
        expect(outcomes.some((message) => message.kind === "event" && message.event.kind === "commandOutcome")).toBe(false);
        expect(state.outbox.map((envelope) => envelope.id), "the refused batch returns to the front").toEqual(["first-edit", "second-edit"]);
        expect(state.pendingMutations.map((envelope) => envelope.id)).toEqual(["first-edit", "second-edit"]);
        expect(state.transientRefusal?.batchLimit).toBe(1);
        send("third-edit");
        expect(state.outbox.map((envelope) => envelope.id), "a later edit waits behind the refused batch").toEqual(["first-edit", "second-edit", "third-edit"]);
        await vi.advanceTimersByTimeAsync(180);
        expect(sentIds(socket), "nothing goes out during the backoff (250 ms, jittered to 187.5)").toHaveLength(1);
        await vi.advanceTimersByTimeAsync(10);
        expect(sentIds(socket).slice(1), "the refused edit goes out again first, bounded to half the refused batch").toEqual([["first-edit"]]);
        await refuse(state, 1);
        await vi.advanceTimersByTimeAsync(370);
        expect(sentIds(socket), "a second refusal doubles the backoff (500 ms, jittered to 375)").toHaveLength(2);
        await vi.advanceTimersByTimeAsync(10);
        expect(sentIds(socket).slice(2)).toEqual([["first-edit"]]);
        await handleAck(state, 2, [{ Applied: { outcome: "Accepted" } }]);
        expect(state.transientRefusal, "an Accepted batch ends the refusal: the hub admits again").toBeNull();
        expect(sentIds(socket).slice(3), "the rest drains behind it, in order").toEqual([["second-edit", "third-edit"]]);
        await handleAck(state, 3, [{ Applied: { outcome: "Accepted" } }]);
        expect(state.outbox).toEqual([]);
        expect(state.pendingMutations).toEqual([]);
        state.hubActorReady = false;
        const queued = Array.from({ length: 20 }, (_, index) => `queued-${index}`);
        for (const id of queued) send(id);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(queued);
        state.hubActorReady = true;
        send("after-shortage");
        expect(sentIds(socket).slice(4), "a grown outbox drains one bounded batch at a time, in order").toEqual([queued.slice(0, 16)]);
        await handleAck(state, 4, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(5)).toEqual([[...queued.slice(16), "after-shortage"]]);
        await handleAck(state, 5, [{ Applied: { outcome: "Accepted" } }]);
        state.hubActorReady = false;
        for (const id of ["large-0", "large-1", "large-2"]) send(id, 100_000);
        state.hubActorReady = true;
        send("small-after-large");
        expect(sentIds(socket).slice(6), "a batch stays within half the declared batch bytes").toEqual([["large-0"]]);
        await handleAck(state, 6, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(7)).toEqual([["large-1"]]);
        await handleAck(state, 7, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(8)).toEqual([["large-2", "small-after-large"]]);
        send("behind-the-drop");
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(["behind-the-drop"]);
        socket.close();
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox.map((envelope) => envelope.id), "a dropped socket's unacked batch returns AHEAD of what queued behind it").toEqual(["large-2", "small-after-large", "behind-the-drop"]);
      } finally {
        closeArtifact(documentId, "space-1");
        vi.useRealTimers();
        randomSpy.mockRestore();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

    it("resumes a suspended child only on the same execution target, adopting a newer checkpoint and revalidation witness", async () => {
      const { sameExecutionTargetV1 } = await import("../../🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts");
      const { readFile } = await import("node:fs/promises");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(await readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json", source.url), "utf8")) as { current: { fixture: string; pointer: "/manifest" }; cases: { case: string; patch: unknown; resumes: boolean }[] };
      const manifest = (JSON.parse(await readFile(new URL(`../../../${fixture.current.fixture}`, source.url), "utf8")) as { manifest: unknown }).manifest;
      const merge = (target: unknown, patch: unknown): unknown => {
        if (patch === null || typeof patch !== "object" || Array.isArray(patch)) return patch;
        const merged: Record<string, unknown> = target !== null && typeof target === "object" && !Array.isArray(target) ? { ...(target as Record<string, unknown>) } : {};
        for (const [key, value] of Object.entries(patch)) {
          if (value === null) delete merged[key];
          else merged[key] = merge(merged[key], value);
        }
        return merged;
      };
      type Fields = ReturnType<typeof parseDocumentExecutionTargetLeaseFieldsV1>;
      const oracle = (current: Fields, next: Fields): boolean => {
        const { checkpoint: currentCheckpoint, revalidation: currentWitness, ...currentTarget } = current;
        const { checkpoint: nextCheckpoint, revalidation: nextWitness, ...nextTarget } = next;
        const notBehind = (from: number | undefined, to: number | undefined) => (from === undefined) === (to === undefined) && (from === undefined || to! >= from);
        return (
          equal(currentTarget, nextTarget) &&
          currentCheckpoint.descriptorDigestV1 === nextCheckpoint.descriptorDigestV1 &&
          currentCheckpoint.baselineFrontier.documentId === nextCheckpoint.baselineFrontier.documentId &&
          nextCheckpoint.baselineFrontier.headEditOrdinal >= currentCheckpoint.baselineFrontier.headEditOrdinal &&
          nextCheckpoint.baselineFrontier.lastCommitSeq >= currentCheckpoint.baselineFrontier.lastCommitSeq &&
          (["directoryRevision", "membershipGeneration", "sessionGeneration", "shareGeneration"] as const).every((key) => notBehind(currentWitness[key], nextWitness[key]))
        );
      };
      const current = parseDocumentExecutionTargetLeaseFieldsV1(manifest);
      expect(fixture.cases.filter((row) => row.resumes).length).toBeGreaterThan(1);
      expect(fixture.cases.filter((row) => !row.resumes).length).toBeGreaterThan(1);
      for (const row of fixture.cases) {
        const next = parseDocumentExecutionTargetLeaseFieldsV1(merge(structuredClone(manifest), row.patch));
        expect(sameExecutionTargetV1(current, next), row.case).toBe(row.resumes);
        expect(oracle(current, next), `fast-deep-equal oracle: ${row.case}`).toBe(row.resumes);
        expect(sameLeaseFieldsV1(current, next), `full-field relation: ${row.case}`).toBe(equal(row.patch, {}));
      }
    });

    it("retires an unsuspendable mounted child when its socket closes, forgets its document and reopens like a first open", async () => {
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
      const actor = `hub.v1.${"6".repeat(64)}`;
      testSeams.documentSocketGrantTestIssue = async () => ({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: actor, expiresAtMs: Number.MAX_SAFE_INTEGER });
      const edit = (documentId: string, id: string) => encodeBackboneMessage({
        kind: "mutations",
        envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: id, document_id: documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue(id) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 3n }, transaction: null, verb: null, line: null }]),
      });
      const hello = (socket: FakeHubWebSocket) => {
        const decoded = decodeClientFrame(socket.sent[0]!).frame;
        if (typeof decoded !== "object" || decoded === null || !("SocketHelloV1" in decoded)) throw new Error("expected SocketHelloV1");
        return decoded.SocketHelloV1;
      };
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);
      vi.useFakeTimers();
      try {
        const documentId = "doc-retire-mounted";
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        await flushSocketGrantTurns();
        const socket = FakeHubWebSocket.instances.at(-1)!;
        socket.open();
        const state = artifactState(documentId, "space-1")!;
        const baseline = installVerifiedDocumentBackbonePair(state);
        state.resumeToken = "resume-before-the-cut";
        await handleHubFrame(state, { Session: { actor, color: 1 } });
        for (const id of ["edit-in-flight", "edit-queued"]) handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: edit(documentId, id) } });
        let childClosed = false;
        state.browserActorReservation = { suspended: false, suspendLink: () => false, close: () => { childClosed = true; state.browserActorReservation = null; } } as unknown as ArtifactState["browserActorReservation"];
        expect(state.pendingBatches.size).toBe(1);
        expect(state.frontier).toEqual(baseline);
        socket.close();
        expect(childClosed, "the unsuspendable child is retired").toBe(true);
        expect({ pair: state.verifiedColdPair, lease: state.executionTargetLease, pack: state.currentPack, spr: state.currentSpr, frontier: state.frontier, resumeToken: state.resumeToken }, "the document the child held is forgotten").toEqual({ pair: null, lease: null, pack: null, spr: null, frontier: null, resumeToken: null });
        expect(state.outbox.map((envelope) => envelope.id), "nothing unsent is lost, in send order").toEqual(["edit-in-flight", "edit-queued"]);
        expect([...state.ingestedMutationIds].sort(), "their echo stays suppressed").toEqual(["edit-in-flight", "edit-queued"]);
        expect(posted.filter((message) => message.kind === "artifact-rebootstrap-required"), "the Shell discards the stale surface").toEqual([expect.objectContaining({ documentId, retryable: true })]);
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS + 1);
        const next = FakeHubWebSocket.instances.at(-1)!;
        expect(next).not.toBe(socket);
        next.open();
        expect(hello(next), "the next connection opens like a first one").toMatchObject({ frontier: null, resume_token: null });
        closeArtifact(documentId, "space-1");

        const plainId = "doc-retire-unmounted";
        openArtifact({ documentId: plainId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        await flushSocketGrantTurns();
        const plainSocket = FakeHubWebSocket.instances.at(-1)!;
        plainSocket.open();
        const plain = artifactState(plainId, "space-1")!;
        const plainFrontier = { document_id: plainId, head_edit_ordinal: 3, head_edit_id: "edit-3", last_commit_seq: 3, chain_hash: new Array(32).fill(3) };
        plain.frontier = plainFrontier;
        plain.resumeToken = "resume-plain";
        const postedBefore = posted.length;
        plainSocket.close();
        expect({ frontier: plain.frontier, resumeToken: plain.resumeToken }, "a document without a mounted child resumes from where it was").toEqual({ frontier: plainFrontier, resumeToken: "resume-plain" });
        expect(posted.slice(postedBefore).some((message) => message.kind === "artifact-rebootstrap-required")).toBe(false);
        closeArtifact(plainId, "space-1");
      } finally {
        closeArtifact("doc-retire-mounted", "space-1");
        closeArtifact("doc-retire-unmounted", "space-1");
        vi.useRealTimers();
        randomSpy.mockRestore();
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
      testSeams.documentSocketGrantTestIssue = async () => ({
        schema: "semio.hub.document-socket-grant/v1",
        protocol: "semio.session.v1",
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
      const mirrorRetirement: { release: (() => void) | null } = { release: null };
      try {
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
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
        state.canonicalFolderMirror = { binding: { kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/rebootstrap-raw" }, documentId, epoch: 1, capability: "a".repeat(64) };
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          await new Promise<void>((resolve) => { mirrorRetirement.release = resolve; });
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
        expect(posted.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", code: "local.backbone-pair-unavailable", reason: "document backbone canonical pair unavailable", messages: [] } } });
        handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "localMutations", envelopes: [second] } });
        expect(state.pendingDocumentBackboneBytes).toBe(firstMessage.byteLength);
        expect(state.pendingMutations.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual([first.id]);
        expect(oldSocket.sent).toHaveLength(sentBeforeRefusal);
        expect(posted.at(-1)).toMatchObject({ kind: "event", event: { kind: "commandOutcome", outcome: { kind: "rejected", code: "local.backbone-pair-unavailable", reason: "document backbone canonical pair unavailable", messages: [], detail: { bytes: 0 } } } });

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

        mirrorRetirement.release?.();
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
        mirrorRetirement.release?.();
        closeArtifact(documentId, "space-1");
        testSeams.workerPostTestSink = null;
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
