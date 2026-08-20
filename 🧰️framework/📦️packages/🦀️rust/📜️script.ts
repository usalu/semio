#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework` task router: `bun ./📜️script.ts test|generate|check|lint`. */
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, runCmdStatus, runVitest, resolveTestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework"], this.repoRoot, rest);
    await runVitest(this.root, rest, "../🟦️typescript/🧪️vitest.config.ts");
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework"], this.root, segments);
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the `#[cfg(test, feature = "typegen")]` test in `📦️glue.rs` that calls `TS::export()` for every mirrored type. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 📁️ ts-rs' default per-crate export directory — a scratch dir, never committed. */
function bindingsDir(root: string): string {
  return join(root, "bindings");
}

/** 🎯️ Shape V2 target: the mirror lives at `<owner>/🤖️generated/🟦️manifest.ts`, a sibling of `📦️packages`, never inside it (data relocates to the owner root — see `🔣️taxonomy.json`'s `rootDataDirNames`). */
function generatedManifestPath(root: string): string {
  return join(root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated", "🟦️manifest.ts");
}

/** 🔗️ The sibling ts-rs mirror — same `🤖️generated/` directory as `🟦️manifest.ts` — that the manifest surface has grown cross-crate field-type references into (`Label`/`StyleSpec`/etc. from `semio-framework-ui-contract`, via `@semio-tech/ui-contract-rs:generate`). Consolidation strips ALL `import` lines (see `stripTsRsBoilerplate`), which was correct while every referenced type lived in this same file; it no longer is, so `consolidateBindings` re-adds exactly the imports this file's own body needs from it. */
function uiContractMirrorPath(root: string): string {
  return join(root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated", "🟦️ui-contract.ts");
}

/** 🧬️ Runs the ts-rs export test with the `typegen` feature enabled, populating `bindings/*.ts`. */
function runTypegenExportTest(root: string): void {
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], {
    cwd: root,
    env: process.env,
    budgetMs: buildBudgetMs(),
  });
  if (status !== 0) {
    console.error("framework typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

/** ✂️ Strips ts-rs' per-file header comment and local `import type { ... } from "./X"` boilerplate, leaving the bare `export type`/`export interface` declaration. */
function stripTsRsBoilerplate(source: string): string {
  return source
    .split("\n")
    .filter((line) => line.trim().length > 0 && !line.startsWith("//") && !line.startsWith("import "))
    .join("\n")
    .trim();
}

/** 🔗️ Every top-level `export type`/`export interface` name `mirrorPath` declares — `Set()` if the file is absent (a from-scratch checkout before `ui-contract-rs:generate` has ever run). */
function declaredTypeNames(mirrorPath: string): Set<string> {
  if (!existsSync(mirrorPath)) return new Set();
  const source = readFileSync(mirrorPath, "utf8");
  const names = new Set<string>();
  for (const match of source.matchAll(/^export (?:type|interface) (\w+)/gm)) names.add(match[1]);
  return names;
}

/** ✂️ Drops ts-rs' inline `/** ... *\/` field-doc comments before scanning body text for identifier
 * references — a name only mentioned in prose (an `{@link X}`) must never trigger an import. */
function stripBlockComments(body: string): string {
  return body.replace(/\/\*\*[\s\S]*?\*\//g, "");
}

/** 🔗️ Builds the `import type { ... } from "<specifier>"` line for every name `mirrorPath` declares
 * that `body` references in actual code (never inside a doc comment) but does not itself declare —
 * the fix for the sibling-mirror split (see `uiContractMirrorPath`'s doc comment). Returns `""` when
 * nothing is needed, so a manifest with no cross-mirror references is byte-identical to before this
 * mechanism existed. */
function crossMirrorImportLine(body: string, declaredHere: Set<string>, mirrorPath: string, importSpecifier: string): string {
  const mirrorNames = declaredTypeNames(mirrorPath);
  const codeOnly = stripBlockComments(body);
  const needed = [...mirrorNames].filter((name) => !declaredHere.has(name) && new RegExp(`\\b${name}\\b`).test(codeOnly)).sort();
  return needed.length === 0 ? "" : `import type { ${needed.join(", ")} } from "${importSpecifier}";\n\n`;
}

/** 🧬️ Reads every ts-rs per-type file out of `dir`, strips its boilerplate, and flattens them into one de-duplicated, alphabetically ordered manifest body — then re-adds an explicit `import type` for any name the body references but does not declare itself, resolved against the sibling `🟦️ui-contract.ts` mirror in the same `🤖️generated/` directory. */
function consolidateBindings(dir: string, mirrorPath: string, mirrorImportSpecifier: string): string {
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
  const bodyText = blocks.join("\n\n");
  const importLine = crossMirrorImportLine(bodyText, seen, mirrorPath, mirrorImportSpecifier);
  return `/** @generated by \`bun nx run @semio-tech/framework:generate\` from framework/📦️packages/🦀️rust/📦️glue.rs via ts-rs. Do not edit. */\n\n${importLine}${bodyText}\n`;
}

/** 🧬️ Runs the ts-rs export test into a scratch `bindings/` dir, consolidates it in memory, then removes the scratch dir — the caller decides whether to write the result. */
function buildManifest(root: string): string {
  const dir = bindingsDir(root);
  rmSync(dir, { recursive: true, force: true });
  runTypegenExportTest(root);
  if (!existsSync(dir)) {
    console.error(`framework typegen: expected ts-rs to write ${dir}, found nothing.`);
    process.exit(1);
  }
  const manifest = consolidateBindings(dir, uiContractMirrorPath(root), "./🟦️ui-contract.ts");
  rmSync(dir, { recursive: true, force: true });
  return manifest;
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const manifest = buildManifest(this.root);
    const outPath = generatedManifestPath(this.root);
    mkdirSync(join(this.root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated"), { recursive: true });
    writeFileSync(outPath, manifest);
    console.log(`framework typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🔎️ Rebuilds the manifest in memory and byte-compares it against `<owner>/🤖️generated/🟦️manifest.ts` — never writes that file (a lint/verify step must never let the auto-commit daemon land regenerated files). */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const manifest = buildManifest(this.root);
    const outPath = generatedManifestPath(this.root);
    if (!existsSync(outPath) || readFileSync(outPath, "utf8") !== manifest) {
      console.error(`framework typescript mirror is stale: ${outPath}`);
      console.error("run `bun nx run @semio-tech/framework:generate` to refresh.");
      process.exit(1);
    }
    console.log("framework typescript mirror is fresh.");
  }
}
//#endregion 🔖️Typegen

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("generate", GenerateScript).register("check", CheckScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
