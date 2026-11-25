import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const owner = resolve(import.meta.dir, "../../"), read = (path: string) => readFileSync(resolve(owner, path), "utf8");
const dialect = JSON.parse(read("🧪️tests/🧫️fixtures/🏷️written-dialect/🔣️.json"));
const function_body = (source: string, name: string) => {
  const start = source.indexOf(`fn ${name}`);
  expect(start).toBeGreaterThanOrEqual(0);
  const open = source.indexOf("{", start);
  let depth = 0, end = open;
  for (; end < source.length; end++) {
    if (source[end] === "{") depth++;
    if (source[end] === "}" && --depth === 0) break;
  }
  return source.slice(open, end + 1);
};

test("one constant owns the written DWG dialect", () => {
  const source = read(dialect.ownerSource);
  expect(source).toContain(`pub const ${dialect.owner}: &str = "${dialect.stamp}";`);
  expect(source.match(new RegExp(`pub const ${dialect.owner}\\b`, "g"))).toHaveLength(1);
  expect(dialect.stamp).toHaveLength(dialect.stampLength);
  expect(dialect.readOnlyStamps).not.toContain(dialect.stamp);
});

test("no writer or reader entry re-spells the written stamp", () => {
  for (const entry of dialect.literalFreeSources) {
    const [file, name] = entry.split("#");
    const body = function_body(read(file), name);
    expect(body).not.toContain(`"${dialect.stamp}"`);
    expect(body).toContain(dialect.owner);
  }
});

test("the committed AutoCAD drawing carries the owned stamp", () => {
  const bytes = readFileSync(resolve(owner, "🖼️assets/🏛️architectural/🏛️architectural.dwg"));
  expect(bytes.subarray(dialect.stampOffset, dialect.stampOffset + dialect.stampLength).toString("latin1")).toBe(dialect.stamp);
});

test("the stepped writer runs the phases its fixture names, in order", () => {
  const cursor = read("🚪️io/📤️export/🧵️write-cursor/🦀️.rs"), fixture = JSON.parse(read("🚪️io/📤️export/🧪️tests/🧫️fixtures/🧵️write-cursor/🔣️.json"));
  const declared = cursor.slice(cursor.indexOf("enum Phase {"), cursor.indexOf("}", cursor.indexOf("enum Phase {"))).match(/^\s+([A-Z][a-z]+)/gm)!.map((name) => name.trim().toLowerCase());
  const named = declared.filter((name) => !["taken", "failed"].includes(name));
  expect(named).toEqual(fixture.phaseOrder);
  expect(fixture.oracleEntityCount).toBe(68);
});
