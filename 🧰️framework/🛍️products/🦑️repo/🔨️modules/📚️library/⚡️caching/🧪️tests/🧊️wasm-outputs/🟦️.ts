import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import path, { delimiter, dirname, join } from "node:path";
import {type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1,acquireCompilerCapacityLeaseV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🗂️directories/🟦️.ts";

/** 🧊️ Executes restored renderer artifacts and compares them with an independent native Trunk build. */
export async function testWgpuWasmOutputs(workspace: string, output: string, invocation:ScriptInvocation): Promise<void> {
  const control=scriptCompilerAdmissionControlV1(invocation,"test:wgpu-wasm",600000),compiler=cargoDirectories(workspace),leases=join(workspace,".cache/b/l/m/data");
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧊️wasm-outputs/🔣️.json"), "utf8"));
  const packagePath = join(fixture.owner, "📦️packages/🦀️rust"), project = JSON.parse(readFileSync(join(workspace, fixture.owner, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  assert.equal(project.metadata.nativeRoot, packagePath, "Renderer compiler inputs must resolve from the Cargo owner");
  const ts = require("typescript"), consumerText = readFileSync(join(workspace, fixture.consumer), "utf8"), consumer = ts.createSourceFile("vite.ts", consumerText, ts.ScriptTarget.Latest, true);
  let directoryExpression = "";
  const visit = (node: any) => { if (ts.isVariableDeclaration(node) && node.name.getText(consumer) === "rendererModulesDir") directoryExpression = node.initializer.getText(consumer); ts.forEachChild(node, visit); };
  visit(consumer);
  assert.match(consumerText, /find: "\/renderer-modules\/wgpu", replacement: rendererModulesDir/);
  const rendererDirectory = new Function("path", "playDir", "buildMode", `return ${directoryExpression};`);
  const consumerRoot = path.resolve(workspace, dirname(fixture.consumer), "../..");
  for (const row of fixture.profiles) {
    const target = project.targets[row.target];
    assert.equal(target?.cache, true, `${row.target} must cache its compiler deliverables`);
    assert.deepEqual(target.outputs.map((path:string)=>path.replaceAll("{options.env.SEMIO_BUILD_MODE}",row.profile)), [`{workspaceRoot}/${packagePath}/${row.output}`]);
    assert.ok(target.dependsOn.includes("workspace:deps-trunk"));
    assert.ok(target.dependsOn.includes("workspace:deps-wasm-opt"));
    assert.ok(target.options.command.endsWith(" build"));
    assert.deepEqual(target.configurations[row.profile].args,[row.profile]);
    assert.equal(rendererDirectory(path, consumerRoot, row.profile), join(workspace, packagePath, row.output));
  }
  const compilerOwner = join(workspace, fixture.owner, "🏗️compiler/🌐️wasm/📜️script.ts");
  const bundle = await require("esbuild").build({ entryPoints: [compilerOwner], absWorkingDir: workspace, bundle: true, packages: "external", platform: "node", format: "esm", write: false, metafile: true });
  assert.ok(!Object.keys(bundle.metafile.inputs).some(path => path.endsWith("🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts")), "Finite renderer compilation must not import browser/application tests or live stores");
  const root = mkdtempSync(join(output, "w"));
  const put = (path: string, value: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), value); };
  const bindgen = (Bun.TOML.parse(readFileSync(join(workspace, "Cargo.lock"), "utf8")) as any).package.find((row: any) => row.name === "wasm-bindgen").version;
  put("Cargo.toml", `[package]\nname=${JSON.stringify(fixture.crate)}\nversion="0.1.0"\nedition="2024"\n[lib]\npath="🦀️.rs"\ncrate-type=["cdylib"]\n[dependencies]\nwasm-bindgen="=${bindgen}"\n[workspace]\n`);
  put("🦀️.rs", fixture.rust.replace("VALUE", String(fixture.values[0])));
  put("package.json", JSON.stringify({ name: "wgpu-wasm-fixture", private: true, type: "module" }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\ndist\nstate\noracle\n.runs\n.🧬semio\n");
  const targets = {wasm:{executor:"nx:run-commands",cache:true,inputs:["{projectRoot}/Cargo.toml","{projectRoot}/Cargo.lock","{projectRoot}/🦀️.rs","{projectRoot}/📜️script.ts"],outputs:["{projectRoot}/dist/wasm-{options.env.SEMIO_BUILD_MODE}"],options:{command:"repo ./📜️script.ts build",args:["prototype"],cwd:"."},configurations:Object.fromEntries(fixture.profiles.map((row:any)=>[row.profile,{args:[row.profile],env:{SEMIO_BUILD_MODE:row.profile}}])),defaultConfiguration:"prototype"}};
  put("project.json", JSON.stringify({ name: "renderer", targets }));
  put("📜️script.ts", `import { appendFileSync } from "node:fs";\nimport { buildTrunkRenderer } from ${JSON.stringify(compilerOwner)};\nimport {phaseCompilerAdmissionControlV1} from ${JSON.stringify(join(workspace,"🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts"))};\nawait buildTrunkRenderer({ rustPackageRoot: process.cwd(), componentRoot: process.cwd(), workspace: ${JSON.stringify(workspace)}, toolWorkspace: ${JSON.stringify(workspace)}, profile: process.argv[3], control:phaseCompilerAdmissionControlV1(new AbortController().signal,120000,()=>undefined,()=>new Promise(done=>setImmediate(done))),stateRoot: process.cwd()+"/s" });\nappendFileSync(".runs", process.argv[3] + "\\n");\n`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const env: NodeJS.ProcessEnv = { ...process.env, NX_DAEMON: "false", SEMIO_WORKSPACE_ROOT: root, SEMIO_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_REPO_ROOT: root, CARGO_TARGET_DIR: join(root, "t"), CARGO_BUILD_BUILD_DIR: compiler.build, TRUNK_BUILD_DIST: join(root, "forbidden"), TRUNK_BUILD_RELEASE: "true", TRUNK_TOOLS_WASM_BINDGEN: "0.0.0" };
  delete env.NX_SKIP_NX_CACHE;
  const execute = async (args: string[], environment = env) => {
    let stdout="",stderr="";try{await runOwnedCommand(args[0]!,args.slice(1),root,"test:wgpu-wasm",compilerAdmissionBudgetV1(control),{env:environment,signal:control.signal,stdout:"stderr",output:{maximumBytes:134217728,maximumLines:65536},onLine:(line,stream)=>{if(stream==="stdout")stdout+=line+"\n";else stderr+=line+"\n";},onProgress:async()=>{await control.publish({resource:"test:wgpu-wasm",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}});}catch(error){throw Error(stdout+stderr,{cause:error});}return stdout;
  };
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js"), runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").length;
  await execute(["cargo", "generate-lockfile", "--offline"]);
  const run = (profile: string) => execute(["node", cli, "run", "renderer:wasm", "--configuration="+profile,"--outputStyle=static"]);
  const files = (directory: string): Map<string, Buffer> => new Map(readdirSync(directory, { recursive: true, withFileTypes: true }).filter(row => row.isFile()).map(row => [join(row.parentPath, row.name).slice(directory.length + 1), readFileSync(join(row.parentPath, row.name))]));
  const value = async (path: string) => JSON.parse(await execute(["node", "--input-type=module", "-e", `import { readFileSync } from "node:fs"; import { pathToFileURL } from "node:url"; const module = await import(pathToFileURL(process.argv[1].replace(/_bg.wasm$/, ".js")).href); await module.default({ module_or_path: readFileSync(process.argv[1]) }); console.log(JSON.stringify({ value: module.answer() }));`, path])).value;
  const { preparedBinaryen } = await import("@semio-tech/nx-bootstrap-tools/wasm");
  const oracleEnv: NodeJS.ProcessEnv = { ...env, PATH: `${dirname(preparedBinaryen(workspace))}${delimiter}${env.PATH}`, CARGO_TARGET_DIR: join(root, "state/oracle-target") };
  for (const key of Object.keys(oracleEnv)) if (key.startsWith("TRUNK_") || ["NO_COLOR", "FORCE_COLOR"].includes(key)) delete oracleEnv[key];
  const oracle = async (profile: string, expected: number) => {
    put("oracle/index.html", `<!doctype html><html><head><link data-trunk rel="rust" href="../Cargo.toml" data-cargo-profile="${profile}" data-wasm-opt="${profile==="release"?"z":"0"}" data-type="worker" data-bindgen-target="web" /></head><body></body></html>\n`);
    put("oracle/Trunk.toml", `required_version="=0.21.14"\noffline=true\n[build]\ntarget="index.html"\ndist="dist"\nlocked=true\nfrozen=true\nfilehash=false\nrelease=${profile === "release"}\n[tools]\nwasm_bindgen="${bindgen}"\nwasm_opt="version_130"\n`);
    const original={...oracleEnv,SEMIO_BUILD_MODE:profile},capacity=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:leases,control,environment:original});let lease:Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>>|undefined;try{lease=await acquireCargoBuildLeaseV1({directory:leases,buildDirectory:compiler.build,targetDirectory:compiler.target,args:["--profile",profile],signal:control.signal,control});await execute(["trunk", "build", "--config", join(root, "oracle/Trunk.toml"), "--skip-version-check", "--offline", "true"],{...original,SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:String(capacity.parallelism),CARGO_BUILD_JOBS:String(capacity.parallelism)});}finally{lease?.release();capacity.release();}
    assert.equal(await value(join(root, "oracle/dist", `${fixture.crate}_bg.wasm`)), expected);
    for (const extension of [".js", "_bg.wasm"]) assert.deepEqual(readFileSync(join(root, "oracle/dist", `${fixture.crate}${extension}`)), readFileSync(join(root, `dist/wasm-${profile}`, `${fixture.crate}${extension}`)), "Renderer bytes must match the independent native Trunk oracle");
  };
  let count = 0;
  for (const row of fixture.profiles) {
    await run(row.profile); assert.equal(runs(), ++count);
    const published = join(root, row.output), bytes = files(published), wasm = join(published, `${fixture.crate}_bg.wasm`);
    assert.equal(await value(wasm), fixture.values[0]);
    assert.deepEqual([...bytes.keys()].filter(name => name !== ".semio-artifact.json").sort(), fixture.deliverables.map((suffix: string) => fixture.crate + suffix).sort(), "Publish each compiler artifact once without duplicate aliases");
    assert.ok(![...bytes.keys()].some(name => /plugin-modules|extension-modules|assets|target|staging/.test(name)));
    await run(row.profile); assert.equal(runs(), count);
    rmSync(published, { recursive: true });
    await run(row.profile); assert.equal(runs(), count);
    assert.deepEqual(files(published), bytes);
    assert.equal(await value(wasm), fixture.values[0]);
    await oracle(row.profile, fixture.values[0]);
    const fixtureConsumer = join(root, path.relative(workspace, consumerRoot)), fixtureRenderer = rendererDirectory(path, fixtureConsumer, row.profile);
    if (!existsSync(dirname(fixtureRenderer))) {
      mkdirSync(dirname(dirname(fixtureRenderer)), { recursive: true });
      symlinkSync(join(root, "dist"), dirname(fixtureRenderer), process.platform === "win32" ? "junction" : "dir");
    }
    const server = await require("vite").createServer({ configFile: false, root, publicDir: false, cacheDir: join(root, "state/vite"), resolve: { alias: [{ find: "/renderer-modules/wgpu", replacement: fixtureRenderer }] }, server: { port: 0, host: "127.0.0.1", watch: null, fs: { allow: [root] } }, logLevel: "silent" });
    try {
      await server.listen();
      const address = server.httpServer.address(), response = await fetch(`http://127.0.0.1:${address.port}/renderer-modules/wgpu/${fixture.crate}_bg.wasm`);
      assert.equal(response.status, 200);
      assert.deepEqual(Buffer.from(await response.arrayBuffer()), readFileSync(wasm), "The real Vite consumer must serve restored compiler bytes");
    } finally { await server.close(); }
  }
  put("🦀️.rs", fixture.rust.replace("VALUE", String(fixture.values[1])));
  await run("prototype"); assert.equal(runs(), ++count);
  const published = join(root, "dist/wasm-prototype"), wasm = join(published, `${fixture.crate}_bg.wasm`);
  assert.equal(await value(wasm), fixture.values[1]);
  await oracle("prototype", fixture.values[1]);
  assert.equal(existsSync(join(root, "forbidden")), false, "Ambient Trunk configuration must not redirect owned outputs");
  const retained = files(published);
  put("🦀️.rs", "compile_error!(\"preserve the last successful renderer\");\n");
  await assert.rejects(run("prototype"), /preserve the last successful renderer/);
  assert.deepEqual(files(published), retained);
  assert.ok(!existsSync(join(root, "s/staging")) || readdirSync(join(root, "s/staging")).length === 0);
  rmSync(root, { recursive: true, force: true });
}
