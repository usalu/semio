#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-contract` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 🔖️test
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️conformance
/** @emoji 🧪️ Runs only `🦀️conformance.rs`'s corpus harness — every fixture under
 * `📚️examples/🧪️conformance/` deserializes, validates/patches through this crate's own
 * `validate_snapshot`/`apply_patch`, and matches its declarative expectation. Same test binary as
 * `test`, filtered to the `conformance::` module path so iterating on the corpus does not pay for the
 * whole crate's suite. */
class ConformanceScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest, "--", "conformance::"]);
  }
}
//#endregion 🔖️conformance

//#region 🔖️check-wasm
/** @emoji 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui-contract", ...args], { cwd: packageRoot, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-wasip2"]);
    check(["--target", "wasm32-unknown-unknown"]);
    check(["--target", "wasm32-wasip2", "--features", "typegen"]);
  }
}
//#endregion 🔖️check-wasm

//#region 🔖️typegen
/** 🧬️ Name of the `tests/typegen_export.rs` integration test that calls `TS::export_all_to` for
 * every wire-facing type — kept as a `tests/*.rs` file (not a `#[cfg(test)] mod` inside the crate's
 * own `🦀️*.rs` files) because packet `manifest-typegen` doesn't own those files; see the test's own
 * header for the full rationale and the registrar-request this stands in for. */
const TYPEGEN_TEST_NAME = "typegen_export";

/** 📁️ ts-rs' default per-crate export directory — a scratch dir, never committed. */
function bindingsDir(root: string): string {
  return join(root, "bindings");
}

/** 🎯️ The mirror lives at `🔨️modules/🛂️manifest/🤖️generated/🟦️ui-contract.ts`, a sibling of the
 * hand-written `🟦️component.ts` that re-exports it — never inside `📦️packages`, matching the
 * `🤖️generated` placement `@semio-tech/framework-rs:generate` already uses for `🟦️manifest.ts`. */
function generatedUiContractPath(root: string): string {
  return join(root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated", "🟦️ui-contract.ts");
}

/** 🧬️ Runs the `typegen_export` integration test with the `typegen` feature enabled, populating `bindings/*.ts`. */
function runTypegenExportTest(root: string): void {
  const status = runCmdStatus("cargo", ["test", "-p", "semio-framework-ui-contract", "--features", "typegen", "--test", TYPEGEN_TEST_NAME], {
    cwd: root,
    env: process.env,
    budgetMs: buildBudgetMs(),
  });
  if (status !== 0) {
    console.error("ui-contract typegen: `cargo test --features typegen --test typegen_export` failed — see output above.");
    process.exit(status);
  }
}

/** ✂️ Strips ts-rs' per-file header comment and local `import type { ... } from "./X"` boilerplate,
 * leaving the bare `export type`/`export interface` declaration — every type lands in the same
 * consolidated file, so a sibling import is meaningless and a forward reference resolves fine in
 * TypeScript's type space. Mirrors `@semio-tech/framework-rs:generate`'s own helper of the same name;
 * duplicated rather than shared because the library package backing both scripts isn't in this
 * packet's OWNS list. */
function stripTsRsBoilerplate(source: string): string {
  return source
    .split("\n")
    .filter((line) => line.trim().length > 0 && !line.startsWith("//") && !line.startsWith("import "))
    .join("\n")
    .trim();
}

/** 🧬️ Reads every ts-rs per-type file out of `dir`, strips its boilerplate, and flattens them into
 * one de-duplicated, alphabetically ordered file — one bundle rather than ~80 per-type files because
 * the consumer is `🟦️component.ts`'s single `import type { ... } from "./🤖️generated/🟦️ui-contract.ts"`
 * block, and one import statement is easier to keep correct than ~80. */
function consolidateBindings(dir: string): string {
  const files = readdirSync(dir)
    .filter((name) => name.endsWith(".ts"))
    .sort();
  const seen = new Set<string>();
  const blocks: string[] = [];
  for (const name of files) {
    const body = stripTsRsBoilerplate(readFileSync(join(dir, name), "utf8"));
    const typeName = body.match(/^export (?:type|interface) (\w+)/)?.[1] ?? name.replace(/\.ts$/, "");
    if (seen.has(typeName)) continue;
    seen.add(typeName);
    blocks.push(body);
  }
  return `/** @generated by \`bun nx run @semio-tech/ui-contract-rs:generate\` from ui/🧬️contract/📦️packages/🦀️rust/📦️glue.rs via ts-rs. Do not edit. */\n\n${blocks.join("\n\n")}\n`;
}

/** 🧬️ Runs the typegen export test into a scratch `bindings/` dir, consolidates it in memory, then
 * removes the scratch dir — the caller decides whether to write the result. */
function buildUiContract(root: string): string {
  const dir = bindingsDir(root);
  rmSync(dir, { recursive: true, force: true });
  runTypegenExportTest(root);
  if (!existsSync(dir)) {
    console.error(`ui-contract typegen: expected ts-rs to write ${dir}, found nothing.`);
    process.exit(1);
  }
  const contract = consolidateBindings(dir);
  rmSync(dir, { recursive: true, force: true });
  return contract;
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const contract = buildUiContract(this.root);
    const outPath = generatedUiContractPath(this.root);
    mkdirSync(join(this.root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated"), { recursive: true });
    writeFileSync(outPath, contract);
    console.log(`ui-contract typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🔎️ Rebuilds the contract in memory and byte-compares it against `🤖️generated/🟦️ui-contract.ts` —
 * never writes that file (a lint/verify step must never let the auto-commit daemon land regenerated
 * files). Byte-identical output on a repeated run is the idempotency guarantee `generate` needs. */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const contract = buildUiContract(this.root);
    const outPath = generatedUiContractPath(this.root);
    if (!existsSync(outPath) || readFileSync(outPath, "utf8") !== contract) {
      console.error(`ui-contract typescript mirror is stale: ${outPath}`);
      console.error("run `bun nx run @semio-tech/ui-contract-rs:generate` to refresh.");
      process.exit(1);
    }
    console.log("ui-contract typescript mirror is fresh.");
  }
}
//#endregion 🔖️typegen

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir)
    .register("test", TestScript)
    .register("conformance", ConformanceScript)
    .register("check-wasm", CheckWasmScript)
    .register("generate", GenerateScript)
    .register("check", CheckScript);
  await runBundleScriptMain(router, import.meta.url);
}
