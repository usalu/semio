import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

/** 🏗️ Proves native preparation is an uncached, application-independent Nx leaf against Cargo and esbuild. */
export async function testNativeDependencies(workspace: string, output: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), vectors = resolve(import.meta.dir, "../../🧫️fixtures/📦️native-dependencies");
  const fixture = JSON.parse(readFileSync(join(vectors, "🔣️.json"), "utf8"));
  const projects = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry,...fixture.sourceEntrypoints], outdir: join(output,"native-source-closure"), bundle: true, write: false, metafile: true, platform: "node", packages: "external", format: "esm", logLevel: "silent" });
  for (const source of Object.keys(graph.metafile.inputs)) {
    assert.notEqual(resolve(workspace, source), join(workspace, "📜️script.ts"));
    for (const forbidden of fixture.forbiddenImports) assert.ok(!source.includes(forbidden), source);
  }
  for (const value of Object.values(graph.metafile.outputs) as { imports: { path: string }[] }[]) for (const dependency of value.imports) assert.ok(dependency.path.startsWith("node:"), dependency.path);
  const { prepareDependencies, NativeDependenciesScript } = await import(pathToFileURL(join(workspace, fixture.entry)).href);
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const root = mkdtempSync(join(output, "native-sync-")), controller = new AbortController();
  const stop = (): void => controller.abort(), timer = setTimeout(() => controller.abort(new Error("Native synchronization probe exceeded 90s")), 90000);
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  let passed = false;
  const authorCargoFixture = (): void => { for (const [path, contents] of Object.entries(fixture.cargo.files)) writeFileSync(join(root, path), contents as string); };
  const inheritedBrowserPath = process.env.PLAYWRIGHT_BROWSERS_PATH;
  try {
    authorCargoFixture();
    const { cargoRepositoryPackages } = await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts");
    assert.deepEqual(cargoRepositoryPackages(root), fixture.cargo.expectedMembers);
    delete process.env.PLAYWRIGHT_BROWSERS_PATH;
    writeFileSync(join(root, "Cargo.lock"), fixture.bindgenLock);
    const interpolate = (value: string): string => value.replace("{workspace}", root).replaceAll("/", process.platform === "win32" && value.startsWith("{workspace}") ? "\\" : "/");
    for (const row of fixture.environments) {
      const target = projects.targets[`deps-${row.kind}`], calls: string[][] = [];
      assert.equal(target.cache, false); assert.deepEqual(target.outputs, []);
      assert.equal(target.options.command, `bun "${fixture.entry}" sync ${row.kind}`);
      await prepareDependencies(row.kind, root, controller.signal, async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(cwd, root); assert.equal(signal, controller.signal);
        if (capture) return "";
        calls.push([command, ...args]);
        for (const [key, value] of Object.entries(row.environment ?? {})) assert.equal(env[key], interpolate(value as string));
        return "";
      });
      assert.deepEqual(calls, row.commands.map((command: string[]) => command.map(interpolate)), row.kind);
    }
    for (const row of fixture.browserPaths) {
      if (row.input === null) delete process.env.PLAYWRIGHT_BROWSERS_PATH;
      else process.env.PLAYWRIGHT_BROWSERS_PATH = interpolate(row.input);
      let acquisitions = 0;
      await prepareDependencies("browsers", root, controller.signal, async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(command, "bun");
        assert.deepEqual(args, [join(root, "node_modules/playwright/cli.js"), "install", "chromium"]);
        assert.equal(env.PLAYWRIGHT_BROWSERS_PATH, interpolate(row.expected));
        acquisitions += 1;
        return "";
      });
      assert.equal(acquisitions, 1);
    }
    writeFileSync(join(root, "Cargo.lock"), "version = 4\n");
    await assert.rejects(prepareDependencies("trunk", root, controller.signal, async () => { assert.fail("Invalid lock must fail before acquisition"); }), /exactly one wasm-bindgen/);
    for (const args of fixture.rejected) await assert.rejects(new NativeDependenciesScript(root, root).run(args));
    authorCargoFixture();
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "workspace", private: true }));
    writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false }));
    writeFileSync(join(root, "📜️script.ts"), "throw new Error('Application code must not run during native setup');\n");
    writeFileSync(join(root, "project.json"), JSON.stringify({ name: "workspace", targets: { "deps-cargo": projects.targets["deps-cargo"], "deps-cargo-lock": projects.targets["deps-cargo-lock"] } }));
    for (const source of Object.keys(graph.metafile.inputs)) {
      const destination = join(root, source); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(workspace, source), destination);
    }
    for(const source of fixture.requiredPhysicalSources) {if(!existsSync(join(root,source))){mkdirSync(dirname(join(root,source)),{recursive:true});copyFileSync(join(workspace,source),join(root,source));}assert.equal(existsSync(join(root,source)),true,source);}
    const env: NodeJS.ProcessEnv = { ...process.env, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_ROOT: root, REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NODE_PATH: join(workspace, "node_modules"), CARGO_TARGET_DIR: join(root, "target"), CARGO_BUILD_BUILD_DIR: join(root, "build"), FORCE_COLOR: "0", NO_COLOR: "1" };
    delete env.NX_SKIP_NX_CACHE;
    const run = (command: string, args: string[]): Promise<string> => runTool(command, args, root, controller.signal, true, env);
    await run("cargo", ["fetch", "--locked", "--offline"]);
    const oracle = JSON.parse(await run("cargo", ["metadata", "--locked", "--offline", "--no-deps", "--format-version=1"]));
    const assertCargoOwner = (value: any): void => {
      assert.equal(value.packages.length, fixture.cargo.expectedMembers.length);
      assert.equal(value.packages[0].name, fixture.cargo.package);
      assert.equal(resolve(value.packages[0].manifest_path), join(root, "Cargo.toml"));
      assert.deepEqual(value.workspace_members, [value.packages[0].id]);
      assert.equal(resolve(value.workspace_root), root);
    };
    assertCargoOwner(oracle);
    const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
    assert.deepEqual(Bun.TOML.parse(lock), require("smol-toml").parse(lock));
    writeFileSync(join(root,"owned-input.txt"),fixture.cargo.ownedInput);
    mkdirSync(join(root,"owned"),{recursive:true});writeFileSync(join(root,"owned/program.js"),fixture.cargo.programSource);writeFileSync(join(root,"owned/📜️script.ts"),fixture.cargo.preparationScript);
    writeFileSync(join(root,"Cargo.toml"),fixture.cargo.files["Cargo.toml"]+fixture.cargo.preparationMetadata);
    await assert.rejects(run("cargo", ["fetch", "--locked", "--offline"]));
    assert.equal(readFileSync(join(root,"Cargo.lock"),"utf8"),lock);
    assert.equal(existsSync(join(root,"generated/Cargo.toml")),true,"Actual owner preparation must introduce the dependency before locked verification");
    const preparationCalls = (): number => readFileSync(join(root,"preparation-calls.jsonl"),"utf8").trim().split("\n").length;
    let preparedLock: string | undefined;
    for (let attempt = 0; attempt < 2; attempt++) {
      const before = preparationCalls();
      const log = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo", "--output-style=stream"]);
      writeFileSync(join(root, `native-nx-${attempt}.log`), log);
      assert.equal(preparationCalls()-before,1,"The adjacent update/fetch transaction must prepare exactly once");
      assert.ok(!log.includes("[local cache]")); assert.match(log, /Successfully ran target deps-cargo/);
      const currentLock=readFileSync(join(root,"Cargo.lock"),"utf8"), parsed=Bun.TOML.parse(currentLock);
      assert.deepEqual(parsed,require("smol-toml").parse(currentLock));
      assert.deepEqual((parsed.package as {name:string}[]).map(value=>value.name).sort(),fixture.cargo.preparedPackages);
      if(preparedLock!==undefined)assert.equal(currentLock,preparedLock);preparedLock=currentLock;
      const actual=JSON.parse(await run("cargo",["metadata","--locked","--offline","--no-deps","--format-version=1"]));
      assert.deepEqual(actual.packages.map((value:any)=>value.name).sort(),fixture.cargo.preparedPackages);
      await run("cargo",["fetch","--locked","--offline"]);
    }
    assert.equal(existsSync(join(root, "target")), false, "Dependency synchronization must not compile deliverables");
    writeFileSync(join(root, "Cargo.lock"), fixture.cargo.invalidLock);
    const refreshed = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo", "--output-style=stream"]);
    assert.ok(!refreshed.includes("[local cache]")); assert.match(refreshed, /Successfully ran target deps-cargo/);
    assert.equal(readFileSync(join(root,"Cargo.lock"),"utf8"),preparedLock);
    const explicit = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo-lock", "--output-style=stream"]);
    assert.match(explicit,/Successfully ran target deps-cargo-lock/);
    await run("cargo",["fetch","--locked","--offline"]);
    assert.equal(existsSync(join(root, "target")), false, "Lock refresh must not compile deliverables");
    const custodySchemaPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧬️schema/🔣️.json";
    const schema=JSON.parse(readFileSync(join(workspace,custodySchemaPath),"utf8")),Ajv=require("ajv"),validate=new Ajv({strict:false}).compile(schema);
    const {inventorySchemaScopes}=await import("../../../🔍️discovery/🟦️.ts");
    const inventoryRoot=join(root,"schema-owner");mkdirSync(dirname(join(inventoryRoot,custodySchemaPath)),{recursive:true});copyFileSync(join(workspace,custodySchemaPath),join(inventoryRoot,custodySchemaPath));
    const inventory=inventorySchemaScopes(inventoryRoot,JSON.parse(readFileSync(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"))),scope=inventory.modules.find(module=>module.modulePath===dirname(custodySchemaPath));
    assert.ok(scope?.level);assert.equal(scope.scopeId,"repo.library.workspaces.cargo.preparation.custody");assert.deepEqual(scope.documents[0].exports.sort(),Object.keys(schema.$defs).sort());
    assert.equal(scope.hashes["🔣️.json"],new Bun.CryptoHasher("sha256").update(readFileSync(join(workspace,custodySchemaPath))).digest("hex"));
    assert.deepEqual(inventory.diagnostics.filter(row=>row.path.startsWith(dirname(custodySchemaPath))),[]);
    const pairPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
    const {withPreparedCargoDependencyPairV1}=await import(pathToFileURL(join(root,pairPath)).href);
    const recipe=join(root,"owned/📜️script.ts"),initial=fixture.cargo.preparationScript;
    for(const row of fixture.custodyMutations){
      writeFileSync(join(root,"owned-input.txt"),fixture.cargo.ownedInput);writeFileSync(join(root,"owned/program.js"),fixture.cargo.programSource);rmSync(join(root,"owned/program.ts"),{force:true});
      const script=row.kind==="read-race"?initial.replace('observeCargoPreparationInputV1(input,"file",bytes);','writeFileSync(input,"8");observeCargoPreparationInputV1(input,"file",bytes);'):row.kind==="unknown"?initial.replace("completeCargoPreparationObservationV1();",""):row.kind==="member"?initial.replace(String.raw`member-manifests = [\"Cargo.toml\", \"generated/Cargo.toml\"]`,String.raw`member-manifests = [\"Cargo.toml\", \"generated/Cargo.toml\", \"additional/*/Cargo.toml\"]`):initial;
      if(row.kind==="member")assert.notEqual(script,initial);
      writeFileSync(recipe,script);let calls=0;
      await assert.rejects(withPreparedCargoDependencyPairV1(root,"Cargo.toml",controller.signal,async(args:string[])=>{
        calls++;
        if(args[0]==="update"){
          if(row.kind==="routing-program")writeFileSync(join(root,"owned/program.js"),"export const prepareValue=8;\n");
          if(row.kind==="resolution")writeFileSync(join(root,"owned/program.ts"),"export const prepareValue=7;\n");
          if(row.kind==="source")writeFileSync(recipe,script+"\n");
          if(row.kind==="input")writeFileSync(join(root,"owned-input.txt"),"8");
          if(row.kind==="output")writeFileSync(join(root,"generated/lib.rs"),"pub fn generated()->u32{8}\n");
          if(row.kind==="member"){mkdirSync(join(root,"additional/new"),{recursive:true});writeFileSync(join(root,"additional/new/Cargo.toml"),'[package]\nname="additional-member"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n');writeFileSync(join(root,"additional/new/lib.rs"),"");}
        }else if(row.kind==="lock")writeFileSync(join(root,"Cargo.lock"),"version = 4\n");
      },[recipe]),new RegExp(row.expected));
      assert.equal(calls,["unknown","read-race"].includes(row.kind)?0:row.kind==="lock"?2:1,row.id);
      rmSync(join(root,"owned/program.ts"),{force:true});writeFileSync(join(root,"owned/program.js"),fixture.cargo.programSource);
      rmSync(join(root,"additional"),{recursive:true,force:true});
      writeFileSync(recipe,initial);writeFileSync(join(root,"owned-input.txt"),fixture.cargo.ownedInput);writeFileSync(join(root,"Cargo.lock"),preparedLock!);
    }
    const abort=new AbortController();abort.abort();await assert.rejects(withPreparedCargoDependencyPairV1(root,"Cargo.toml",abort.signal,async()=>assert.fail("Cancelled preparation must not run Cargo")));
    assert.equal(validate({}),false);
    const ordered=fixture.dependencyPreparation;
    writeFileSync(join(root,"generated/Cargo.toml"),readFileSync(join(root,"generated/Cargo.toml"),"utf8")+ordered.metadata);
    writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);rmSync(join(root,"generated/child-output.txt"),{force:true});
    writeFileSync(recipe,initial.replace("observeCargoPreparationSourceV1(import.meta.path);","observeCargoPreparationSourceV1(import.meta.path);\n"+ordered.parentRead));
    let dependencyCalls=0;
    await withPreparedCargoDependencyPairV1(root,"Cargo.toml",controller.signal,async(args:string[])=>{dependencyCalls++;assert.equal(readFileSync(join(root,"generated/child-output.txt"),"utf8"),"7");const oracle=Bun.spawnSync(["cargo","metadata","--manifest-path",join(root,"Cargo.toml"),"--locked","--offline","--no-deps","--format-version=1"],{cwd:root,stdout:"pipe",stderr:"pipe"});assert.equal(oracle.exitCode,0,oracle.stderr.toString());const tree=JSON.parse(oracle.stdout.toString());assert.deepEqual(tree.packages.map((row:any)=>row.name).sort(),fixture.cargo.preparedPackages);});
    assert.equal(dependencyCalls,2);


    passed = true;
  } finally {
    if (inheritedBrowserPath === undefined) delete process.env.PLAYWRIGHT_BROWSERS_PATH;
    else process.env.PLAYWRIGHT_BROWSERS_PATH = inheritedBrowserPath;
    controller.abort(); clearTimeout(timer); process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (passed) rmSync(root, { recursive: true, force: true });
  }
}
