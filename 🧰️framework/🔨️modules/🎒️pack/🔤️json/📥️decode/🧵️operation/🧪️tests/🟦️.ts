import grantSchema from"../../../../../🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";import{semioSchemaAjvV1}from"../../../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import {expect, test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync, existsSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

test("original JSON operation declares every normal and retirement authority", () => {
  const validate = semioSchemaAjvV1({strict: true, allErrors: true}).compile(grantSchema);
  for (const owner of ["normalAuthority", "retirementAuthority", "turn", "closeTurn", "outcomeNormalAuthority"] as const) {
    expect(validate(fixture[owner])).toBe(true);
    for (const axis of Object.keys(fixture[owner])) {
      const denied = structuredClone(fixture[owner]) as Record<string, number>;
      delete denied[axis];
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
  console.log("[DEBUG] JSON original operation canonical Grant25 missing-axis refusals and independent SQLite five-axis/UTF8 laws passed");
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

test("JSON outcomes retain their original UTF8 owners behind borrowed views", () => {

  const database = new Database(":memory:");
  try {
    for (const row of fixture.outcomes) {
      const expected = database.query("SELECT count(*) AS occurrences, length(CAST(? AS BLOB)) AS bytes FROM json_each(?) WHERE CASE WHEN type='text' THEN value ELSE key END=?").get(row.text, row.source, row.text) as {occurrences: number; bytes: number};
      expect(expected).toEqual({occurrences: row.occurrences, bytes: row.utf8Bytes});
      expect(row.occurrences > 1 ? "duplicateMember" : "string").toBe(row.expected);
    }
  } finally {
    database.close();
  }
  const source = readFileSync(resolve(import.meta.dir, "../🦀️.rs"), "utf8");
  expect(source.includes("Result<Option<&V>,&JsonError>")).toBe(true);
  console.log("[DEBUG] JSON value and duplicate-key diagnostic original UTF8 custody matches independent SQLite corpus");
});

test("current Pack JSON examples have no whole-trial schema authority",async()=>{const{existsSync}=await import("node:fs");expect(existsSync(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);});
