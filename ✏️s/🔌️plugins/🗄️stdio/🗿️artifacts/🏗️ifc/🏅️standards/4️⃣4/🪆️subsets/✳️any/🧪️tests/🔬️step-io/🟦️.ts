/** 🧪️ Neutral IFC4 STEP boundary contracts and unchanged official buildingSMART samples. */
import {test} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {binary64} from "@semio-tech/framework/value/ieee754";
import {decodeIfc4, encodeIfc4} from "../../🚪️io/🟦️.ts";
import type {IfcSnapshot, IfcValue} from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {parsePart21, writePart21, type Part21Value} from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🟦️.ts";

const fixture = (name: string): URL => new URL("../🧫️fixtures/🏛️official/" + name, import.meta.url);
const contract = JSON.parse(readFileSync(fixture("🔣️.json"), "utf8"));
const frame = (schema: string, data: string): string => "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA" + schema + ";\nENDSEC;\nDATA;\n" + data + "\nENDSEC;\nEND-ISO-10303-21;\n";

function unchecked(text: string): IfcSnapshot {
  const document = parsePart21(text);
  const value = (item: Part21Value): IfcValue => {
    if (item.kind === "real") return {kind: "real", value: binary64(Number(item.value))};
    if (item.kind === "aggregate") return {kind: "aggregate", value: item.value.map(value)};
    if (item.kind === "typedValue") return {kind: "typedValue", value: {name: item.value.name, items: item.value.items.map(value)}};
    return {...item};
  };
  return {schema: "stdio.ifc", header: {fileDescription: document.header.fileDescription.map(value), fileName: document.header.fileName.map(value), fileSchema: document.header.fileSchema.map(value)}, entities: document.instances.map(instance => ({id: instance.id, name: instance.entities[0]!.name, args: instance.entities[0]!.args.map(value), complex: instance.entities.slice(1).map(record => ({name: record.name, args: record.args.map(value)}))}))};
}

export function checkOfficial(): void {
  for (const sample of contract.samples) {
    const bytes = readFileSync(fixture(sample.file));
    assert.equal(bytes.length, sample.bytes);
    assert.equal(createHash("sha256").update(bytes).digest("hex"), sample.sha256);
    const snapshot = decodeIfc4(bytes);
    assert.equal(snapshot.entities.length, sample.entityCount);
    const output = encodeIfc4(snapshot);
    assert.notDeepEqual(Buffer.from(output), bytes);
    assert.deepEqual(decodeIfc4(output), snapshot);
  }
}

export function checkBoundaries(): void {
  let exportCases = 0;
  for (const item of contract.boundaryCases) {
    const text = item.text ?? frame(item.schema, item.data);
    let actual = true;
    try {decodeIfc4(text);} catch {actual = false;}
    assert.equal(actual, item.valid, item.id + " import");
    let snapshot: IfcSnapshot;
    try {snapshot = unchecked(text);} catch {continue;}
    exportCases++;
    actual = true;
    try {encodeIfc4(snapshot);} catch {actual = false;}
    assert.equal(actual, item.exportValid ?? item.valid, item.id + " export");
  }
  assert.ok(exportCases >= 9);
}

export function checkScalars(): void {
  const text = frame("(('IFC4'))", "#18446744073709551615=(IFCTHING(-9223372036854775808,9223372036854775807,-0.,1.E+10,'O''Brien \\X2\\D83DDE00\\X0\\ \\X4\\0001F30D\\X0\\ \\S\\D',IFCLABEL((#18446744073709551615))) IFCOTHER($,*));");
  const snapshot = decodeIfc4(text);
  assert.equal(snapshot.entities[0]!.id, 18446744073709551615n);
  assert.equal(snapshot.entities[0]!.args[0]!.kind, "integer");
  assert.equal((snapshot.entities[0]!.args[0] as {value: bigint}).value, -9223372036854775808n);
  assert.equal((snapshot.entities[0]!.args[4] as {value: string}).value, "O'Brien 😀 🌍 Ä");
  assert.deepEqual(decodeIfc4(encodeIfc4(snapshot)), snapshot);
  const binary = frame("(('IFC4'))", '#1=THING("0FF");');
  assert.deepEqual(parsePart21(writePart21(parsePart21(binary))), parsePart21(binary));
  assert.deepEqual(decodeIfc4(encodeIfc4(decodeIfc4(binary))), decodeIfc4(binary));
  assert.throws(() => decodeIfc4(frame("(('IFC4'))", "#1=THING(1.E9999);")), /finite/);
  for (const literal of contract.binaryLiterals.valid) {
    const input = frame("(('IFC4'))", '#1=THING("' + literal + '");');
    assert.deepEqual(decodeIfc4(encodeIfc4(decodeIfc4(input))), decodeIfc4(input));
  }
  for (const literal of contract.binaryLiterals.invalid) assert.throws(() => decodeIfc4(frame("(('IFC4'))", '#1=THING("' + literal + '");')), /binary/);
}

export function checkLimits(): void {
  const text = frame("(('IFC4'))", "#1=IFCCARTESIANPOINT((0.,0.,0.));");
  assert.throws(() => decodeIfc4(text, {progress: () => false}), /cancelled/);
  assert.throws(() => decodeIfc4(text, {maxWork: 10}), /limit/);
  assert.throws(() => decodeIfc4(text, {maxInputLength: 10}), /length/);
  assert.throws(() => decodeIfc4(frame("(('IFC4'))", "#1=THING(" + "(".repeat(80) + "$" + ")".repeat(80) + ");")), /depth/);
  assert.throws(() => decodeIfc4(Uint8Array.from([255])), /encoded|encoding|UTF/i);
  assert.throws(() => encodeIfc4(decodeIfc4(text), {progress: () => false}), /cancelled/);
  for (const maxWork of [NaN, Infinity, -1]) {
    assert.throws(() => decodeIfc4(text, {maxWork}), /limit/);
    assert.throws(() => encodeIfc4(decodeIfc4(text), {maxWork}), /limit/);
  }
  const snapshot = decodeIfc4(text);
  snapshot.entities[0]!.args = [{kind: "real", value: binary64(Number.NaN)}];
  assert.throws(() => encodeIfc4(snapshot), /nonfinite/);
}


export function checkOptionalComplexAndProgress(): void {
  const text = readFileSync(fixture(contract.samples[0].file), "utf8");
  const original = decodeIfc4(text);
  const omitted = structuredClone(original);
  for (const entity of omitted.entities) delete entity.complex;
  assert.deepEqual(decodeIfc4(encodeIfc4(omitted)), original);
  let parserWork = 0;
  parsePart21(text, {progress: work => {parserWork = work; return true;}});
  const observe = () => {
    let previous = -1;
    return {progress: (work: number, budget: number) => {assert.ok(work >= previous, "operation progress is monotonic"); assert.ok(work <= budget); previous = work; return true;}, total: () => previous};
  };
  const decoding = observe(); decodeIfc4(text, {progress: decoding.progress});
  assert.ok(decoding.total() > parserWork, "operation work includes parse, validation and conversion");
  assert.throws(() => decodeIfc4(text, {maxWork: parserWork + 1}), /limit/);
  const encoding = observe(); encodeIfc4(original, {progress: encoding.progress});
  assert.throws(() => encodeIfc4(original, {maxWork: encoding.total() - 1}), /limit/);
}

export const officialChecks = test("official IFC4 samples retain every snapshot value through STEP", checkOfficial);
export const boundaryChecks = test("neutral IFC4 framing/header/identity/reference contract matches both boundaries", checkBoundaries);
export const scalarChecks = test("STEP values retain exact integers complex records Unicode typed aggregates and negative zero", checkScalars);
export const limitChecks = test("STEP parse/write work depth UTF-8 and cancellation limits refuse explicitly", checkLimits);
export const optionalComplexAndProgressChecks = test("canonical omitted complex records and operation-wide budgets", checkOptionalComplexAndProgress);
