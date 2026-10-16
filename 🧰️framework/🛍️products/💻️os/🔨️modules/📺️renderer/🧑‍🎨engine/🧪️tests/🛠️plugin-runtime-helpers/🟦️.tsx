/** 🛠 Shared fixtures of the PluginRuntime suites: packed carriers, fake handles and the shard-client swap. */
import type { HistoryPatch } from "@semio-tech/framework";
import type { LocalInteractionCapture } from "@semio-tech/framework-replication";
import type { ShardBudget, ShardClient, ShardEventEnvelope } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import type { PluginManifest } from "../../🧱️elements/🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import type { PluginRuntimeTestDependenciesV1, PluginWasmHandle } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";

export type PluginRuntimeVitest = Pick<typeof import("vitest"), "describe" | "expect" | "it" | "vi">;
export type PluginRuntimeDependencies = PluginRuntimeTestDependenciesV1;
export type PluginRuntimeSource = { url: string };

type FakeHandleOptions = {
  readonly prepareForeign?: (instanceId: number) => readonly Uint8Array[];
  readonly prepareRejection?: Uint8Array;
  readonly commitRejects?: boolean;
  readonly pack?: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null;
};

export const fakeLocalInteraction = (instanceId: number): LocalInteractionCapture => ({
  identity: { appInstanceId: instanceId, generation: "1", revision: "0".repeat(64), documentRevision: "0".repeat(64), topologyRevision: "0".repeat(64) },
  state: { selection: {}, activeMode: {}, activeGranularity: {} },
});

const unexpectedMediaRequest = async (): Promise<never> => { throw new Error("Transaction fixture received an unexpected media request"); };

export const unexpectedMediaRequests = { submitMediaExport: unexpectedMediaRequest, pollMediaExport: unexpectedMediaRequest, cancelMediaExport: unexpectedMediaRequest, takeMediaExportChunk: unexpectedMediaRequest };

/** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime). No real sleeps anywhere
 * below — every wait is a `queueMicrotask` flush or a deferred promise the test itself settles. */
export const flushMicrotasks = async (times = 4): Promise<void> => {
  for (let index = 0; index < times; index += 1) await new Promise<void>((resolve) => queueMicrotask(resolve));
};

export function createPluginRuntimeSupport(dependencies: PluginRuntimeDependencies) {
  const { testState, encodePackValue } = dependencies;
  function encodeFaultBytes(code: string): Uint8Array {
    return encodePackValue({ origin: "os", code, severity: "error", message: code, scope: {}, retryable: false });
  }

  function fakeHandle(pluginId: string, calls: string[], commitOrder: string[], options: FakeHandleOptions = {}): PluginWasmHandle {
    return {
      ...unexpectedMediaRequests,
      pluginId,
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
      documentPack: (instanceId) => (options.pack !== undefined ? options.pack : { pack: new Uint8Array([1]), spr: new Uint8Array([instanceId]) }),
      transactionPrepare: async (instanceId, _txnId, _request) => {
        calls.push(`${pluginId}:${instanceId}:prepare`);
        if (options.prepareRejection) return { foreign: [], rejection: options.prepareRejection };
        return { foreign: options.prepareForeign?.(instanceId) ?? [], rejection: null };
      },
      transactionCommit: async (instanceId, _txnId) => {
        if (options.commitRejects) return { rejection: encodeFaultBytes("transaction.commit-failed") };
        commitOrder.push(`${pluginId}:${instanceId}`);
        return { editId: `edit-${pluginId}-${instanceId}` };
      },
      transactionRollback: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:rollback`);
      },
      transactionUndo: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:undo`);
      },
      transactionRedo: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:redo`);
      },
      setMergePolicy: async () => {},
      resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
      readConflicts: async () => [],
      dispose: async () => {},
    };
  }

  function withFakeShardClient<T>(turnImpl: (actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget) => Promise<unknown>, run: () => Promise<T>): Promise<T> {
    const previous = testState.sharedShardClient;
    testState.sharedShardClient = { turn: turnImpl } as unknown as ShardClient;
    return run().finally(() => {
      testState.sharedShardClient = previous;
    });
  }
  return { encodeFaultBytes, fakeHandle, withFakeShardClient };
}
