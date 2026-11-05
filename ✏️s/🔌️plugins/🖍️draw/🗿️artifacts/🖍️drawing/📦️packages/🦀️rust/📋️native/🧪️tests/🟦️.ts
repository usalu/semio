/** 🧪️ The owned native package catalog covers current Cargo members and agrees with independent TOML, Ajv and SQLite oracles. */
import { test } from "bun:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { validateJsonSchemaSubset } from "@semio-tech/framework/schema";
import catalog from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };
import fixtureSchema from "./🧬️.json" with { type: "json" };
import { drawingNativeOptions } from "../🟦️.ts";

type Package = { name: string; manifest: string; check: boolean; test: boolean };
const sorted = (rows: readonly Package[]): Package[] => [...rows].sort((left, right) => left.name.localeCompare(right.name));
const toml = (path: string): Record<string, any> => Bun.TOML.parse(readFileSync(path, "utf8"));

test("Drawing native catalog covers every workspace test and check owner", () => {
  assert.equal(new Ajv({ strict: true }).validate(fixtureSchema, fixture), true);
  const root = resolve(import.meta.dirname, "../../../..");
  const members = toml(resolve(root, "Cargo.toml")).workspace.members as string[];
  const expected = sorted(members.map(member => ({ name: toml(resolve(root, member, "Cargo.toml")).package.name as string, manifest: member + "/Cargo.toml", check: true, test: true })));
  assert.equal(expected.length, fixture.expectedPackages);
  assert.deepEqual(sorted(catalog.packages), expected);
  assert.deepEqual([...drawingNativeOptions.packages].sort(), expected.map(row => row.name));
  assert.deepEqual(drawingNativeOptions.testPackages, drawingNativeOptions.packages);
  for (const name of fixture.requiredPackages) assert(drawingNativeOptions.testPackages.includes(name));
  const ajv = new Ajv({ strict: true, allErrors: true }).compile(schema);
  const db = new Database(":memory:");
  db.exec("create table expected(name text,manifest text,checking integer,testing integer);create table actual(name text,manifest text,checking integer,testing integer)");
  try {
    for (const row of expected) db.query("insert into expected values(?,?,?,?)").run(row.name, row.manifest, Number(row.check), Number(row.test));
    for (const row of fixture.cases) {
      const value = structuredClone(catalog);
      if (row.effect === "omit") value.packages.pop();
      if (row.effect === "duplicate") value.packages.push({ ...value.packages[0]! });
      if (row.effect === "unknown") value.packages[0]!.name = "semio-s-artifact-draw-drawing-unknown";
      if (row.effect === "wrong-manifest") value.packages[0]!.manifest = "unknown/Cargo.toml";
      if (row.effect === "omit-check") value.packages[0]!.check = false;
      if (row.effect === "omit-test") value.packages[0]!.test = false;
      assert.equal(validateJsonSchemaSubset(schema, value).length === 0, row.schemaAccepted, row.effect);
      assert.equal(ajv(value), row.schemaAccepted, row.effect);
      db.exec("delete from actual");
      for (const entry of value.packages) db.query("insert into actual values(?,?,?,?)").run(entry.name, entry.manifest, Number(entry.check), Number(entry.test));
      const oracle = db.query("select (select count(*) from actual)=(select count(*) from expected) and not exists(select name,manifest,checking,testing from actual except select name,manifest,checking,testing from expected) and not exists(select name,manifest,checking,testing from expected except select name,manifest,checking,testing from actual) complete").get() as { complete: number };
      assert.equal(JSON.stringify(sorted(value.packages)) === JSON.stringify(expected), row.complete, row.effect);
      assert.equal(Boolean(oracle.complete), row.complete, row.effect);
    }
  } finally { db.close(); }
});
