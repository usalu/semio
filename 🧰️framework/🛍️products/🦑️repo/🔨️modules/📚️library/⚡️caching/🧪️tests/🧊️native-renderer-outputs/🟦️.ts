import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Restores the production native renderer publisher's outputs with Nx and executes them against Cargo. */
export async function testNativeRendererOutputs(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const rust = fixture.owner + "/📦️packages/🦀️rust", typescript = fixture.owner + "/📦️packages/🟦️typescript";
  const project = JSON.parse(readFileSync(join(workspace, typescript, "📋️project.json"), "utf8"));
  assert.equal(project.name, fixture.project);
  for (const row of fixture.profiles) {
    assert.equal(project.targets[row.target].cache, true);
    assert.deepEqual(project.targets[row.target].outputs, [`{workspaceRoot}/${rust}/${row.output}`], "Nx must restore the Rust package output consumed by the native runner");
  }
  const root = mkdtempSync(join(output, "native-renderer-outputs-"));
  const put = (path: string, text: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
  const targets: Record<string, unknown> = {};
  for (const row of fixture.profiles) targets[row.target] = {
    executor: "nx:run-commands", cache: true, parallelism: false,
    inputs: [`{workspaceRoot}/${rust}/Cargo.toml`, `{workspaceRoot}/${rust}/Cargo.lock`, `{workspaceRoot}/${rust}/🦀️.rs`, "{workspaceRoot}/📜️script.ts"],
    outputs: project.targets[row.target].outputs,
    options: { command: `bun ./📜️script.ts ${row.profile}`, cwd: "." }
  };
  put("package.json", JSON.stringify({ name: "native-renderer-output-fixture", private: true }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(`${typescript}/project.json`, JSON.stringify({ name: "native-renderer", targets }));
  put(".gitignore", "node_modules\n.nx\ndist\noracle\n.runs\n");
  put(`${rust}/Cargo.toml`, `[workspace]\n[package]\nname=${JSON.stringify(fixture.crate)}\nversion="0.0.0"\nedition="2021"\n[features]\nnative-bin=[]\n[[bin]]\nname=${JSON.stringify(fixture.binary)}\npath="🦀️.rs"\nrequired-features=["native-bin"]\n`);
  put(`${rust}/Cargo.lock`, `version=4\n[[package]]\nname=${JSON.stringify(fixture.crate)}\nversion="0.0.0"\n`);
  const source = (stdout: string) => `fn main() { print!(${JSON.stringify(stdout)}); }\n`;
  put(`${rust}/🦀️.rs`, source(fixture.stdout));
  put("📜️script.ts", `import { appendFileSync } from "node:fs";
import { join } from "node:path";
import { buildNativeRenderer } from ${JSON.stringify(join(workspace, fixture.owner, "🏗️compiler/🦀️native/📜️script.ts"))};
const root = process.cwd(), profile = process.argv[2];
await buildNativeRenderer(join(root, ${JSON.stringify(rust)}), profile, root);
appendFileSync(join(root, ".runs"), profile + "\\n");
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_REPO_ROOT: root, CARGO_NET_OFFLINE: "true" };
  const execute = async (args: string[]) => {
    const child = Bun.spawn(args, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr); return stdout;
  };
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const run = () => execute(["node", cli, "run-many", "--projects=native-renderer", "--targets=native-build,native-build-release", "--outputStyle=static"]);
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").sort();
  const binary = fixture.binary + (process.platform === "win32" ? ".exe" : "");
  const check = async (expected: string) => {
    const oracle = await execute(["cargo", "run", "--locked", "--offline", "--manifest-path", join(root, rust, "Cargo.toml"), "--features", "native-bin", "--bin", fixture.binary, "--target-dir", join(root, "oracle")]);
    assert.equal(oracle, expected);
    for (const row of fixture.profiles) {
      assert.equal(await execute([join(root, rust, row.output, binary)]), oracle);
      assert.ok(readdirSync(join(root, rust, row.output)).includes(".nx-artifact.json"));
    }
  };
  await run(); assert.deepEqual(runs(), ["dev", "release"]); await check(fixture.stdout);
  const bytes = fixture.profiles.map((row: { output: string }) => readFileSync(join(root, rust, row.output, binary)));
  await run(); assert.deepEqual(runs(), ["dev", "release"]);
  for (const row of fixture.profiles) rmSync(join(root, rust, row.output), { recursive: true });
  await run(); assert.deepEqual(runs(), ["dev", "release"]);
  for (const [index, row] of fixture.profiles.entries()) assert.deepEqual(readFileSync(join(root, rust, row.output, binary)), bytes[index]);
  await check(fixture.stdout);
  put(`${rust}/🦀️.rs`, source(fixture.changedStdout));
  await run(); assert.deepEqual(runs(), ["dev", "dev", "release", "release"]); await check(fixture.changedStdout);
  console.log("[DEBUG] Native renderer dev/release outputs match Cargo, retain executable bytes and permissions through Nx restoration, and rebuild after source changes PASS");
}
