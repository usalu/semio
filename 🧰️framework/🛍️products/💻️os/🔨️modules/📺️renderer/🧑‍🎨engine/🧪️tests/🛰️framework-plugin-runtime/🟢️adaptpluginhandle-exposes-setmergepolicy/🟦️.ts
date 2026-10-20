/** 🟢️ The adaptpluginhandle exposes setmergepolicy resolveconflict tests of the framework plugin runtime suite. */
import { expect, it } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { adaptPluginHandle, initialShellState, selectOpenConflicts, selectQuarantinedConflicts, shellReducer } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { exchangeStyleChannel } from "../🟣️shared/🟦️.ts";

/** 🔵 Registers the adaptpluginhandle exposes setmergepolicy resolveconflict tests of the framework plugin runtime suite. */
export function registerAdaptpluginhandleExposesSetmergepolicyResolveconflictTests(): void {
  it("adaptPluginHandle exposes setMergePolicy/resolveConflict/readConflicts and sends the real AppCommand wire frames — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane K2: the merge-policy Settings control and Conflicts panel Accept/Discard used to call `plugin.handle.setMergePolicy?.(…)` against a handle that never had the method, so the optional call silently no-opped — this asserts the method genuinely exists AND that calling it round-trips through the real `AppCommand`/`AppFrame` codecs, not an internal spy", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue } = await import("@semio-tech/framework-os");
    const sentCommands: unknown[] = [];
    const fakeHandle = {
      manifest: { pluginId: "mock-merge", label: "Mock Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 9,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        sentCommands.push(command);
        if (command && typeof command === "object" && "setMergePolicy" in command) return [encodeAppFrame({ Done: { in_reply_to: command.setMergePolicy.seq } })];
        if (command && typeof command === "object" && "resolveConflict" in command) {
          const seq = command.resolveConflict.seq;
          return [
            encodeAppFrame({ MergeReport: { in_reply_to: seq, report: Array.from(encodePackValue({ policy: "Normal", accepted: true, insertionIndex: 0, replayed: [], worst: null, conflict: null })) } }),
            encodeAppFrame({ Conflicts: { in_reply_to: seq, conflicts: Array.from(encodePackValue([])) } }),
          ];
        }
        if (command && typeof command === "object" && "readConflicts" in command) {
          return [encodeAppFrame({ Conflicts: { in_reply_to: command.readConflicts.seq, conflicts: Array.from(encodePackValue([])) } })];
        }
        throw new Error(`unexpected command ${JSON.stringify(command)}`);
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    expect(typeof handle.setMergePolicy).toBe("function");
    expect(typeof handle.resolveConflict).toBe("function");
    expect(typeof handle.readConflicts).toBe("function");
    const instanceId = await handle.createApp("main");

    await handle.setMergePolicy(instanceId, "Vigilant");
    expect(sentCommands[0]).toEqual({ setMergePolicy: { seq: 1, policy: 2 } });

    const resolved = await handle.resolveConflict(instanceId, "conflict-abc", "accept");
    expect(sentCommands[1]).toEqual({ resolveConflict: { seq: 2, conflict_id: "conflict-abc", resolution: 0 } });
    expect(resolved.mergeReport?.accepted).toBe(true);
    expect(resolved.conflicts).toEqual([]);

    await handle.readConflicts(instanceId);
    expect(sentCommands[2]).toEqual({ readConflicts: { seq: 3 } });
  });
  it("adaptPluginHandle.applyMutations decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane L1 gap 1: a peer's ApplyEnvelopes ingest batches MergeReport/Conflicts frames alongside it (contract freeze §C6/§C9 'pushed unsolicited after every ingest'), but `applyMutations` used to only look for an Error frame and silently drop everything else — this asserts the guest's real roster survives the decode AND that dispatching it through `shellReducer`'s SET_CONFLICTS (what ShellHost's `applyRemoteMerge` does) lands a remote-origin quarantined conflict in both `selectOpenConflicts` and `selectQuarantinedConflicts`, exactly the panel/badge lane K2 wired", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, encodeMutationEnvelopesPack } = await import("@semio-tech/framework-os");
    const remoteConflict = {
      id: "conflict-remote-1",
      kind: { kind: "quarantined", envelopes: [] },
      status: "open",
      messages: [{ level: "error", code: "mutation.target-missing", message: "peer deleted the renamed node" }],
      actors: ["peer-actor"],
      timestamp: { actor: 7, physical_ms: 1000, logical: 1 },
    };
    const fakeHandle = {
      manifest: { pluginId: "mock-remote-merge", label: "Mock Remote Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 11,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("ApplyEnvelopes" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
        const seq = command.ApplyEnvelopes.seq;
        return [
          encodeAppFrame({ Done: { in_reply_to: seq } }),
          encodeAppFrame({ MergeReport: { in_reply_to: null, report: Array.from(encodePackValue({ policy: "Normal", accepted: false, insertionIndex: 0, replayed: [], worst: "error", conflict: remoteConflict.id })) } }),
          encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: Array.from(encodePackValue([remoteConflict])) } }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-remote-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");

    if (!handle.applyMutations) throw new Error("Mutation fixture requires its owned mutation handler");
    const result = await handle.applyMutations(instanceId, encodeMutationEnvelopesPack([]));
    expect(result.mergeReport?.accepted).toBe(false);
    expect(result.mergeReport?.worst).toBe("error");
    expect(result.conflicts).toEqual([remoteConflict]);

    const state = shellReducer(initialShellState({ locale: "en", plugins: [], storage: createMemoryStoragePort() }), { type: "SET_CONFLICTS", value: result.conflicts ?? [] });
    expect(selectOpenConflicts(state)).toEqual([remoteConflict]);
    expect(selectQuarantinedConflicts(state)).toEqual([remoteConflict]);
  });
}
