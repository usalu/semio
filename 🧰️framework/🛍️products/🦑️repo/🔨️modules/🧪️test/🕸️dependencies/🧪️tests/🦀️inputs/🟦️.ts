import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, symlinkSync, rmSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

/** 🦀️ Compares inferred test ownership with Cargo's local package graph and mounted compiler inputs. */
export async function testInferredNativeInputs(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url);
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const root = mkdtempSync(join(output, "inferred-native-inputs-"));
  const put = (path: string, content: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); };
  for (const [path, content] of Object.entries(fixture.files)) put(path, String(content));
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  const vocabulary = JSON.parse(readFileSync(join(workspace, library, "🔣️taxonomy.json"), "utf8"));
  vocabulary.testDomainPath = fixture.domain;
  vocabulary.generatorContracts = { fixture: fixture.generator };
  put(`${library}/🔣️taxonomy.json`, JSON.stringify(vocabulary));
  put(`${library}/⚡️caching/🔣️policy.json`, readFileSync(join(workspace, library, "⚡️caching/🔣️policy.json"), "utf8"));
  const caseRoot = `${fixture.owner}/${vocabulary.testsDirName}/${fixture.case}`;
  const kind = vocabulary.fileKinds[vocabulary.testFeatureFileKindId];
  const feature = `${caseRoot}/${kind.emoji}${kind.extensionChains[0]}`;
  put(feature, "Feature: Native cache inputs\n  Scenario: Source closure\n    Then all compiler inputs are owned\n");
  const plugin = (await import("../../../🟨️.mjs")).default;
  const nodes = await plugin.createNodesV2[1]([feature], {}, { workspaceRoot: root });
  const project: any = Object.values(nodes[0][1].projects)[0];
  const commandSources = project.namedInputs?.testCommandSources?.filter((input: any) => typeof input === "string" && input.endsWith(".ts")) ?? [];
  assert.deepEqual([...commandSources].sort(), fixture.commandSources.map((path: string) => `{workspaceRoot}/${path}`).sort(), "Every phase must hash the executable router's imports outside the test domain");
  const bundled = await require("esbuild").build({ entryPoints: [join(root, fixture.domain, "📜️script.ts")], absWorkingDir: root, bundle: true, write: false, platform: "node", packages: "external", metafile: true, logLevel: "silent" });
  assert.deepEqual([...commandSources].sort(), Object.keys(bundled.metafile.inputs).map((path) => `{workspaceRoot}/${path}`).sort());
  assert.ok(Object.values(project.targets).every((target: any) => target.inputs.includes("testCommandSources")));
  const references = (target: any) => target.inputs.flatMap((input: any) => input.input === "nativeSources" ? input.projects : []).sort();
  assert.deepEqual(references(project.targets["test-subject"]), [...fixture.subjectRoots].sort(), "A nested case must hash the subject, transitive dependencies, host and local oracle packages");
  assert.deepEqual(references(project.targets["test-oracle"]), [...fixture.oracleRoots].sort(), "Oracle execution must not compile the subject");
  for (const phase of ["lint", "test-contract"]) assert.deepEqual(references(project.targets[phase]), []);
  assert.ok(project.targets["test-subject"].dependsOn.includes(fixture.generator.target));
  assert.ok(!project.targets["test-oracle"].dependsOn?.includes(fixture.generator.target));
  assert.ok(project.targets["test-subject"].inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  for (const path of ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"]) assert.ok(project.targets["test-subject"].inputs.includes(`{workspaceRoot}/${path}`), path);
  const cargo = Bun.spawnSync(["cargo", "metadata", "--offline", "--format-version", "1"], { cwd: root, env: { ...process.env, CARGO_TARGET_DIR: join(root, ".compiler") }, stdout: "pipe", stderr: "pipe" });
  assert.equal(cargo.exitCode, 0, cargo.stderr.toString());
  const metadata = JSON.parse(cargo.stdout.toString());
  const cargoRoots = metadata.packages.map((pkg: any) => relative(root, dirname(pkg.manifest_path)).replaceAll("\\", "/")).sort();
  assert.deepEqual(cargoRoots, Object.keys(fixture.projects).sort());
  const { cacheInternals } = await import("../../../../📚️library/🟨️.mjs");
  const projects: Record<string, any> = { [project.name]: project };
  for (const [path, name] of Object.entries(fixture.projects)) projects[String(name)] = { name, root: path, namedInputs: cacheInternals.projectInputs({ name, targets: {} }, path, root, new Map()), targets: {} };
  const edges = await plugin.createDependencies({}, { workspaceRoot: root, projects });
  assert.deepEqual(edges.filter((edge: any) => edge.source === project.name).map((edge: any) => edge.target).sort(), [...fixture.subjectRoots].sort());
  assert.ok(edges.every((edge: any) => edge.sourceFile === feature));
  const matches = (path: string) => fixture.subjectRoots.some((name: string) => {
    const owner = projects[name], inputs = owner.namedInputs.nativeSources;
    const match = (pattern: string) => require("minimatch").minimatch(path, pattern.replace("{workspaceRoot}/", "").replace("{projectRoot}/", `${owner.root}/`));
    return inputs.some((input: any) => typeof input === "string" && !input.startsWith("!") && match(input)) && !inputs.some((input: any) => typeof input === "string" && input.startsWith("!") && match(input.slice(1)));
  });
  for (const path of fixture.subjectInputs) assert.ok(matches(path), `Mounted compiler source missing: ${path}`);
  put("package.json", JSON.stringify({ private: true, name: "native-input-cache-fixture" }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, namedInputs: { sharedGlobals: [] }, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\n.compiler\n.results\n.generated\n.invocations\n");
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const subjectTarget = project.targets["test-subject"];
  project.targets = { "test-subject": { ...subjectTarget, outputs: ["{workspaceRoot}/.results/test.txt"], options: { command: "bun ./📜️script.ts subject", cwd: ".", forwardAllArgs: false } } };
  projects.shared.targets.generate = { cache: true, executor: "nx:run-commands", inputs: ["{workspaceRoot}/value.json", "{workspaceRoot}/📜️script.ts"], outputs: ["{workspaceRoot}/.generated/shared"], options: { command: "bun ./📜️script.ts generate", cwd: "." } };
  for (const entry of Object.values(projects)) put(`${entry.root}/project.json`, JSON.stringify(entry));
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const execute = async () => {
    const child = Bun.spawn(["node", cli, "run", `${project.name}:test-subject`, "--outputStyle=static"], { cwd: root, env: { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr);
    return readFileSync(join(root, ".results/test.txt"), "utf8");
  };
  const invocations = () => readFileSync(join(root, ".invocations"), "utf8");
  assert.equal(await execute(), fixture.nativeCache.initial);
  const cold = invocations();
  assert.equal(await execute(), fixture.nativeCache.initial);
  assert.equal(invocations(), cold, "An identical native Nx run must not execute a compiler or generator");
  rmSync(join(root, ".results"), { recursive: true });
  rmSync(join(root, ".generated"), { recursive: true });
  assert.equal(await execute(), fixture.nativeCache.initial);
  assert.equal(invocations(), cold, "Nx must restore deleted prerequisite and consumer outputs");
  put(fixture.nativeCache.unrelated, "unrelated source change\n");
  assert.equal(await execute(), fixture.nativeCache.initial);
  assert.equal(invocations(), cold, "Unrelated files must not invalidate the test");
  put(fixture.nativeCache.source, fixture.nativeCache.replacement);
  assert.equal(await execute(), fixture.nativeCache.changed);
  assert.equal(invocations(), cold + "subject\n", "A transitive source change must rerun only the consumer");
  put(fixture.commandSources[1], "export const value = 2;\n");
  assert.equal(await execute(), fixture.nativeCache.changed);
  assert.equal(invocations(), cold + "subject\nsubject\n", "A shared router implementation change must invalidate the consumer");
  const testDomain = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";
  const helper = readFileSync(join(workspace, testDomain, "🕸️dependencies/🟨️.mjs"), "utf8");
  put("plugin/🟨️.mjs", readFileSync(join(workspace, testDomain, "🟨️.mjs"), "utf8"));
  put("plugin/🕸️dependencies/🟨️.mjs", helper);
  put("📚️library/🟨️.mjs", `export { cacheInternals } from ${JSON.stringify(pathToFileURL(join(workspace, library, "🟨️.mjs")).href)};\n`);
  put("reload/📜️script.ts", [
    'import assert from "node:assert/strict";',
    'import { readFileSync, writeFileSync } from "node:fs";',
    'import { resolve, join } from "node:path";',
    'import { pathToFileURL } from "node:url";',
    'const root = resolve(import.meta.dirname, ".."), feature = process.argv[2];',
    'const path = join(root, "plugin/🕸️dependencies/🟨️.mjs"), helper = readFileSync(path, "utf8");',
    'const resident = (await import(pathToFileURL(join(root, "plugin/🟨️.mjs")).href)).default;',
    'await resident.createNodesV2[1]([feature], {}, { workspaceRoot: root });',
    'writeFileSync(path, helper + "\\nexport const invalid = ;\\n");',
    'await assert.rejects(async () => resident.createNodesV2[1]([feature], {}, { workspaceRoot: root }), /Unexpected|Expected|Parse|Syntax/i);',
    'writeFileSync(path, helper);',
    'await resident.createNodesV2[1]([feature], {}, { workspaceRoot: root });',
  ].join("\n"));
  const reload = Bun.spawn(["node", join(root, "reload/📜️script.ts"), feature], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [reloadOut, reloadError, reloadCode] = await Promise.all([new Response(reload.stdout).text(), new Response(reload.stderr).text(), reload.exited]);
  assert.equal(reloadCode, 0, reloadOut + reloadError);
  console.log("[DEBUG] Inferred Rust cases cover Cargo packages, mounted sources and generator prerequisites; oracle phases exclude subject compilation PASS");
  console.log("[DEBUG] Native Nx cold/warm, deleted-output restoration, unrelated-source reuse and transitive-source and shared-router invalidation execute the expected Rust consumer PASS");
  console.log("[DEBUG] Resident test inference rejects changed invalid helper code and recovers after correction without a graph/cache reset PASS");
}
