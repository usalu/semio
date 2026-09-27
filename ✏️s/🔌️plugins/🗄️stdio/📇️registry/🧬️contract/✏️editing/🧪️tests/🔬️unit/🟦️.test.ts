import { describe, expect, test } from "bun:test";
import { applyPatch, type Operation } from "fast-json-patch";
import Ajv from "ajv";
import { snapshotEditSource, snapshotFromEditSource, applySnapshotEdit, SnapshotEditError, type SnapshotEditCodec, type SnapshotEditEvent, type SnapshotValue } from "../../🟦️";

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
