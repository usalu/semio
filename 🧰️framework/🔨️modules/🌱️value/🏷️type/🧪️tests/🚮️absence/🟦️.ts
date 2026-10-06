/** 🚮️ Runs the actual lower type laws with every higher consumer physically absent. */
import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🚮️absence/🔣️.json";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");

test("all original neutral type laws execute without Graph, products, S, or Hub", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, {recursive: true});
  const workspace = mkdtempSync(resolve(output, "value-type-absence-"));
  try {
    const owner = "🧰️framework/🔨️modules/🌱️value", target = resolve(workspace, owner);
    mkdirSync(resolve(target, ".."), {recursive: true});
    cpSync(resolve(root, owner), target, {recursive: true, dereference: false, filter: (path) => {if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path); return true;}});
    for (const absent of contract.absentRoots) expect(existsSync(resolve(workspace, absent)), absent).toBe(false);
    await runOwnedCommand(process.execPath, ["test", resolve(workspace, owner, "🏷️type/🧪️tests/🟦️.ts")], workspace, "value:type:consumers-absent", 30000);
  } finally {rmSync(workspace, {recursive: true, force: true});}
}, 30000);
