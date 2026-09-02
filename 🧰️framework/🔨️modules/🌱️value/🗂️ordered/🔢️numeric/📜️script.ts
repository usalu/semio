#!/usr/bin/env bun
//#region 🧪️NumericIndexTests
import { strict as assert } from "node:assert";
import { runInNewContext } from "node:vm";
import Ajv from "ajv";
import { enableMapSet, produce } from "immer";
import ts from "typescript";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "@semio-tech/repo-lib";
import { NumericIndex, type NumericIndexEdit, type NumericIndexRetirement } from "./🟦️.ts";
import fixture from "./🧪️fixtures/🔣️numeric-index.json";
import schema from "./🧪️fixtures/🔣️numeric-index.schema.json";
import referenceFixture from "./🧪️fixtures/🔣️references.json";
import referenceSchema from "./🧪️fixtures/🔣️references.schema.json";

function close<V>(owner: NumericIndexRetirement<V>, bytes: number, retired: V[]): void {
  for (let turns = 0; turns < 100_000; turns++) {
    const step = owner.advance({ maxItems: 1, maxBytes: bytes });
    assert(step.items <= 1 && step.bytes <= bytes);
    if (step.kind === "retired") retired.push(step.value);
    if (step.kind === "complete") { assert(owner.terminalIsEmpty()); return; }
    assert.notEqual(step.kind, "blocked");
  }
  assert.fail("Retirement failed to terminate");
}

function apply<V>(cursor: NumericIndexEdit<V>, bytes: number, retired: V[]): NumericIndex<V> {
  for (let turns = 0; turns < 100_000; turns++) {
    const step = cursor.advance({ maxItems: 1, maxBytes: bytes });
    assert(step.items <= 1 && step.bytes <= bytes);
    if (step.kind === "retired") retired.push(step.value);
    if (step.kind === "ready") {
      const result = cursor.takeResult();
      assert(result);
      close(cursor.beginClose(), bytes, retired);
      assert(cursor.terminalIsEmpty());
      return result;
    }
    assert.notEqual(step.kind, "blocked");
  }
  return assert.fail("Edit failed to terminate");
}

function lifecycleLaws(): number {
  type Payload = { readonly text: string };
  const initial = Array.from({ length: 15 }, (_, id) => ({ id, value: { text: String(id) } }));
  let base = NumericIndex.empty<Payload>();
  const retired: Payload[] = [];
  for (const { id, value } of initial) {
    const next = apply(base.beginSet(id, value), 256, retired);
    close(base.beginClose(), 256, retired);
    base = next;
  }
  const expected = [...base];
  let laws = 0;
  for (const mode of ["set", "remove"] as const) {
    for (let stop = 0; stop < 300; stop++) {
      const payload = { text: fixture.lifecycle.payload.unit.repeat(fixture.lifecycle.payload.repeat) };
      const edit = mode === "set" ? base.beginSet(7, payload) : base.beginRemove(7);
      for (const grant of fixture.lifecycle.invalidGrants) assert.equal(edit.advance(grant).kind, "blocked");
      let ready = false;
      for (let n = 0; n < stop; n++) {
        const step = edit.advance({ maxItems: 1, maxBytes: 256 });
        assert(step.items <= 1 && step.bytes <= 256);
        if (step.kind === "retired") retired.push(step.value);
        if (step.kind === "ready") { ready = true; break; }
      }
      if (ready) {
        close(edit.beginClose(), 256, retired);
        break;
      }
      close(edit.beginClose(), 256, retired);
      assert.deepEqual([...base], expected);
      if (mode === "set") assert.equal(retired.filter((value) => value === payload).length, 1);
      laws++;
    }
  }
  const reader = base.beginRead();
  const lookup = base.beginLookup(7);
  const retained = base.capture();
  const winnerValue = { text: "winner" };
  const loserValue = { text: "loser" };
  const winner = base.beginSet(7, winnerValue);
  const loser = base.beginSet(7, loserValue);
  close(base.beginClose(), 256, retired);
  for (let n = 0; n < 12; n++) {
    const step = loser.advance({ maxItems: 1, maxBytes: 256 });
    if (step.kind === "retired") retired.push(step.value);
  }
  const result = apply(winner, 256, retired);
  close(loser.beginClose(), 256, retired);
  assert.equal(result.get(7), winnerValue);
  assert.deepEqual([...retained], expected);
  close(retained.beginClose(), 256, retired);
  close(result.beginClose(), 256, retired);
  assert.equal(retired.filter((value) => value === loserValue).length, 1);
  const found: (readonly [number, Payload])[] = [];
  for (let n = 0; n < 200; n++) {
    const step = reader.advance({ maxItems: 1, maxBytes: 256 });
    assert(step.items <= 1 && step.bytes <= 256);
    if (step.kind === "value") found.push([step.id, step.value]);
    if (step.kind === "complete") break;
  }
  assert.deepEqual(found, expected);
  assert(initial.every(({ value }) => !retired.includes(value)));
  let lookupValue: Payload | undefined;
  for (let n = 0; n < 200; n++) {
    const step = lookup.advance({ maxItems: 1, maxBytes: 256 });
    if (step.kind === "value") lookupValue = step.value;
    if (step.kind === "complete") break;
  }
  assert.equal(lookupValue, initial[7]!.value);
  close(reader.beginClose(), 256, retired);
  assert(initial.every(({ value }) => !retired.includes(value)));
  close(lookup.beginClose(), 256, retired);
  assert(initial.every(({ value }) => retired.filter((item) => item === value).length === 1));
  assert.equal(retired.filter((value) => value === winnerValue).length, 1);
  return laws + 1;
}

function ordinalLaws(): number {
  for (const vector of fixture.ordinals) {
    const seed = { ...vector.start };
    const index = NumericIndex.empty<string>(seed);
    seed.low = 0;
    const edit = index.beginSet(Number.MAX_SAFE_INTEGER, vector.name);
    const retired: string[] = [];
    if (vector.outcome === "ready") {
      const next = apply(edit, 256, retired);
      assert.deepEqual(next.nextOrdinal(), vector.next);
      assert.deepEqual([...next], [[Number.MAX_SAFE_INTEGER, vector.name]]);
      close(next.beginClose(), 256, retired);
    } else {
      let rejected = false;
      for (let n = 0; n < 10; n++) {
        const step = edit.advance({ maxItems: 1, maxBytes: 256 });
        if (step.kind === "rejected") { assert.equal(step.reason, vector.outcome); rejected = true; break; }
      }
      assert(rejected);
      assert.equal(edit.takeResult(), null);
      close(edit.beginClose(), 256, retired);
    }
    assert.deepEqual(index.nextOrdinal(), vector.start);
    assert.equal(index.size, 0);
    close(index.beginClose(), 256, retired);
    assert.deepEqual(retired, [vector.name]);
  }
  return fixture.ordinals.length;
}

function stressLaws(): number {
  let index = NumericIndex.empty<string>();
  let oracle = new Map<number, string>();
  const retired: string[] = [];
  let sets = 0;
  for (let round = 0; round < fixture.stress.rounds; round++) for (let n = 0; n < fixture.stress.size; n++) {
    const id = (n * fixture.stress.multiplier) % fixture.stress.size;
    const value = `${round}:${id}`;
    const remove = round === 1;
    const next = apply(remove ? index.beginRemove(id) : index.beginSet(id, value), 256, retired);
    oracle = produce(oracle, (draft) => { if (remove) draft.delete(id); else draft.set(id, value); });
    sets += Number(!remove);
    assert.deepEqual([...next], [...oracle]);
    assert.equal(next.size, oracle.size);
    assert.equal(next.get(id), oracle.get(id));
    close(index.beginClose(), 256, retired);
    index = next;
  }
  close(index.beginClose(), 256, retired);
  assert.equal(retired.length, sets);
  assert.equal(new Set(retired).size, sets);
  return fixture.stress.rounds * fixture.stress.size;
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    enableMapSet();
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert(!validate({ ...fixture, extra: true }));
    let laws = 0;
    for (const bytes of fixture.grants) for (const vector of fixture.cases) {
      let index = NumericIndex.empty<string>();
      let oracle = new Map<number, string>();
      const retired: string[] = [];
      for (const operation of vector.operations) {
        const captured = index.capture();
        const before = [...oracle];
        const edit = operation.op === "set" ? index.beginSet(operation.id, operation.value!) : index.beginRemove(operation.id);
        assert.equal(edit.advance({ maxItems: 0, maxBytes: bytes }).kind, "blocked");
        const next = apply(edit, bytes, retired);
        oracle = produce(oracle, (draft) => { if (operation.op === "set") draft.set(operation.id, operation.value!); else draft.delete(operation.id); });
        assert.deepEqual([...captured], before);
        assert.deepEqual([...next], [...oracle]);
        close(index.beginClose(), bytes, retired);
        close(captured.beginClose(), bytes, retired);
        index = next;
      }
      assert.deepEqual([...index], vector.expected);
      const sortedReader = index.beginSortedRead();
      const sorted: (readonly [number, string])[] = [];
      for (let n = 0; n < 1000; n++) {
        const step = sortedReader.advance({ maxItems: 1, maxBytes: bytes });
        if (step.kind === "value") { sorted.push([step.id, step.value]); assert.equal(step.ordinal.high, 0); }
        if (step.kind === "complete") break;
      }
      assert.deepEqual(sorted, [...oracle].sort((left, right) => left[0] - right[0]));
      close(sortedReader.beginClose(), bytes, retired);
      if (vector.name === "negative-zero-normalizes-like-map") {
        assert(Object.is(vector.operations[0]!.id, -0));
        for (const reader of [index.beginRead(), index.beginLookup(-0), index.beginLookup(0)]) {
          let found = false;
          for (let n = 0; n < 20; n++) {
            const step = reader.advance({ maxItems: 1, maxBytes: bytes });
            if (step.kind === "value") { assert(Object.is(step.id, 0)); assert.equal(step.value, "replacement"); found = true; }
            if (step.kind === "complete") break;
          }
          assert(found);
          close(reader.beginClose(), bytes, retired);
        }
      }
      close(index.beginClose(), bytes, retired);
      assert.equal(retired.length, vector.operations.filter((operation) => operation.op === "set").length);
      laws++;
    }
    for (const id of [...fixture.invalidIds, NaN, Infinity]) {
      const index = NumericIndex.empty<string>();
      assert.throws(() => index.beginSet(id, "invalid"));
      close(index.beginClose(), 256, []);
    }
    const lifecycle = lifecycleLaws();
    const ordinals = ordinalLaws();
    const stress = stressLaws();
    assert(new Ajv({ strict: true, allErrors: true }).compile(referenceSchema)(referenceFixture));
    const source = await Bun.file(`${import.meta.dir}/🟦️.ts`).text();
    const probe = ts.transpileModule(`${source}\nnumericReferenceSaturation();`, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
    const references: unknown = JSON.parse(JSON.stringify(runInNewContext(probe, { exports: {} })));
    assert(Array.isArray(references));
    assert.deepEqual(references, referenceFixture.cases.map(name => ({ name, ...referenceFixture.expected })));
    const program = ts.createProgram([`${import.meta.dir}/🟦️.ts`], { strict: true, noEmit: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: [], lib: ["lib.es2022.d.ts"] });
    const diagnostics = ts.getPreEmitDiagnostics(program);
    assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCanonicalFileName: (name) => name, getCurrentDirectory: () => import.meta.dir, getNewLine: () => "\n" }));
    console.log(`[DEBUG] Numeric-index laws=${laws} lifecycle=${lifecycle} ordinals=${ordinals} stress=${stress} references=${references.length} invalidIds=5 oracle=Immer+Map grants=256,4096 strictTS=0`);
  }
}
//#endregion 🧪️NumericIndexTests

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
