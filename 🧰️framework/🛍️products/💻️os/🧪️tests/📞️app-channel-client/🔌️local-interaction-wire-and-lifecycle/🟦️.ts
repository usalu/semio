/** 🔌️ The local interaction outer wire, presence snapshot and client lifecycle tests. */
import { expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { source, type AppChannelHandle, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../../🛠️backbone-wire-helpers/🟦️.ts";
import { commandSeq } from "../🧰️app-channel-client-fixtures/🟦️.ts";
import type { LocalInteractionIdentity, LocalInteractionQueryReply, LocalInteractionQueryToken } from "../🧰️app-channel-client-fixtures/🟦️.ts";

/** 🧬 Registers the strict outer wire, presence snapshot, ACK ownership, sequence admission, reopened client and exhausted admission tests. */
export function registerLocalInteractionWireTests(): void {
  it("local interaction outer wire matches strict fixtures and the independent LEB128 oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported: unknown = await import(oracleModule);
    if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
    const oracle: unknown = Reflect.get(imported, "default");
    if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
    const encodeUnsigned: unknown = Reflect.get(oracle, "encodeUIntBuffer");
    if (typeof encodeUnsigned !== "function") throw new Error("missing LEB128 oracle encoder");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    const read: AppCommandValue = { LocalInteractionQuery: { seq: 9, command: { kind: "read", requestId: "13" } } };
    const rejected: AppFrameValue = { LocalInteractionQuery: { reply: { kind: "rejected", requestId: "13", code: "busy" } } };
    const u64 = (value: bigint): number[] => {
      const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(value);
      const encoded: unknown = encodeUnsigned(bytes);
      if (!(encoded instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
      return Array.from(encoded);
    };
    expect([...encodeAppCommand(read)]).toEqual([29, ...u64(9n), ...u64(2n), 0, ...u64(13n)]);
    expect(Buffer.from(encodeAppCommand(read)).toString("hex")).toBe(fixture.outerReadHex);
    expect(Buffer.from(encodeAppFrame(rejected)).toString("hex")).toBe(fixture.outerRejectedHex);
    const receipt: AppFrameValue = { Done: { in_reply_to: 2 } };
    expect([...encodeAppFrame(receipt)]).toEqual([0, ...u64(2n)]);
    expect(Buffer.from(encodeAppFrame(receipt)).toString("hex")).toBe(fixture.receiptHex);
    for (const row of fixture.sequenceCases) {
      if (row.result === null) continue;
      const sequence = row.result.sequence;
      expect([...encodeAppCommand({ ReadDocument: { seq: sequence } })]).toEqual([7, ...u64(BigInt(sequence))]);
      if (row.result.request !== null) {
        const inner = [0, ...u64(BigInt(row.result.request))];
        expect([...encodeAppCommand({ LocalInteractionQuery: { seq: sequence, command: { kind: "read", requestId: row.result.request } } })]).toEqual([29, ...u64(BigInt(sequence)), ...u64(BigInt(inner.length)), ...inner]);
      }
    }
    expect(decodeAppCommand(encodeAppCommand(read))).toEqual(read);
    expect(decodeAppFrame(encodeAppFrame(rejected))).toEqual(rejected);
    expect(() => decodeAppCommand(Uint8Array.from([...encodeAppCommand(read), 0]))).toThrow();
    expect(() => decodeAppFrame(Uint8Array.from([...encodeAppFrame(rejected), 0]))).toThrow();
  });
  it("keeps each instance's last published Ephemeral frame as its presence snapshot, from any outcome", async () => {
    const { decodePresenceInteraction, encodePresenceInteraction } = await import("@semio-tech/framework-replication");
    const presence = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/👕️canvas-presence/🔣️.json", source.url), "utf8")) as { readonly paint: readonly { readonly roster: readonly { readonly actor: string; readonly interaction?: Parameters<typeof encodePresenceInteraction>[0] }[] }[] };
    const selecting = presence.paint[0]!.roster.find((row) => row.actor === "peer-a")!.interaction!;
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const handle: AppChannelHandle = { enqueue: () => undefined, outcomes: broadcast.stream };
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
    const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
    expect(client.ephemeral()).toBeNull();
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Ephemeral: { presence: [1], presence_generation: 1, transient_generation: 1, interaction: [], tool_run: [], history_edit: [] } })] });
    await flush();
    expect(client.ephemeral()).toEqual({ presence: [1], presenceGeneration: 1, transientGeneration: 1, interaction: [], toolRun: [], historyEdit: [] });
    const { encodePresenceHistoryEdit } = await import("@semio-tech/framework-replication");
    const editing = encodePresenceHistoryEdit({ mutationId: "m-2", stage: "reviewing", drafts: 1 });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 0 } }), encodeAppFrame({ Ephemeral: { presence: [2, 3], presence_generation: 2, transient_generation: 5, interaction: encodePresenceInteraction(selecting), tool_run: [], history_edit: editing } })] });
    broadcast.push({ instanceId: 8, frames: [encodeAppFrame({ Ephemeral: { presence: [9], presence_generation: 9, transient_generation: 9, interaction: [], tool_run: [], history_edit: [] } })] });
    await flush();
    const snapshot = client.ephemeral();
    expect(snapshot?.presence).toEqual([2, 3]);
    expect([snapshot?.presenceGeneration, snapshot?.transientGeneration]).toEqual([2, 5]);
    expect(decodePresenceInteraction(Uint8Array.from(snapshot!.interaction), [0])).toEqual(selecting);
    expect([snapshot?.toolRun, snapshot?.historyEdit]).toEqual([[], editing]);
    client.dispose();
  });
  it("local interaction client fixture lifecycles preserve ACK ownership and ordinary replies", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    for (const row of fixture.lifecycles) {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_id, events) => {
        for (const command of events.map(decodeAppCommand)) {
          sent.push(command);
          if ("LocalInteractionQuery" in command && !(["disposal-before-read-receipt", "coalesced-query-and-ordinary-receipts"].includes(row.id) && command.LocalInteractionQuery.command.kind === "read")) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
      const abort = new AbortController();
      let consume!: () => void;
      const consumed = new Promise<void>((resolve) => { consume = resolve; });
      let complete = false;
      let queryResult: Promise<LocalInteractionIdentity | unknown> | undefined;
      let ordinary: Promise<AppFrameValue[]> | undefined;
      let ordinaryComplete = false;
      const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "41", identity: { appInstanceId: 7, generation: "9007199254740993", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
      const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
      for (const event of row.events) {
        if (event === "read") queryResult = client.readLocalInteractionPages(() => {
          if (row.id === "synchronous-consumer-failure") throw new Error("synchronous consumer fixture");
          if (row.id === "consumer-failure") return Promise.reject(new Error("consumer fixture"));
          return consumed;
        }, abort.signal).then((identity) => { complete = true; return identity; }, (error: unknown) => { complete = true; return error; });
        else if (event === "ordinary") ordinary = client.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
        else if (event === "started") push({ kind: "started", token });
        else if (event === "abort") abort.abort();
        else if (event === "dispose") client.dispose();
        else if (event === "malformed") broadcast.push({ instanceId: 7, frames: [Uint8Array.from(Buffer.from(fixture.malformedFrameHex, "hex"))] });
        else if (event === "readReceipt") broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } })] });
        else if (event === "wrongReceipt" || event === "duplicateReadReceipt") {
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: event === "wrongReceipt" ? 999 : 1 } })] });
          await flush();
          expect(ordinaryComplete).toBe(false);
        }
        else if (event === "ordinaryDone") { broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] }); expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]); }
        else if (event === "ordinaryEmpty") { broadcast.push({ instanceId: 7, frames: [] }); await flush(); expect(ordinaryComplete).toBe(false); }
        else if (event === "mixedReceipts") {
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 3 } })] });
          await flush(); expect(ordinaryComplete).toBe(true);
          expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]);
        }
        else if (event === "startedWithNotice" || event === "pageWithNotice") {
          const reply: LocalInteractionQueryReply = event === "startedWithNotice" ? { kind: "started", token } : { kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } };
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } }), encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [], tool_run: [], history_edit: [] } }), encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "fixture", kind: "graph", revision: 1, base_revision: 0, ops: [] } })] });
          await flush(); expect(ordinaryComplete).toBe(false);
        }
        else if (event === "page") push({ kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } });
        else if (event === "consume") consume();
        else if (event === "consumeError") await flush();
        else if (event === "ack" || event === "cancel") {
          await flush();
          const last = sent.at(-1);
          expect(last && "LocalInteractionQuery" in last ? last.LocalInteractionQuery.command : null).toEqual({ kind: event === "ack" ? "acknowledge" : "cancel", token });
        } else if (event === "closed") {
          expect(complete).toBe(false);
          push({ kind: "closed", token, cancelled: row.cancelled });
        }
        await flush();
      }
      const result = await queryResult;
      expect(complete).toBe(true);
      if (!row.cancelled) expect(result).toEqual(token.identity);
      else expect(result).toBeInstanceOf(Error);
      client.dispose();
    }
  });
  it("local interaction sequence admission matches the checked shared-owner fixture", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
    for (const row of fixture.sequenceCases) {
      const owner = new AppChannelRequestSequence(row.sequence, BigInt(row.request));
      const allocate = () => row.operation === "query" ? owner.nextQuery() : { sequence: owner.nextSequence(), cancelSequence: null, request: null };
      if (row.result === null) expect(allocate).toThrow();
      else expect(allocate()).toEqual(row.result);
      expect(owner.checkpoint()).toEqual(row.after);
    }
    for (const invalid of [-1, Number.MAX_SAFE_INTEGER + 1, NaN, 0.5]) expect(() => new AppChannelRequestSequence(invalid)).toThrow();
    for (const invalid of [-1n, 0x1_0000_0000_0000_0000n]) expect(() => new AppChannelRequestSequence(0, invalid)).toThrow();
  });
  it("local interaction reopened clients reject delayed Started pages and ordinary receipts", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8")).reopen;
    const owner = new AppChannelRequestSequence();
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
      for (const command of bytes.map(decodeAppCommand)) {
        sent.push(command);
        if ("LocalInteractionQuery" in command) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
      }
    }, outcomes: broadcast.stream };
    const flush = async () => { for (let index = 0; index < 16; index += 1) await Promise.resolve(); };
    const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
    const oldToken: LocalInteractionQueryToken = { requestId: fixture.requests[0], queryGeneration: "41", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
    const newToken: LocalInteractionQueryToken = { ...oldToken, requestId: fixture.requests[1], queryGeneration: "42" };
    const first = new AppChannelClient(handle, owner, 7, "fixture");
    const firstResult = first.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
    first.dispose();
    await flush();
    push({ kind: "started", token: oldToken });
    await flush();
    push({ kind: "closed", token: oldToken, cancelled: true });
    expect(await firstResult).toBeInstanceOf(Error);
    const second = new AppChannelClient(handle, owner, 7, "fixture");
    let consumed = 0;
    let ordinaryComplete = false;
    const secondResult = second.readLocalInteractionPages(async () => { consumed += 1; });
    const ordinary = second.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
    for (const sequence of [...fixture.readSequences.slice(0, 1), ...fixture.cancelSequences.slice(0, 1), 999]) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: sequence } })] });
    push({ kind: "started", token: oldToken });
    push({ kind: "page", page: { ...oldToken, terminal: true, bytes: [1] } });
    push({ kind: "closed", token: oldToken, cancelled: false });
    await flush();
    expect(consumed).toBe(0);
    expect(ordinaryComplete).toBe(false);
    push({ kind: "started", token: newToken });
    push({ kind: "page", page: { ...newToken, terminal: true, bytes: [2] } });
    await flush();
    expect(consumed).toBe(1);
    expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: fixture.ackSequence, command: { kind: "acknowledge", token: newToken } } });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: fixture.ordinarySequence } })] });
    expect(await ordinary).toEqual([{ Done: { in_reply_to: fixture.ordinarySequence } }]);
    push({ kind: "closed", token: newToken, cancelled: false });
    expect(await secondResult).toEqual(newToken.identity);
    expect(sent.filter((command) => "LocalInteractionQuery" in command && command.LocalInteractionQuery.command.kind === "read").map((command) => "LocalInteractionQuery" in command ? command.LocalInteractionQuery.seq : 0)).toEqual(fixture.readSequences);
    second.dispose();
  });
  it("local interaction exhausted admission leaves no query slot and retains a cancellation sequence", async () => {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
      for (const command of bytes.map(decodeAppCommand)) {
        sent.push(command);
        broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: commandSeq(command) } })] });
      }
    }, outcomes: broadcast.stream };
    const exhausted = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 1), 7, "fixture");
    await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
    await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
    expect(sent).toHaveLength(0);
    await exhausted.readDocument();
    expect(sent).toEqual([{ ReadDocument: { seq: Number.MAX_SAFE_INTEGER } }]);
    exhausted.dispose();
    sent.length = 0;
    const last = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 2), 7, "fixture");
    const result = last.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
    const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "43", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "started", token } } }), encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "page", page: { ...token, terminal: true, bytes: [1] } } } })] });
    for (let index = 0; index < 16; index += 1) await Promise.resolve();
    expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: Number.MAX_SAFE_INTEGER, command: { kind: "cancel", token } } });
    broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "closed", token, cancelled: true } } })] });
    expect(await result).toBeInstanceOf(Error);
    last.dispose();
  });
}
