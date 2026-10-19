/** 🏁️ The unsolicited typed operation completion, progress and shared vector tests. */
import { expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { encodePackValue, packUInt } from "@semio-tech/framework/packs";
import { source, type AppFrameValue } from "../../🛠️backbone-wire-helpers/🟦️.ts";
import { fakeHandle } from "../🧰️app-channel-client-fixtures/🟦️.ts";

/** ✅ Registers the unsolicited completion, progress scope and history patch, shared completion vector and archive merge vector tests. */
export function registerTypedOperationCompletionTests(): void {
  /**
   * 🏁️ The defect this frame exists for: a retained typed operation reaches its terminal result on a
   * continuation turn no host call is awaiting. The old wire published that completion as an
   * `Invocation` with `in_reply_to: 0`, which correlates with NO pending waiter, so the shell simply
   * dropped it — the outliner, the inspection panel and the History list all kept showing the
   * previous document after an example was selected.
   */
  it("delivers an unsolicited typed-operation completion to its subscriber exactly once, never to a pending command waiter", async () => {
    const completions: unknown[] = [];
    const handle = fakeHandle((_instanceId, commands) => [
      {
        OperationCompleted: {
          operation: 9n,
          revision: 4n,
          ui_scope: Array.from(encodePackValue("full")),
          history_patch: Array.from(encodePackValue({ cursor: packUInt(7n), upserts: [{ seq: packUInt(41n), label: "Set Active Example", count: packUInt(1n) }], canUndo: true, canRedo: false, commandFilter: "all" })),
        },
      },
      { Invocation: { in_reply_to: Object.values(commands[0]!)[0]!.seq, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const unsubscribe = client.onOperationCompleted((completion) => completions.push(completion));
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(1);
    expect(frames.every((frame) => "Invocation" in frame)).toBe(true);
    expect(completions).toHaveLength(1);
    const completion = completions[0] as { readonly instanceId: number; readonly operation: bigint; readonly revision: bigint; readonly uiScope: unknown; readonly historyPatch: { readonly cursor: number; readonly upserts: readonly { readonly seq: number }[] } };
    expect(completion.instanceId).toBe(1);
    expect(completion.operation).toBe(9n);
    expect(completion.revision).toBe(4n);
    expect(completion.uiScope).toBe("full");
    expect(completion.historyPatch.cursor).toBe(7);
    expect(completion.historyPatch.upserts[0]!.seq).toBe(41);
    expect(`framework.history.entry.${completion.historyPatch.upserts[0]!.seq}`).toBe("framework.history.entry.41");
    unsubscribe();
    await client.command(new Uint8Array([2]), {});
    expect(completions).toHaveLength(1);
  });
  /**
   * 🎞️ A running operation (a tool run) hands the shell its dirty scope mid-operation as an `Invocation` answering no
   * command. Once the operation runs on the background drain that frame correlates with no waiter, so without this
   * subscription the renderer saw nothing until the run ended and then everything at once.
   */
  it("delivers an unsolicited operation progress scope and history patch to its subscriber, never to a pending command waiter", async () => {
    const progress: unknown[] = [];
    const replay = { cursor: 9, timeTravel: { sessionId: "1", generation: 4, stage: "replaying", done: 3, total: 8, blocking: false, acceptedCount: 1 } };
    const handle = fakeHandle((_instanceId, commands) => [
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue({ kind: "partial", panelBodies: ["framework.body.toolRun"] })), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: Array.from(encodePackValue(replay)), messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: Object.values(commands[0]!)[0]!.seq, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue("full")), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const unsubscribe = client.onOperationProgress((uiScope) => progress.push(uiScope));
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(1);
    expect(progress).toEqual([{ uiScope: { kind: "partial", panelBodies: ["framework.body.toolRun"] } }, { historyPatch: replay }]);
    unsubscribe();
    await client.command(new Uint8Array([2]), {});
    expect(progress).toHaveLength(2);
  });
  it("matches the shared cross-language typed-operation completion fixture vector, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const vectors = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures", "📡️channel", "🏁️app-frame-operation-completed.json"), "utf8")) as Record<string, string>;
    for (const [key, operation, revision] of [["OperationCompleted", 7n, 5n], ["OperationCompletedWideRevision", 7n, 0xfedc_ba98_7654_3210n], ["OperationCompletedWideOperation", 0xffff_ffff_ffff_ffffn, 5n]] as const) {
      const frame: AppFrameValue = { OperationCompleted: { operation, revision, ui_scope: [1], history_patch: [2] } };
      const hex = Array.from(encodeAppFrame(frame), (byte) => byte.toString(16).padStart(2, "0")).join("");
      expect(hex).toBe(vectors[key]);
      expect(decodeAppFrame(new Uint8Array(Buffer.from(vectors[key]!, "hex")))).toEqual(frame);
    }
  });
  it("matches the shared cross-language document-archive merge vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const vectors = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures", "📡️channel", "🧲️document-archive-merge.json"), "utf8")) as { seq: number; archive: { parent_pack: number[]; parent_spr: number[]; members: [] }; LoadDocumentArchive: string; MergeDocumentArchive: string; status: { in_reply_to: number; operation: number; state: string; completed: number; total: number; ahead: number; fault: number[] }; DocumentArchiveLoad: string };
    const hex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
    const load = { LoadDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };
    const merge = { MergeDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };
    expect(hex(encodeAppCommand(load))).toBe(vectors.LoadDocumentArchive);
    expect(hex(encodeAppCommand(merge))).toBe(vectors.MergeDocumentArchive);
    expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.LoadDocumentArchive, "hex")))).toEqual(load);
    expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.MergeDocumentArchive, "hex")))).toEqual(merge);
    expect(vectors.status.state).toBe("ready");
    const frame: AppFrameValue = { DocumentArchiveLoad: { in_reply_to: vectors.status.in_reply_to, status: { operation: vectors.status.operation, state: "ready", completed: vectors.status.completed, total: vectors.status.total, ahead: vectors.status.ahead, fault: vectors.status.fault } } };
    expect(hex(encodeAppFrame(frame))).toBe(vectors.DocumentArchiveLoad);
    expect(decodeAppFrame(new Uint8Array(Buffer.from(vectors.DocumentArchiveLoad, "hex")))).toEqual(frame);
  });
}
