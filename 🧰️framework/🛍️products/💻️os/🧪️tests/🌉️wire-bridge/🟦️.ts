/** 🌉️ The worker's wire bridge: envelope round trips, frame decoding, Commands batches and session stamping. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, sampleEnvelope, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
type ArtifactPresencePeer = import("@semio-tech/framework-replication").ArtifactPresencePeer;
type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🧾️document-state/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerWireBridgeTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, toWireEnvelope, decodePackPayload, fromWireEnvelope, rollbackEnvelope, decodeClientFrame, decodeServerFrame, extractServerCommandsDocumentBackboneBatchExact, newArtifactState, documentRuntimeKeyForConfig, handleHubFrame, encodeBackboneMessage, parseDocumentBackboneMessage, parseDocumentExecutionTargetLeaseFieldsV1, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, stampSession } = dependencies;
  describe("backbone-worker wire bridge", () => {
    beforeEach(resetWorkerLaw);
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
      const batch = new Uint8Array(Buffer.from("01016d01640161000000017301aa016902bbcc03ffffffffffffffffff010500", "hex"));
      const serverFrame = Uint8Array.from([0, 3, ...batch, 6, ...new TextEncoder().encode("remote"), 1, 100, 0, 1, 101, 0, ...new Array(32).fill(0)]);
      const decoded = decodeServerFrame(serverFrame).frame;
      const exactBatch = extractServerCommandsDocumentBackboneBatchExact(serverFrame);
      if (typeof decoded === "string" || !("Commands" in decoded) || exactBatch === null) throw new Error("expected exact server Commands frame");
      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };
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

    it("stampSession fills color/surface from actor state, overwriting whatever the caller set", () => {
      const installedTarget = parseDocumentExecutionTargetLeaseFieldsV1({
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: { spaceId: "studio-1", documentId: "doc-1" },
        descriptorDigestV1: "5".repeat(64),
        catalog: { generationId: "6".repeat(64) },
        package: { pluginId: "test-plugin", packageId: "semio:test-plugin", version: "1", componentSha256: "1".repeat(64), componentBlake3: "2".repeat(64), descriptorByteSha256: "3".repeat(64), executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 }, componentReceipt: { schema: "semio.plugin.component-receipt/v1", pluginId: "test-plugin", packageId: "semio:test-plugin", profile: "release", target: "wasm32-wasip2", wasmSha256: "1".repeat(64), wasmBytes: 4096, semanticDescriptorSha256: "3".repeat(64), toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "a".repeat(64), provenanceSha256: "b".repeat(64) } },
        component: { sha256: "1".repeat(64), blake3: "2".repeat(64), byteLength: 4096 },
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
      const hubConfig: ArtifactActorConfig = { documentId: "doc-1", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1", installedTarget }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
      const hubState = { config: hubConfig, sessionColor: 7 } as unknown as ArtifactState;
      const peer: ArtifactPresencePeer = { actor: "actor-1", connectedAtMs: 1000, color: 99, surface: "shell-should-never-set-this", views: [] };
      const stamped = stampSession(peer, hubState);
      expect(stamped.color).toBe(7);
      expect(stamped.surface).toBe("s.space.home@1/*#editor");

      const folderConfig: ArtifactActorConfig = { documentId: "doc-2", schema: "demo/v1", bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/doc-2" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
      const folderState = { config: folderConfig, sessionColor: null } as unknown as ArtifactState;
      const stampedFolder = stampSession(peer, folderState);
      expect(stampedFolder.color).toBeUndefined();
      expect(stampedFolder.surface).toBeUndefined();
    });

    it("stampSession publishes an actor-bound document's own presence pack, interaction, tool run and history edit, never the Shell's local instance's", async () => {
      const { encodePresenceHistoryEdit, encodePresenceInteraction, encodePresenceToolRun } = await import("@semio-tech/framework-replication");
      const fixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/👕️canvas-presence/🔣️.json", source.url), "utf8")) as { readonly paint: readonly { readonly roster: readonly ArtifactPresencePeer[] }[] };
      const selecting = fixture.paint[0]!.roster.find((row) => row.actor === "peer-a")!;
      const config: ArtifactActorConfig = { documentId: "doc-4", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
      const localEdit = { mutationId: "local", stage: "editing", drafts: 0 } as const;
      const shellPeer: ArtifactPresencePeer = { actor: "actor-1", connectedAtMs: 1000, views: [], presencePack: [9, 9], interaction: { app_id: "local-instance", domains: [] }, historyEdit: localEdit };
      const toolRun = { toolId: "fill", state: "running", stage: 1, completed: 2, total: 5 } as const;
      const historyEdit = { mutationId: "m-2", stage: "replaying", drafts: 1 } as const;
      const snapshot = { presence: [1, 2, 3], presenceGeneration: 4, transientGeneration: 5, interaction: encodePresenceInteraction(selecting.interaction!), toolRun: encodePresenceToolRun(toolRun), historyEdit: encodePresenceHistoryEdit(historyEdit) };
      const bound = stampSession(shellPeer, { config, sessionColor: 3, browserActorReservation: { ephemeralSnapshot: snapshot } } as unknown as ArtifactState);
      expect(bound.presencePack).toEqual([1, 2, 3]);
      expect(bound.interaction).toEqual(selecting.interaction);
      expect([bound.toolRun, bound.historyEdit]).toEqual([toolRun, historyEdit]);
      const unpublished = stampSession(shellPeer, { config, sessionColor: 3, browserActorReservation: { ephemeralSnapshot: null } } as unknown as ArtifactState);
      expect([unpublished.presencePack, unpublished.interaction, unpublished.historyEdit]).toEqual([undefined, undefined, undefined]);
      const local = stampSession(shellPeer, { config, sessionColor: 3, browserActorReservation: null } as unknown as ArtifactState);
      expect([local.presencePack, local.interaction, local.historyEdit]).toEqual([shellPeer.presencePack, shellPeer.interaction, localEdit]);
    });

    it("handleHubFrame stores the hub-assigned session color on a Session frame", () => {
      const config: ArtifactActorConfig = { documentId: "doc-3", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
      const state = { config, actor: "", hubActorReady: false, pendingSocketActorId: "actor-1", outbox: [], sessionColor: null } as unknown as ArtifactState;
      handleHubFrame(state, { Session: { actor: "actor-1", color: 3 } });
      expect(state.sessionColor).toBe(3);
    });
  });
}
