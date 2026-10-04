import { describe, expect, test } from "bun:test";
import { applyPatch, type Operation } from "fast-json-patch";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { createHash } from "node:crypto";
import { snapshotEditSource, snapshotFromEditSource, applySnapshotEdit, SnapshotEditError, type SnapshotEditCodec, type SnapshotEditEvent, type SnapshotValue } from "../../🟦️";

const retainedNativeFixture = await Bun.file(new URL("../../🧫️fixtures/🧵️retained-native/🔣️.json", import.meta.url)).json() as {
  routeCases: { id: string; mutation: number; recognized: boolean; primaryAccepts: boolean; expected: "primary" | "fallback" | "refused" }[];
  lifecycle: { units: number; cancelAfter: number; expectedProgress: number[]; expectedCloseSteps: number };
  textCopy: { byteLength: number; pageBytes: number; expectedDataSteps: number; cancelAfterBytes: number; expectedCloseSteps: number; interruptedUnicode: string; partialByteStops: number[] };
  commandAdmission: { maximumBytes: number; cases: { id: string; value: string; accepted: boolean }[] };
  structuralCopy: { runCount: number; tableRowCount: number; meshVertexCount: number; brepVertexCount: number; nurbsPointCount: number; textureByteLength: number; pageBytes: number; cancelAfterTurns: number[] };
  documentCopy: { siblingByteLength: number; replacementText: string; replacementRepeats: number; pageBytes: number; cancelAfterTurns: number; minimumCompleteTurns: number };
};
const retainedNativeSchema = await Bun.file(new URL("../../🧫️fixtures/🧵️retained-native/🧬️schema/🔣️.json", import.meta.url)).json();
const sourceDiagnosticFixture = await Bun.file(new URL("../../🧫️fixtures/🩺️source-diagnostic/🔣️.json", import.meta.url)).json() as {
  cases: { id: string; source: string; expected: { code: string; line: number; column: number; length: number } }[];
};
const sourceDiagnosticSchema = await Bun.file(new URL("../../🧫️fixtures/🩺️source-diagnostic/🧬️schema/🔣️.json", import.meta.url)).json();

const fixture = await Bun.file(new URL("../../🧫️fixtures/🪆️snapshot-edits/🔣️patch-cases.json", import.meta.url)).json() as {
  base: SnapshotValue;
  sourceRoundTrip: SnapshotValue;
  wideIntegerSource: string;
  floatInputs: { source: string; accepted: boolean }[];
  rejectedSources: {id: string; source: string; code: string}[];
  accepted: { id: string; event: SnapshotEditEvent; patch: { op: string; path: string; from?: string; value?: SnapshotValue }[] }[];
  rejected: { id: string; event: SnapshotEditEvent; code: string }[];
};

function oracle(base: SnapshotValue, operations: typeof fixture.accepted[number]["patch"]): SnapshotValue {
  const document = structuredClone(base);
  const dictionaries = (value: SnapshotValue): void => {
    if (value === null || typeof value !== "object") return;
    for (const child of Object.values(value)) dictionaries(child);
    if (!Array.isArray(value)) Object.setPrototypeOf(value, null);
  };
  dictionaries(document);
  return JSON.parse(JSON.stringify(applyPatch(document, operations as Operation[], true, true, false).newDocument)) as SnapshotValue;
}

const keys = ["schema", "title", "active", "count", "ratio", "optional", "choice", "labels", "items"];
const codec: SnapshotEditCodec<SnapshotValue> = {
  validate: (value) => {
    const fail = (): never => { throw new SnapshotEditError("snapshot-edit.schema-invalid", "", "fixture schema rejected edit"); };
    if (value === null || Array.isArray(value) || typeof value !== "object" || Object.keys(value).some((key) => !keys.includes(key))) fail();
    if (typeof value.schema !== "string" || typeof value.title !== "string" || typeof value.active !== "boolean" || !Number.isInteger(value.count) || typeof value.ratio !== "number") fail();
    return value;
  },
};

describe("snapshot edit fixture", () => {
  for (const row of fixture.accepted) test(row.id, () => expect(applySnapshotEdit(fixture.base, row.event, codec)).toEqual(oracle(fixture.base, row.patch)));
  for (const row of fixture.rejected) test(row.id, () => {
    const before = structuredClone(fixture.base);
    try { applySnapshotEdit(fixture.base, row.event, codec); throw new Error("edit unexpectedly accepted"); }
    catch (error) { expect(error).toBeInstanceOf(SnapshotEditError); expect((error as SnapshotEditError).code).toBe(row.code); }
    expect(fixture.base).toEqual(before);
  });
});

test("source diagnostic fixture is schema-valid and every source is independently malformed", () => {
  expect(new Ajv2020({ strict: true }).compile(sourceDiagnosticSchema)(sourceDiagnosticFixture)).toBe(true);
  for (const row of sourceDiagnosticFixture.cases) {
    expect(() => JSON.parse(row.source)).toThrow();
    expect(row.expected.code).toBe("snapshot-edit.invalid-source");
    expect(row.expected.line).toBeGreaterThan(0);
    expect(row.expected.column).toBeGreaterThan(0);
  }
});

test("retained native route and cancellation fixture matches the independent oracle", () => {
  expect(new Ajv({ strict: true }).compile(retainedNativeSchema)(retainedNativeFixture)).toBe(true);
  for (const row of retainedNativeFixture.routeCases) {
    const actual = row.recognized ? (row.primaryAccepts ? "primary" : "refused") : "fallback";
    expect(actual).toBe(row.expected);
  }
  const progress = Array.from({ length: retainedNativeFixture.lifecycle.cancelAfter }, (_, index) => index + 1);
  expect(progress).toEqual(retainedNativeFixture.lifecycle.expectedProgress);
  expect(retainedNativeFixture.lifecycle.units - retainedNativeFixture.lifecycle.cancelAfter + 1).toBe(retainedNativeFixture.lifecycle.expectedCloseSteps);
  const text = "x".repeat(retainedNativeFixture.textCopy.byteLength);
  expect(new TextEncoder().encode(text).byteLength).toBe(retainedNativeFixture.textCopy.byteLength);
  expect(Math.ceil(text.length / retainedNativeFixture.textCopy.pageBytes)).toBe(retainedNativeFixture.textCopy.expectedDataSteps);
  expect(Math.ceil(retainedNativeFixture.textCopy.cancelAfterBytes / retainedNativeFixture.textCopy.pageBytes) + 1).toBe(retainedNativeFixture.textCopy.expectedCloseSteps);
  const unicode = new TextEncoder().encode(retainedNativeFixture.textCopy.interruptedUnicode);
  for (const stop of retainedNativeFixture.textCopy.partialByteStops) expect(unicode.slice(0, stop).byteLength).toBe(stop);
  for (const row of retainedNativeFixture.commandAdmission.cases) {
    expect(new TextEncoder().encode(row.value).byteLength <= retainedNativeFixture.commandAdmission.maximumBytes).toBe(row.accepted);
  }
  const structure = retainedNativeFixture.structuralCopy;
  const paragraph = { runs: Array.from({ length: structure.runCount }, (_, index) => ({ text: `r${index}`, bold: false, italic: false, underline: false, extraRunProperties: [] })), style: null, extraParagraphProperties: [] };
  const oracle = structuredClone(paragraph);
  oracle.runs[structure.runCount - 1]!.text = "updated";
  expect(paragraph.runs[structure.runCount - 1]!.text).not.toBe(oracle.runs[structure.runCount - 1]!.text);
  const mesh = {
    positions: Array.from({ length: structure.meshVertexCount }, (_, index) => ({ x: index, y: index + 1, z: index + 2 })),
    texture: Uint8Array.from({ length: structure.textureByteLength }, (_, index) => index % 251),
  };
  const meshOracle = structuredClone(mesh);
  meshOracle.positions[structure.meshVertexCount - 1] = { x: -1, y: -2, z: -3 };
  expect(mesh.positions[structure.meshVertexCount - 1]).not.toEqual(meshOracle.positions[structure.meshVertexCount - 1]);
  expect(createHash("sha256").update(mesh.texture).digest("hex")).toBe(createHash("sha256").update(meshOracle.texture).digest("hex"));
  const brep = {
    vertices: Array.from({ length: structure.brepVertexCount }, (_, index) => ({ id: `v-${index}`, point: { x: index, y: 0, z: 0 } })),
    nurbs: Array.from({ length: structure.nurbsPointCount }, (_, index) => ({ x: index, y: index, z: 0 })),
  };
  const brepOracle = structuredClone(brep);
  brepOracle.vertices[structure.brepVertexCount - 1]!.point.z = 5;
  expect(brep.vertices[structure.brepVertexCount - 1]!.point.z).toBe(0);
  expect(brepOracle.nurbs).toEqual(brep.nurbs);
  expect(structure.cancelAfterTurns.every((turn, index, turns) => index === 0 || turn > turns[index - 1]!)).toBe(true);
  const document = retainedNativeFixture.documentCopy;
  const sibling = Uint8Array.from({ length: document.siblingByteLength }, (_, index) => index % 251);
  const copied = new Uint8Array(sibling.length);
  for (let offset = 0; offset < sibling.length; offset += document.pageBytes) copied.set(sibling.subarray(offset, offset + document.pageBytes), offset);
  const digest = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
  expect(digest(copied)).toBe(digest(sibling));
  expect(new TextEncoder().encode(document.replacementText.repeat(document.replacementRepeats)).byteLength).toBeGreaterThanOrEqual(131072);
  expect(Math.ceil(sibling.length / document.pageBytes)).toBeGreaterThan(document.minimumCompleteTurns);
});

test("complete source matches the independent JSON oracle", () => {
  const source = snapshotEditSource(fixture.sourceRoundTrip);
  expect(JSON.parse(source)).toEqual(fixture.sourceRoundTrip);
  expect(snapshotFromEditSource(source, codec)).toEqual(fixture.sourceRoundTrip);
  expect(applySnapshotEdit(fixture.base, { operation: "replaceSource", source }, codec)).toEqual(fixture.sourceRoundTrip);
});

for (const row of fixture.rejectedSources) test(row.id, () => {
  try { snapshotFromEditSource(row.source, codec); throw new Error("invalid source accepted"); }
  catch (error) { expect(error).toBeInstanceOf(SnapshotEditError); expect((error as SnapshotEditError).code).toBe(row.code); }
});

test("source preserves complete signed and unsigned 64-bit integers", () => {
  const parsed = snapshotFromEditSource(fixture.wideIntegerSource);
  expect(parsed).toEqual({ unsigned: 18446744073709551615n, signed: -9223372036854775808n });
  expect(snapshotEditSource(parsed).replace(/\s/gu, "")).toBe(fixture.wideIntegerSource);
});

for (const row of fixture.floatInputs) test(`typed float input ${row.source}`, () => {
  const source = `{${Object.entries(fixture.base as Record<string, SnapshotValue>).map(([key, value]) => `${JSON.stringify(key)}:${key === "ratio" ? row.source : JSON.stringify(value)}`).join(",")}}`;
  if (row.accepted) expect(snapshotFromEditSource(source, codec)).toEqual(JSON.parse(source));
  else expect(() => snapshotFromEditSource(source, codec)).toThrow(SnapshotEditError);
});

test("typed edit codecs cannot silently discard unrelated details", () => {
  const normalized: SnapshotEditCodec<SnapshotValue> = { validate: (value) => {
    const copy = structuredClone(value) as { [key: string]: SnapshotValue };
    delete copy.optional;
    return copy;
  } };
  const row = fixture.accepted.find((item) => item.id === "set-string") ?? fixture.accepted[0];
  expect(() => applySnapshotEdit(fixture.base, row.event, normalized)).toThrow("normalize or discard");
  expect(fixture.base).toEqual(oracle(fixture.base, []));
});

const pathFixture = await Bun.file(new URL("../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔁️codec/🧭️path/🧫️fixtures/🔣️.json", import.meta.url)).json() as {
  accepted: { id: string; before: SnapshotValue; path: string[]; edit: { operation: "set" | "insert" | "remove"; value?: SnapshotValue }; expected: SnapshotValue }[];
};
const pathSchema = await Bun.file(new URL("../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔁️codec/🧭️path/🧬️schema/🔣️.json", import.meta.url)).json();
const validatePath = new Ajv({ strict: true, allErrors: true }).compile(pathSchema);

describe("shared typed path fixture", () => {
  for (const row of pathFixture.accepted) test(row.id, () => {
    expect(validatePath({ path: row.path, edit: row.edit })).toBe(true);
    const path = row.path.length ? "/" + row.path.map((segment) => segment.replaceAll("~", "~0").replaceAll("/", "~1")).join("/") : "";
    const event: SnapshotEditEvent = row.edit.operation === "remove"
      ? { operation: "removeValue", path }
      : { operation: row.edit.operation === "set" ? "setValue" : "insertValue", path, value: row.edit.value! };
    const patch = [{ op: row.edit.operation === "set" ? "replace" : row.edit.operation === "insert" ? "add" : "remove", path, ...(row.edit.operation === "remove" ? {} : { value: row.edit.value }) }];
    expect(oracle(row.before, patch)).toEqual(row.expected);
    expect(applySnapshotEdit(row.before, event)).toEqual(row.expected);
  });
});
