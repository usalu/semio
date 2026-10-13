import assert from "node:assert/strict";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { mkdir, mkdtemp, rm, writeFile, symlink } from "node:fs/promises";
import { createRequire } from "node:module";
import { compilerStateDirectory, withCompilerState } from "../../🏗️state/🟦️.ts";
import { buildViteArtifact, serveVite } from "../../🌐️vite/🟦️.ts";
import { typecheckProject } from "../../🟦️typescript/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import { join } from "node:path";

/** 🧪️ Proves browser compiler policy and state ownership against real Vite and TypeScript. */
export async function proveWebBuild(workspace: string, output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️cases.json"), "utf8"));
  const { semioViteBuild } = await import("../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts");
  for (const row of fixture.modes) {
    const policy = semioViteBuild(undefined, row.mode);
    for (const key of ["minify", "cssMinify", "sourcemap"] as const) assert.equal(policy[key], row[key]);
  }
  for (const mode of fixture.invalidModes) assert.throws(() => semioViteBuild(undefined, mode));
  const stateSchema = JSON.parse(readFileSync(join(import.meta.dir, "../../🏗️state/🧬️schema/🔣️.json"), "utf8"));
  const require = createRequire(join(workspace, "package.json")), { resolveConfig } = await import("vite"), typescript = require("typescript");
  for (const row of fixture.modes) {
    const oracle = await resolveConfig({ configFile: false, build: semioViteBuild(undefined, row.mode) }, "build", row.mode);
    for (const key of ["minify", "cssMinify", "sourcemap"] as const) assert.equal(oracle.build[key], row[key]);
  }
  await mkdir(output, { recursive: true });
  const temporary = await mkdtemp(join(output, "web-")), app = join(temporary, "app"), stateRoot = repoCacheDirectory(workspace);
  await mkdir(app);
  await writeFile(join(app, "package.json"), '{"type":"module"}');
  await symlink(join(workspace, "node_modules"), join(app, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  await writeFile(join(app, "index.html"), '<!doctype html><script type="module" src="/entry.js"></script>');
  await writeFile(join(app, "entry.js"), fixture.source);
  const config = join(app, "🟦️.ts"), policyPath = join(workspace, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts");
  await writeFile(config, `import { semioViteBuild } from ${JSON.stringify(policyPath)}; export default { publicDir: false, logLevel: "silent", cacheDir: process.env.SEMIO_VITE_CACHE_DIRECTORY, build: semioViteBuild() };`);
  const options = { root: app, workspace, config, owner: fixture.owner, temporaryRoot: temporary };
  const sizes = new Map<string, number>();
  try {
    for (const row of fixture.modes) {
      const destination = join(temporary, row.mode), started = Date.now();
      await buildViteArtifact({ ...options, output: destination, mode: row.mode });
      const assets = readdirSync(join(destination, "assets")), js = assets.find(name => name.endsWith(".js"))!;
      assert.ok(js); assert.equal(assets.some(name => name.endsWith(".js.map")), row.sourcemap);
      sizes.set(row.mode, readFileSync(join(destination, "assets", js)).length);
      console.log(`[DEBUG] Vite ${row.mode}: bytes=${sizes.get(row.mode)} elapsedMs=${Date.now() - started}`);
    }
    assert.ok(sizes.get("release")! < sizes.get("prototype")!);
    await Promise.all([0, 1].map(() => buildViteArtifact({ ...options, output: join(temporary, "prototype"), mode: "prototype" })));
    assert.ok(existsSync(join(temporary, "prototype", ".nx-artifact.json")));
    const serving = [new AbortController(), new AbortController()], addresses: string[] = [], listeners: Promise<void>[] = [];
    try {
      await Promise.all(serving.map((controller, index) => new Promise<void>((accept, reject) => {
        const running = serveVite({ root: app, config, mode: "prototype", temporaryRoot: temporary, host: "127.0.0.1", port: 0, signal: controller.signal, ready: url => { addresses[index] = url; accept(); } });
        running.catch(reject); listeners.push(running);
      })));
      assert.notEqual(addresses[0], addresses[1]);
      for (const address of addresses) assert.equal((await fetch(address)).status, 200);
    } finally { serving.forEach(controller => controller.abort()); await Promise.all(listeners); }
    const warmService = new AbortController();
    let warmed!: Promise<void>;
    await new Promise<void>((accept, reject) => {
      warmed = serveVite({ root: app, config, mode: "prototype", temporaryRoot: temporary, host: "127.0.0.1", port: 0, signal: warmService.signal, ready: async url => { assert.equal((await fetch(url)).status, 200); accept(); } });
      warmed.catch(reject);
    });
    warmService.abort(); await warmed;
    console.log("[DEBUG] Concurrent isolated Vite listeners and warm optimizer snapshot publication verified");
    const state = { root: stateRoot, owner: fixture.owner, compiler: "test:1", configuration: config, mode: "prototype" as const };
    assert.equal(require("jsonschema").validate(state, stateSchema).valid, true);
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, mode: "release" }));
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, owner: "fixture:other" }));
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, compiler: "test:2" }));
    let active = 0, maximum = 0;
    await Promise.all([0, 1].map(() => withCompilerState({ ...state, signal: new AbortController().signal }, async directory => {
      active++; maximum = Math.max(maximum, active);
      await writeFile(join(directory, "probe"), String(active));
      await new Promise(resolve => setTimeout(resolve, 25)); active--;
    })));
    assert.equal(maximum, 1);
    const cancelled = new AbortController(); cancelled.abort();
    await assert.rejects(() => withCompilerState({ ...state, signal: cancelled.signal }, async () => assert.fail("Cancelled compiler admitted")));
    const project = join(app, "tsconfig.json");
    await writeFile(project, JSON.stringify({ compilerOptions: { target: "ES2022", module: "ESNext", types: [], noEmit: true, strict: true, skipLibCheck: true }, files: ["value.ts"] }));
    await writeFile(join(app, "value.ts"), 'export const value: number = 42;');
    const parsed = typescript.getParsedCommandLineOfConfigFile(project, {}, { ...typescript.sys, onUnRecoverableConfigFileDiagnostic: (diagnostic: unknown) => assert.fail(JSON.stringify(diagnostic)) });
    assert.deepEqual(parsed.fileNames, [join(app, "value.ts")]);
    const diagnostics: string[][] = [];
    for (const label of ["cold", "warm"]) {
      const lines: string[] = [], started = Date.now();
      await typecheckProject({ workspace, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000, args: ["--extendedDiagnostics"], execution: { onLine: line => lines.push(line) } });
      diagnostics.push(lines);
      console.log(`[DEBUG] TypeScript ${label}: elapsedMs=${Date.now() - started} ${lines.filter(line => /Check time|Total time/.test(line)).join("; ")}`);
    }
    assert.ok(diagnostics[0].some(line => /^Check time:/.test(line)));
    assert.equal(diagnostics[1].some(line => /^Check time:/.test(line)), false);
    await Promise.all([0, 1].map(() => typecheckProject({ workspace, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000 })));
    await typecheckProject({ workspace, cwd: app, project, owner: "fixture:web-typecheck", mode: "release", timeoutMs: 30_000 });
    await writeFile(join(app, "value.ts"), 'export const value: number = "bad";');
    await assert.rejects(() => typecheckProject({ workspace, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000 }));
    console.log("[DEBUG] Browser prototype/release artifacts, concurrent publication, cancellation, TypeScript warm reuse and error invalidation verified");
  } finally { await rm(temporary, { recursive: true, force: true }); }

}
