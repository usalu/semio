/** 🎛️ The composer surface, retire and outcome stream tests. */
import { expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { encodePackValue } from "@semio-tech/framework/packs";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { type AppChannelHandle, type AppCommandValue, type TurnOutcome } from "../../🛠️backbone-wire-helpers/🟦️.ts";

/** 🌊 Registers the composer surface, retire, ended outcome stream, error outcome and undecodable frame tests. */
export function registerComposerSurfaceAndOutcomeStreamTests(): void {
  /** 🔌️ A handle whose replies the test answers by hand and whose outcome subscription counts how often it was asked to end. */
  function scriptedHandle(): { readonly handle: AppChannelHandle; readonly broadcast: ReturnType<typeof createTurnOutcomeBroadcast<TurnOutcome>>; readonly sent: AppCommandValue[]; readonly ended: { count: number } } {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const ended = { count: 0 };
    const outcomes: AsyncIterable<TurnOutcome> = {
      [Symbol.asyncIterator]: () => {
        const inner = broadcast.stream[Symbol.asyncIterator]();
        return { next: () => inner.next(), return: () => { ended.count += 1; return inner.return!(); } };
      },
    };
    return { handle: { enqueue: (_instanceId, events) => { sent.push(...events.map(decodeAppCommand)); }, outcomes }, broadcast, sent, ended };
  }
  const settle = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
  it("wires the composer to the full public surface of the client and to nothing else", () => {
    const surface = ["applyEnvelopes", "cancelMediaExport", "clearDefaultApp", "command", "configure", "contextMenu", "dispose", "documentPack", "ephemeral", "loadDocumentArchive", "loadWindowConfig", "mediaIn", "mediaOut", "mergeDocumentArchive", "onOperationCompleted", "onOperationProgress", "openArtifact", "pollMediaExport", "pushPresence", "readChildHeads", "readConflicts", "readDocument", "readDocumentArchive", "readDocumentIdentity", "readHistory", "readLocalInteractionPages", "readWindowConfigs", "resolveConflict", "retire", "setDefaultApp", "setMergePolicy", "submitMediaExport", "takeMediaExportChunk", "transactionCommit", "transactionPrepareOwner", "transactionPreparePlanned", "transactionRedo", "transactionRollback", "transactionUndo"];
    const { handle } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    for (const name of surface) expect(typeof (client as unknown as Record<string, unknown>)[name], name).toBe("function");
    expect(Object.getOwnPropertyNames(AppChannelClient.prototype).filter((name) => name !== "constructor").sort()).toEqual(surface);
    client.dispose();
  });
  it("retire() fails the queued commands, silences completion subscribers only and forgets the pack; dispose() then ends the subscription", async () => {
    const { handle, broadcast, sent, ended } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = client.readDocument();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Document: { in_reply_to: 1, pack: [9, 9], spr: [8], ops: "" } })] });
    await first;
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
    const completions: unknown[] = [];
    const progress: unknown[] = [];
    client.onOperationCompleted((completion) => completions.push(completion));
    client.onOperationProgress((update) => progress.push(update));
    const queued = client.configure({ locale: "en" });
    const queuedFailure = expect(queued).rejects.toThrow("app-channel.disposed");
    expect(sent.at(-1)).toEqual({ ConfigCommand: { seq: 2, command: Array.from(encodePackValue({ locale: "en" })) } });
    client.retire();
    await queuedFailure;
    expect(client.documentPack()).toBeNull();
    await expect(client.readDocument()).rejects.toThrow("app-channel.disposed");
    await expect(client.readLocalInteractionPages(async () => undefined)).rejects.toThrow("app-channel.disposed");
    const completed = encodeAppFrame({ OperationCompleted: { operation: 9n, revision: 4n, ui_scope: Array.from(encodePackValue("full")), history_patch: [] } });
    const progressing = encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: Array.from(encodePackValue("full")), history_patch: [], messages: [], mutations: [], inverse_group: [] } });
    broadcast.push({ instanceId: 1, frames: [completed, progressing] });
    await settle();
    expect([completions.length, progress.length, ended.count]).toEqual([0, 1, 0]);
    client.retire();
    expect(ended.count).toBe(0);
    client.dispose();
    expect(ended.count).toBe(1);
    broadcast.push({ instanceId: 1, frames: [progressing] });
    await settle();
    expect(progress).toHaveLength(1);
  });
  it("an ended outcome stream rejects every waiter and the running local interaction query and forgets the pack", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = client.readDocument();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Document: { in_reply_to: 1, pack: [5], spr: [6], ops: "" } })] });
    await first;
    const waiting = client.readDocument();
    const waitingFailure = expect(waiting).rejects.toThrow("app-channel.closed");
    const query = client.readLocalInteractionPages(async () => undefined);
    const queryFailure = expect(query).rejects.toThrow("local-interaction.channel-closed");
    broadcast.complete();
    await Promise.all([waitingFailure, queryFailure]);
    expect(client.documentPack()).toBeNull();
  });
  it("an error outcome fails only the oldest waiter, and a sibling instance's outcome settles nobody", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const outcomes: string[] = [];
    const oldest = client.readDocument().then(() => outcomes.push("oldest resolved"), (error: unknown) => outcomes.push(`oldest rejected ${(error as Error).message}`));
    const newest = client.readHistory().then(() => outcomes.push("newest resolved"), (error: unknown) => outcomes.push(`newest rejected ${(error as Error).message}`));
    broadcast.push({ instanceId: 2, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 2 } })] });
    broadcast.push({ instanceId: 2, error: new Error("sibling trapped") });
    await settle();
    expect(outcomes).toEqual([]);
    broadcast.push({ instanceId: 1, error: new Error("trapped") });
    await oldest;
    expect(outcomes).toEqual(["oldest rejected trapped"]);
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Done: { in_reply_to: 2 } })] });
    await newest;
    expect(outcomes).toEqual(["oldest rejected trapped", "newest resolved"]);
    client.dispose();
  });
  it("an undecodable frame fails every waiting command and leaves the channel usable", async () => {
    const { handle, broadcast } = scriptedHandle();
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = expect(client.readDocument()).rejects.toThrow();
    const second = expect(client.readHistory()).rejects.toThrow();
    broadcast.push({ instanceId: 1, frames: [Uint8Array.of(255, 255, 255, 255)] });
    await Promise.all([first, second]);
    const third = client.readHistory();
    broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] });
    expect(await third).toEqual([{ Done: { in_reply_to: 3 } }]);
    client.dispose();
  });
}
