/** 🏗 PluginRuntime transaction coordinator and history snapshot bounded retry. */
import type { HistoryPatch } from "@semio-tech/framework";
import type { PluginManifest } from "../../🧱️elements/🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import type { PluginRuntimeTestDependenciesV1, PluginWasmHandle } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { fakeLocalInteraction, unexpectedMediaRequests, createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTransactionCoordinatorTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { ArtifactMutationRouter, encodePackValue, InstanceDirectory, MAX_TRANSACTION_DEPTH, TransactionCoordinator } = dependencies;
  const { describe, expect, it } = vitest;
  const { encodeFaultBytes, fakeHandle } = createPluginRuntimeSupport(dependencies);
    function encodeForeignStepBytes(step: {
      readonly target: { readonly artifactId: string; readonly artifactKind: string };
      readonly mutationId: string;
      readonly payload: readonly number[];
      readonly label: string;
    }): Uint8Array {
      return encodePackValue(step);
    }

    describe("PluginRuntime TransactionCoordinator", () => {
      it("runs proposal -> prepare x2 -> commit, committing in reverse discovery order", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
  
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "duplicate" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [new Uint8Array([1])],
          foreign: [foreign],
        });
  
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) return;
        expect(calls).toEqual(["a-plugin:10:prepare", "b-plugin:20:prepare"]);
        // 🎯️ Reverse discovery order: initiator (a) discovered first, foreign target (b) discovered
        // second — commit visits b before a (contract freeze §5.6).
        expect(commitOrder).toEqual(["b-plugin:20", "a-plugin:10"]);
        expect(outcome.editIds.get("artifact-a")).toBe("edit-a-plugin-10");
        expect(outcome.editIds.get("artifact-b")).toBe("edit-b-plugin-20");
      });
  
      it("undoGroup fans TransactionUndo out to every member of a completed transaction", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [new Uint8Array([1])],
          foreign: [foreign],
        });
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) return;
  
        calls.length = 0;
        const undoResult = await coordinator.undoGroup(outcome.txnId);
        expect(undoResult.ok).toBe(true);
        expect(new Set(calls)).toEqual(new Set(["a-plugin:10:undo", "b-plugin:20:undo"]));
  
        const unknownUndo = await coordinator.undoGroup("not-a-real-group");
        expect(unknownUndo.ok).toBe(false);
      });
  
      it("rolls back a commit-failed transaction: undoes what already committed, rolls back the rest", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.doc" });
        directory.register("artifact-c", { pluginId: "c-plugin", instanceId: 30, artifactKind: "s.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.doc", "s#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { commitRejects: true })],
          ["c-plugin", fakeHandle("c-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreignB = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [1], label: "x" });
        const foreignC = encodeForeignStepBytes({ target: { artifactId: "artifact-c", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [2], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.doc",
          localOps: [new Uint8Array([1])],
          foreign: [foreignB, foreignC],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.commit-failed" });
        // discovery order was [a, b, c]; commit visits c (succeeds) then b (fails) — c is already
        // committed so it gets undone, a (never reached) gets rolled back alongside the failing b.
        expect(commitOrder).toEqual(["c-plugin:30"]);
        expect(calls).toContain("c-plugin:30:undo");
        expect(calls).toContain("a-plugin:10:rollback");
        expect(calls).toContain("b-plugin:20:rollback");
      });
  
      it("reaches transaction.unknown-target when the initiator plugin has no registered handle", async () => {
        const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), new Map());
        const outcome = await coordinator.run({
          initiatorPluginId: "missing-plugin",
          initiatorInstanceId: 1,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
      });
  
      it("reaches transaction.unknown-target when InstanceDirectory has no entry for the foreign target", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder)]]);
        const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-unknown", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
      });
  
      it("reaches transaction.unknown-mutation when the router has no entry for a foreign step", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, new ArtifactMutationRouter(), plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#nope", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-mutation" });
      });
  
      it("reaches transaction.contribution-not-permitted when a contributed mutation has no planner wired", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerContributed(
          "s.b.doc",
          "aec-building",
          "b-plugin",
          { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        );
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins); // no planner injected
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.contribution-not-permitted" });
      });
  
      it("plans and prepares a contributed mutation using the target's cached document pack", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerContributed(
          "s.b.doc",
          "aec-building",
          "b-plugin",
          { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        );
        const seenPlanRequests: { readonly targetPack: Uint8Array; readonly targetSpr: Uint8Array }[] = [];
        const targetPack = { pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) };
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { pack: targetPack })],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins, async (contributorPluginId, request) => {
          seenPlanRequests.push({ targetPack: request.targetPack, targetSpr: request.targetSpr });
          expect(contributorPluginId).toBe("aec-building");
          return { ops: [new Uint8Array([42])], label: "aec-building add-room" };
        });
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign],
        });
        expect(outcome.ok).toBe(true);
        expect(seenPlanRequests).toEqual([{ targetPack: targetPack.pack, targetSpr: targetPack.spr }]);
      });
  
      it("reaches transaction.cycle when the same (artifact, mutation, payload) step repeats", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign, foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.cycle" });
      });
  
      it("reaches transaction.depth-exceeded when a foreign-step chain runs past MAX_TRANSACTION_DEPTH", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        const chainLength = MAX_TRANSACTION_DEPTH + 4;
        for (let index = 1; index <= chainLength; index += 1) {
          directory.register(`artifact-${index}`, { pluginId: "chain-plugin", instanceId: index, artifactKind: "s.chain.doc" });
        }
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.chain.doc", "s.chain#step");
        const chainHandle: PluginWasmHandle = {
          ...unexpectedMediaRequests,
          pluginId: "chain-plugin",
          manifest: {} as unknown as PluginManifest,
          createApp: async () => 0,
          destroyApp: async () => {},
          takeSegmentedDownloadChunk: async () => undefined,
          handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
          refreshUi: async () => ({}),
          contextMenu: async () => [],
          readHistory: async () => ({ cursor: 0 }) as unknown as HistoryPatch,
          readLocalInteraction: async (instanceId) => fakeLocalInteraction(instanceId),
          subscribeOperationCompletions: () => () => {},
        subscribeOperationProgress: () => () => {},
          readWindowConfigPacks: async () => [],
          loadWindowConfigPack: async () => {},
          readAppDocumentIdentity: async (instanceId) => ({ app_instance_id: instanceId, parent_document_id: `fake-document-${instanceId}` }),
          documentPack: () => ({ pack: new Uint8Array([1]), spr: new Uint8Array([2]) }),
          transactionPrepare: async (instanceId) => {
            calls.push(`chain:${instanceId}:prepare`);
            const next = instanceId + 1;
            if (next > chainLength) return { foreign: [], rejection: null };
            return {
              foreign: [encodeForeignStepBytes({ target: { artifactId: `artifact-${next}`, artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [next], label: "x" })],
              rejection: null,
            };
          },
          transactionCommit: async (instanceId) => {
            commitOrder.push(`chain:${instanceId}`);
            return { editId: `edit-${instanceId}` };
          },
          transactionRollback: async (instanceId) => {
            calls.push(`chain:${instanceId}:rollback`);
          },
          transactionUndo: async () => {},
          transactionRedo: async () => {},
          setMergePolicy: async () => {},
          resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
          readConflicts: async () => [],
          dispose: async () => {},
        };
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["chain-plugin", chainHandle],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-1", artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.depth-exceeded" });
      });
  
      it("passes a member's TransactionPrepared.rejection code straight through — instance-busy, generation-mismatch, and the member-rejected default all reachable", async () => {
        const directory = new InstanceDirectory();
        const router = new ArtifactMutationRouter();
        for (const code of ["transaction.instance-busy", "transaction.generation-mismatch", "not-a-real-fault-code"]) {
          const calls: string[] = [];
          const commitOrder: string[] = [];
          const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder, { prepareRejection: code === "not-a-real-fault-code" ? new Uint8Array([255, 255, 255]) : encodeFaultBytes(code) })]]);
          const coordinator = new TransactionCoordinator(directory, router, plugins);
          const outcome = await coordinator.run({
            initiatorPluginId: "a-plugin",
            initiatorInstanceId: 10,
            initiatorArtifactId: "artifact-a",
            initiatorArtifactKind: "s.a.doc",
            localOps: [],
            foreign: [],
          });
          const expectedCode = code === "not-a-real-fault-code" ? "transaction.member-rejected" : code;
          expect(outcome).toEqual({ ok: false, code: expectedCode });
        }
      });
    });

    /** 🧾️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice C1 — the history-snapshot read's
     * refusal law. `plugin_exchange`'s `ReadHistory` arm takes the instance cell with `try_lock`, so a
     * read racing the same instance's own turn comes back as an `Error` frame with no `HistorySnapshot`
     * beside it. The adapter used to report that as `missing HistorySnapshot frame` and throw away the
     * guest's own sentence, which is why the collaboration e2e's blocker was read for a month as a hub
     * bootstrap-ordering bug rather than a per-instance lock race. Three laws are pinned: a transient
     * refusal is retried on a bounded doubling ladder, an exhausted ladder raises the guest's text, and
     * a fault that is NOT the transient one is raised on the first reply without a single retry. */
    describe("history snapshot bounded retry", () => {
      const busyFrames = (instanceId: number): readonly unknown[] => [
        { Error: { in_reply_to: 1, fault: Array.from(new TextEncoder().encode(`plugin.internal: instance busy or poisoned: ${instanceId}`)), report: [] } },
      ];

      it("retries a transient `instance busy` refusal on a doubling backoff and returns the snapshot the guest finally yields", async () => {
        const { readHistoryWithBoundedRetryV1, encodePackValue, PLUGIN_INSTANCE_BUSY_BACKOFF_MS } = dependencies;
        const waits: number[] = [];
        let reads = 0;
        const channel = {
          readHistory: async () => {
            reads += 1;
            if (reads < 3) return busyFrames(7);
            return [{ HistorySnapshot: { in_reply_to: 1, history_patch: Array.from(encodePackValue({ cursor: 3 })) } }];
          },
        } as unknown as Parameters<PluginRuntimeTestDependenciesV1["readHistoryWithBoundedRetryV1"]>[0];
        const patch = await readHistoryWithBoundedRetryV1(channel, 7, async (ms: number) => { waits.push(ms); });
        expect(reads, "one read per attempt until the guest answers").toBe(3);
        expect(waits, "a doubling ladder, never a spin").toEqual([PLUGIN_INSTANCE_BUSY_BACKOFF_MS, PLUGIN_INSTANCE_BUSY_BACKOFF_MS * 2]);
        expect(patch).toEqual({ cursor: 3 });
      });

      it("stops at the bound and raises the guest's own fault text rather than a `missing HistorySnapshot frame` sentence", async () => {
        const { readHistoryWithBoundedRetryV1, PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS } = dependencies;
        const waits: number[] = [];
        let reads = 0;
        const channel = { readHistory: async () => { reads += 1; return busyFrames(7); } } as unknown as Parameters<PluginRuntimeTestDependenciesV1["readHistoryWithBoundedRetryV1"]>[0];
        let raised: unknown = null;
        try {
          await readHistoryWithBoundedRetryV1(channel, 7, async (ms: number) => { waits.push(ms); });
        } catch (error) {
          raised = error;
        }
        expect(reads, "bounded — the ladder never re-arms itself").toBe(PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS);
        expect(waits.length, "and never waits after the last attempt").toBe(PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS - 1);
        expect(String(raised)).toContain("instance busy or poisoned: 7");
        expect(String(raised)).not.toContain("missing HistorySnapshot frame");
      });

      it("raises a fault that is not the transient lock refusal on the first reply, with no retry at all", async () => {
        const { readHistoryWithBoundedRetryV1 } = dependencies;
        let reads = 0;
        const channel = {
          readHistory: async () => {
            reads += 1;
            return [{ Error: { in_reply_to: 1, fault: Array.from(new TextEncoder().encode("plugin.internal: history projection is poisoned")), report: [] } }];
          },
        } as unknown as Parameters<PluginRuntimeTestDependenciesV1["readHistoryWithBoundedRetryV1"]>[0];
        let raised: unknown = null;
        try {
          await readHistoryWithBoundedRetryV1(channel, 4, async () => {});
        } catch (error) {
          raised = error;
        }
        expect(reads, "a terminal fault is terminal on the first reply").toBe(1);
        expect(String(raised)).toContain("history projection is poisoned");
      });

      it("names the transient refusal by its guest sentence and nothing else", () => {
        const { isPluginInstanceBusyFaultV1 } = dependencies;
        expect(isPluginInstanceBusyFaultV1("plugin.internal: instance busy or poisoned: 12")).toBe(true);
        expect(isPluginInstanceBusyFaultV1("plugin.internal: no actor for instance 12")).toBe(false);
      });
    });
}
