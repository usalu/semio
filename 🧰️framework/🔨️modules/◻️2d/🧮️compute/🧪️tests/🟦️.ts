import { expect, test } from "bun:test";
import Ajv from "ajv";
import { createRequire } from "node:module";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { engineKey, engineKeyPreimage } from "../🟦️.ts";
import { blake3Hex } from "../../../🔏️hash/🟦️.ts";

type Key = { id: string; engineId: string; inputHex: string; preimageHex: string; keyHex:string };
type Operation = { op: "register"; engineId: string; mode: string } | { op: "derive"; key: string; handle: string } | { op: "read"; handle: string } | { op: "read-key"; key: string } | { op: "read-forged"; handle: string; engineId: string };
type Result = { kind: string; key?: string; code?: string; outputHex?: string; entries: number; usedBytes: number; lru: string[] };
type Scenario = { id: string; budgetBytes: number; operations: Operation[]; expected: Result[] };
type Corpus = { retainedNativeLaws: string[]; keys: Key[]; invalidIdentities: { id: string; utf16Units: number[]; reason: string }[]; distinctTuples: { id: string; left: string; right: string }[]; representations: { id: string; values: number[]; expected: { total: number; len: number } }[]; scenarios: Scenario[] };
type LruReference = { length: number; itemCount: number; set(key: string, value: Uint8Array): void; get(key: string): Uint8Array | undefined; peek(key: string): Uint8Array | undefined; pop(): unknown; keys(): string[] };
const require = createRequire(import.meta.url);
const Lru = require("lru-cache") as new (options: { max: number; length(value: Uint8Array): number }) => LruReference;
const sum = require("lodash/sum") as (values: readonly number[]) => number;
const owner = resolve(import.meta.dir, "..");
const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as Corpus;
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
const hex = (bytes: Uint8Array): string => Buffer.from(bytes).toString("hex");
const keys = new Map(corpus.keys.map(key => [key.id, key]));
const keyHashes = new Map(corpus.keys.map(key => [key.id, blake3Hex(Buffer.from(key.preimageHex, "hex"))]));

function retain(id: string, value: unknown): void {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, "compute-schema-" + id + ".json"), JSON.stringify(value));
}

test("portable compute corpus is closed with unique identities and six retained native laws", () => {
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  for (const rows of [corpus.keys, corpus.invalidIdentities, corpus.scenarios, corpus.distinctTuples, corpus.representations]) expect(new Set(rows.map(row => row.id)).size).toBe(rows.length);
  expect(corpus.retainedNativeLaws).toContain("engine_rep_build_is_deterministic");
  expect(corpus.retainedNativeLaws).toHaveLength(6);
  expect(validate({ ...corpus, unknown: true })).toBe(false);
  for (const scenario of corpus.scenarios) expect(scenario.expected).toHaveLength(scenario.operations.length);
});

test("independent Node Buffer produces the exact canonical tuple fields", () => {
  const script = "const rows=" + JSON.stringify(corpus.keys) + ",invalid="+JSON.stringify(corpus.invalidIdentities)+";console.log(JSON.stringify({keys:rows.map(row=>{const id=Buffer.from(row.engineId,'utf8'),input=Buffer.from(row.inputHex,'hex'),a=Buffer.alloc(8),b=Buffer.alloc(8);a.writeBigUInt64LE(BigInt(id.length));b.writeBigUInt64LE(BigInt(input.length));return {id:row.id,preimageHex:Buffer.concat([a,id,b,input]).toString('hex')};}),invalidIdentities:invalid.map(row=>{const id=String.fromCharCode(...row.utf16Units);return {id:row.id,lossless:Buffer.from(id,'utf8').toString('utf8')===id};})}));";
  const child = spawnSync("node", ["--eval", script], { encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  const actual = JSON.parse(child.stdout);
  expect(actual).toEqual({keys:corpus.keys.map(row => ({ id: row.id, preimageHex: row.preimageHex })),invalidIdentities:corpus.invalidIdentities.map(row=>({id:row.id,lossless:false}))});
  retain("node-key-fields", actual);
});

for (const row of corpus.keys) test("key fields " + row.id, () => {
  const input = Buffer.from(row.inputHex, "hex"), preimage = engineKeyPreimage(row.engineId, input);
  expect(hex(preimage)).toBe(row.preimageHex);
  expect(hex(engineKey(row.engineId, input))).toBe(keyHashes.get(row.id)!);
  expect(keyHashes.get(row.id)).toBe(row.keyHex);
  expect(engineKey(row.engineId, input).length).toBe(32);
  retain("key-" + row.id, { preimageHex: hex(preimage), keyHex: hex(engineKey(row.engineId, input)) });
});

for(const row of corpus.invalidIdentities)test("non-scalar identity " + row.id,()=>{
  const identity=String.fromCharCode(...row.utf16Units);
  expect(Buffer.from(identity,"utf8").toString("utf8")).not.toBe(identity);
  expect(validate({...corpus,keys:corpus.keys.map((key,index)=>index===0?{...key,engineId:identity}:key)})).toBe(false);
  expect(()=>engineKeyPreimage(identity,new Uint8Array())).toThrow("engine identity must contain Unicode scalar values");
  expect(()=>engineKey(identity,new Uint8Array())).toThrow("engine identity must contain Unicode scalar values");
});

for (const row of corpus.distinctTuples) test("distinct semantic tuple " + row.id, () => {
  const left = keys.get(row.left)!, right = keys.get(row.right)!;
  expect(left.engineId === right.engineId && left.inputHex === right.inputHex).toBe(false);
  expect(left.preimageHex).not.toBe(right.preimageHex);
  expect(hex(engineKey(left.engineId, Buffer.from(left.inputHex, "hex")))).not.toBe(hex(engineKey(right.engineId, Buffer.from(right.inputHex, "hex"))));
});

for (const row of corpus.representations) test("independent snapshot sum " + row.id, () => {
  const build = (): { total: number; len: number } => ({ total: sum(row.values), len: row.values.length });
  expect(build()).toEqual(row.expected);
  expect(build()).toEqual(build());
  retain("representation-" + row.id, build());
});

for (const row of corpus.scenarios) test("independent weighted LRU " + row.id, () => {
  const cache = new Lru({ max: Number.MAX_SAFE_INTEGER, length: value => value.length });
  const engines = new Map<string, string>(), handles = new Map<string, string>(), labels = new Map<string, string>();
  const observe = (): Pick<Result, "entries" | "usedBytes" | "lru"> => ({ entries: cache.itemCount, usedBytes: cache.length, lru: cache.keys().reverse().map(key => labels.get(key)!) });
  const output: Result[] = [];
  for (const operation of row.operations) {
    if (operation.op === "register") {
      engines.set(operation.engineId, operation.mode);
      output.push({ kind: "registered", ...observe() });
      continue;
    }
    if (operation.op === "derive") {
      const tuple = keys.get(operation.key)!, key = keyHashes.get(operation.key)!;
      if (cache.get(key) === undefined) {
        const mode = engines.get(tuple.engineId);
        if (!mode || mode === "compute-fault" || mode === "invalid-input") {
          output.push({ kind: "fault", code: !mode ? "UnknownEngine" : mode === "compute-fault" ? "Compute" : "InvalidInput", ...observe() });
          continue;
        }
        const input = Buffer.from(tuple.inputHex, "hex"), value = mode === "double" ? Uint8Array.from([...input, ...input]) : mode === "empty" ? new Uint8Array() : input;
        while (cache.length + value.length > row.budgetBytes && cache.itemCount) cache.pop();
        cache.set(key, value);
        labels.set(key, operation.key);
      }
      handles.set(operation.handle, key);
      output.push({ kind: "derived", key: operation.key, ...observe() });
      continue;
    }
    const key = operation.op === "read" || operation.op === "read-forged" ? handles.get(operation.handle) : keyHashes.get(operation.key);
    const value = key === undefined ? undefined : cache.peek(key);
    output.push(value === undefined ? { kind: "fault", code: "Evicted", ...observe() } : { kind: "read", outputHex: hex(value), ...observe() });
  }
  expect(output).toEqual(row.expected);
  retain("lru-" + row.id, output);
  console.log("[compute-schema] " + row.id + ": " + output.length + " reference turns");
});
