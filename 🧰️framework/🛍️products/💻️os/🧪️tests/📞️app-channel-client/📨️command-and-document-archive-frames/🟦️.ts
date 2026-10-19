/** 📨️ The command sequencing, document configure and archive frame tests. */
import { expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { encodePackValue } from "@semio-tech/framework/packs";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { source, documentArchive, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../../🛠️backbone-wire-helpers/🟦️.ts";
import { commandSeq, fakeHandle } from "../🧰️app-channel-client-fixtures/🟦️.ts";

/** 🗃 Registers the command sequence, child heads, configure and read, archive load and cancel, window config, cache candidate and transaction tests. */
export function registerCommandAndDocumentArchiveFrameTests(): void {
  it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
    const seqsSeen: number[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      const cmd = commands[0];
      if (cmd && "Command" in cmd) seqsSeen.push(cmd.Command.seq);
      return [
        { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
        { UiPatch: { in_reply_to: seqsSeen.at(-1) ?? 0, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
      ];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
    const second = await client.command(new Uint8Array([3]), { cursor: 1 });
    expect(seqsSeen).toEqual([1, 2]);
    expect(first).toHaveLength(2);
    expect(second).toHaveLength(2);
  });
  it("readChildHeads() sends ReadChildHeads and answers the ChildHeads entries of its own sequence, owning their bytes", async () => {
    const seen: AppCommandValue[] = [];
    const heads = [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8], owner: "content/forms-1" }];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      return "ReadChildHeads" in command ? [{ ChildHeads: { in_reply_to: command.ReadChildHeads.seq, entries: heads } }] : [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const entries = await client.readChildHeads();
    heads[0]!.head_pack.fill(0);
    expect([seen, entries]).toEqual([[{ ReadChildHeads: { seq: 1 } }], [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8], owner: "content/forms-1" }]]);
    const refusing = new AppChannelClient(fakeHandle((_instanceId, commands) => [{ Done: { in_reply_to: commandSeq(commands[0]!) } }]), new AppChannelRequestSequence(), 1, "app.demo");
    await expect(refusing.readChildHeads()).rejects.toThrow("missing ChildHeads frame for seq 1");
  });
  it("configure()/readDocument() frame the right AppCommand variant", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.configure({ locale: "en" });
    await client.readDocument();
    expect(seen).toEqual([{ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } }, { ReadDocument: { seq: 2 } }]);
  });
  it("loadDocumentArchive()/readDocumentArchive() preserve the complete recursive closure and root cache", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("ReadDocumentArchive" in command) return [{ DocumentArchive: { in_reply_to: command.ReadDocumentArchive.seq, archive: documentArchive } }];
      if ("PollDocumentArchiveLoad" in command) return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "ready", completed: 3, total: 3, ahead: 0, fault: [] } } }];
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.loadDocumentArchive(documentArchive);
    const actual = await client.readDocumentArchive();
    expect(seen).toEqual([
      { LoadDocumentArchive: { seq: 1, archive: documentArchive } },
      { PollDocumentArchiveLoad: { seq: 2, operation: 1 } },
      { AcknowledgeDocumentArchiveLoad: { seq: 3, operation: 1 } },
      { ReadDocumentArchive: { seq: 4 } },
    ]);
    expect(actual).toEqual(documentArchive);
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
  });
  it("loadDocumentArchive() sends nothing for a cancel before admission, and cancels, polls and acknowledges the exact retained operation after it", async () => {
    const seen: AppCommandValue[] = [];
    const controller = new AbortController();
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("LoadDocumentArchive" in command) controller.abort(new Error("caller cancelled archive"));
      if ("PollDocumentArchiveLoad" in command) {
        return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "cancelled", completed: 0, total: 3, ahead: 0, fault: [] } } }];
      }
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const early = new AbortController();
    early.abort(new Error("caller cancelled before admission"));
    await expect(client.loadDocumentArchive(documentArchive, early.signal)).rejects.toThrow("caller cancelled before admission");
    expect(seen).toEqual([]);
    await expect(client.loadDocumentArchive(documentArchive, controller.signal)).rejects.toThrow("caller cancelled archive");
    expect(seen).toEqual([
      { LoadDocumentArchive: { seq: 1, archive: documentArchive } },
      { CancelDocumentArchiveLoad: { seq: 2, operation: 1 } },
      { PollDocumentArchiveLoad: { seq: 3, operation: 1 } },
      { AcknowledgeDocumentArchiveLoad: { seq: 4, operation: 1 } },
    ]);
  });
  it("loadDocumentArchive() reports a cancel the guest decided on its own (a person's Cancel in the history body) as an AbortError, after its progress and acknowledgement", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("PollDocumentArchiveLoad" in command) {
        const state = seen.filter((sent) => "PollDocumentArchiveLoad" in sent).length === 1 ? "running" : "cancelled";
        return [{ DocumentArchiveLoad: { in_reply_to: command.PollDocumentArchiveLoad.seq, status: { operation: command.PollDocumentArchiveLoad.operation, state, completed: 1, total: 3, ahead: 0, fault: [] } } }];
      }
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const progress: string[] = [];
    const outcome = await client.loadDocumentArchive(documentArchive, undefined, (status) => progress.push(`${status.state} ${status.completed}/${status.total}`)).then(() => null, (error: unknown) => error);
    expect([outcome instanceof DOMException && outcome.name === "AbortError", progress, seen.map((sent) => Object.keys(sent)[0]), client.documentPack()]).toEqual([true, ["running 1/3", "cancelled 1/3"], ["LoadDocumentArchive", "PollDocumentArchiveLoad", "PollDocumentArchiveLoad", "AcknowledgeDocumentArchiveLoad"], null]);
  });
  it("loads and reads exact persisted-local window config envelopes", async () => {
    const seen: AppCommandValue[] = [];
    const expected = [
      { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] },
      { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [3] },
    ];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      const command = commands[0]!;
      if ("ReadWindowConfigs" in command) return [{ WindowConfigs: { in_reply_to: command.ReadWindowConfigs.seq, entries: expected } }];
      return [{ Done: { in_reply_to: commandSeq(command) } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const source = { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] };
    await client.loadWindowConfig(source);
    const actual = await client.readWindowConfigs();
    source.envelope_pack.fill(255);
    expect(seen).toEqual([
      { LoadWindowConfig: { seq: 1, entry: { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] } } },
      { ReadWindowConfigs: { seq: 2 } },
    ]);
    expect(actual).toEqual(expected);
  });
  it("publishes only accepted document cache candidates and owns both byte arrays", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📦️document-cache/🔣️.json", source.url), "utf8"));
    const pair = (value: { pack: number[]; spr: number[] }) => ({ pack: Uint8Array.from(value.pack), spr: Uint8Array.from(value.spr) });
    const archiveOf = (value: { pack: number[]; spr: number[] }) => ({ parent_pack: [...value.pack], parent_spr: [...value.spr], members: [] });
    for (const row of fixture.cases) {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      let admission = "done";
      const client = new AppChannelClient({
        outcomes: broadcast.stream,
        enqueue: (_instanceId: number, commands: readonly Uint8Array[]) => {
          const command = decodeAppCommand(commands[0]!);
          if ("LoadDocumentArchive" in command && admission === "enqueue") throw new Error("document-cache.enqueue");
          const seq = commandSeq(command);
          const document = { Document: { in_reply_to: seq, pack: fixture.reply.pack, spr: fixture.reply.spr, ops: "" } };
          const error = { Error: { in_reply_to: seq, fault: [99], report: [] } };
          const reply: AppFrameValue[] | "transport" =
            "PollDocumentArchiveLoad" in command ? [{ DocumentArchiveLoad: { in_reply_to: seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "ready", completed: 1, total: 1, ahead: 0, fault: [] } } }] :
            !("LoadDocumentArchive" in command) || admission === "done" ? [{ Done: { in_reply_to: seq } }] :
            admission === "transport" ? "transport" :
            admission === "document" ? [document] :
            admission === "error-document" ? [document, error] : [error];
          queueMicrotask(() => broadcast.push(reply === "transport" ? { instanceId: 1, error: new Error("document-cache.transport") } : { instanceId: 1, frames: reply.map(encodeAppFrame) }));
        },
      }, new AppChannelRequestSequence(), 1, "cache");
      try {
        await client.loadDocumentArchive(archiveOf(fixture.initial));
        expect(client.documentPack()).toEqual(pair(fixture.initial));
        admission = row.outcome;
        const candidate = archiveOf(fixture.candidate);
        const settled = client.loadDocumentArchive(candidate).then(() => "resolved", () => "rejected");
        candidate.parent_pack.fill(255);
        candidate.parent_spr.fill(255);
        expect(client.documentPack()).toEqual(pair(fixture.initial));
        expect(await settled).toBe(row.outcome === "done" ? "resolved" : "rejected");
        expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
        const exposed = client.documentPack()!;
        exposed.pack.fill(254);
        exposed.spr.fill(254);
        expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
        client.dispose();
        expect(client.documentPack()).toBeNull();
      } finally {
        client.dispose();
        broadcast.complete();
      }
    }
  });
  it("caches the document pack from every AppFrame::Document reply, most recent wins", async () => {
    const handle = fakeHandle((_instanceId, commands) => {
      const cmd = commands[0];
      if (cmd && "ReadDocument" in cmd) {
        return [{ Document: { in_reply_to: cmd.ReadDocument.seq, pack: [9, 9], spr: [8], ops: "" } }];
      }
      return [{ Done: { in_reply_to: 1 } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.readDocument();
    expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
  });
  it("transactionPrepareOwner()/transactionPreparePlanned()/transactionCommit()/transactionRollback()/transactionUndo()/transactionRedo() frame the right AppCommand variant", async () => {
    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.transactionPrepareOwner("txn-1", "s.doc#kind", new Uint8Array([1]));
    await client.transactionPreparePlanned("txn-1", [new Uint8Array([2]), new Uint8Array([3])], new Uint8Array([4]));
    await client.transactionCommit("txn-1");
    await client.transactionRollback("txn-1");
    await client.transactionUndo("grp-1");
    await client.transactionRedo("grp-1");
    expect(seen[0]).toEqual({ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.doc#kind", payload: [1], prepared_ops: [], origin: [], prepared_child_ops: [] } });
    expect(seen[1]).toEqual({ transactionPrepare: { seq: 2, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[2], [3]], origin: [4], prepared_child_ops: [] } });
    expect(seen[2]).toEqual({ transactionCommit: { seq: 3, txn_id: "txn-1" } });
    expect(seen[3]).toEqual({ transactionRollback: { seq: 4, txn_id: "txn-1" } });
    expect(seen[4]).toEqual({ transactionUndo: { seq: 5, group_id: "grp-1" } });
    expect(seen[5]).toEqual({ transactionRedo: { seq: 6, group_id: "grp-1" } });
  });
}
