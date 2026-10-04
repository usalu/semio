import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const owner = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🚮️absence/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🚮️absence/🔣️.json"), "utf8"));

test("closed Rust syntax absence authority has independent AJV admission", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const changed of [{ ...fixture, extra: true }, { ...fixture, absentRoots: [] }]) expect(validate(changed)).toBe(false);
});

test("actual neutral Rust syntax laws execute without products, S, Hub, DSL, Schema or Value", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const workspace = mkdtempSync(resolve(output, "rust-syntax-absence-"));
  try {
    const target = resolve(workspace, fixture.owner);
    mkdirSync(resolve(target, ".."), { recursive: true });
    cpSync(resolve(root, fixture.owner), target, { recursive: true, dereference: false, filter: path => {
      if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path);
      return true;
    } });
    for (const path of fixture.absentRoots) expect(existsSync(resolve(workspace, path)), path).toBe(false);
    await runOwnedCommand(process.execPath, ["test", resolve(target, fixture.lawSuffix)], workspace, "compiler:syntax:rust:consumers-absent", 30000);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
}, 30000);

