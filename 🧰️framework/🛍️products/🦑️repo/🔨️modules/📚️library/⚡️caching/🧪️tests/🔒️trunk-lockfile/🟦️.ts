import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";

/** 🔒️ Keeps Trunk's build/serve Cargo invocation on the committed dependency lock. */
export async function testTrunkLockfile(workspace: string, native = false): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔒️trunk-lockfile/🔣️.json"), "utf8"));
  for (const path of fixture.configs) {
    const source = readFileSync(join(workspace, path), "utf8"), parsed = Bun.TOML.parse(source) as { build: Record<string, unknown>; hooks?: { stage: string; command: string; command_arguments: string[] }[] };
    assert.deepEqual(parsed, require("smol-toml").parse(source));
    for (const [key, value] of Object.entries(fixture.build)) assert.equal(parsed.build[key], value, `${path}: build.${key}`);
    const hook = parsed.hooks?.find((value) => value.stage === fixture.hook.stage);
    assert.ok(hook, `${path}: locked Cargo metadata must precede every Trunk pipeline`);
    assert.equal(hook.command, fixture.hook.command);
    assert.deepEqual(hook.command_arguments.slice(1, -1), fixture.hook.command_arguments);
    assert.equal(resolve(dirname(join(workspace, path)), hook.command_arguments[0]), resolve(import.meta.dir, "../../🦀️cargo/📜️script.ts"));
    assert.equal(resolve(workspace, hook.command_arguments.at(-1)!), join(dirname(join(workspace, path)), "Cargo.toml"));
    const project = JSON.parse(readFileSync(join(dirname(join(workspace, path)), "📋️project.json"), "utf8")), target = project.targets[fixture.target];
    assert.equal(project.name, fixture.project);
    assert.equal(target.cache, false);
    assert.deepEqual(target.outputs, []);
    assert.ok(target.options.command.includes("native cargo metadata --manifest"));
    const { cacheInternals } = await import("../../../🟨️.mjs");
    const projectRoot = fixture.configs[0].slice(0, -"/Trunk.toml".length);
    const prepared = cacheInternals.withNativePreparation({ ...structuredClone(project), root: projectRoot }, workspace, { fixture: { ownership: "owned", target: "fixture:generate", nativeConsumers: [projectRoot] } });
    assert.deepEqual(prepared.targets[fixture.target].dependsOn ?? [], [], "Cargo metadata must not schedule native source generators");
    for (const file of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const editor = Bun.JSONC.parse(readFileSync(join(workspace, file), "utf8"));
      assert.equal(editor.configurations.filter((value: any) => value.command === `bun nx run ${fixture.project}:${fixture.target}`).length, 1, file);
    }
    if (native) {
      const env = { ...process.env }; delete env.NO_COLOR; delete env.FORCE_COLOR;
      const result = spawnSync("trunk", ["config", "--config", join(workspace, path), "--skip-version-check", "show"], { cwd: workspace, env, encoding: "utf8", timeout: 15000 });
      assert.equal(result.status, 0, result.stderr);
      assert.match(result.stdout, /\bbuild: Build \{[\s\S]*?\blocked: true,/);
    }
  }
  console.log(`[DEBUG] Trunk locked metadata hook matches Bun/smol-toml${native ? " and native Trunk config" : ""} PASS`);
}

/** 🔬️ Proves stale-lock rejection before compilation, including subsequent native watch rebuilds. */
export async function testNativeTrunkLockfile(workspace: string, generated: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔒️trunk-lockfile/🔣️.json"), "utf8")), root = mkdtempSync(join(generated, "trunk-lockfile-"));
  const config = Bun.TOML.parse(readFileSync(join(workspace, fixture.configs[0]), "utf8")) as any;
  const hook = config.hooks.find((value: any) => value.stage === fixture.hook.stage);
  const args = [...hook.command_arguments];
  args[0] = resolve(dirname(join(workspace, fixture.configs[0])), args[0]);
  args[args.length - 1] = join(root, "Cargo.toml");
  for (const [path, contents] of Object.entries(fixture.probe.files)) writeFileSync(join(root, path), contents as string);
  writeFileSync(join(root, "Trunk.toml"), fixture.probe.files["Trunk.toml"] + `\n[[hooks]]\nstage=${JSON.stringify(hook.stage)}\ncommand=${JSON.stringify(hook.command)}\ncommand_arguments=${JSON.stringify(args)}\n`);
  const env = { ...process.env, CARGO_TARGET_DIR: join(root, "target") }; delete env.NO_COLOR; delete env.FORCE_COLOR;
  const trunkArgs = ["--config", join(root, "Trunk.toml"), "--skip-version-check", "--log", "info", "--color", "never"];
  const result = spawnSync("trunk", ["build", ...trunkArgs], { cwd: root, env, encoding: "utf8", timeout: 15000 });
  writeFileSync(join(root, "stale-build.log"), result.stdout + result.stderr);
  assert.equal(result.status, fixture.probe.expectedStatus, (result.stdout + result.stderr).slice(-3000));
  assert.ok((result.stdout + result.stderr).includes(fixture.probe.diagnostic));
  assert.equal(readFileSync(join(root, "Cargo.lock"), "utf8"), fixture.probe.files["Cargo.lock"]);
  writeFileSync(join(root, "Cargo.lock"), fixture.probe.validLock);
  const oracle = spawnSync("cargo", ["metadata", "--locked", "--format-version=1", "--manifest-path", join(root, "Cargo.toml")], { cwd: root, env, encoding: "utf8", timeout: 15000 });
  assert.equal(oracle.status, 0, oracle.stderr);
  assert.equal(JSON.parse(oracle.stdout).packages[0].name, "trunk-lock-fixture");
  const watch = spawn("trunk", ["watch", ...trunkArgs], { cwd: root, env, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
  let output = "", exited = false;
  watch.stdout.on("data", (bytes) => { output += bytes; }); watch.stderr.on("data", (bytes) => { output += bytes; });
  const closed = new Promise<void>((accept) => { watch.once("close", () => { exited = true; accept(); }); watch.once("error", (error) => { output += error.message; }); });
  const stop = (): void => {
    if (!watch.pid || exited) return;
    if (process.platform === "win32") spawnSync("taskkill", ["/pid", String(watch.pid), "/t", "/f"], { stdio: "ignore" });
    else try { process.kill(-watch.pid, "SIGTERM"); } catch {}
  };
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const until = async (pattern: string, offset = 0): Promise<void> => {
    const start = Date.now();
    while (!output.slice(offset).includes(pattern)) {
      assert.ok(!exited && Date.now() - start < 30000, `Trunk did not emit ${pattern}: ${output.slice(-3000)}`);
      await new Promise((accept) => setTimeout(accept, 100));
    }
  };
  const progress = setInterval(() => console.log("[DEBUG] Native Trunk watch lockfile probe running"), 10000);
  try {
    await until("success");
    const files = readdirSync(join(root, "dist"), { recursive: true, withFileTypes: true }).filter((file) => file.isFile()).map((file) => join(file.parentPath, file.name)).sort();
    const digest = (): string => createHash("sha256").update(Buffer.concat(files.map((file) => readFileSync(file)))).digest("hex"), before = digest();
    const offset = output.length;
    writeFileSync(join(root, "Cargo.lock"), fixture.probe.files["Cargo.lock"]);
    writeFileSync(join(root, "🦀️.rs"), fixture.probe.files["🦀️.rs"] + "pub fn changed() -> u32 { 43 }\n");
    await until(fixture.probe.diagnostic, offset);
    await until("hook call to", offset);
    assert.equal(readFileSync(join(root, "Cargo.lock"), "utf8"), fixture.probe.files["Cargo.lock"]);
    assert.equal(digest(), before, "Rejected watch rebuild must preserve the previous deliverables");
    console.log("[DEBUG] Native Trunk build/watch reject stale lock bytes before compilation and retain prior deliverables PASS");
  } finally {
    stop();
    const force = setTimeout(() => { if (!exited && watch.pid && process.platform !== "win32") try { process.kill(-watch.pid, "SIGKILL"); } catch {} }, 2000);
    await closed;
    clearTimeout(force); clearInterval(progress);
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
    writeFileSync(join(root, "watch.log"), output);
  }
}
