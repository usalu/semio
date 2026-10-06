/** 🚮️ Runs every canonical JSON ownership law with specialization roots physically absent. */
import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

import contract from "../../🧫️fixtures/🚮️absence/🔣️.json";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");

test("the complete JSON reference route executes with products, S, Hub and ticket sources absent", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, {recursive: true});
  const workspace = mkdtempSync(resolve(output, "json-absence-"));
  try {
    for (const owner of contract.neutralRoots) {
      const target = resolve(workspace, owner);
      mkdirSync(resolve(target, ".."), {recursive: true});
      cpSync(resolve(root, owner), target, {recursive: true, dereference: false, filter: (path) => { if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path); return true; }});
    }
    for (const dependency of contract.testDependencies) {
      const target = resolve(workspace, "node_modules", dependency);
      mkdirSync(resolve(target, ".."), {recursive: true});
      cpSync(resolve(root, "node_modules", dependency), target, {recursive: true, dereference: true});
    }
    for (const absent of contract.absentRoots) expect(existsSync(resolve(workspace, absent)), absent).toBe(false);
    expect(existsSync(resolve(workspace, ".🧬semio"))).toBe(false);
    writeFileSync(resolve(workspace, "package.json"), JSON.stringify({private: true, type: "module"}));
    writeFileSync(resolve(workspace, "nx.json"), "{}");
    await runBudgetedTestCommand(process.execPath, [resolve(workspace, contract.reference.script), contract.reference.command], {cwd: workspace, budgetMs: contract.reference.budgetMs, env: process.env, throwOnFailure: true});
  } finally {
    rmSync(workspace, {recursive: true, force: true});
  }
}, 30000);
