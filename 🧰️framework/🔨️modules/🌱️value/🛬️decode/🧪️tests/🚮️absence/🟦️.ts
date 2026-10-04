/** 🚮️ Executes the original lower portable corpus with every declared specialization physically absent. */
import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import contract from "../../🧫️fixtures/🧩️ownership/🔣️.json";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");

test("the neutral ownership and original portable laws execute without products, S, or Hub", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const workspace = mkdtempSync(resolve(output, "decode-absence-"));
  try {
    for (const owner of ["🧰️framework/🔨️modules/🌱️value", "🧰️framework/🔨️modules/🧬️schema/✅️validator"]) {
      const target = resolve(workspace, owner);
      mkdirSync(resolve(target, ".."), { recursive: true });
      cpSync(resolve(root, owner), target, { recursive: true, dereference: false, filter: (path) => { if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path); return true; } });
    }
    for (const absent of contract.absentRoots) expect(existsSync(resolve(workspace, absent)), absent).toBe(false);
    await runOwnedCommand(process.execPath, ["test", resolve(workspace, "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(workspace, "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts")], workspace, "value:decode:products-absent", 30_000);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
}, 30_000);
