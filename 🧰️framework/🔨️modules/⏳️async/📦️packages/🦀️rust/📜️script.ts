#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-async` task router: `bun ./📜️script.ts <test|typegen>`. */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmdStatus, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

//#region 🦀️Checks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}
//#endregion 🦀️Checks

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-async"], this.repoRoot, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the versioned owned-schema export test in `🦀️component.rs`. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 🎯️ The mirror lives at `<owner>/🤖️generated/🟦️async.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "🟦️async.ts");
}

function runTypegenExportTest(root: string, outPath: string): void {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: root, env, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.error("framework-async typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

class TypegenScript extends BundleScript {
  run(): void {
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    for (const name of readdirSync(dirname(outPath))) if (name !== basename(outPath)) rmSync(join(dirname(outPath), name), { recursive: true, force: true });
    console.log(`framework-async typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact exporter against isolated output/target directories and emits only canonical JSON. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const targetPath = generatedBindingsPath(this.root);
    const temp = mkdtempSync(join(tmpdir(), "semio-async-typegen-"));
    let content: Buffer;
    try {
      const outPath = join(temp, basename(targetPath));
      const result = Bun.spawnSync(["cargo", "test", "--locked", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: this.root, env: { ...process.env, CARGO_TARGET_DIR: join(temp, "target"), SEMIO_TYPEGEN_OUT: outPath }, stderr: "pipe", stdout: "pipe" });
      if (result.exitCode !== 0) throw new Error(`framework-async preview export failed: ${result.stderr.toString()}`);
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
    process.stdout.write(`${JSON.stringify({ contractId: "async-typegen", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}
//#endregion 🔖️Typegen

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
