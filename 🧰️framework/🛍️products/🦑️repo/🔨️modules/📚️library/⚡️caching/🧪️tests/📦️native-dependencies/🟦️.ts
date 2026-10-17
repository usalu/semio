import{captureOwnedCommandV1}from"../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {testRecordV1} from "../📋️values/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import assert from "node:assert/strict";
import { existsSync, realpathSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve, relative, sep } from "node:path";
import { pathToFileURL } from "node:url";
import {createRequestedIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{nxAuthorizerWorkspaceCandidateV1,nxAuthorizerWorkspaceIndexV1,resolveNxAuthorizerWorkspaceV1}from"../../🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🟦️.ts";
import{readNxAuthorizerSourceV1}from"../../🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {nativeCallerPhaseV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🏗️ Proves native preparation is an uncached, application-independent build leaf against Cargo and esbuild. */
export async function testNativeDependencies(workspace: string, output: string,invocation:ScriptInvocation,parent:ResourceLeaseControl): Promise<void> {
  requireIssuedResourceControlInvocationV1(parent,invocation);
  const require = createRequire(join(workspace, "package.json")), vectors = resolve(import.meta.dir, "../🧫️fixtures/📦️native-dependencies");
  const fixture = JSON.parse(readFileSync(join(vectors, "🔣️.json"), "utf8"));
  const custodyPath=join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts");for(const [row,key] of [[fixture.cargo,"preparationScript"],[fixture.dependencyPreparation,"script"],[fixture.generatedPreparation,"script"]] as const)row[key]=row[key].replace("{CUSTODY}",JSON.stringify(custodyPath.replaceAll("\\","/")));
  const projects = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  const captured=new Map<string,Uint8Array>([["package.json",await readNxAuthorizerSourceV1(workspace,"package.json",parent.signal)]]),firstParty={name:"authentic-first-party-fixture",setup(build:{onResolve(options:{filter:RegExp},resolve:(request:{path:string})=>Promise<{path:string}>):void}){build.onResolve({filter:/^@semio-tech\//},async request=>{const located=relative(workspace,realpathSync(Bun.resolveSync(request.path,workspace))).split(sep).join("/"),manifest=nxAuthorizerWorkspaceCandidateV1(captured.get("package.json")!,located);if(!captured.has(manifest))captured.set(manifest,await readNxAuthorizerSourceV1(workspace,manifest,parent.signal));const selected=resolveNxAuthorizerWorkspaceV1(nxAuthorizerWorkspaceIndexV1(captured),request.path);if(selected!==located)throw Error("Actual fixture export differs from declared first-party Source");return{path:join(workspace,selected)};});}};
  const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry,...fixture.sourceEntrypoints], outdir: join(output,"native-source-closure"), bundle: true, write: false, metafile: true, platform: "node", packages: "external",plugins:[firstParty], format: "esm", logLevel: "silent" });
  checkResourceLeaseControl(parent);
  for (const source of Object.keys(graph.metafile.inputs)) {
    assert.notEqual(resolve(workspace, source), join(workspace, "📜️script.ts"));
    for (const forbidden of fixture.forbiddenImports) assert.ok(!source.includes(forbidden), source);
  }
  for (const value of Object.values(graph.metafile.outputs) as { imports: { path: string }[] }[]) for (const dependency of value.imports) assert.ok(dependency.path.startsWith("node:")||fixture.systemModules.includes(dependency.path), dependency.path);
  const { prepareDependencies, NativeDependenciesScript } = await import(pathToFileURL(join(workspace, fixture.entry)).href);
  const root=mkdtempSync(join(output,"native-sync-")),controller=new AbortController(),progress:string[]=[],phases=new Map<AbortSignal,ReturnType<typeof nativeCallerPhaseV1>>();
  const storage={version:1 as const,directory:join(root,"preparation"),leaseDirectory:join(root,"leases"),program:repositoryCargoPreparationStorageV1(workspace).program};
  const{runNativeDependencyFixtureNegativeV1}=await import("./📥️purpose/🟦️.ts");
  const {withPreparedCargoDependencyPairV1}=await import("../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts"),{cargoWorkspacePreparationInvocationV1}=await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts");
  const phaseFor=(signal:AbortSignal)=>{let phase=phases.get(signal);if(!phase){phase=nativeCallerPhaseV1(invocation,workspace,createRequestedIssuedResourceControlV1(parent,fixture.originalNativeControl.maximumElapsedMilliseconds,signal));phases.set(signal,phase);}return phase;};
  const controlFor=(signal:AbortSignal)=>phaseFor(signal).leaseControl;
  const transportFor=(phase:ReturnType<typeof nativeCallerPhaseV1>)=>({maximumBytes:Math.min(fixture.originalNativeControl.native.transport.maximumBytes,phase.authority.transport.maximumBytes),maximumLines:Math.min(fixture.originalNativeControl.native.transport.maximumLines,phase.authority.transport.maximumLines)});
  const identity=async(cwd:string,signal:AbortSignal)=>{const phase=phaseFor(controller.signal);assert.equal(signal,phase.leaseControl.signal);return captureOwnedCommandV1("cargo",["--version"],cwd,{control:phase.leaseControl,env:phase.options(process.env).env,output:transportFor(phase)});};
  const preparationFor=(signal:AbortSignal)=>{const phase=phaseFor(signal);return{leaseControl:phase.leaseControl,signal:phase.leaseControl.signal,async advance(event:unknown){await phase.advance(JSON.stringify(event));const continuation=phase.leaseControl.yieldContinuation;if(!continuation)throw Error("Issued Native continuation required");await continuation();progress.push("yield");},childBudget:(requested:number)=>Math.min(requested,phase.budget()),childOptions:(environment:Readonly<Record<string,string|undefined>>)=>({...phase.options(environment),signal:phase.leaseControl.signal,output:transportFor(phase)})};};
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
      await prepareDependencies(row.kind, root, invocation,storage,phaseFor(controller.signal), async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(cwd, root); assert.equal(signal,phaseFor(controller.signal).leaseControl.signal);
        if (capture) return command==="cargo"&&args[0]==="--version"?identity(cwd,signal):"";
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
      await prepareDependencies("browsers", root, invocation,storage,phaseFor(controller.signal), async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(command, "bun");
        assert.deepEqual(args, [join(root, "node_modules/playwright/cli.js"), "install", "chromium"]);
        assert.equal(env.PLAYWRIGHT_BROWSERS_PATH, interpolate(row.expected));
        acquisitions += 1;
        return "";
      });
      assert.equal(acquisitions, 1);
    }
    writeFileSync(join(root, "Cargo.lock"), "version = 4\n");
    await assert.rejects(prepareDependencies("trunk", root, invocation,storage,phaseFor(controller.signal), async () => { assert.fail("Invalid lock must fail before acquisition"); }), /exactly one wasm-bindgen/);
    for(const args of fixture.rejected)await assert.rejects(new NativeDependenciesScript(root,workspace,invocation).run(args));
    authorCargoFixture();
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "workspace", private: true }));
    writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false }));
    writeFileSync(join(root, "📜️script.ts"), "throw new Error('Application code must not run during native setup');\n");
    for(const source of fixture.requiredPhysicalSources)assert.equal(existsSync(join(workspace,source)),true,source);
    const env: NodeJS.ProcessEnv = { ...process.env, SEMIO_WORKSPACE_ROOT_PATH: root, SEMIO_WORKSPACE_ROOT: root, REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NODE_PATH: join(workspace, "node_modules"), CARGO_TARGET_DIR: join(root, "target"), CARGO_BUILD_BUILD_DIR: join(root, "build"), FORCE_COLOR: "0", NO_COLOR: "1" };
    delete env.NX_SKIP_NX_CACHE;
    const phase=phaseFor(controller.signal);
    const rawCargo=(args:string[],cwd=root)=>captureOwnedCommandV1("cargo",args.includes("--offline")||args[0]==="--version"?args:[...args,"--offline"],cwd,{control:phase.leaseControl,env:phase.options(env).env,output:transportFor(phase)});
    const pair=async(manifest="Cargo.toml",onCommand?:(args:readonly string[])=>void):Promise<void>=>withPreparedCargoDependencyPairV1(storage,root,manifest,phase.leaseControl,async(args,cwd,signal)=>{assert.equal(signal,phase.leaseControl.signal);onCommand?.(args);await rawCargo(args,cwd);},identity,preparationFor(controller.signal));
    const run=async(command:string,args:string[]):Promise<string>=>{assert.equal(command,"cargo","Fixture execution is a tool-neutral owned Cargo operation");const request=cargoWorkspacePreparationInvocationV1(storage,root,args,root,env);if(request){const at=request.args.indexOf("--manifest"),manifest=request.args[at+1];assert.equal(typeof manifest,"string");await pair(manifest);}return captureOwnedCommandV1(command,args,root,{control:phase.leaseControl,env:phase.options(env).env,output:transportFor(phase)});};
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
    await run("cargo", ["fetch", "--locked", "--offline"]);
    assert.notEqual(readFileSync(join(root,"Cargo.lock"),"utf8"),lock);
    assert.equal(existsSync(join(root,"generated/Cargo.toml")),true,"Actual owner preparation must introduce the dependency before locked verification");
    const preparationCalls = (): number => readFileSync(join(root,"preparation-calls.jsonl"),"utf8").trim().split("\n").length;
    let preparedLock: string | undefined;
    for (let attempt = 0; attempt < 2; attempt++) {
      const before = preparationCalls();
      writeFileSync(join(root,"owned/📜️script.ts"),fixture.cargo.preparationScript+"\n".repeat(attempt+1));
      const commands:string[][]=[];await pair("Cargo.toml",args=>commands.push([...args]));
      assert.equal(preparationCalls()-before,1,"The adjacent update/fetch transaction must prepare exactly once");
      assert.deepEqual(commands.map(args=>args[0]),["update","fetch"],"Each changed observed program requires one actual uncached pair");
      const currentLock=readFileSync(join(root,"Cargo.lock"),"utf8"), parsed=Bun.TOML.parse(currentLock);
      assert.deepEqual(parsed,require("smol-toml").parse(currentLock));
      const packages=testRecordV1(parsed).package;assert(Array.isArray(packages));assert.deepEqual(packages.map(value=>{const name=testRecordV1(value).name;assert.equal(typeof name,"string");return name;}).sort(),fixture.cargo.preparedPackages);
      if(preparedLock!==undefined)assert.equal(currentLock,preparedLock);preparedLock=currentLock;
      const actual=JSON.parse(await run("cargo",["metadata","--locked","--offline","--no-deps","--format-version=1"]));
      assert.deepEqual(actual.packages.map((value:any)=>value.name).sort(),fixture.cargo.preparedPackages);
      await run("cargo",["fetch","--locked","--offline"]);
    }
    assert.equal(existsSync(join(root, "target")), false, "Dependency synchronization must not compile deliverables");
    writeFileSync(join(root, "Cargo.lock"), fixture.cargo.invalidLock);
    await pair();
    assert.equal(readFileSync(join(root,"Cargo.lock"),"utf8"),preparedLock);
    await pair();
    await run("cargo",["fetch","--locked","--offline"]);
    assert.equal(existsSync(join(root, "target")), false, "Lock refresh must not compile deliverables");
    const custodySchemaPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧬️schema/🔣️.json";
    const schema=JSON.parse(readFileSync(join(workspace,custodySchemaPath),"utf8")),Ajv=require("ajv"),validate=new Ajv({strict:false}).compile(schema);
    const {inventorySchemaScopes}=await import("../../../🔍️discovery/🟦️.ts");
    const inventory=inventorySchemaScopes(workspace,JSON.parse(readFileSync(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"))),scope=inventory.modules.find(module=>module.modulePath===dirname(custodySchemaPath));
    writeFileSync(join(output,"cargo-custody-inventory.json"),JSON.stringify({scope,diagnostics:inventory.diagnostics},null,2)+"\n");
    assert.ok(scope?.level);assert.equal(scope.scopeId,"repo.library.workspaces.cargo.preparation.custody");const document=scope.documents[0];assert.ok(document);assert.deepEqual([...document.exports].sort(),Object.keys(schema.$defs).sort());
    assert.equal(scope.hashes["🔣️.json"],new Bun.CryptoHasher("sha256").update(readFileSync(join(workspace,custodySchemaPath))).digest("hex"));
    assert.deepEqual(inventory.diagnostics.filter(row=>row.path.startsWith(dirname(custodySchemaPath))),[]);
    const {cargoPreparationProgramSourcesV1}=await import(pathToFileURL(join(workspace,custodySchemaPath.replace("/🧬️schema/🔣️.json","/🟦️.ts"))).href),ts=require("typescript");
    const importRoot=join(root,"program-imports");mkdirSync(importRoot,{recursive:true});
    for(const row of fixture.programImports){
      const entry=join(importRoot,"entry.ts"),source=`import value from ${row.literal};export default value;\n${row.shadow?`// ${row.shadow}\n`:""}`;
      writeFileSync(entry,source);writeFileSync(join(importRoot,row.path),"export default 7;\n");if(row.shadow)writeFileSync(join(importRoot,row.shadow),"export default 8;\n");
      const parsed=ts.createSourceFile(entry,source,ts.ScriptTarget.Latest,true),specifier=parsed.statements[0].moduleSpecifier.text;
      assert.equal(specifier,row.path);assert.equal(Bun.resolveSync(specifier,importRoot),join(importRoot,row.path));
      const program=await cargoPreparationProgramSourcesV1(root,entry,preparationFor(controller.signal));assert.ok(program.sources.some((input:any)=>input.path===entry));assert.ok(program.sources.some((input:any)=>input.path===join(importRoot,row.path)));assert.deepEqual(program.resolutions.filter((edge:any)=>edge.source===entry),[{source:entry,specifier:row.path,selected:join(importRoot,row.path)}]);
    }
    const pairPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
    assert.equal(existsSync(join(workspace,pairPath)),true);
    const recipe=join(root,"owned/📜️script.ts"),initial=fixture.cargo.preparationScript;
    for(const row of fixture.custodyMutations){
      writeFileSync(join(root,"owned-input.txt"),fixture.cargo.ownedInput);writeFileSync(join(root,"owned/program.js"),fixture.cargo.programSource);rmSync(join(root,"owned/program.ts"),{force:true});
      const script=row.kind==="read-race"?initial.replace('observeCargoPreparationInputV1(input,"file",bytes);','writeFileSync(input,"8");observeCargoPreparationInputV1(input,"file",bytes);'):row.kind==="unknown"?initial.replace("completeCargoPreparationObservationV1();",""):row.kind==="member"?initial.replace(String.raw`member-manifests = [\"Cargo.toml\", \"generated/Cargo.toml\"]`,String.raw`member-manifests = [\"Cargo.toml\", \"generated/Cargo.toml\", \"additional/*/Cargo.toml\"]`):initial;
      if(row.kind==="member")assert.notEqual(script,initial);
      writeFileSync(recipe,script);const calls=["unknown","read-race"].includes(row.kind)?0:row.kind==="lock"?2:1;
      await runNativeDependencyFixtureNegativeV1(workspace,output,root,invocation,parent,"mutation",row.id,calls,new RegExp(row.expected));
      rmSync(join(root,"owned/program.ts"),{force:true});writeFileSync(join(root,"owned/program.js"),fixture.cargo.programSource);
      rmSync(join(root,"additional"),{recursive:true,force:true});
      writeFileSync(recipe,initial);writeFileSync(join(root,"owned-input.txt"),fixture.cargo.ownedInput);writeFileSync(join(root,"Cargo.lock"),preparedLock!);
    }
    const abort=new AbortController();const cancelledControl=controlFor(abort.signal),cancelledPreparation=preparationFor(abort.signal);abort.abort();await assert.rejects(withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",cancelledControl,async()=>assert.fail("Cancelled preparation must not run Cargo"),identity,cancelledPreparation));
    assert.equal(validate({}),false);
    const ordered=fixture.dependencyPreparation;
    writeFileSync(join(root,"generated/Cargo.toml"),readFileSync(join(root,"generated/Cargo.toml"),"utf8")+ordered.metadata);
    writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);rmSync(join(root,"generated/child-output.txt"),{force:true});
    writeFileSync(recipe,initial.replace("observeCargoPreparationSourceV1(import.meta.path);","observeCargoPreparationSourceV1(import.meta.path);\n"+ordered.parentRead));
    let dependencyCalls=0;
    await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",controlFor(controller.signal),async(args:string[])=>{dependencyCalls++;assert.equal(readFileSync(join(root,"generated/child-output.txt"),"utf8"),"7");const oracle=await rawCargo(["metadata","--manifest-path",join(root,"Cargo.toml"),"--locked","--offline","--no-deps","--format-version=1"]);const tree=JSON.parse(oracle);assert.deepEqual(tree.packages.map((row:any)=>row.name).sort(),fixture.cargo.preparedPackages);},identity,preparationFor(controller.signal));
    assert.equal(dependencyCalls,2);
    const generated=fixture.generatedPreparation;rmSync(join(root,"additional"),{recursive:true,force:true});
    const additional={"Cargo.toml":generated.rootManifest,"additional/new/Cargo.toml":generated.manifest,"additional/new/lib.rs":"","additional/new/📜️script.ts":generated.script};
    const publish=`for(const [path,content] of Object.entries(${JSON.stringify(additional)})){const target=resolve(process.cwd(),path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,content as string);observeCargoPreparationOutputV1(target,content as string);}`;
    writeFileSync(recipe,initial.replace("completeCargoPreparationObservationV1();",publish+"\ncompleteCargoPreparationObservationV1();"));
    let generatedCalls=0;
    await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",controlFor(controller.signal),async(args:string[])=>{
      generatedCalls++;assert.equal(readFileSync(join(root,"additional/new/child-output.txt"),"utf8"),"7");
      await rawCargo([...args,"--offline"]);
      const oracle=await rawCargo(["metadata","--manifest-path",join(root,"Cargo.toml"),"--locked","--offline","--no-deps","--format-version=1"]);assert.deepEqual(JSON.parse(oracle).packages.map((row:any)=>row.name).sort(),generated.expectedPackages);
    },identity,preparationFor(controller.signal));
    assert.equal(generatedCalls,2);assert.equal(existsSync(join(root,"target")),false);
    const ancestor=fixture.ancestorPreparation;rmSync(join(root,"additional"),{recursive:true,force:true});
    writeFileSync(join(root,"Cargo.toml"),fixture.cargo.preparedFiles["Cargo.toml"]);writeFileSync(join(root,"generated/Cargo.toml"),fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata);
    const ancestorFiles={"Cargo.toml":ancestor.rootManifest,"additional/new/Cargo.toml":generated.manifest,"additional/new/lib.rs":"","additional/new/📜️script.ts":generated.script};
    const ancestorPublish=`for(const [path,content] of Object.entries(${JSON.stringify(ancestorFiles)})){const target=resolve(process.cwd(),"..",path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,content as string);observeCargoPreparationOutputV1(target,content as string);}`;
    writeFileSync(join(root,"generated/📜️script.ts"),ordered.script.replace('import {writeFileSync}', 'import {mkdirSync,writeFileSync}').replace('import {resolve}', 'import {dirname,resolve}').replace("completeCargoPreparationObservationV1();",ancestorPublish+"\ncompleteCargoPreparationObservationV1();"));
    const parentRecipe=ordered.script.replace('import {writeFileSync}', 'import {readFileSync,writeFileSync}').replace('observeCargoPreparationSourceV1,observeCargoPreparationOutputV1','observeCargoPreparationSourceV1,observeCargoPreparationInputV1,observeCargoPreparationOutputV1').replace("observeCargoPreparationSourceV1(import.meta.path);","observeCargoPreparationSourceV1(import.meta.path);\n"+ancestor.parentRead);
    writeFileSync(recipe,parentRecipe);let ancestorCalls=0;
    await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",controlFor(controller.signal),async(args:string[])=>{
      ancestorCalls++;assert.equal(readFileSync(join(root,"additional/new/child-output.txt"),"utf8"),"7");
      await rawCargo([...args,"--offline"]);
      const oracle=await rawCargo(["metadata","--manifest-path",join(root,"Cargo.toml"),"--locked","--offline","--no-deps","--format-version=1"]);assert.deepEqual(JSON.parse(oracle).packages.map((row:any)=>row.name).sort(),generated.expectedPackages);
    },identity,preparationFor(controller.signal));
    assert.equal(ancestorCalls,2);
    rmSync(join(root,"additional"),{recursive:true,force:true});writeFileSync(join(root,"Cargo.toml"),fixture.cargo.preparedFiles["Cargo.toml"]);writeFileSync(join(root,"generated/Cargo.toml"),fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata);writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);
    const changed=fixture.changedPreparation,changedManifest=fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata.replace('command=["prepare"]',`command=${JSON.stringify(changed.command)}`),change=`const target=resolve(process.cwd(),"generated/Cargo.toml"),content=${JSON.stringify(changedManifest)};writeFileSync(target,content);observeCargoPreparationOutputV1(target,content);`;
    writeFileSync(recipe,ordered.script.replace("completeCargoPreparationObservationV1();",change+"\ncompleteCargoPreparationObservationV1();"));
    await runNativeDependencyFixtureNegativeV1(workspace,output,root,invocation,parent,"changed","changed-preparation",0,new RegExp(changed.expected));





    const scoped=fixture.selectedPreparation;
    const selectionSchema=JSON.parse(readFileSync(join(workspace,custodySchemaPath),"utf8")).$defs.CargoPreparationSelectionV1;
    const selectionValidator=new (require("ajv"))({strict:true}).compile(selectionSchema);
    const {parseCargoPreparationSelectionV1}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts")).href);
    assert.equal(selectionValidator(scoped.selection),true);assert.deepEqual(parseCargoPreparationSelectionV1(scoped.selection),scoped.selection);
    for(const value of scoped.rejectedSelections){assert.equal(selectionValidator(value),false);assert.throws(()=>parseCargoPreparationSelectionV1(value));}
    writeFileSync(join(root,"Cargo.toml"),fixture.cargo.preparedFiles["Cargo.toml"].replace('members = [".", "generated"]','members = [".", "generated", "unrelated"]').replace('member-manifests = ["Cargo.toml", "generated/Cargo.toml"]','member-manifests = ["Cargo.toml", "generated/Cargo.toml", "unrelated/Cargo.toml"]').replace('owner-manifests = []','owner-manifests = ["foreign/Cargo.toml", "selected/Cargo.toml"]').replace('exclude-patterns = []','exclude-patterns = ["foreign/child", "foreign/unused"]'));
    writeFileSync(recipe,parentRecipe.replace(ancestor.parentRead,""));
    writeFileSync(join(root,"generated/Cargo.toml"),fixture.cargo.preparedFiles["generated/Cargo.toml"]+ordered.metadata);writeFileSync(join(root,"generated/📜️script.ts"),ordered.script);
    mkdirSync(join(root,"unrelated"),{recursive:true});writeFileSync(join(root,"unrelated/Cargo.toml"),scoped.unusedManifest);writeFileSync(join(root,"unrelated/lib.rs"),"");writeFileSync(join(root,"unrelated/📜️script.ts"),scoped.unusedScript);
    for(const [path,content] of Object.entries(scoped.foreignFiles)){const target=join(root,path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,content as string);}writeFileSync(join(root,"foreign/Cargo.lock"),scoped.foreignLock);
    let scopedCalls=0;
    await withPreparedCargoDependencyPairV1(storage,root,scoped.selection.manifest,controlFor(controller.signal),async(args:string[])=>{
      scopedCalls++;assert.equal(readFileSync(join(root,"generated/child-output.txt"),"utf8"),"7");
      await rawCargo([...args,"--offline"]);
      const metadata=await rawCargo(["metadata","--locked","--offline","--no-deps","--format-version=1","--manifest-path",scoped.selection.manifest]);assert.deepEqual(JSON.parse(metadata).packages.map((pkg:any)=>pkg.name).sort(),scoped.expectedPackages);
      const tree=await rawCargo(["tree","--locked","--offline","--prefix","none","--format","{p}","--manifest-path",scoped.selection.manifest,"-p",...scoped.selection.packages]);assert.deepEqual(tree.trim().split("\n").map(line=>line.split(" ")[0]).sort(),scoped.expectedTreePackages);
      assert.deepEqual(require("smol-toml").parse(readFileSync(join(root,"selected/Cargo.lock"),"utf8")),Bun.TOML.parse(readFileSync(join(root,"selected/Cargo.lock"),"utf8")));
    },identity,preparationFor(controller.signal),[],scoped.selection.packages);
    assert.equal(scopedCalls,2);assert.equal(readFileSync(join(root,"foreign/Cargo.lock"),"utf8"),scoped.foreignLock);
    const launch=scoped.invocation,environment=Object.fromEntries(Object.entries(launch.environment).map(([key,value])=>[key,(value as string).replaceAll("{root}",root)]));
    const request=cargoWorkspacePreparationInvocationV1(storage,root,launch.args,root,environment);
    assert.deepEqual(request,{command:process.execPath,args:[join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/📜️script.ts"),"synchronize","--manifest",scoped.selection.manifest,...launch.packages.flatMap((name:string)=>["--package",name])],cwd:root,environment:{...environment,SEMIO_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_STORAGE:JSON.stringify(storage)}});
    assert.equal(new (require("ajv"))({strict:true}).compile(schema.$defs.CargoPreparationInvocationV1)(request),true);assert.equal(cargoWorkspacePreparationInvocationV1(storage,root,[launch.rejectedCommand],root,environment),undefined);assert.throws(()=>cargoWorkspacePreparationInvocationV1(storage,root,launch.args,root,{...environment,SEMIO_CARGO_PREPARATION_ACTIVE:"current-recipe"}),/Cargo recursion/);
    const consumed=JSON.parse(await run("cargo",["metadata","--locked","--offline","--no-deps","--format-version=1","--manifest-path","selected/main/Cargo.toml"]));assert.equal(resolve(consumed.workspace_root),join(root,"selected"));assert.deepEqual(consumed.packages.map((pkg:any)=>pkg.name).sort(),scoped.expectedPackages);assert.equal(readFileSync(join(root,"foreign/Cargo.lock"),"utf8"),scoped.foreignLock);

    assert.ok(progress.length>0);assert.ok(progress.includes("yield"));passed = true;
  } finally {
    if (inheritedBrowserPath === undefined) delete process.env.PLAYWRIGHT_BROWSERS_PATH;
    else process.env.PLAYWRIGHT_BROWSERS_PATH = inheritedBrowserPath;
    controller.abort(); clearTimeout(timer); process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (passed) rmSync(root, { recursive: true, force: true });
  }
}
