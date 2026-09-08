#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os-shell-rs` task router: `bun ./📜️script.ts <check|test|typegen|schema-check|preview-generated>`. */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmdStatus, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-shell"], this.repoRoot, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ The `🧬️schema` module is the ONLY source of truth for the mirror: `🔣️.json` is the
 * language-neutral authority and `🧬️schema/🦀️.rs`'s `schema_registry` is its Rust registry. This
 * test renders that registry; nothing in ordinary module code feeds it. */
const TYPEGEN_TEST_FILTER = "schema::tests::exports_typescript_bindings";

/** 🧬️ The `$defs`/registry agreement gate, in the schema module's own tests. */
const SCHEMA_GATE_TEST_FILTER = "schema::tests::owned_json_schema_defs_match_registry";

/** 🧬️ The owner module root — `<owner>/📦️packages/🦀️rust` is where this script runs. */
function ownerRoot(root: string): string {
  return join(root, "..", "..");
}

/** 🧬️ The scope this generator reads from: `<owner>/🧬️schema`. */
function schemaModulePath(root: string, ...segments: string[]): string {
  return join(ownerRoot(root), "🧬️schema", ...segments);
}

/** 🎯️ The mirror this generator writes: `<owner>/🤖️generated/🟦️.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(ownerRoot(root), "🤖️generated", "🟦️.ts");
}

/** 🪪️ The `$id` `🧬️schema/🔣️.json` must declare, mirrored from `schema_registry::OWNED_JSON_SCHEMA_ID`. */
const OWNED_JSON_SCHEMA_ID = "https://semio.tech/schema/os/shell/component.json";
const OWNED_JSON_SCHEMA_DIALECT = "http://json-schema.org/draft-07/schema#";

function runTypegenExportTest(root: string, outPath: string): void {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: root, env, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.error("framework-os-shell typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

/** 🧬️ Renders `<owner>/🤖️generated/🟦️.ts` from `<owner>/🧬️schema` — the schema module's Rust
 * registry, gated against its own `🔣️.json` before a single byte is written. */
class TypegenScript extends BundleScript {
  run(): void {
    assertSchemaModulePresent(this.root);
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    for (const name of readdirSync(dirname(outPath))) if (name !== basename(outPath)) rmSync(join(dirname(outPath), name), { recursive: true, force: true });
    console.log(`framework-os-shell typescript mirror refreshed from 🧬️schema -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact exporter against isolated output/target directories and emits only canonical JSON. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const targetPath = generatedBindingsPath(this.root);
    const temp = mkdtempSync(join(tmpdir(), "semio-shell-typegen-"));
    let content: Buffer;
    try {
      const outPath = join(temp, basename(targetPath));
      const result = Bun.spawnSync(["cargo", "test", "--locked", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: this.root, env: { ...process.env, CARGO_TARGET_DIR: join(temp, "target"), SEMIO_TYPEGEN_OUT: outPath }, stderr: "pipe", stdout: "pipe" });
      if (result.exitCode !== 0) throw new Error(`framework-os-shell preview export failed: ${result.stderr.toString()}`);
      content = readFileSync(outPath);
    } finally {
      rmSync(temp, { recursive: true, force: true });
    }
    const rootPath = relative(this.repoRoot, dirname(targetPath)).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      { bytesBase64: content.toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/${basename(targetPath).normalize("NFC")}` },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const staleRemovals = (existsSync(dirname(targetPath)) ? readdirSync(dirname(targetPath)) : []).filter((name) => name !== basename(targetPath)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "shell-typegen", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}
//#endregion 🔖️Typegen

//#region 🧬️SchemaCheck
/** 🧬️ Reads `🔣️.json` and fails loudly if the schema module is absent or off-dialect. */
function readOwnedJsonSchema(root: string): { $schema?: string; $id?: string; $defs?: Record<string, unknown> } {
  const path = schemaModulePath(root, "🔣️.json");
  if (!existsSync(path)) throw new Error(`framework-os-shell: the 🧬️schema module is missing its authority document at ${path}`);
  return JSON.parse(readFileSync(path, "utf8")) as { $schema?: string; $id?: string; $defs?: Record<string, unknown> };
}

function assertSchemaModulePresent(root: string): void {
  for (const name of ["🔣️.json", "🦀️.rs", "🟦️.ts"]) {
    const path = schemaModulePath(root, name);
    if (!existsSync(path)) throw new Error(`framework-os-shell: the 🧬️schema module is missing ${name} at ${path}`);
  }
}

/** 🧾️ Every `SchemaExport { name: "…" }` row the schema module's Rust registry declares. */
function rustRegistryExportIds(root: string): string[] {
  const source = readFileSync(schemaModulePath(root, "🦀️.rs"), "utf8");
  return [...source.matchAll(/SchemaExport \{\s*\n?\s*name: "([A-Za-z0-9_]+)"/gu)].map((match) => match[1]!);
}

/** 🟦️ Every `export type X` the rendered mirror declares. */
function renderedMirrorExportIds(root: string): string[] {
  const path = generatedBindingsPath(root);
  if (!existsSync(path)) throw new Error(`framework-os-shell: the rendered mirror is missing at ${path} — run \`typegen\` first.`);
  return [...readFileSync(path, "utf8").matchAll(/^export type ([A-Za-z0-9_]+)/gmu)].map((match) => match[1]!);
}

/** 🧬️ Every `parse<ExportId>` the schema module's TypeScript face exports. */
function typescriptParserExportIds(root: string): string[] {
  return [...readFileSync(schemaModulePath(root, "🟦️.ts"), "utf8").matchAll(/^export const parse([A-Za-z0-9_]+) = defineParser</gmu)].map((match) => match[1]!);
}

function reportSetDifference(label: string, expected: string[], actual: string[]): string[] {
  const missing = expected.filter((name) => !actual.includes(name));
  const extra = actual.filter((name) => !expected.includes(name));
  const problems: string[] = [];
  if (missing.length > 0) problems.push(`${label} is missing ${JSON.stringify(missing)}`);
  if (extra.length > 0) problems.push(`${label} declares unknown ${JSON.stringify(extra)}`);
  return problems;
}

/** 🧬️ `🔣️.json`'s `$defs` key set and the Rust registry's exported type set MUST agree — checked
 * here across all three projections (authority, Rust registry, rendered mirror, TypeScript face)
 * and then again inside Rust itself, where the registry is the live value rather than source text. */
class SchemaCheckScript extends BundleScript {
  run(): void {
    assertSchemaModulePresent(this.root);
    const document = readOwnedJsonSchema(this.root);
    const problems: string[] = [];
    if (document.$schema !== OWNED_JSON_SCHEMA_DIALECT) problems.push(`🔣️.json declares dialect '${document.$schema}', expected '${OWNED_JSON_SCHEMA_DIALECT}'`);
    if (document.$id !== OWNED_JSON_SCHEMA_ID) problems.push(`🔣️.json declares $id '${document.$id}', expected '${OWNED_JSON_SCHEMA_ID}'`);
    const declared = Object.keys(document.$defs ?? {}).sort();
    if (declared.length === 0) problems.push("🔣️.json declares no `$defs` exports");
    problems.push(...reportSetDifference("the Rust registry in 🧬️schema/🦀️.rs", declared, rustRegistryExportIds(this.root).sort()));
    problems.push(...reportSetDifference("the rendered mirror 🤖️generated/🟦️.ts", declared, renderedMirrorExportIds(this.root).sort()));
    problems.push(...reportSetDifference("the parsers in 🧬️schema/🟦️.ts", declared, typescriptParserExportIds(this.root).sort()));
    if (problems.length > 0) {
      for (const problem of problems) console.error(`framework-os-shell schema-check: ${problem}`);
      process.exit(1);
    }
    const status = runCmdStatus("cargo", ["test", "--features", "typegen", SCHEMA_GATE_TEST_FILTER], { cwd: this.root, budgetMs: buildBudgetMs() });
    if (status !== 0) {
      console.error("framework-os-shell schema-check: the Rust `$defs`/registry gate failed — see output above.");
      process.exit(status);
    }
    console.log(`framework-os-shell schema-check: ${declared.length} 🔣️.json \`$defs\` exports agree with the Rust registry, the rendered mirror and 🧬️schema/🟦️.ts.`);
  }
}
//#endregion 🧬️SchemaCheck

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("typegen", TypegenScript).register("schema-check", SchemaCheckScript).register("preview-generated", PreviewGeneratedScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
