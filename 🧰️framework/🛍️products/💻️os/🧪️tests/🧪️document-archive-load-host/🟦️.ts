/** 🗃️ Document archive load host law (TS twin): Ajv validates the shared corpus
 * (`🔨️modules/📡️spr/🧵️channel/🧫️fixtures/🧫️document-archive-load-host/🔣️.json`) against its schema, then every case is replayed twice —
 * through {@link DocumentArchiveLoadHost} step by step (the exact commands, operations and outcome the Rust driver is checked
 * against) and through `AppChannelClient.loadDocumentArchive` over a scripted channel, where a cancel is the caller's
 * `AbortSignal` and the outcome is how the promise settles. A load the guest admitted itself (`admittedByGuest`, a whole-document
 * `MediaIn`) replays through `DocumentArchiveLoadHost.admitted`; the client admits every load it drives, so it skips those rows.
 * A `merging` row (design §22.22) replays through `DocumentArchiveLoadHost.merging` and `AppChannelClient.mergeDocumentArchive`. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

import { createTurnOutcomeBroadcast, type TurnOutcome } from "@semio-tech/framework";
import { AppChannelClient, AppChannelRequestSequence, DocumentArchiveFaultError, DocumentArchiveLoadHost, decodeAppCommand, encodeAppFrame, encodePackValue, type AppChannelHandle, type AppCommandValue, type AppFrameValue, type DocumentArchiveLoadState, type DocumentArchiveLoadStatus, type DocumentArchivePack } from "../../🟦️.ts";

type Answer = { readonly kind: "done" } | { readonly kind: "error"; readonly fault: string } | { readonly kind: "status" | "foreignStatus"; readonly state: DocumentArchiveLoadState; readonly completed: number; readonly total: number; readonly ahead?: number; readonly fault?: string };
type Outcome = { readonly kind: "ready"; readonly ahead?: number } | { readonly kind: "cancelled" } | { readonly kind: "unanswered" } | { readonly kind: "fault"; readonly fault: string } | { readonly kind: "refused"; readonly fault: string };
type Case = { readonly name: string; readonly merging?: boolean; readonly admittedByGuest?: boolean; readonly cancel: { readonly beforeStep: number } | { readonly whileStepInFlight: number } | null; readonly exchanges: readonly { readonly sends: string; readonly answer: Answer }[]; readonly outcome: Outcome };
type Law = { readonly firstSequence: number; readonly cases: readonly Case[] };

const channel = new URL("../../🔨️modules/📡️spr/🧵️channel/", import.meta.url);
const law = JSON.parse(readFileSync(new URL("🧫️fixtures/🧫️document-archive-load-host/🔣️.json", channel), "utf8")) as Law;
const archive: DocumentArchivePack = { parent_pack: [7], parent_spr: [9], members: [] };
const bytes = (text: string | undefined): number[] => Array.from(new TextEncoder().encode(text ?? ""));
const text = (fault: readonly number[]): string => new TextDecoder().decode(new Uint8Array(fault));

/** 🏷️ The corpus name of one archive-load command, and the operation it names. */
function sent(command: AppCommandValue): readonly [string, number] {
  if ("LoadDocumentArchive" in command) return ["loadDocumentArchive", command.LoadDocumentArchive.seq];
  if ("MergeDocumentArchive" in command) return ["mergeDocumentArchive", command.MergeDocumentArchive.seq];
  if ("PollDocumentArchiveLoad" in command) return ["pollDocumentArchiveLoad", command.PollDocumentArchiveLoad.operation];
  if ("CancelDocumentArchiveLoad" in command) return ["cancelDocumentArchiveLoad", command.CancelDocumentArchiveLoad.operation];
  if ("AcknowledgeDocumentArchiveLoad" in command) return ["acknowledgeDocumentArchiveLoad", command.AcknowledgeDocumentArchiveLoad.operation];
  throw new Error(`no archive load sends ${JSON.stringify(command)}`);
}

/** 📬️ The scripted guest frame answering `seq` for an operation admitted under `operation`; `fault` encodes a status's fault. */
function frame(answer: Answer, seq: number, operation: number, fault: (text: string | undefined) => number[] = bytes): AppFrameValue {
  if (answer.kind === "done") return { Done: { in_reply_to: seq } };
  if (answer.kind === "error") return { Error: { in_reply_to: seq, fault: bytes(answer.fault), report: [] } };
  const status: DocumentArchiveLoadStatus = { operation: answer.kind === "foreignStatus" ? operation + 1_000 : operation, state: answer.state, completed: answer.completed, total: answer.total, ahead: answer.ahead ?? 0, fault: fault(answer.fault) };
  return { DocumentArchiveLoad: { in_reply_to: seq, status } };
}

const cancelBefore = (row: Case): number | undefined => (row.cancel && "beforeStep" in row.cancel ? row.cancel.beforeStep : undefined);
const cancelInFlight = (row: Case): number | undefined => (row.cancel && "whileStepInFlight" in row.cancel ? row.cancel.whileStepInFlight : undefined);
const statusesOf = (row: Case): readonly (readonly [string, number, number])[] => row.exchanges.flatMap(({ answer }) => (answer.kind === "status" ? [[answer.state, answer.completed, answer.total] as const] : []));



test("DocumentArchiveLoadHost sends the scripted commands and ends in the scripted outcome", () => {
  for (const row of law.cases) {
    const host = row.admittedByGuest ? DocumentArchiveLoadHost.admitted(law.firstSequence) : row.merging ? DocumentArchiveLoadHost.merging(archive) : new DocumentArchiveLoadHost(archive);
    let seq = law.firstSequence + (row.admittedByGuest ? 1 : 0);
    let stopped: Outcome | null = null;
    const statuses: (readonly [string, number, number])[] = [];
    row.exchanges.forEach((exchange, index) => {
      if (cancelBefore(row) === index) host.requestCancel();
      const step = host.step(() => seq);
      if (step.kind !== "send") throw new Error(`${row.name}: step ${index} finished early`);
      expect([row.name, step.seq]).toEqual([row.name, seq]);
      expect([row.name, ...sent(step.command), host.operation]).toEqual([row.name, exchange.sends, law.firstSequence, law.firstSequence]);
      if ("LoadDocumentArchive" in step.command) expect([row.name, row.merging ?? false, step.command.LoadDocumentArchive.archive]).toEqual([row.name, false, archive]);
      if ("MergeDocumentArchive" in step.command) expect([row.name, row.merging ?? false, step.command.MergeDocumentArchive.archive]).toEqual([row.name, true, archive]);
      if (cancelInFlight(row) === index) host.requestCancel();
      const answer = host.answer(seq, frame(exchange.answer, seq, law.firstSequence));
      if (answer.kind === "status" && answer.status) statuses.push([answer.status.state, answer.status.completed, answer.status.total]);
      if (answer.kind === "refused") stopped = { kind: "refused", fault: text(answer.fault) };
      if (answer.kind === "unanswered") stopped = { kind: "unanswered" };
      expect([row.name, stopped === null || index + 1 === row.exchanges.length]).toEqual([row.name, true]);
      seq += 1;
    });
    if (cancelBefore(row) === row.exchanges.length) host.requestCancel();
    const outcome: Outcome = stopped ?? ((): Outcome => {
      const step = host.step(() => { throw new Error(`${row.name}: a finished load mints no sequence`); });
      if (step.kind === "send") throw new Error(`${row.name}: the host still sends ${JSON.stringify(step.command)}`);
      if (step.outcome.kind === "fault") return { kind: "fault", fault: text(step.outcome.fault) };
      return step.outcome.kind === "ready" && host.ahead > 0 ? { kind: "ready", ahead: host.ahead } : { kind: step.outcome.kind };
    })();
    expect([row.name, outcome]).toEqual([row.name, row.outcome]);
    expect([row.name, statuses]).toEqual([row.name, statusesOf(row).map((status) => [...status])]);
  }
});

/** 🧯️ The guest's own pack-encoded fault naming `code`, as a status of a real program carries it. */
const packedFault = (code: string | undefined): number[] => Array.from(encodePackValue({ origin: "framework", code: code ?? "", severity: "error", message: `scripted ${code}`, retryable: false }));

/** 📡️ A client over a channel that answers every command with its scripted frame and aborts where the row cancels. */
function scripted(row: Case, fault: (text: string | undefined) => number[]): { readonly client: AppChannelClient; readonly controller: AbortController; readonly reason: Error; readonly seen: AppCommandValue[]; readonly operation: () => number } {
  const controller = new AbortController();
  const reason = new Error(`${row.name}: caller cancelled`);
  const seen: AppCommandValue[] = [];
  let operation = 0;
  const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
  const handle: AppChannelHandle = {
    enqueue: (instanceId, events) => {
      const command = events.map(decodeAppCommand)[0]!;
      const index = seen.length;
      seen.push(command);
      const exchange = row.exchanges[index];
      if (!exchange) throw new Error(`${row.name}: unscripted command ${JSON.stringify(command)}`);
      if ("LoadDocumentArchive" in command) operation = command.LoadDocumentArchive.seq;
      if ("MergeDocumentArchive" in command) operation = command.MergeDocumentArchive.seq;
      if (cancelBefore(row) === index + 1 || cancelInFlight(row) === index) controller.abort(reason);
      const seq = Object.values(command)[0]!.seq;
      broadcast.push({ instanceId, frames: [encodeAppFrame(frame(exchange.answer, Number(seq), operation, fault))] });
    },
    outcomes: broadcast.stream,
  };
  if (cancelBefore(row) === 0) controller.abort(reason);
  return { client: new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.archive-law"), controller, reason, seen, operation: () => operation };
}

test("AppChannelClient.loadDocumentArchive replays the corpus over a scripted channel", async () => {
  for (const row of law.cases.filter((row) => !row.admittedByGuest && !row.merging)) {
    const { client, controller, reason, seen, operation } = scripted(row, bytes);
    const statuses: (readonly [string, number, number])[] = [];
    const settled = await client.loadDocumentArchive(archive, row.cancel ? controller.signal : undefined, (status) => statuses.push([status.state, status.completed, status.total])).then(
      () => ({ kind: "ready" }) as const,
      (error: unknown) => ({ kind: "rejected", error }) as const,
    );
    expect([row.name, seen.map((command) => sent(command)[0])]).toEqual([row.name, row.exchanges.map((exchange) => exchange.sends)]);
    expect([row.name, seen.every((command) => sent(command)[1] === operation())]).toEqual([row.name, true]);
    expect([row.name, statuses]).toEqual([row.name, statusesOf(row).map((status) => [...status])]);
    const expected = row.outcome;
    if (expected.kind === "ready") {
      expect([row.name, settled.kind, client.documentPack()]).toEqual([row.name, "ready", { pack: new Uint8Array([7]), spr: new Uint8Array([9]) }]);
      continue;
    }
    if (settled.kind !== "rejected") throw new Error(`${row.name}: resolved, scripted ${expected.kind}`);
    expect([row.name, client.documentPack()]).toEqual([row.name, null]);
    if (expected.kind === "cancelled") {
      const error = settled.error;
      expect([row.name, row.cancel ? error === reason : error instanceof DOMException && error.name === "AbortError"]).toEqual([row.name, true]);
    } else {
      const message = settled.error instanceof Error ? settled.error.message : String(settled.error);
      expect([row.name, message.includes(expected.kind === "unanswered" ? "no answer" : expected.fault)]).toEqual([row.name, true]);
    }
  }
});

test("AppChannelClient.mergeDocumentArchive replays the merging rows: what it took and is ahead by, an unmergeable archive as its code, every other fault a rejection", async () => {
  const rows = law.cases.filter((row) => row.merging);
  expect(rows.map((row) => row.outcome.kind).sort()).toEqual(["cancelled", "fault", "ready"]);
  for (const row of rows) {
    const { client, controller, reason, seen, operation } = scripted(row, packedFault);
    const settled = await client.mergeDocumentArchive(archive, row.cancel ? controller.signal : undefined).then(
      (merge) => ({ kind: "resolved", merge }) as const,
      (error: unknown) => ({ kind: "rejected", error }) as const,
    );
    expect([row.name, seen.map((command) => sent(command)[0])]).toEqual([row.name, row.exchanges.map((exchange) => exchange.sends)]);
    expect([row.name, seen.every((command) => sent(command)[1] === operation())]).toEqual([row.name, true]);
    expect([row.name, client.documentPack()]).toEqual([row.name, null]);
    const expected = row.outcome;
    if (expected.kind === "ready") expect([row.name, settled]).toEqual([row.name, { kind: "resolved", merge: { kind: "merged", merged: statusesOf(row).at(-1)?.[1] ?? 0, ahead: expected.ahead ?? 0 } }]);
    else if (expected.kind === "fault") expect([row.name, settled]).toEqual([row.name, { kind: "resolved", merge: { kind: "unmergeable", code: expected.fault } }]);
    else expect([row.name, settled.kind === "rejected" && settled.error === reason]).toEqual([row.name, true]);
  }
  const refused = rows.find((row) => row.outcome.kind === "fault")!;
  const broken: Case = { ...refused, exchanges: refused.exchanges.map((exchange) => (exchange.answer.kind === "status" && exchange.answer.fault ? { ...exchange, answer: { ...exchange.answer, fault: "plugin.document-load.history-invalid" } } : exchange)) };
  const failure = await scripted(broken, packedFault).client.mergeDocumentArchive(archive).then(() => null, (error: unknown) => error);
  expect(failure instanceof DocumentArchiveFaultError ? [failure.fault?.code, failure.message.includes("plugin.document-load.history-invalid")] : failure).toEqual(["plugin.document-load.history-invalid", true]);
  const unread = await scripted(broken, bytes).client.loadDocumentArchive(archive).then(() => null, (error: unknown) => error);
  expect(unread instanceof DocumentArchiveFaultError ? [unread.fault, unread.message.includes("plugin.document-load.history-invalid")] : unread).toEqual([null, true]);
});

test("a merge that took events drops the cached root pair, and one that took none keeps it", async () => {
  for (const merged of [0, 2]) {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    let operation = 0;
    let merging = false;
    const handle: AppChannelHandle = {
      enqueue: (instanceId, events) => {
        const command = decodeAppCommand(events[0]!);
        const seq = Number(Object.values(command)[0]!.seq);
        if ("LoadDocumentArchive" in command || "MergeDocumentArchive" in command) {
          operation = seq;
          merging = "MergeDocumentArchive" in command;
        }
        const count = merging ? merged : 1;
        const answer: AppFrameValue = "PollDocumentArchiveLoad" in command ? { DocumentArchiveLoad: { in_reply_to: seq, status: { operation, state: "ready", completed: count, total: count, ahead: 0, fault: [] } } } : { Done: { in_reply_to: seq } };
        broadcast.push({ instanceId, frames: [encodeAppFrame(answer)] });
      },
      outcomes: broadcast.stream,
    };
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.archive-law");
    await client.loadDocumentArchive(archive);
    expect([merged, client.documentPack()]).toEqual([merged, { pack: new Uint8Array([7]), spr: new Uint8Array([9]) }]);
    expect([merged, await client.mergeDocumentArchive(archive)]).toEqual([merged, { kind: "merged", merged, ahead: 0 }]);
    expect([merged, client.documentPack()]).toEqual([merged, merged === 0 ? { pack: new Uint8Array([7]), spr: new Uint8Array([9]) } : null]);
  }
});
