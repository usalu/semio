/** 🧬️ General codec results agree with the defining closed schema and independent binary and relational oracles. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { ACTOR_CODEC_REFUSAL, actorCodecAnswer, actorComponentCodecSchema, type ActorCodecExports, type ShardCodecAnswer, type ShardCodecDocumentPair, type ShardCodecRequest } from "../../../📦️packages/🟦️typescript/🧬️codec/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import fixtureSchema from "./🧫️fixtures/🧬️.json";

type Pair = { readonly packHex: string; readonly sprHex: string };
type ClosedAnswer = { readonly ok: Uint8Array | ShardCodecDocumentPair | readonly [string, string] } | { readonly fault: Uint8Array };
type Equal<A, B> = (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? true : false;
const closedAnswer: Equal<ShardCodecAnswer, ClosedAnswer> = true;
const bytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../g) ?? [], pair => Number.parseInt(pair, 16));
const hex = (value: Uint8Array): string => Array.from(value, byte => byte.toString(16).padStart(2, "0")).join("");
const pair = (value: Pair): ShardCodecDocumentPair => ({ pack: bytes(value.packHex), spr: bytes(value.sprHex) });

/** 🧾️ Each typed export has its own fixture value; fault identity is preserved through the actual owner. */
test("Codec exports return only the three schema-defined successes or the original guest fault", async () => {
  expect(closedAnswer).toBe(true);
  const ajv = new Ajv({ strict: true }).addSchema(actorComponentCodecSchema);
  expect(ajv.validate(fixtureSchema, fixture), JSON.stringify(ajv.errors)).toBe(true);
  const answerSchema = ajv.compile({ $ref: actorComponentCodecSchema.$id + "#/$defs/ShardCodecAnswerProjection" });
  for (const invalid of fixture.invalidAnswers) expect(answerSchema(invalid)).toBe(false);
  expect(fixture.refusals).toEqual(ACTOR_CODEC_REFUSAL);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE codec_result(id TEXT PRIMARY KEY, expected TEXT NOT NULL, observed TEXT NOT NULL)");
    for (const row of fixture.cases) {
      const component = row.component, calls: string[] = [], pairs: ShardCodecDocumentPair[] = [], fault = bytes(component.faultHex ?? "");
      const owners = { "pack-schema-hash": bytes(component.bytesHex ?? ""), genesis: pair(component.pair ?? { packHex: "", sprHex: "" }), "print-mirror": [component.mirror?.[0] ?? "", component.mirror?.[1] ?? ""] as readonly [string, string] };
      const hostTrap = new Error(component.throws ?? "Unused host trap");
      const refuse = (): void => {
        if (component.throws) throw hostTrap;
        if (component.faultHex !== undefined) throw Object.assign(new Error("guest fault"), { payload: { tag: "fault", val: fault } });
      };
      const codec: ActorCodecExports | undefined = component.exported === false ? undefined : {
        packSchemaHash: async kind => { calls.push(kind); refuse(); return owners["pack-schema-hash"]; },
        genesis: async (kind, documentId) => { calls.push(kind, documentId); refuse(); return owners.genesis; },
        printMirror: async (kind, value) => { calls.push(kind); pairs.push(value); refuse(); return owners["print-mirror"]; },
      };
      const request = (row.request.pair ? { ...row.request, pair: pair(row.request.pair) } : row.request) as ShardCodecRequest;
      let answer: ShardCodecAnswer | undefined, observed: object = {};
      try { answer = await actorCodecAnswer(codec, request); }
      catch (error) { if (component.throws) expect(error).toBe(hostTrap); observed = { refusal: error instanceof Error ? error.message : String(error) }; }
      if (answer) {
        if ("ok" in answer) expect(answer.ok).toBe(owners[request.operation]);
        if ("fault" in answer) { expect(answer.fault).toBe(fault); observed = { faultHex: hex(answer.fault) }; }
        else if (answer.ok instanceof Uint8Array) observed = { okHex: hex(answer.ok) };
        else if ("pack" in answer.ok) observed = { okPair: { packHex: hex(answer.ok.pack), sprHex: hex(answer.ok.spr) } };
        else observed = { okMirror: [...answer.ok] };
        expect(answerSchema(observed), row.id).toBe(true);
      }
      for (const value of [component.bytesHex, component.faultHex, component.pair?.packHex, component.pair?.sprHex, row.request.pair?.packHex, row.request.pair?.sprHex]) {
        if (value !== undefined) { expect([...bytes(value)]).toEqual([...Buffer.from(value, "hex")]); expect(hex(bytes(value))).toBe(Buffer.from(value, "hex").toString("hex")); }
      }
      if (calls.length) expect(calls).toEqual(row.request.operation === "genesis" ? [row.request.artifactKind, row.request.documentId!] : [row.request.artifactKind]);
      if (row.request.pair) expect(pairs).toEqual([request.operation === "print-mirror" ? request.pair : undefined]);
      expect(observed, row.id).toEqual(row.expected);
      db.query("INSERT INTO codec_result VALUES(?,?,?)").run(row.id, JSON.stringify(row.expected), JSON.stringify(observed));
    }
    expect(db.query("SELECT count(*) AS mismatches FROM codec_result WHERE json(expected)<>json(observed)").get()).toEqual({ mismatches: 0 });
    expect(db.query("SELECT count(*) AS cases FROM codec_result").get()).toEqual({ cases: fixture.cases.length });
  } finally { db.close(); }
  console.log("[DEBUG] General codec public answer corpus agrees with Ajv, Node Buffer and SQLite", { cases: fixture.cases.length, invalidAnswers: fixture.invalidAnswers.length });
});
