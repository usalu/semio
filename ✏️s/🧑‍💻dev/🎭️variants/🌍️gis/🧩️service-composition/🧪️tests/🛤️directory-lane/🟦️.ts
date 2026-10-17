/** 🚉️ GIS directory lane: a scoped stream retired terminally on 4401 revokes the retained inference approval Undo owner. */
import { workerLawFakesV1, FakeDirectoryWebSocket, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../🧰️dependencies/🟦️.ts";

type BackboneWorkerResponse = import("@semio-tech/framework-os").BackboneWorkerResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerGisDirectoryLaneTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, handleTsRequest, openArtifact, artifactState, HUB_RECONNECT_MAX_MS, scopedDirectoryStreams, artifacts, closeDirectory } = dependencies;
  //#region 🔖️DirectoryLaneTests
  describe("backbone-worker directory lane", () => {

    beforeEach(resetWorkerLaw);

    it("backbone worker owns one full scoped stream and retires it terminally on 4401", async () => {
      vi.useFakeTimers();
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const paths: string[] = [];
      testSeams.socketGrantTestIssue = async (_baseUrl, path) => {
        paths.push(path);
        return {
          schema: "semio.hub.socket-grant/v1",
          protocol: "semio.socket.v1",
          grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
          actorId: `hub.v1.${"3".repeat(64)}`,
          expiresAtMs: Number.MAX_SAFE_INTEGER,
        };
      };
      const scope = { spaceId: "space/a", documentId: "document b" };
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        handleTsRequest({ kind: "directory-scope-open", baseUrl: "http://hub.test", scope, since: 7 });
        for (let turn = 0; turn < 16 && FakeDirectoryWebSocket.instances.length === 0; turn += 1) await Promise.resolve();
        const socket = FakeDirectoryWebSocket.instances[0]!;
        expect(paths).toEqual(["/directory/spaces/space%2Fa/documents/document%20b/socket-grants"]);
        expect(socket.url).toBe("ws://hub.test/directory/spaces/space%2Fa/documents/document%20b/socket/v1?since=7");
        socket.triggerOpen();
        const issue = testSeams.documentSocketGrantTestIssue;
        testSeams.documentSocketGrantTestIssue = null;
        openArtifact({ documentId: scope.documentId, schema: "gis.map", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: scope.spaceId }], actor: "caller", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
        testSeams.documentSocketGrantTestIssue = issue;
        const state = artifactState(scope.documentId, scope.spaceId)!;
        testSeams.inferenceApprovalUndoOwner = {
          historyEpoch: ++testSeams.inferenceApprovalUndoEpoch,
          scope,
          clientInstanceId: state.openClientInstanceId,
          sessionEpoch: testSeams.directorySessionEpoch,
          sessionFence: { token: {}, sessionBindingSha256: "c".repeat(64), authorizationGeneration: 1 },
          transport: { call: async () => { throw new Error("A revoked approval undo owner issues no document service call"); } },
          receipt: {
            schema: "semio.hub.inference-approval-receipt/v1",
            jobId: "1".repeat(32),
            mutationId: "2".repeat(32),
            commandHash: "3".repeat(64),
            proposalHash: "4".repeat(64),
            applied: true,
            undo: { targetId: "5".repeat(32), expectedCurrent: { documentId: scope.documentId, headEditOrdinal: 1, headEditId: "edit-1", lastCommitSeq: 1, chainSha256: "6".repeat(64) } },
          },
          idempotencyKey: "7".repeat(32),
          sourceCatalogGenerationId: "8".repeat(64),
          sourceComponentSha256: "9".repeat(64),
          sourceDescriptorSha256: "a".repeat(64),
          sourceBrowserActorSha256: "b".repeat(64),
          sourceDirectoryRevision: 1,
          sourceMembershipGeneration: 1,
          sourceSessionGeneration: 1,
          abort: new AbortController(),
          mount: null,
          phase: "submitting",
          retryable: true,
        };
        const revokedOwner = testSeams.inferenceApprovalUndoOwner;
        if (revokedOwner === null) throw new Error("inference approval undo owner missing before revocation");
        socket.triggerClose(4401);
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(scopedDirectoryStreams.size).toBe(0);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        expect(artifacts.has(state.runtimeKey)).toBe(false);
        expect(revokedOwner.abort.signal.aborted).toBe(true);
        expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
        expect(posted.some((message) => message.kind === "inference-history-status" && message.historyEpoch === revokedOwner.historyEpoch && message.status.phase === "unavailable")).toBe(true);
        expect(posted.some((message) => message.kind === "directory-scope-revoked")).toBe(true);
      } finally {
        closeDirectory();
        testSeams.socketGrantTestIssue = null;
        testSeams.workerPostTestSink = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        vi.useRealTimers();
      }
    });

  });
  //#endregion 🔖️DirectoryLaneTests
}
