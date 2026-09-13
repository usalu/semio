import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Qualifies native variant prerequisites and completed component publication through native Nx. */
export async function testNativeRuntime(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const root = mkdtempSync(join(output, "native-runtime-")), owner = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
  const put = (path: string, value: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), value); };
  for (const [path, name] of [["🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust", "surface"], ["🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust", "editor"], ["🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", "flow"], ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", "renderer"]]) put(join(path!, "📋️project.json"), JSON.stringify({ name, targets: { wasm: {}, "wasm-release": {} } }));
  put(join(owner, "package.json"), '{"name":"native-owner","private":true}');
  put("component/Cargo.toml", '[package]\nname = "sample"\nversion = "0.1.0"\n[package.metadata.component]\npackage = "semio:sample"\n[package.metadata.semio]\nrole = "plugin"\n[[package.metadata.semio.playground]]\nvariant = "fixture"\n'.replaceAll("\\n", "\n"));
  const { cacheInternals } = await import("../../../🟨️.mjs");
  const targets = cacheInternals.playgroundPreparationTargets(["component/Cargo.toml"], root, owner);
  for (const profile of fixture.profiles) {
    const prepare = targets[`prepare-${fixture.variant}-native-${profile}`];
    assert.ok(prepare, "Every native variant/profile needs an outer Nx preparation producer");
    assert.equal(prepare.cache, true);
    assert.deepEqual(prepare.outputs, [`{projectRoot}/dist/runtime/native/${profile}/${fixture.variant}`]);
    assert.deepEqual(prepare.dependsOn, [`@semio-tech/plugin-registry:session-${fixture.variant}`, `sample:materialize-${profile}`]);
    for (const operation of ["run", "smoke"]) {
      const target = targets[`${operation}-${fixture.variant}-native-${profile}`];
      assert.equal(target.cache, false);
      assert.equal(target.continuous, operation === "run");
      assert.deepEqual(target.dependsOn, [`prepare-${fixture.variant}-native-${profile}`, `renderer:native-build${profile === "release" ? "-release" : ""}`]);
    }
  }

  const native = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint"), publication = await import(join(native, "📦️modules/🟦️.ts"));
  const execution = await import(join(native, "📜️script.ts"));
  const ts = require("typescript"), launcher = ts.createSourceFile("launcher.ts", readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const declaration = launcher.statements.find((node: any) => node.name?.text === "resolveNxInvocation");
  const resolveInvocation = new Function("process", ts.transpileModule(declaration.getText(launcher), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText.replace("export ", "") + "; return resolveNxInvocation;")({ env: { SEMIO_PLUGIN: fixture.variant } });
  for (const profile of fixture.profiles) for (const operation of ["run", "smoke"]) {
    const selected = profile === "release" ? ["--release"] : [];
    if (operation === "smoke") selected.push("--smoke");
    assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", "--", fixture.variant, ...selected]).args, ["run", `@semio-tech/framework-os-dev:${operation}-${fixture.variant}-native-${profile}`]);
  }
  assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native"]).args, ["run", `@semio-tech/framework-os-dev:run-${fixture.variant}-native-dev`]);
  assert.throws(() => resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", "--", fixture.variant, "--unknown"]));
  assert.deepEqual(resolveInvocation(["run", `@semio-tech/framework-os-dev:run-${fixture.variant}-native-release`]).env, { SEMIO_PLUGIN: fixture.variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: "ship" });
  assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", "--", "--scale", "fixture.json", "--release"]).args, ["run", "@semio-tech/framework-renderer-wgpu:native-scale-release", "--", "--scale", "fixture.json"]);
  const script = readFileSync(join(native, "📜️script.ts"), "utf8"), old = readFileSync(join(native, "../📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  assert.ok(!/runCmdStatus|new NativeRunScript|\["nx", "run"/.test(script));
  assert.ok(!/class NativeRunScript|ensureAssetServer/.test(old));
  const capture = await require("esbuild").build({ entryPoints: [join(native, "📦️modules/📜️script.ts")], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  assert.ok(!Object.keys(capture.metafile.inputs).some(path => path.includes("🏗️compiler") || path.includes("styling/🏗️builder") || path.endsWith("📚️library/🟦️.ts")), "Native publication must not load application tooling or asset servers");
  const command = "bun ./📜️script.ts";
  const nxTargets: Record<string, any> = {};
  for (const profile of fixture.profiles) {
    nxTargets[`materialize-${profile}`] = { executor: "nx:run-commands", cache: true, inputs: [`{workspaceRoot}/source/${profile}/*`, "{workspaceRoot}/📜️script.ts"], outputs: [`{workspaceRoot}/compiled/${profile}`], options: { command: `${command} materialize ${profile}` } };
    nxTargets[`prepare-${profile}`] = { executor: "nx:run-commands", cache: true, inputs: ["{workspaceRoot}/📜️script.ts", { dependentTasksOutputFiles: "**/*" }], dependsOn: [`materialize-${profile}`], outputs: [`{workspaceRoot}/dist/runtime/native/${profile}/${fixture.variant}`], options: { command: `${command} publish ${profile}` } };
    nxTargets[`consume-${profile}`] = { executor: "nx:run-commands", cache: false, outputs: [], dependsOn: [`prepare-${profile}`], options: { command: `${command} run ${profile}` } };
    const bytes = Buffer.concat([Buffer.from(fixture.componentHex, "hex"), Buffer.from([0, 3, 1, 112, profile === "dev" ? 100 : 114])]);
    put(`source/${profile}/component.wasm`, "");
    writeFileSync(join(root, `source/${profile}/component.wasm`), bytes);
    put(`source/${profile}/descriptor.json`, JSON.stringify({ manifest: { pluginId: fixture.pluginId }, hashes: { wasmSha256: createHash("sha256").update(bytes).digest("hex") } }));
  }
  put("package.json", '{"name":"native-runtime-fixture","private":true,"type":"module"}');
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put("project.json", JSON.stringify({ name: "fixture", targets: nxTargets }));
  put(".gitignore", "node_modules\n.nx\ndist\n.counts\n*.pid\n.🧬semio\n");
  put("📜️script.ts", `import assert from "node:assert/strict";
import { appendFileSync, copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { join } from "node:path";
const [operation, profile] = process.argv.slice(2), root = process.cwd();
const fixture = ${JSON.stringify(fixture)};
if (operation === "consumer" || operation === "waiting") {
  const response = await fetch(process.env.SEMIO_ASSET_BASE_URL + "/asset");
  assert.equal(await response.text(), fixture.asset);
  assert.equal(process.env.S_USER, undefined);
  assert.equal(process.env.NPM_TOKEN, undefined);
  assert.equal(process.env.SEMIO_DIRECT_CHILD_BENIGN, "preserved");
  if (operation === "waiting") {
    const child = spawn(process.execPath, ["-e", "setInterval(() => {}, 1000)"], { stdio: "ignore" });
    writeFileSync(join(root, "child.pid"), JSON.stringify([process.pid, child.pid]));
    setInterval(() => {}, 1000);
  } else {
    const directory = join(root, "dist/runtime/native", profile, fixture.variant), manifest = JSON.parse(readFileSync(join(directory, "🔣️runtime.json"), "utf8")), module = manifest.modules.find(row => row.pluginId === fixture.pluginId);
    const bytes = readFileSync(join(directory, module.wasmPath)), descriptor = JSON.parse(readFileSync(join(directory, module.descriptorPath), "utf8"));
    assert.equal(bytes.subarray(0, 8).toString("hex"), fixture.componentHex);
    assert.equal(createHash("sha256").update(bytes).digest("hex"), descriptor.hashes.wasmSha256);
    console.log("[DEBUG] Native consumer fetched its live asset and read the restored component digest");
  }
} else {
  const { publishNativeRuntime } = await import(${JSON.stringify(join(native, "📦️modules/🟦️.ts"))});
  const { runNativeSession } = await import(${JSON.stringify(join(native, "📜️script.ts"))});
  if (operation === "materialize") {
    mkdirSync(join(root, "compiled", profile), { recursive: true });
    for (const name of ["component.wasm", "descriptor.json"]) copyFileSync(join(root, "source", profile, name), join(root, "compiled", profile, name));
  } else if (operation === "publish") {
    await publishNativeRuntime(root, fixture.variant, profile, [{ pluginId: fixture.pluginId, wasm: join(root, "compiled", profile, "component.wasm"), descriptor: join(root, "compiled", profile, "descriptor.json") }], new AbortController().signal);
    appendFileSync(join(root, ".counts"), profile + "\\n");
  } else {
    let server;
    await runNativeSession("node", [join(root, "📜️script.ts"), "consumer", profile], { ...process.env, S_USER: "poison", NPM_TOKEN: "poison" }, root, new AbortController().signal, () => {
      server = createServer((_request, response) => response.end(fixture.asset));
      server.listen(0, "127.0.0.1");
      return server;
    });
    assert.equal(server.listening, false);
  }
}
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const env = { ...process.env, SEMIO_REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") };
  const run = async (profile: string) => {
    const child = Bun.spawn(["node", cli, "run", `fixture:consume-${profile}`, "--outputStyle=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const cancel = () => child.kill("SIGTERM"), deadline = setTimeout(cancel, 60_000);
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      assert.equal(status, 0, stdout + stderr);
      assert.ok(stdout.includes("Native consumer fetched its live asset"));
    } finally { clearTimeout(deadline); process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  };
  const validateManifest = new (require("ajv"))().compile(JSON.parse(readFileSync(join(native, "📦️modules/🧬️schema/🔣️.json"), "utf8")));
  for (const profile of fixture.profiles) for (const cycle of fixture.cycles) {
    const runtime = publication.nativeRuntimeDirectory(root, fixture.variant, profile);
    if (cycle.remove) { rmSync(runtime, { recursive: true }); rmSync(join(root, "compiled", profile), { recursive: true }); }
    await run(profile);
    const manifest = JSON.parse(readFileSync(join(runtime, "🔣️runtime.json"), "utf8"));
    assert.equal(validateManifest(manifest), true, JSON.stringify(validateManifest.errors));
    assert.deepEqual(require("lodash").map(manifest.modules, "pluginId"), [fixture.pluginId]);
    assert.equal(readFileSync(join(root, ".counts"), "utf8").trim().split("\n").filter(value => value === profile).length, cycle.publications);
    assert.deepEqual(readdirSync(runtime).sort(), [".nx-artifact.json", "🔣️runtime.json"].sort(), "Variant runtimes must not duplicate shared component or descriptor bytes");
    assert.deepEqual(readFileSync(join(runtime, manifest.modules[0].wasmPath)), readFileSync(join(root, "source", profile, "component.wasm")));
  }
  const locked = require("@iarna/toml").parse(readFileSync(join(workspace, "Cargo.lock"), "utf8")).package;
  const version = (name: string) => locked.find((entry: any) => entry.name === name).version;
  put("reader/Cargo.toml", `[package]\nname = "native-runtime-reader"\nversion = "0.1.0"\nedition = "2024"\n[[bin]]\nname = "reader"\npath = "🦀️.rs"\n[dependencies]\nserde = { version = "=${version("serde")}", features = ["derive"] }\nserde_json = "=${version("serde_json")}"\nsha2 = "=0.10.9"\n[workspace]\n`);
  put("reader/🦀️.rs", "#[path = " + JSON.stringify(join(native, "📦️modules/🦀️.rs")) + "]\nmod native;\n" + "use std::{fs, io::Read, path::Path};\nuse sha2::{Digest, Sha256};\nfn main() {\n    let args = std::env::args().collect::<Vec<_>>();\n    let path = Path::new(&args[1]);\n    let bytes = fs::read(path).unwrap();\n    let manifest = match native::NativeRuntimeManifest::read(native::NativeJsonPages::new(bytes.chunks(3)), &args[2]) {\n        Ok(value) => value,\n        Err(_) => std::process::exit(2),\n    };\n    if args.get(3).is_some_and(|arg| arg == \"validate\") { return; }\n    let mut rows = Vec::new();\n    for module in manifest.modules {\n        let wasm = fs::read(path.parent().unwrap().join(module.wasm_path)).unwrap();\n        assert_eq!(&wasm[..8], &[0, 97, 115, 109, 13, 0, 1, 0]);\n        assert_eq!(format!(\"{:x}\", Sha256::digest(&wasm)), module.wasm_sha256);\n        let bytes = fs::read(path.parent().unwrap().join(module.descriptor_path)).unwrap();\n        let descriptor: serde_json::Value = serde_json::from_reader(native::NativeJsonPages::new(bytes.chunks(7))).unwrap();\n        assert_eq!(descriptor[\"manifest\"][\"pluginId\"], module.plugin_id);\n        assert_eq!(descriptor[\"hashes\"][\"wasmSha256\"], module.wasm_sha256);\n        rows.push(serde_json::json!({\"pluginId\": module.plugin_id, \"wasmSha256\": module.wasm_sha256}));\n    }\n    let pages: [&[u8]; 4] = [b\"\", b\"first\", b\"\", b\"second\"];\n    let mut reader = native::NativeJsonPages::new(pages.into_iter());\n    assert_eq!(reader.read(&mut []).unwrap(), 0);\n    let mut joined = Vec::new();\n    reader.read_to_end(&mut joined).unwrap();\n    assert_eq!(joined, b\"firstsecond\");\n    println!(\"{}\", serde_json::to_string(&rows).unwrap());\n}\n");
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const compile = new AbortController(), deadline = setTimeout(() => compile.abort(new Error("Native reader deadline")), 90_000);
  const cancelCompile = () => compile.abort(new Error("Native reader cancelled"));
  process.once("SIGINT", cancelCompile); process.once("SIGTERM", cancelCompile);
  const nativeEnv = { ...env, CARGO_TARGET_DIR: join(root, ".native-target"), CARGO_BUILD_BUILD_DIR: join(root, ".native-build"), CARGO_NET_OFFLINE: "true" };
  try {
    for (const args of [["generate-lockfile", "--offline"], ["build", "--locked", "--offline"]]) await runTool("cargo", [...args, "--manifest-path", join(root, "reader/Cargo.toml")], root, compile.signal, false, nativeEnv);
    const binary = join(root, ".native-target/debug", process.platform === "win32" ? "reader.exe" : "reader");
    for (const profile of fixture.profiles) {
      const path = join(publication.nativeRuntimeDirectory(root, fixture.variant, profile), "🔣️runtime.json");
      const manifest = JSON.parse(readFileSync(path, "utf8"));
      const rows = await runTool(binary, [path, fixture.variant], root, compile.signal, true, nativeEnv);
      assert.deepEqual(JSON.parse(rows), manifest.modules.map(({ pluginId, wasmSha256 }: any) => ({ pluginId, wasmSha256 })));
      await assert.rejects(() => runTool(binary, [path, "wrong-variant"], root, compile.signal, true, nativeEnv), /failed \(2\)/);
    }
    const path = join(root, "reader/manifest.json"), valid = JSON.parse(readFileSync(join(publication.nativeRuntimeDirectory(root, fixture.variant, "dev"), "🔣️runtime.json"), "utf8"));
    const validateNative = async (value: unknown, accepted: boolean) => {
      put("reader/manifest.json", typeof value === "string" ? value : JSON.stringify(value));
      const result = runTool(binary, [path, fixture.variant, "validate"], root, compile.signal, true, nativeEnv);
      if (accepted) await result; else await assert.rejects(result, /failed \(2\)/);
    };
    for (const entry of fixture.paths) {
      const value = { ...valid, modules: [{ ...valid.modules[0], wasmPath: entry.value }] };
      assert.equal(validateManifest(value), entry.valid, entry.value);
      await validateNative(value, entry.valid);
    }
    for (const value of [{ ...valid, version: 2 }, { ...valid, extra: true }, { ...valid, modules: [...valid.modules, ...valid.modules] }]) {
      assert.equal(validateManifest(value), false);
      await validateNative(value, false);
    }
    const text = JSON.stringify(valid), bytes = Buffer.byteLength(text);
    await validateNative(text + " ".repeat(fixture.manifestLimitBytes - bytes), true);
    await validateNative(text + " ".repeat(fixture.manifestLimitBytes - bytes + 1), false);
    await validateNative(text + " ".repeat(fixture.manifestLimitBytes - bytes) + "null", false);
    console.log("[DEBUG] Native Rust reader verified both restored profiles with Serde, SHA-256, portable paths, bounded JSON and fragmented payload pages");
  } finally { clearTimeout(deadline); process.removeListener("SIGINT", cancelCompile); process.removeListener("SIGTERM", cancelCompile); }
  put("source/unrelated.test.ts", "throw new Error('unrelated')");
  await run("dev");
  assert.equal(readFileSync(join(root, ".counts"), "utf8").trim().split("\n").filter(value => value === "dev").length, 1);
  const changed = Buffer.concat([Buffer.from(fixture.componentHex, "hex"), Buffer.from([0, 3, 1, 112, 120])]);
  writeFileSync(join(root, "source/dev/component.wasm"), changed);
  put("source/dev/descriptor.json", JSON.stringify({ manifest: { pluginId: fixture.pluginId }, hashes: { wasmSha256: createHash("sha256").update(changed).digest("hex") } }));
  await run("dev");
  await run("release");
  const counts = readFileSync(join(root, ".counts"), "utf8").trim().split("\n");
  assert.equal(counts.filter(value => value === "dev").length, 2);
  assert.equal(counts.filter(value => value === "release").length, 1);
  const before = readFileSync(join(root, "dist/runtime/native/dev", fixture.variant, "🔣️runtime.json"), "utf8");
  const source = { pluginId: fixture.pluginId, wasm: join(root, "source/dev/component.wasm"), descriptor: join(root, "source/dev/descriptor.json") };
  await assert.rejects(() => publication.publishNativeRuntime(root, fixture.variant, "dev", [source], AbortSignal.abort(new Error("cancelled"))), /cancelled/);
  writeFileSync(source.descriptor, JSON.stringify({ manifest: { pluginId: fixture.pluginId }, hashes: { wasmSha256: "0".repeat(64) } }));
  await assert.rejects(() => publication.publishNativeRuntime(root, fixture.variant, "dev", [source], new AbortController().signal), /digest mismatch/);
  writeFileSync(source.wasm, Buffer.from("0061736d01000000", "hex"));
  await assert.rejects(() => publication.publishNativeRuntime(root, fixture.variant, "dev", [source], new AbortController().signal), /component-model/);
  assert.equal(readFileSync(join(root, "dist/runtime/native/dev", fixture.variant, "🔣️runtime.json"), "utf8"), before);
  const controller = new AbortController();
  let server: ReturnType<typeof createServer> | undefined, port = 0;
  const running = execution.runNativeSession("node", [join(root, "📜️script.ts"), "waiting", "dev"], env, root, controller.signal, () => {
    server = createServer((_request, response) => response.end(fixture.asset));
    server.once("listening", () => { port = (server!.address() as any).port; });
    server.listen(0, "127.0.0.1");
    return server;
  });
  const settled = running.then(() => undefined, (error: unknown) => error);
  try {
    const deadline = Date.now() + 15_000;
    while (!existsSync(join(root, "child.pid")) && Date.now() < deadline) await new Promise(accept => setTimeout(accept, 25));
    assert.ok(existsSync(join(root, "child.pid")), "Native child must become ready while its parent serves HTTP");
  } finally { controller.abort(new Error("native cancelled")); }
  assert.match(String(await settled), /native cancelled/);
  assert.equal(server!.listening, false);
  const pids: number[] = JSON.parse(readFileSync(join(root, "child.pid"), "utf8"));
  const alive = (pid: number): boolean => { try { process.kill(pid, 0); return true; } catch (error: any) { if (error.code === "ESRCH") return false; throw error; } };
  const reapingDeadline = Date.now() + 5_000;
  while (pids.some(alive) && Date.now() < reapingDeadline) await new Promise(accept => setTimeout(accept, 25));
  assert.deepEqual(pids.filter(alive), [], "Cancelled native process tree must leave the OS process table");
  await assert.rejects(() => fetch(`http://127.0.0.1:${port}/asset`));
  rmSync(root, { recursive: true, force: true });
  console.log("[DEBUG] Native runtime graph, component identity, both-profile Nx reuse/restoration, responsive asset service, protected environment and owned cancellation PASS");

}
