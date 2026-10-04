import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const owner = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🚮️absence/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🚮️absence/🔣️.json"), "utf8"));

test("closed schema and JSON absence authority has independent AJV admission", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const changed of [{ ...fixture, extra: true }, { ...fixture, absentRoots: [] }, { ...fixture, sharedSources: [] }]) expect(validate(changed)).toBe(false);
});

test("all actual schema closure and JSON syntax laws execute with products, S and Hub absent", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const workspace = mkdtempSync(resolve(output, "schema-json-absence-"));
  try {
    for (const path of [...fixture.owners, ...fixture.sharedSources]) {
      const destination = resolve(workspace, path);
      mkdirSync(resolve(destination, ".."), { recursive: true });
      cpSync(resolve(root, path), destination, { recursive: true, dereference: false, filter: path => {
        if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path);
        return true;
      } });
    }
    for (const path of fixture.absentRoots) expect(existsSync(resolve(workspace, path)), path).toBe(false);
    await runOwnedCommand(process.execPath, ["test", ...fixture.owners.map((path: string) => resolve(workspace, path, fixture.lawSuffix))], workspace, "schema:source:closure:consumers-absent", 30000);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
}, 30000);

