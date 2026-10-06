/** 🧾️ Document-port control turn law (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, live faults F4 and B1's unhandled
 * `receipt-count`). A control turn may carry the program's own frames beside its receipt:
 * {@link splitDocumentBackboneControlTurnV1} keeps exactly the frames that name the receipt schema, in order, and hands every
 * other frame on; a turn owes exactly one receipt and says how many it carried, and a damaged receipt fails under its own
 * code. Binding and releasing are total: Ajv validates the corpus (`🧫️fixtures/🔣️.json`), every case is replayed through a
 * host that binds as the shells do ({@link releaseActorDocumentBindingV1}, {@link bindActorDocumentV1}), a failed bind
 * settles as ITSELF whatever its retirement answers, a binding the program did not let go of is released before the next
 * bind, and fast-check holds over arbitrary answers and steps that every step settles and no rejection is left unhandled.
 * The control maps' wire bytes are the binding fixture's `codec.control` rows, which the Rust unit law pins too. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import fc from "fast-check";
import { ActorDocumentBindingV1, DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1, DOCUMENT_BACKBONE_STALE_GENERATION_CODE, bindActorDocumentV1, decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1, releaseActorDocumentBindingV1, soleDocumentBackboneReceiptV1, splitDocumentBackboneControlTurnV1, type DocumentBackboneControlV1 } from "../../🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";

type Answer = "receipt" | "stale" | "refused" | "absent" | "twice" | "damaged" | "transport";
type Step = "bind" | "retire";
type Case = { readonly id: string; readonly answers: readonly Answer[]; readonly steps: readonly Step[]; readonly outcomes: readonly string[]; readonly controls: readonly string[]; readonly told: readonly string[] };

const fixtures = new URL("🧫️fixtures/", import.meta.url);
const corpus = JSON.parse(readFileSync(new URL("🔣️.json", fixtures), "utf8")) as { readonly cases: readonly Case[] };
const owner = { runtimeKey: "board.ports.directed.v1", clientInstanceId: "client-a", scope: null, actorId: "actor-1", activationGeneration: 1n, instanceId: 4 } as const;
const limits = { messageBytes: 1_024, pendingBytes: 4_096, pendingMessages: 8 };
const receiptOf = (command: DocumentBackboneControlV1): Uint8Array => encodeDocumentBackboneControlV1({ ...command, schema: DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1, operation: command.operation === "bind" ? "bound" : "retired" });
const refusalOf = (command: DocumentBackboneControlV1, code: string): Uint8Array => encodeDocumentBackboneControlV1({ ...command, schema: DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1, operation: "refused", code });
const statusFrame = Uint8Array.of(15, 0, 7, 1, 2, 3);
const refusalText = new TextEncoder().encode("patch-capacity-refused:surface:7");
const message = (error: unknown): string => (error instanceof Error ? error.message : String(error));

/** 📬️ The frames one control turn of the program carries for a scripted answer. */
function turn(answer: Answer, command: DocumentBackboneControlV1): readonly Uint8Array[] {
  if (answer === "transport") throw new Error("transport lost");
  if (answer === "absent") return [statusFrame];
  if (answer === "twice") return [receiptOf(command), statusFrame, receiptOf(command)];
  if (answer === "damaged") return [Uint8Array.of(...receiptOf(command), 0)];
  if (answer === "stale") return [refusalOf(command, DOCUMENT_BACKBONE_STALE_GENERATION_CODE)];
  if (answer === "refused") return [refusalOf(command, "plugin.document-backbone.binding-live")];
  return [statusFrame, receiptOf(command)];
}

/** 🖥️ A host that binds one program's document port as the shells do, over a program answering from `answers`. */
function host(answers: readonly Answer[], exhausted: Answer | null) {
  const controls: string[] = [];
  const told: string[] = [];
  const routed: Uint8Array[][] = [];
  let generation = 0n;
  let current: ActorDocumentBindingV1 | undefined;
  const exchange = async (command: DocumentBackboneControlV1): Promise<readonly Uint8Array[]> => {
    const answer = answers[controls.length] ?? exhausted;
    controls.push(`${command.operation}:${command.bindingGeneration}`);
    if (answer === null) throw new Error(`the host sent a control turn (${controls.at(-1)}) the case scripts no answer for`);
    const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(turn(answer, command));
    if (unsolicited.length > 0) routed.push([...unsolicited]);
    return receipts;
  };
  const bind = async (): Promise<void> => {
    const previous = current;
    if (previous && !previous.port.closing) throw new Error("actor-document-control.binding-live");
    if (previous) await releaseActorDocumentBindingV1(previous);
    generation += 1n;
    const binding = new ActorDocumentBindingV1(owner, generation, { current: () => true, assertActive: () => {}, limits, send: () => {}, deliver: async () => {}, exchange });
    current = binding;
    await bindActorDocumentV1(binding, undefined, (retirement) => {
      if (retirement !== null) told.push(message(retirement.error));
      else if (current === binding) current = undefined;
    });
  };
  const step = (kind: Step): Promise<void> => (kind === "bind" ? bind() : current ? current.port.retire() : Promise.resolve());
  return { controls, told, routed, step, sends: (): boolean => current?.port.send(current.port.uri, Uint8Array.of(1)) ?? false };
}

test("the corpus is valid and hostile rows are refused", () => {
  const first = corpus.cases[0]!;
});

test("every corpus case settles each step as scripted, sends exactly its control turns and tells exactly its retirement failures", async () => {
  for (const row of corpus.cases) {
    const run = host(row.answers, null);
    const outcomes: string[] = [];
    for (const kind of row.steps) outcomes.push(await run.step(kind).then(() => "ok", message));
    expect([row.id, outcomes]).toEqual([row.id, [...row.outcomes]]);
    expect([row.id, run.controls]).toEqual([row.id, [...row.controls]]);
    expect([row.id, run.told]).toEqual([row.id, [...row.told]]);
    expect([row.id, run.sends()]).toEqual([row.id, row.outcomes.at(-1) === "ok" && row.steps.at(-1) === "bind"]);
  }
});

test("a control turn that carries the program's own frames binds and retires, and those frames are routed", async () => {
  const run = host(["receipt", "receipt"], null);
  await run.step("bind");
  expect(run.sends()).toBe(true);
  await run.step("retire");
  expect(run.sends()).toBe(false);
  expect(run.routed).toEqual([[statusFrame], [statusFrame]]);
});

test("a turn owes exactly one receipt and says how many it carried", () => {
  const command: DocumentBackboneControlV1 = { schema: "semio.plugin.document-backbone-binding.v1", operation: "bind", instanceId: 4, bindingGeneration: 2n, uri: "actor://board.ports.directed.v1" };
  const receipt = receiptOf(command);
  expect(soleDocumentBackboneReceiptV1([receipt])).toBe(receipt);
  expect(() => soleDocumentBackboneReceiptV1([])).toThrow("actor-document-control.receipt-count:0");
  expect(() => soleDocumentBackboneReceiptV1([receipt, receipt, receipt])).toThrow("actor-document-control.receipt-count:3");
});

test("the split keeps every frame once, in order: a frame is a receipt exactly when it names the receipt schema", () => {
  const command: DocumentBackboneControlV1 = { schema: "semio.plugin.document-backbone-binding.v1", operation: "bind", instanceId: 4, bindingGeneration: 2n, uri: "actor://board.ports.directed.v1" };
  const receipt = receiptOf(command);
  const damaged = Uint8Array.of(9, ...receipt, 0);
  const named = (bytes: Uint8Array): boolean => bytes === receipt || bytes === damaged;
  const frame = fc.oneof(fc.constant(receipt), fc.constant(damaged), fc.constant(encodeDocumentBackboneControlV1(command)), fc.constant(refusalText), fc.uint8Array({ minLength: 1, maxLength: 40 }));
  fc.assert(
    fc.property(fc.array(frame, { maxLength: 12 }), (frames) => {
      const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(frames);
      expect(receipts).toEqual(frames.filter(named));
      expect(unsolicited).toEqual(frames.filter((bytes) => !named(bytes)));
    }),
    { numRuns: 200 },
  );
});

test("over arbitrary answers and steps every step settles, a failed bind tells its own failure, and no rejection is left unhandled", async () => {
  const unhandled: unknown[] = [];
  const listen = (reason: unknown): void => { unhandled.push(reason); };
  process.on("unhandledRejection", listen);
  try {
    const answer = fc.constantFrom<Answer>("receipt", "stale", "refused", "absent", "twice", "damaged", "transport");
    await fc.assert(
      fc.asyncProperty(fc.array(answer, { maxLength: 12 }), answer, fc.array(fc.constantFrom<Step>("bind", "retire"), { minLength: 1, maxLength: 8 }), async (answers, exhausted, steps) => {
        const run = host(answers, exhausted);
        for (const kind of steps) {
          const before = run.controls.length;
          const outcome = await run.step(kind).then(() => "ok", message);
          const sent = run.controls.slice(before);
          const bound = sent.findIndex((control) => control.startsWith("bind:"));
          if (kind !== "bind" || bound < 0 || outcome === "ok") continue;
          const own = [...(answers[before + bound] === undefined ? [exhausted] : [answers[before + bound]!])][0]!;
          const expected = own === "transport" ? "transport lost" : own === "absent" ? "actor-document-control.receipt-count:0" : own === "twice" ? "actor-document-control.receipt-count:2" : own === "damaged" ? "actor-document-control.terminal" : own === "refused" ? "plugin.document-backbone.binding-live" : own === "stale" ? DOCUMENT_BACKBONE_STALE_GENERATION_CODE : "ok";
          expect([steps, answers, outcome]).toEqual([steps, answers, expected]);
        }
      }),
      { numRuns: 300 },
    );
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(unhandled.map(message)).toEqual([]);
  } finally {
    process.off("unhandledRejection", listen);
  }
});

test("the control maps are the golden wire bytes the Rust law pins too", () => {
  type Row = { readonly id: string; readonly value: Omit<DocumentBackboneControlV1, "bindingGeneration"> & { readonly bindingGeneration: string }; readonly hex: string };
  const binding = JSON.parse(readFileSync(new URL("../../🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as { readonly codec: { readonly control: readonly Row[] } };
  const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  const rows = binding.codec.control;
  expect(rows.map((row) => `${row.value.schema.endsWith("receipt.v1") ? "receipt" : "command"}:${row.value.operation}`).sort()).toEqual(expect.arrayContaining(["command:bind", "command:retire", "receipt:bound", "receipt:refused", "receipt:retired"]));
  for (const row of rows) {
    const value: DocumentBackboneControlV1 = { ...row.value, bindingGeneration: BigInt(row.value.bindingGeneration) };
    const golden = Uint8Array.from(row.hex.match(/../g) ?? [], (pair) => Number.parseInt(pair, 16));
    expect([row.id, hex(encodeDocumentBackboneControlV1(value))]).toEqual([row.id, row.hex]);
    expect([row.id, decodeDocumentBackboneControlV1(golden)]).toEqual([row.id, value]);
    expect([row.id, splitDocumentBackboneControlTurnV1([golden]).receipts.length]).toEqual([row.id, value.schema === DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1 ? 1 : 0]);
  }
});

test("a receipt whose members stand in struct order names the receipt schema and fails as noncanonical (live fault F9)", async () => {
  const command: DocumentBackboneControlV1 = { schema: "semio.plugin.document-backbone-binding.v1", operation: "bind", instanceId: 7, bindingGeneration: 3n, uri: "actor://v1:7:3:space-amap" };
  const canonical = receiptOf(command);
  const head = [0x01, 0x01, 0x11, 0x10, 0x05];
  const start = canonical.findIndex((_, index) => head.every((byte, offset) => canonical[index + offset] === byte)) + head.length;
  expect(start).toBeGreaterThan(head.length);
  const members = new Map<string, Uint8Array>();
  for (let position = start; position < canonical.length; ) {
    const from = position;
    expect(canonical[position]).toBe(0x07);
    const length = canonical[position + 1]!;
    const key = new TextDecoder().decode(canonical.subarray(position + 2, position + 2 + length));
    position += 2 + length + 2;
    members.set(key, canonical.subarray(from, position));
  }
  expect([...members.keys()]).toEqual(["bindingGeneration", "instanceId", "operation", "schema", "uri"]);
  const authored = Uint8Array.from([...canonical.subarray(0, start), ...["schema", "operation", "instanceId", "bindingGeneration", "uri"].flatMap((key) => Array.from(members.get(key)!))]);
  expect(authored.length).toBe(canonical.length);
  expect(() => decodeDocumentBackboneControlV1(authored)).toThrow("actor-document-control.noncanonical");
  expect(splitDocumentBackboneControlTurnV1([statusFrame, authored])).toEqual({ receipts: [authored], unsolicited: [statusFrame] });
  const run = new ActorDocumentBindingV1(owner, 3n, { current: () => true, assertActive: () => {}, limits, send: () => {}, deliver: async () => {}, exchange: async () => splitDocumentBackboneControlTurnV1([authored]).receipts });
  await expect(run.bind()).rejects.toThrow("actor-document-control.noncanonical");
});
