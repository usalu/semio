import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("declared native defaults agree with independent schema default construction", () => {
  const actual = structuredClone(fixture.input);
  expect(new Ajv({ strict: true, useDefaults: true }).compile(schema)(actual)).toBe(true);
  expect(actual).toEqual(fixture.expected);
  const db = new Database(":memory:");
  try {
    expect(db.query("SELECT json_extract(?, '$.values') AS list, json_extract(?, '$.scale') AS scale, json_extract(?, '$.enabled') AS enabled").get(JSON.stringify(actual), JSON.stringify(actual), JSON.stringify(actual))).toEqual({ list: "[]", scale: 0, enabled: 0 });
  } finally { db.close(); }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("pub trait NativeDefault");
  expect(source).toContain("retained_clone_grant_admits(grant,demand)");
  expect(source).not.toContain("Box::new");
  expect(source).not.toContain("Default::default()");
  expect(fixture.zeroHeapBirth && fixture.sameOriginalSource).toBe(true);
});
