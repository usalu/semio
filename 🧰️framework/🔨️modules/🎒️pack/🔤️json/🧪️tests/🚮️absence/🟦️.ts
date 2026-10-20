/** 🚮️ Runs every canonical JSON ownership law with specialization roots physically absent. */
import { expect, test } from "bun:test";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";

import contract from "../../🧪️tests/🧫️fixtures/🚮️absence/🔣️.json";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");

/** 🔗️ Links every neutral workspace package into the isolated workspace like an install would. */
const linkWorkspacePackages = (directory: string, workspace: string): void => {
  for (const entry of readdirSync(directory, {withFileTypes: true})) {
    if (entry.name === "node_modules") continue;
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) linkWorkspacePackages(path, workspace);
    else if (entry.name === "package.json") {
      const name = (JSON.parse(readFileSync(path, "utf8")) as {name?: unknown}).name;
      if (typeof name !== "string" || !name.startsWith("@semio-tech/")) continue;
      const target = resolve(workspace, "node_modules", name);
      mkdirSync(dirname(target), {recursive: true});
      if (!existsSync(target)) symlinkSync(directory, target, "junction");
    }
  }
};

test("the complete JSON reference route executes with products, S, Hub and ticket sources absent", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, {recursive: true});
  const workspace = mkdtempSync(resolve(output, "json-absence-"));
  try {
    for (const owner of contract.neutralRoots) {
      const target = resolve(workspace, owner);
      mkdirSync(resolve(target, ".."), {recursive: true});
      cpSync(resolve(root, owner), target, {recursive: true, dereference: false, filter: (path) => { if (basename(path) === "node_modules") return false; if (lstatSync(path).isSymbolicLink()) throw Error("linked projection input: " + path); return true; }});
    }
    for (const dependency of contract.testDependencies) {
      let store = dirname(realpathSync(resolve(root, "node_modules", dependency)));
      while (basename(store) !== "node_modules") store = dirname(store);
      cpSync(store, resolve(workspace, "node_modules"), {recursive: true, dereference: true, force: false, filter: (path) => basename(path) !== ".bin"});
    }
    for (const owner of contract.neutralRoots) linkWorkspacePackages(resolve(workspace, owner), workspace);
    for (const absent of contract.absentRoots) expect(existsSync(resolve(workspace, absent)), absent).toBe(false);
    expect(existsSync(resolve(workspace, ".🧬semio"))).toBe(false);
    writeFileSync(resolve(workspace, "package.json"), JSON.stringify({private: true, type: "module", workspaces: []}));
    writeFileSync(resolve(workspace, "📋️project.json"), "{}");
    await runBudgetedTestCommand(process.execPath, [resolve(workspace, contract.reference.script), contract.reference.command], {cwd: workspace, budgetMs: contract.reference.budgetMs, env: process.env, throwOnFailure: true});
  } finally {
    rmSync(workspace, {recursive: true, force: true});
  }
}, 30000);
