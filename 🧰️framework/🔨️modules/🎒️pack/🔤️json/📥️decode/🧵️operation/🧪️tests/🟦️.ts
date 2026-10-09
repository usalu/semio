import {expect, test} from "bun:test";
import {Database} from "bun:sqlite";
import {createRequire} from "node:module";
import {readFileSync, existsSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("original JSON operation declares every normal and retirement authority", () => {
  const require = createRequire(import.meta.url);
  const validate = new (require("ajv"))({strict: true, allErrors: true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const owner of ["normalAuthority", "retirementAuthority", "turn", "closeTurn"] as const) {
    for (const axis of Object.keys(fixture[owner])) {
      const denied = structuredClone(fixture);
      delete (denied[owner] as Record<string, number>)[axis];
      expect(validate(denied)).toBe(false);
    }
  }
  const database = new Database(":memory:");
  try {
    const bytes = database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(fixture.source) as {bytes: number};
    expect(bytes.bytes).toBe(fixture.limits.maximumBytes);
    expect(JSON.parse(fixture.source)).toBe("😀");
    for (const axis of fixture.axes) {
      const expected = database.query("SELECT min(?, ? - ?) AS remaining").get(axis.incoming, axis.total, axis.used) as {remaining: number};
      expect(expected.remaining).toBe(axis.remaining);
    }
  } finally {
    database.close();
  }
  console.log("[DEBUG] JSON original operation strict20 missing-axis refusals and independent SQLite five-axis/UTF8 corpus passed");
});

test("the defining JSON operation binds the original control and retained source", () => {
  const path = resolve(import.meta.dir, "../🦀️.rs");
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  expect(source.includes("control:&'operation mut NativeDecodeControl<'callback>")).toBe(true);
  expect(source.includes("JsonSourceCursor::new(source,members,policy.limits)")).toBe(true);
  expect(source.includes("policy.normal")).toBe(true);
  expect(source.includes("policy.retirement")).toBe(true);
  expect(source.includes("NativeDecodeControl::new(")).toBe(false);
  expect(source.includes("pub fn new_with_")).toBe(false);
});
