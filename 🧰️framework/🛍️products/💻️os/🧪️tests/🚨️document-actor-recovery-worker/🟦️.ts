/** 🚨️ Document actor recovery in the worker: in-flight batches return to the outbox, catch-up gates input and a refusing child reopens. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type MutationEnvelope = import("../../../../🔨️modules/📡️replication/🟦️.ts").MutationEnvelope;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDocumentActorRecoveryWorkerTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, newArtifactState, documentRuntimeKeyForConfig, decodeServerFrame, extractServerCommandsDocumentBackboneBatchExact, handleHubFrame } = dependencies;
  describe("🚑️ document actor recovery (worker)", () => {
    beforeEach(resetWorkerLaw);
    it("returns in-flight batches to the outbox front in batch order, never twice, retires the child as actor-lost and reconnects at once; past the bound it posts the typed exhausted fault", () => {
      const requestRecovery = dependencies.requestDocumentActorRecoveryV1;
      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local" };
      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };
      const envelope = (id: string) => ({ id }) as unknown as MutationEnvelope;
      const mount = () => {
        state.browserActorReservation = { close() {} } as unknown as ArtifactState["browserActorReservation"];
      };
      const priorSink = testSeams.workerPostTestSink;
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        state.pendingBatches.set(2, [envelope("c")]);
        state.pendingBatches.set(1, [envelope("a"), envelope("b")]);
        state.outbox = [envelope("d"), envelope("a"), envelope("e")];
        mount();
        requestRecovery(state, "action-unconfirmed");
        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);
        expect(state.pendingBatches.size).toBe(0);
        expect(state.actorRecoveryRequested).toBe(true);
        expect(posted.map((message) => `${message.kind}:${"message" in message ? message.message : ""}`)).toEqual(["artifact-rebootstrap-required:actor-lost"]);
        for (let loss = 2; loss <= 3; loss += 1) {
          mount();
          state.actorRecoveryRequested = false;
          requestRecovery(state, "inbound-frame");
          expect(state.actorRecoveryRequested, `loss ${loss}`).toBe(true);
        }
        mount();
        state.actorRecoveryRequested = false;
        posted.length = 0;
        requestRecovery(state, "turn-failed");
        expect(state.actorRecovery.exhausted).toBe(true);
        expect(state.actorRecoveryRequested).toBe(false);
        expect(posted.map((message) => `${message.kind}:${"code" in message ? message.code : ""}`)).toEqual(["artifact-bootstrap-failed:recovery-exhausted"]);
        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);
      } finally {
        testSeams.workerPostTestSink = priorSink;
      }
    });

    it("refuses input while the document catches up with the hub: every catch-up fact alone gates, only the caught-up actor applies", () => {
      const catchingUp = dependencies.documentCatchingUpV1;
      const caughtUp = { bound: true, backboneReady: true, bootstrap: false, rebootstrap: false, tail: false, coldApplied: true, coldTransfer: false } as const;
      expect(catchingUp(caughtUp), "a bound actor that applied its cold pair and the whole tail takes input").toBe(false);
      for (const [fact, value] of [["bound", false], ["backboneReady", false], ["bootstrap", true], ["rebootstrap", true], ["tail", true], ["coldApplied", false], ["coldTransfer", true]] as const) {
        expect(catchingUp({ ...caughtUp, [fact]: value }), `${fact} alone keeps the document catching up`).toBe(true);
      }
    });

    it("recovers a child that refuses an inbound hub frame: actor-lost reopen, frontier kept, never a malformed-frame rebuild", async () => {
      const batch = new Uint8Array(Buffer.from("01016d01640161000000017301aa016902bbcc03ffffffffffffffffff010500", "hex"));
      const serverFrame = Uint8Array.from([0, 3, ...batch, 6, ...new TextEncoder().encode("remote"), 1, 100, 0, 1, 101, 0, ...new Array(32).fill(0)]);
      const decoded = decodeServerFrame(serverFrame).frame;
      const exactBatch = extractServerCommandsDocumentBackboneBatchExact(serverFrame);
      if (typeof decoded === "string" || !("Commands" in decoded) || exactBatch === null) throw new Error("expected exact server Commands frame");
      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local" };
      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };
      const confirmed = { document_id: "d", head_edit_ordinal: 4, head_edit_id: "edit-4", last_commit_seq: 4, chain_hash: new Array(32).fill(4) };
      state.frontier = confirmed;
      state.pendingBatches.set(0, [{ id: "in-flight" } as unknown as MutationEnvelope]);
      let refused = 0;
      state.browserActorReservation = { close() {}, receiveBackbone: async () => { refused += 1; throw new Error("browser actor child: invocation rejected: invoke reactor/poll: guest fault"); } } as unknown as ArtifactState["browserActorReservation"];
      const priorSink = testSeams.workerPostTestSink;
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        await handleHubFrame(state, decoded, null, null, exactBatch);
      } finally {
        testSeams.workerPostTestSink = priorSink;
      }
      expect(refused, "the child saw the frame once").toBe(1);
      expect(state.actorRecoveryRequested, "the lost actor reopens").toBe(true);
      expect(posted.map((message) => `${message.kind}:${"message" in message ? message.message : ""}`)).toEqual(["artifact-rebootstrap-required:actor-lost"]);
      expect(state.frontier, "a frame the child never applied does not advance the frontier").toEqual(confirmed);
      expect(state.outbox.map((entry) => entry.id), "the in-flight batch waits at the outbox front").toEqual(["in-flight"]);
      expect(state.artifactRebootstrapRequired, "no authoritative rebuild").toBe(false);
    });
  });
}
