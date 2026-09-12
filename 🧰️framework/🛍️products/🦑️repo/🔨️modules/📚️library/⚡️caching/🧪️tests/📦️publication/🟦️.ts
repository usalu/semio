import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
import { runInNewContext } from "node:vm";
import { stageArtifacts } from "../../📦️artifacts/🟦️.ts";
import { acquireResourceLease } from "../../🔒️leases/🟦️.ts";
import { buildCargoArtifacts } from "../../🦀️cargo/📜️script.ts";

/** 📦️ Verifies contended publication, cancellation and complete output against Python file reads. */
export async function testArtifactPublication(output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/artifact-publication/🔣️.json"), "utf8"));
  const root = mkdtempSync(join(output, "artifact-publication-")), staging = join(root, "dist"), leaseDirectory = join(root, "leases");
  const files = new Map<string, string>();
  for (const [name, text] of Object.entries(fixture.files)) {
    const path = join(root, "input", name); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, String(text)); files.set(name, path);
  }
  const controller = new AbortController();
  const lease = await acquireResourceLease({ directory: leaseDirectory, resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal: controller.signal });
  try {
    let published = false, waits = 0;
    const cancelled = new AbortController();
    const rejected = stageArtifacts(staging, fixture.owner, files, { leaseDirectory, signal: cancelled.signal, onWait: () => cancelled.abort(new Error(fixture.cancelReason)) });
    await assert.rejects(() => rejected, (error: Error) => error.name === "AbortError" || error.message === fixture.cancelReason);
    const pending = stageArtifacts(staging, fixture.owner, files, { leaseDirectory, onWait: () => { waits++; } }).then(() => { published = true; });
    await Bun.sleep(80);
    assert.equal(published, false, "Concurrent publication must wait for the active publisher");
    assert.ok(waits > 0, "Waiting must report progress");
    lease.release();
    await pending;
    const program = "import json,pathlib,sys\np=pathlib.Path(sys.argv[1]);print(json.dumps({str(f.relative_to(p)).replace('\\\\','/'):f.read_text() for f in p.rglob('*') if f.is_file() and f.name!='.nx-artifact.json'}))";
    const oracle = Bun.spawnSync([Bun.which("python3") ?? "python", "-c", program, staging], { stdout: "pipe", stderr: "pipe" });
    assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
    assert.deepEqual(JSON.parse(oracle.stdout.toString()), fixture.files);
    await assert.rejects(() => stageArtifacts(staging, "foreign", files, { leaseDirectory }), /Unowned/);
    await assert.rejects(() => stageArtifacts(staging, fixture.owner, new Map([["../escape", files.values().next().value!]]), { leaseDirectory }), /Invalid artifact/);
    assert.deepEqual(readdirSync(staging).sort(), [".nx-artifact.json", "nested", "support.js"]);
    console.log("[DEBUG] Artifact publication waits, cancels, preserves ownership and matches Python file content PASS");
  } finally { lease.release(); rmSync(root, { recursive: true, force: true }); }
  await testCargoArtifactPublication(output);
  testWasmArtifactPublication(output);
}

/** 🕸️ Checks private compiler output retirement on wasm-pack success, failure and missing output. */
function testWasmArtifactPublication(output: string): void {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/artifact-publication/🔣️.json"), "utf8"));
  const ts = createRequire(import.meta.url)("typescript");
  const source = ts.createSourceFile("library.ts", readFileSync(resolve(import.meta.dirname, "../../../🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const definition = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "runWasmPackWebBuild");
  const code = ts.transpileModule(definition.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  for (const row of fixture.wasm) {
    const root = mkdtempSync(join(output, "wasm-publication-"));
    let target: string | undefined;
    try {
      const run = runInNewContext(`${code}; runWasmPackWebBuild;`, {
        join, mkdirSync, mkdtempSync, existsSync, rmSync, writeFileSync, Date, console, process: { execPath: process.execPath, env: {}, exit: (status: number) => { throw new Error(`Compiler exited ${status}`); } },
        wasmOutputDirectory: (directory: string, name: string) => join(directory, name), semioBuildMode: () => "dev", wasmBuildArguments: () => ({ pack: ["--dev"], cargo: ["--profile", "dev"] }), cargoProfileDir: () => "debug",
        wasmBuildEnvironment: () => ({}), getWorkspaceRoot: () => root, resolveWasmBindgenBin: () => "wasm-bindgen", wasmPackEnvironment: (_root: string, _bindgen: string, env: unknown) => env, resolveWorkspaceBin: () => undefined, buildBudgetMs: () => 1000,
        runCmdStatus: (_command: string, _args: string[], options: { env: NodeJS.ProcessEnv }) => {
          target = options.env.CARGO_TARGET_DIR;
          assert.ok(target?.startsWith(join(root, "dist")), "WASM compilers need private final outputs and shared intermediates");
          mkdirSync(target!, { recursive: true });
          if (row.output) { mkdirSync(join(root, "pkg")); writeFileSync(join(root, "pkg/map_bg.wasm"), new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0])); }
          return row.status;
        },
        wasmPackSnippetFiles: () => [], statSync: () => ({ size: 8 }),
      });
      const invoke = () => run({ rsDir: root, logPrefix: "fixture", pkg: { files: [] }, wasmBaseName: "map" });
      if (row.output) { invoke(); assert.ok(WebAssembly.validate(readFileSync(join(root, "pkg/map_bg.wasm")))); }
      else assert.throws(invoke, /(?:failed|missing|exited)/);
      assert.ok(target && !existsSync(target), "Compiler output must retire for every terminal outcome");
    } finally { rmSync(root, { recursive: true, force: true }); }
  }
  console.log("[DEBUG] WASM compiler output retires on success, failure and missing output PASS");
}

/** 🦀️ Compares a captured executable with Cargo while retaining shared intermediates and retiring private outputs. */
async function testCargoArtifactPublication(output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/artifact-publication/🔣️.json"), "utf8")).cargo;
  const root = mkdtempSync(join(output, "cargo-publication-"));
  try {
    mkdirSync(join(root, ".cargo"));
    writeFileSync(join(root, ".cargo/config.toml"), '[build]\ntarget-dir="shared-output"\nbuild-dir="build-cache"\n');
    writeFileSync(join(root, "Cargo.toml"), `[workspace]\n[package]\nname=${JSON.stringify(fixture.name)}\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname=${JSON.stringify(fixture.name)}\npath="main.rs"\n`);
    writeFileSync(join(root, "main.rs"), `fn main() { print!(${JSON.stringify(fixture.stdout)}); }\n`);
    const oracle = Bun.spawn(["cargo", "build", "--offline", "--target-dir", join(root, "oracle")], { cwd: root, stdout: "pipe", stderr: "pipe" });
    const [status, diagnostic] = await Promise.all([oracle.exited, new Response(oracle.stderr).text()]);
    assert.equal(status, 0, diagnostic);
    const executable = fixture.name + (process.platform === "win32" ? ".exe" : "");
    const expected = Bun.spawnSync([join(root, "oracle/debug", executable)], { stdout: "pipe", stderr: "pipe" });
    assert.equal(expected.exitCode, 0, expected.stderr.toString());
    assert.equal(expected.stdout.toString(), fixture.stdout);
    await buildCargoArtifacts("Cargo.toml", ["--bin", fixture.name], root);
    assert.equal(existsSync(join(root, "shared-output/debug", executable)), false, "Captured builds must not contend for the workspace's uplifted artifacts");
    assert.ok(existsSync(join(root, "build-cache")), "Compiler intermediates must remain shared");
    const delivered = Bun.spawnSync([join(root, "dist/build", executable)], { stdout: "pipe", stderr: "pipe" });
    assert.equal(delivered.exitCode, 0, delivered.stderr.toString());
    assert.deepEqual(delivered.stdout, expected.stdout);
    assert.deepEqual(readdirSync(join(root, "dist")), ["build"], "Private Cargo outputs must retire after publication");
    console.log("[DEBUG] Private Cargo publication matches native Cargo and retains shared compiler intermediates PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
