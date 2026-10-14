import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import fixture from "./🧫️fixtures/🧩️owners/🔣️.json";
import {catalogNxContributionV1} from "../../📣️publication/🟦️.ts";

test("current graph owners precede native and transitive component compilation in Nx", async () => {
  const workspace = process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd();
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


test("component description verifies the shipped bytes with its native emitter prerequisite", async () => {
  const workspace = process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd();
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const crate = "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust", emitter = "@semio-tech/os-plugin-describe-rs:build";
  const targets = await cacheInternals.componentBuildTargets(crate, workspace);
  expect(targets.component.dependsOn).toContain(emitter);
  expect(targets.component.options.command).toContain("component --manifest");
  const { createTaskGraph } = createRequire(import.meta.url)("nx/src/tasks-runner/create-task-graph");
  const graph = { nodes: { consumer: { name: "consumer", type: "lib", data: { name: "consumer", root: crate, targets } } }, dependencies: { consumer: [] } } as any;
  for (const dependency of targets.component.dependsOn) {
    const split = dependency.lastIndexOf(":"), name = dependency.slice(0, split), target = dependency.slice(split + 1);
    graph.nodes[name] = { name, type: "lib", data: { name, root: ".", targets: { [target]: { executor: "nx:run-commands", options: { command: "bun ./📜️script.ts build" } } } } };
    graph.dependencies[name] = [];
  }
  expect(createTaskGraph(graph, {}, ["consumer"], ["component"], undefined, {}, false).dependencies["consumer:component"]).toContain(emitter);
  const driver = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
  const { componentDriverStagesV1 } = await import(pathToFileURL(join(workspace, driver, "🏗️build/🧩️driver-stages/🟦️.ts")).href);
  const { componentDeliverableV1, SHIPPED_COMPONENT_PROFILE } = await import(pathToFileURL(join(workspace, driver, "🖨️describe/🏷️component-receipt/🟦️.ts")).href);
  const manifest = join(workspace, crate, "Cargo.toml"), name = (Bun.TOML.parse(readFileSync(manifest, "utf8")) as { package: { name: string } }).package.name;
  const deliverable = componentDeliverableV1(join(workspace, crate), name, SHIPPED_COMPONENT_PROFILE);
  const { admitNativeBuildPlanV1 } = await import(pathToFileURL(join(workspace, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts")).href);
  const plan = admitNativeBuildPlanV1({ version: 1, passes: [{ id: "shipped", manifest, packages: [{ name, manifest }], target: "wasm32-wasip2", profile: SHIPPED_COMPONENT_PROFILE, output: "dist/components/release" }] });
  const pass = plan.passes[0], stages = componentDriverStagesV1(workspace)({ pass, package: pass.packages[0], wasm: deliverable.wasm, staged: deliverable.wasm });
  expect(stages.find((stage: any) => stage.name === "describe")).toBeDefined();
  expect(stages.find((stage: any) => stage.name === "publish").after).toContain("describe");
  expect(stages.find((stage: any) => stage.name === "publish").after).toContain("probe");
  console.log("[DEBUG] actual component emitter precedes shipped-byte driver; materialization publication waits for semantic description and probe");
});


test("a session reads the committed semantic descriptors of its closure without any component build", async () => {
  const workspace = process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd();
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const crate = "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust";
  const targets = await cacheInternals.playgroundSessionTargets([crate + "/Cargo.toml"], workspace,[catalogNxContributionV1()]);
  expect(targets["session-draw"].dependsOn).toEqual(["@semio-tech/s-dev-composition:generate"]);
  expect(targets["session-draw"].inputs).toContain("{workspaceRoot}/🌎️hub/🧩️compositions/🖍️draw/🧩️component/🛂️.descriptor.semio");
  const {createTaskGraph} = createRequire(import.meta.url)("nx/src/tasks-runner/create-task-graph");
  const graph:any = {nodes:{registry:{name:"registry",type:"lib",data:{name:"registry",root:".",targets:{...Object.fromEntries(Object.entries(targets).map(([name,target])=>{if(target===null||typeof target!=="object"||Array.isArray(target))throw Error("Actual session target must be an object: "+name);return [name,{...target,executor:"nx:run-commands"}];})),generate:{executor:"nx:run-commands",options:{command:"bun ./📜️script.ts generate"}}}}},"@semio-tech/draw-plugin":{name:"@semio-tech/draw-plugin",type:"lib",data:{name:"@semio-tech/draw-plugin",root:crate,targets:{describe:{executor:"nx:run-commands",options:{command:"bun ./📜️script.ts describe"}}}}}},dependencies:{registry:[],"@semio-tech/draw-plugin":[],"@semio-tech/s-dev-composition":[]}};
  graph.nodes["@semio-tech/s-dev-composition"]={name:"@semio-tech/s-dev-composition",type:"lib",data:{name:"@semio-tech/s-dev-composition",root:"✏️s/🧑‍💻dev",targets:{generate:{executor:"nx:run-commands",options:{command:"bun ./📜️script.ts catalog-publication generate"}}}}};
  expect(createTaskGraph(graph,{},["registry"],["session-draw"],undefined,{},false).dependencies["registry:session-draw"]).toEqual(["@semio-tech/s-dev-composition:generate"]);
});

test("browser and native renderer inputs precede consumers in the installed Nx graph", async () => {
  const workspace=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),require=createRequire(import.meta.url);
  const corpus=JSON.parse(readFileSync(join(import.meta.dir,"./🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const {cacheInternals}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const manifests=JSON.parse(readFileSync(join(workspace,catalogNxContributionV1().publication.outputRoot,"🗂️sources.json"),"utf8")).map((row:{cratePath:string})=>row.cratePath+"/Cargo.toml");
  const targets=cacheInternals.playgroundPreparationTargets(manifests,workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",{},[catalogNxContributionV1()]);
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

test("native foreground tasks retain their child failure outcome",async()=>{
  const workspace=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),corpus=JSON.parse(readFileSync(join(import.meta.dir,"./🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const {cacheInternals}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const manifests=JSON.parse(readFileSync(join(workspace,catalogNxContributionV1().publication.outputRoot,"🗂️sources.json"),"utf8")).map((row:{cratePath:string})=>row.cratePath+"/Cargo.toml");
  const targets=cacheInternals.playgroundPreparationTargets(manifests,workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",{},[catalogNxContributionV1()]);
  for(const profile of corpus.profiles)for(const operation of ["run","smoke"])expect(targets[`${operation}-${corpus.variant}-native-${profile}`].continuous).toBe(corpus.nativeProcessStatus.continuous);
  const execute=createRequire(import.meta.url)("nx/src/executors/run-commands/run-commands.impl").default;
  const outcome=await execute({command:`bun -e "process.exit(${corpus.nativeProcessStatus.childFailureExit})"`,cwd:workspace,parallel:false,color:false,__unparsed__:[]},{root:workspace,isVerbose:false});
  expect(outcome.success).toBe(false);
});
