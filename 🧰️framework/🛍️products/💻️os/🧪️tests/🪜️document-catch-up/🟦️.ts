/** 🪜️ Document catch-up: a remote batch before the child exists, remote operations folded over pending local ones and the rebuild welcome. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDocumentCatchUpTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, encodeDocumentBackboneEnvelopeBatchExact, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1, encodePackValue, handleHubFrame, encodeBackboneMessage, dropDocumentExecutionTargetLease, closeArtifactRuntime } = dependencies;
  describe("browser actor catch-up tail", () => {
    beforeEach(resetWorkerLaw);
    it("keeps a remote batch that arrives before an actor-bound document's child exists for that child, never the Shell", async () => {
      const fixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8"));
      const hexBytes = (hex: string): Uint8Array => Uint8Array.from({ length: hex.length / 2 }, (_unused, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
      const { encodeDocumentBackboneEnvelopeBatchExact: encodeBatch } = await import("@semio-tech/framework-replication");
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        for (const [documentId, actorBound] of [["tail-before-session", true], ["tail-without-actor-lease", false]] as const) {
          openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "local-ui", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
          const state = artifactState(documentId)!;
          artifacts.delete(state.runtimeKey);
          const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "tail-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
          state.config = { ...state.config, documentId, bindings: [binding] };
          state.runtimeKey = documentRuntimeKeyForConfig(state.config);
          artifacts.set(state.runtimeKey, state);
          const fields = structuredClone(fixture.manifest);
          fields.scope = { spaceId: binding.spaceId, documentId };
          fields.checkpoint = { ...fields.checkpoint, baselineFrontier: { ...fields.checkpoint.baselineFrontier, documentId } };
          state.executionTargetLease = !actorBound ? null : new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, hexBytes(fixture.componentHex), hexBytes(fixture.descriptorHex));
          state.actor = "hub.v1.self";
          const envelope = { mutation_id: `${documentId}:m-1`, document_id: documentId, actor: "hub.v1.peer", dependencies: [], observed: null, target: [], diff: { schema: "demo.map.operation", payload: encodePackValue({ op: 1 }) }, inverse: { schema: "demo.map.operation.inverse", payload: encodePackValue(null) }, timestamp: { actor: 2n, physical_ms: 3n, logical: 0n }, transaction: null, verb: null, line: null };
          const batch = encodeBatch([envelope]);
          const frontier = { document_id: documentId, head_edit_ordinal: 1, head_edit_id: envelope.mutation_id, last_commit_seq: 1, chain_hash: new Array(32).fill(1), verb: null, line: null };
          posted.length = 0;
          await handleHubFrame(state, { Commands: { envelopes: [{ ...envelope, diff: { schema: envelope.diff.schema, payload: Array.from(envelope.diff.payload) }, inverse: { schema: envelope.inverse.schema, payload: Array.from(envelope.inverse.payload) }, timestamp: { actor: 2, physical_ms: 3, logical: 0 } }], origin: "hub.v1.peer", frontier } } as unknown as Parameters<typeof handleHubFrame>[1], null, null, batch);
          const shellDeliveries = posted.filter((message) => message.kind === "event" && message.event.kind === "documentBackbone");
          if (actorBound) {
            expect(shellDeliveries, documentId).toEqual([]);
            expect(state.browserActorBackboneBeforeReservation.map((message) => Array.from(message)), documentId).toEqual([Array.from(encodeBackboneMessage({ kind: "mutations", envelopes: batch }))]);
            dropDocumentExecutionTargetLease(state);
            expect(state.browserActorBackboneBeforeReservation, documentId).toEqual([]);
          } else {
            expect(shellDeliveries, documentId).toHaveLength(1);
            expect(state.browserActorBackboneBeforeReservation, documentId).toEqual([]);
          }
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = null;
      }
    });
  });

  describe("remote operations folded over pending local ones", () => {
    beforeEach(resetWorkerLaw);
    it("rebuilds an actor-bound document from the hub once its own operations are accepted after another human's were folded over them", async () => {
      const fixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8"));
      const hexBytes = (hex: string): Uint8Array => Uint8Array.from({ length: hex.length / 2 }, (_unused, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
      const { encodeDocumentBackboneEnvelopeBatchExact: encodeBatch } = await import("@semio-tech/framework-replication");
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        for (const [documentId, interleaved] of [["fold-remote-over-local", true], ["fold-local-only", false]] as const) {
          openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "local-ui", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
          const state = artifactState(documentId)!;
          artifacts.delete(state.runtimeKey);
          const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "fold-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
          state.config = { ...state.config, documentId, bindings: [binding] };
          state.runtimeKey = documentRuntimeKeyForConfig(state.config);
          artifacts.set(state.runtimeKey, state);
          const fields = structuredClone(fixture.manifest);
          fields.scope = { spaceId: binding.spaceId, documentId };
          fields.checkpoint = { ...fields.checkpoint, baselineFrontier: { ...fields.checkpoint.baselineFrontier, documentId } };
          state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, hexBytes(fixture.componentHex), hexBytes(fixture.descriptorHex));
          state.actor = "hub.v1.self";
          const local = { id: `${documentId}:local-1`, documentId, actor: "hub.v1.self" } as unknown as import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState["pendingMutations"][number];
          state.pendingMutations = [local];
          state.pendingBatches.set(7, [local]);
          const frontier = (ordinal: number, head: string) => ({ document_id: documentId, head_edit_ordinal: ordinal, head_edit_id: head, last_commit_seq: ordinal, chain_hash: new Array(32).fill(ordinal) });
          if (interleaved) {
            const remote = { mutation_id: `${documentId}:peer-1`, document_id: documentId, actor: "hub.v1.peer", dependencies: [], observed: null, target: [], diff: { schema: "demo.map.operation", payload: encodePackValue({ op: 1 }) }, inverse: { schema: "demo.map.operation.inverse", payload: encodePackValue(null) }, timestamp: { actor: 2n, physical_ms: 3n, logical: 0n }, transaction: null, verb: null, line: null };
            await handleHubFrame(state, { Commands: { envelopes: [{ ...remote, diff: { schema: remote.diff.schema, payload: Array.from(remote.diff.payload) }, inverse: { schema: remote.inverse.schema, payload: Array.from(remote.inverse.payload) }, timestamp: { actor: 2, physical_ms: 3, logical: 0 } }], origin: "hub.v1.peer", frontier: frontier(1, remote.mutation_id) } } as unknown as Parameters<typeof handleHubFrame>[1], null, null, encodeBatch([remote]));
          }
          expect(state.remoteFoldedOverLocal, documentId).toBe(interleaved);
          posted.length = 0;
          await handleHubFrame(state, { Ack: { batch_id: 7, stages: [{ Applied: { outcome: "Accepted" } }], frontier: frontier(interleaved ? 2 : 1, `${documentId}:local-1`) } } as unknown as Parameters<typeof handleHubFrame>[1], null, null, null);
          expect(state.pendingMutations, documentId).toEqual([]);
          expect(posted.filter((message) => message.kind === "event" && message.event.kind === "commandOutcome").map((message) => (message as { event: { outcome: unknown } }).event.outcome), documentId).toEqual([{ kind: "accepted" }]);
          expect(posted.some((message) => message.kind === "artifact-rebootstrap-required"), documentId).toBe(interleaved);
          expect(state.remoteFoldedOverLocal, documentId).toBe(false);
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = null;
      }
    });
  });

  describe("document rebuild welcome", () => {
    beforeEach(resetWorkerLaw);
    it("rebuilds an actor-bound document on the hub's own tail and keeps refusing a bare tail elsewhere", async () => {
      const { readFile } = await import("node:fs/promises");
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/🏪️store/🧪️tests/🧫️fixtures/🔁️document-rebuild-welcome-v1/🔣️.json", source.url), "utf8")) as {
        readonly rows: readonly Readonly<{ name: string; lane: "browser-actor" | "local"; welcome: "None" | "Tail"; accepted: boolean }>[];
      };
      const fixture = JSON.parse(await readFile(new URL("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8"));
      const hexBytes = (hex: string): Uint8Array => Uint8Array.from({ length: hex.length / 2 }, (_unused, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        for (const [index, row] of corpus.rows.entries()) {
          const documentId = `rebuild-welcome-${index}`;
          openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "local-ui", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
          const state = artifactState(documentId)!;
          artifacts.delete(state.runtimeKey);
          const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "rebuild-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
          state.config = { ...state.config, documentId, bindings: [binding] };
          state.runtimeKey = documentRuntimeKeyForConfig(state.config);
          artifacts.set(state.runtimeKey, state);
          if (row.lane === "browser-actor") {
            const fields = structuredClone(fixture.manifest);
            fields.scope = { spaceId: binding.spaceId, documentId };
            fields.checkpoint = { ...fields.checkpoint, baselineFrontier: { ...fields.checkpoint.baselineFrontier, documentId } };
            state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, hexBytes(fixture.componentHex), hexBytes(fixture.descriptorHex));
          }
          const frontier = { document_id: documentId, head_edit_ordinal: 2, head_edit_id: `${documentId}:head`, last_commit_seq: 2, chain_hash: new Array(32).fill(2) };
          let closes = 0;
          const stale = { close: () => closes++ } as unknown as WebSocket;
          state.socket = stale;
          await handleHubFrame(state, { RebootstrapRequired: { control: { space_id: binding.spaceId, document_id: documentId, checkpoint_id: new Array(32).fill(1), descriptor_hash: new Array(32).fill(3), baseline_frontier: frontier } } }, null, stale);
          expect(closes, row.name).toBe(1);
          expect(state.artifactRebootstrapRequired, row.name).toBe(true);
          posted.length = 0;
          const fresh = { close: () => closes++ } as unknown as WebSocket;
          state.socket = fresh;
          await handleHubFrame(state, { Welcome: { session_id: "session-rebuild", resume_token: "resume-rebuild", server_frontier: frontier, bootstrap: row.welcome } } as unknown as Parameters<typeof handleHubFrame>[1], null, fresh);
          const failed = posted.some((message) => message.kind === "artifact-bootstrap-failed");
          expect({ accepted: closes === 1 && !failed && !state.artifactRebootstrapRequired }, row.name).toEqual({ accepted: row.accepted });
          if (row.accepted && row.welcome === "Tail") expect(state.requiredTailFrontier, row.name).toEqual(frontier);
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = null;
      }
    });
  });
}
