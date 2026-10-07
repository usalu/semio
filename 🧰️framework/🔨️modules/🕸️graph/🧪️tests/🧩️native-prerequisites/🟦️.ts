import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import fixture from "../../🧫️fixtures/🧩️native-prerequisites/🔣️.json";

test("current graph owners precede native and transitive component compilation in Nx", async () => {
  const workspace = process.env.NX_WORKSPACE_ROOT ?? process.cwd();
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const contracts = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts;
  const require = createRequire(import.meta.url);
  const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph");
  const selections = [...fixture.owners.map(row => ({ packageRoot: row.packageRoot, target: row.target.split(":")[0] + ":build", required: [row.target] })), ...fixture.consumers];
  for (const row of selections) {
    const name = row.target.split(":")[0]!, target = row.target.split(":")[1]!;
    const project: { name: string; root: string; targets: Record<string, { executor: string; options: { command: string }; dependsOn?: string[] }> } = { name, root: row.packageRoot, targets: { [target]: { executor: "nx:run-commands", options: { command: "bun ./📜️script.ts build" } } } };
    cacheInternals.withNativePreparation(project, workspace, contracts);
    const graph: { nodes: Record<string, { name: string; type: string; data: typeof project }>; dependencies: Record<string, unknown[]> } = { nodes: { [name]: { name, type: "lib", data: project } }, dependencies: { [name]: [] } };
    for (const id of project.targets[target]!.dependsOn ?? []) {
      const owner = id.split(":")[0]!, generator = id.split(":")[1]!;
      graph.nodes[owner] ??= { name: owner, type: "lib", data: { name: owner, root: ".", targets: {} } };
      graph.nodes[owner]!.data.targets[generator] = { executor: "nx:run-commands", options: { command: "bun ./📜️script.ts generate" } };
      graph.dependencies[owner] ??= [];
    }
    const tasks = createTaskGraph(graph, {}, [name], [target], undefined, {}, false);
    for (const required of row.required) {
      expect(tasks.tasks[required]).toBeDefined();
      expect(tasks.dependencies[row.target]).toContain(required);
    }
  }
  console.log(`[DEBUG] native graph prerequisites selections=${selections.length} owners=${fixture.owners.length}`);
}, 30_000);


test("component description declares its native emitter prerequisite", async () => {
  const workspace = process.env.NX_WORKSPACE_ROOT ?? process.cwd();
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const targets = await cacheInternals.componentTargets("🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust", workspace, []);
  expect(targets.describe.dependsOn).toContain("component-dev");
  expect(targets.describe.dependsOn).toContain("@semio-tech/os-plugin-describe-rs:build");
});


test("a session observes descriptors completed by its selected component closure", async () => {
  const workspace = process.env.NX_WORKSPACE_ROOT ?? process.cwd();
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const crate = "🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust";
  const targets = await cacheInternals.playgroundSessionTargets([crate + "/Cargo.toml"], workspace);
  expect(targets["session-draw"].dependsOn).toContain("@semio-tech/draw-plugin:describe");
  const {createTaskGraph} = createRequire(import.meta.url)("nx/src/tasks-runner/create-task-graph");
  const graph = {nodes:{registry:{name:"registry",type:"lib",data:{name:"registry",root:".",targets:{...Object.fromEntries(Object.entries(targets).map(([name,target])=>[name,{...target,executor:"nx:run-commands"}])),generate:{executor:"nx:run-commands",options:{command:"bun ./📜️script.ts generate"}}}}},"@semio-tech/draw-plugin":{name:"@semio-tech/draw-plugin",type:"lib",data:{name:"@semio-tech/draw-plugin",root:crate,targets:{describe:{executor:"nx:run-commands",options:{command:"bun ./📜️script.ts describe"}}}}}},dependencies:{registry:[],"@semio-tech/draw-plugin":[]}};
  expect(createTaskGraph(graph,{},["registry"],["session-draw"],undefined,{},false).dependencies["registry:session-draw"]).toContain("@semio-tech/draw-plugin:describe");
});

test("browser and native renderer inputs precede consumers in the installed Nx graph", async () => {
  const workspace=process.env.NX_WORKSPACE_ROOT??process.cwd(),require=createRequire(import.meta.url);
  const corpus=JSON.parse(readFileSync(join(import.meta.dir,"../../🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const {cacheInternals}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const targets=cacheInternals.playgroundPreparationTargets([corpus.crate+"/Cargo.toml"],workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
  const {createTaskGraph}=require("nx/src/tasks-runner/create-task-graph");
  for(const profile of corpus.profiles){
    for(const renderer of corpus.browserRenderers){
      const name=`prepare-${corpus.variant}-${renderer}-${profile}`,target=targets[name];
      const graph:any={nodes:{consumer:{name:"consumer",type:"lib",data:{name:"consumer",root:".",targets:{[name]:{...target,executor:"nx:run-commands"}}}}},dependencies:{consumer:[]}};
      for(const dependency of target.dependsOn){const split=dependency.lastIndexOf(":"),owner=dependency.slice(0,split),command=dependency.slice(split+1);graph.nodes[owner]??={name:owner,type:"lib",data:{name:owner,root:".",targets:{}}};graph.nodes[owner].data.targets[command]={executor:"nx:run-commands",options:{command:"bun ./📜️script.ts build"}};graph.dependencies[owner]??=[];}
      expect(createTaskGraph(graph,{},["consumer"],[name],undefined,{},false).dependencies[`consumer:${name}`]).toContain(corpus.flowProducer);
    }
    for(const operation of ["run","smoke"])expect(targets[`${operation}-${corpus.variant}-native-${profile}`].options.forwardAllArgs).toBe(true);
  }
  expect(targets[`build-${corpus.variant}-react-release`].dependsOn).toContain(corpus.flowProducer);
  console.log("[DEBUG] renderer inputs checked through installed Nx");
});

test("native publication and launch read the completed variant catalog", async () => {
  const workspace=process.env.NX_WORKSPACE_ROOT??process.cwd(),require=createRequire(import.meta.url),ts=require("typescript");
  const {mkdirSync,mkdtempSync,writeFileSync}=await import("node:fs"),{dirname,resolve,isAbsolute,relative}=await import("node:path");
  const {repoTestArtifactEnvironment}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts")).href);
  const root=mkdtempSync(join(repoTestArtifactEnvironment(workspace,"graph-native-prerequisites").SEMIO_TEST_ARTIFACT_DIR!,"native-session-catalog-"));
  const put=(path:string,value:unknown)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),typeof value==="string"?value:JSON.stringify(value));};
  const corpus=JSON.parse(readFileSync(join(import.meta.dir,"../../🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const registryPath="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",native=join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint");
  const {readGeneratedCatalogProjection}=await import(pathToFileURL(join(workspace,registryPath,"📖️catalog-view/🟦️.ts")).href);
  const scoped=`${registryPath}/dist/sessions/${corpus.variant}`;
  put(`${registryPath}/🤖️generated/🔌️plugins.json`,[]);put(`${registryPath}/🤖️generated/🎠️playgrounds.json`,[]);
  put(`${scoped}/🔌️plugins.json`,[corpus.entry]);put(`${scoped}/🎠️playgrounds.json`,[corpus.playground]);
  put(`${scoped}/🎮️playground-session/🟦️.ts`,`export const PLAYGROUND_SESSION=${JSON.stringify({variant:corpus.variant,plugins:[{pluginId:corpus.entry.pluginId}]})};`);
  put("runtime/🔣️runtime.json",{version:1,variant:corpus.variant,profile:"dev"});put("runtime/.nx-artifact.json",{});
  const declarations=(file:string,names:string[])=>{const source=ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true);return names.map(name=>source.statements.find((node:any)=>node.name?.text===name||node.declarationList?.declarations.some((row:any)=>row.name?.text===name)).getText(source)).join("\n");};
  let published:any,launched:any;
  const dependencies:any={BundleScript:class{repoRoot=root;},join,resolve,relative,isAbsolute,pathToFileURL,readFileSync,readGeneratedCatalogProjection,AbortController,process,console,registryPath,ownerPath:"owner",COMPONENT_MODULE_DIRECTORIES:[{pluginId:corpus.entry.pluginId,directoryName:corpus.entry.directoryName}],moduleDirectoryName:()=>corpus.entry.directoryName,pluginModulesRootIn:(repo:string,profile:string)=>join(repo,"modules",profile),nativeRuntimeDirectory:(path:string)=>path==="."?path:join(root,"runtime"),publishNativeRuntime:async(...args:any[])=>{published=args;},existsSync:()=>true,playgroundNativeHostArtifactV1:()=>process.execPath,nativeRendererBinary:()=>{throw Error("Unexpected fallback binary");},runNativeSession:async(...args:any[])=>{launched=args;},createAssetHttpServerV1:()=>{throw Error("No assets requested");},PLAYGROUND_ASSET_PROVIDERS_V1:[]};
  const load=(file:string,names:string[],result:string)=>new Function(...Object.keys(dependencies),ts.transpileModule(declarations(file,names).replaceAll("import.meta.dir",JSON.stringify(native)),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText+`;return ${result};`)(...Object.values(dependencies));
  const Publish=load(join(native,"📦️modules/📜️script.ts"),["PublishScript"],"PublishScript"),Run=load(join(native,"📜️script.ts"),["BOOT_AXIS_FLAGS","selection","RunScript"],"RunScript");
  await new Publish().run([corpus.variant,"dev"]);
  const expected=require("lodash/find")([corpus.entry],{pluginId:corpus.entry.pluginId});
  expect(published[3]).toEqual([{pluginId:expected.pluginId,wasm:join(root,expected.cratePath,"dist/component-dev",expected.wasmOut),descriptor:join(root,"modules/dev",expected.directoryName,"🔣️.json")}]);
  await new Run().run([corpus.variant,"dev",...corpus.nativeArgs]);
  expect(launched[1]).toEqual(["--plugin",corpus.variant,"--example","🎬️demo","--app",corpus.playground.app,"--brand",corpus.playground.brand,"--smoke"]);
  expect(launched[2].SEMIO_PLUGIN_MODULES).toBe(join(root,"runtime"));
  console.log("[DEBUG] native scoped publication and launch checked against independent lodash selection");
});

test("native foreground tasks retain their child failure outcome",async()=>{
  const workspace=process.env.NX_WORKSPACE_ROOT??process.cwd(),corpus=JSON.parse(readFileSync(join(import.meta.dir,"../../🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const {cacheInternals}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const targets=cacheInternals.playgroundPreparationTargets([corpus.crate+"/Cargo.toml"],workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
  for(const profile of corpus.profiles)for(const operation of ["run","smoke"])expect(targets[`${operation}-${corpus.variant}-native-${profile}`].continuous).toBe(corpus.nativeProcessStatus.continuous);
  const execute=createRequire(import.meta.url)("nx/src/executors/run-commands/run-commands.impl").default;
  const outcome=await execute({command:`bun -e "process.exit(${corpus.nativeProcessStatus.childFailureExit})"`,cwd:workspace,parallel:false,color:false,__unparsed__:[]},{root:workspace,isVerbose:false});
  expect(outcome.success).toBe(false);
});
