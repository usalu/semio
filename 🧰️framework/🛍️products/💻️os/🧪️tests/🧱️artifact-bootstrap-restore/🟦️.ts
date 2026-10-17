/** 🧱️ Atomic artifact bootstrap restore: the bootstrap owner, staged pairs, rebootstrap bounds and the durable folder envelope. */
import { stubFetch } from "../🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, bytesFromHex, fixtureConfig, fixtureRequiredFrontier, sampleEnvelope } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type ServerFrame = import("@semio-tech/framework-replication").ServerFrame;
type WireFrontierSummary = import("@semio-tech/framework-replication").WireFrontierSummary;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerArtifactBootstrapRestoreTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw, artifactBootstrapFixture, decodeFixtureFrame, installFixture } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, executionTargetHex, ArtifactBootstrapAssembler, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, parseDocumentExecutionTargetLeaseFieldsV1, closeArtifactRuntime, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, handleHubFrame, closeArtifact, artifactBootstrapFailure, ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES, abortArtifactBootstrap, queueOutbox } = dependencies;
  //#region 🧪️ArtifactBootstrapRestore
  describe("artifact bootstrap atomic restore", () => {

    beforeEach(resetWorkerLaw);

    it("retains the exact bootstrap owner across hash completion and progress callbacks", async () => {
      const { readFile } = await import("node:fs/promises");
      const { createHash } = await import("node:crypto");
      const corpus = JSON.parse(await readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧵️artifact-bootstrap-owner-v1.json", source.url), "utf8"));
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
        const binding = { kind: "hub" as const, dataClass: "persistedShared" as const, baseUrl: "http://hub.test", spaceId: "bootstrap-owner-space" };
        state.config = { ...state.config, bindings: [binding] };
        state.runtimeKey = documentRuntimeKeyForConfig(state.config);
        artifacts.set(state.runtimeKey, state);
        state.currentPack = new Uint8Array([9]);
        state.currentSpr = new Uint8Array([8]);
        if (row.name.startsWith("lease-")) {
          const leaseFixture = JSON.parse(await readFile(new URL("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", source.url), "utf8")),
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
        let successor: InstanceType<typeof ArtifactBootstrapAssembler> | undefined;
        const returned: { pack: Uint8Array; spr: Uint8Array }[] = [];
        const finish = vi.spyOn(ArtifactBootstrapAssembler.prototype, "finish").mockImplementation(async function (this: InstanceType<typeof ArtifactBootstrapAssembler>, done, control) {
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
        state.config = { ...state.config, bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "bootstrap-watchdog-space" }] };
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
          const decoded = structuredClone(decodeFixtureFrame(fixture.wire.inlineWelcomeHex));
          if (!("Welcome" in decoded)) throw new Error("welcome fixture missing");
          // 🧭️ `bootstrap` is readonly on the decoded frame — the variant is chosen by rebuilding the
          // frame, which is also what the wire does, rather than by writing over a decoded one.
          const welcome = outcome === "success" ? decoded : { Welcome: { ...decoded.Welcome, bootstrap: outcome === "none" ? ("None" as const) : ("Tail" as const) } };
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
      state.config = { ...state.config, bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-a" }] };
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
      state.config = { ...state.config, bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/bootstrap-put-failure" }] };
      state.currentPack = Uint8Array.of(1);
      state.currentSpr = Uint8Array.of(2);
      const priorFrontier: WireFrontierSummary = { document_id: state.config.documentId, head_edit_ordinal: 1, head_edit_id: "old", last_commit_seq: 1, chain_hash: Array(32).fill(6) };
      state.frontier = priorFrontier;
      const originalFetch = globalThis.fetch;
      let puts = 0;
      globalThis.fetch = stubFetch(async (input: string | URL | Request, init?: RequestInit) => {
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
      });
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
      const corpus = JSON.parse(await readFile(new URL("./🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1/🔣️.json", source.url), "utf8"));
      expect(corpus.hostile.filter((row: { name: string }) => row.name.startsWith("client-stale-")).map((row: { name: string }) => row.name)).toEqual(["client-stale-stage", "client-stale-publish"]);
      const fixture = await artifactBootstrapFixture();
      const frame = decodeFixtureFrame(fixture.wire.inlineWelcomeHex);
      const originalFetch = globalThis.fetch;
      try {
        for (const phase of ["stage", "publish"] as const) {
          const config = fixtureConfig(fixture);
          openArtifact(config);
          const state = artifactState(config.documentId)!;
          state.config = { ...state.config, bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: `/tmp/bootstrap-stale-${phase}` }] };
          state.currentPack = Uint8Array.of(1);
          state.currentSpr = Uint8Array.of(2);
          let publishes = 0,
            retires = 0;
          globalThis.fetch = stubFetch(async (input: string | URL | Request) => {
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
          });
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
}
