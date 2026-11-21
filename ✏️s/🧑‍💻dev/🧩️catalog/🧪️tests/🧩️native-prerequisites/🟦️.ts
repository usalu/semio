import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import fixture from "./🧫️fixtures/🧩️owners/🔣️.json";
import {catalogBuildContributionV1} from "../../📣️publication/🟦️.ts";
import corpus from "./🧫️fixtures/🎮️renderer-inputs/🔣️.json";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { declareNxOracleNamespaceV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🔌️nx-oracle/🟦️.ts";
import { componentStagePlanV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📋️contract/🟦️.ts";

const workspace = resolve(process.env.SEMIO_WORKSPACE_ROOT ?? join(import.meta.dir, "../../../../.."));
const readWorkspaceText = (file: string) => readFileSync(join(workspace, file), "utf8");
const contribution = () => catalogBuildContributionV1({ readText: readWorkspaceText });
const nxRequire = createRequire(join(workspace, ".nx/installation/package.json"));

function nxOracle(id: string) {
  declareNxOracleNamespaceV1(workspace);
  return nxRequire(id);
}

const inference = () => import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);

async function preparationTargets() {
  const { cacheInternals } = await inference();
  const catalog = contribution();
  const manifest = corpus.crate + "/Cargo.toml";
  const publishedSources: { cratePath: string }[] = JSON.parse(readWorkspaceText(join(catalog.publication.outputRoot, "🗂️sources.json")));
  expect(publishedSources.some(row => row.cratePath === corpus.crate)).toBe(true);
  await cacheInternals.playgroundSessionTargets([manifest], workspace, [catalog]);
  const capabilities = createNativeBuildCallerCapabilitiesV1(workspace, ["show", "projects"], "nx", process.env);
  const envelope = createScriptProcessEnvelope({ version: 1, owner: "s-dev-native-prerequisites", maximumElapsedMilliseconds: 25_000 }, capabilities, Date.now());
  return withScriptProcessEnvelope(envelope, invocation => cacheInternals.playgroundPreparationTargets(
    [manifest], workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript", {}, [catalog], { workspaceRoot: workspace, invocation },
  ));
}

test("current graph owners precede native and transitive component compilation in Nx", async () => {
  const { cacheInternals } = await inference();
  const contracts = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts;
  const { createTaskGraph } = nxOracle("nx/src/tasks-runner/create-task-graph");
  const selections = [...fixture.owners.map(row => ({ packageRoot: row.packageRoot, target: row.target.split(":")[0] + ":build", required: [row.target] })), ...fixture.consumers];
  const manifests = new Map(), closures = new Map();
  for (const row of selections) {
    const name = row.target.split(":")[0]!, target = row.target.split(":")[1]!;
    const project: { name: string; root: string; targets: Record<string, { executor: string; options: { command: string }; dependsOn?: string[] }> } = { name, root: row.packageRoot, targets: { [target]: { executor: "nx:run-commands", options: { command: "repo ./📜️script.ts build" } } } };
    cacheInternals.withNativePreparation(project, workspace, contracts, manifests, new Map(), closures);
    const graph: { nodes: Record<string, { name: string; type: string; data: typeof project }>; dependencies: Record<string, unknown[]> } = { nodes: { [name]: { name, type: "lib", data: project } }, dependencies: { [name]: [] } };
    for (const id of project.targets[target]!.dependsOn ?? []) {
      const owner = id.split(":")[0]!, generator = id.split(":")[1]!;
      graph.nodes[owner] ??= { name: owner, type: "lib", data: { name: owner, root: ".", targets: {} } };
      graph.nodes[owner]!.data.targets[generator] = { executor: "nx:run-commands", options: { command: "repo ./📜️script.ts generate" } };
      graph.dependencies[owner] ??= [];
    }
    const tasks = createTaskGraph(graph, {}, [name], [target], undefined, {}, false);
    for (const required of row.required) {
      expect(tasks.tasks[required], row.target).toBeDefined();
      expect(tasks.dependencies[row.target], row.target).toContain(required);
    }
  }
}, 30_000);


test("component description verifies the shipped bytes with its native emitter prerequisite", async () => {
  const { cacheInternals } = await inference();
  const crate = corpus.crate, emitter = corpus.componentDescribe.emitter;
  const targets = await cacheInternals.componentBuildTargets(crate, workspace);
  expect(targets.component.dependsOn).toContain(emitter);
  expect(targets.component.options.command).toContain("component --manifest");
  const { createTaskGraph } = nxOracle("nx/src/tasks-runner/create-task-graph");
  const graph = { nodes: { consumer: { name: "consumer", type: "lib", data: { name: "consumer", root: crate, targets } } }, dependencies: { consumer: [] } } as any;
  for (const dependency of targets.component.dependsOn) {
    const split = dependency.lastIndexOf(":"), name = dependency.slice(0, split), target = dependency.slice(split + 1);
    graph.nodes[name] = { name, type: "lib", data: { name, root: ".", targets: { [target]: { executor: "nx:run-commands", options: { command: "repo ./📜️script.ts build" } } } } };
    graph.dependencies[name] = [];
  }
  expect(createTaskGraph(graph, {}, ["consumer"], ["component"], undefined, {}, false).dependencies["consumer:component"]).toContain(emitter);
  for (const facet of ["model", "browser"] as const) {
    expect(componentStagePlanV1({ version: 1, facet }, true)).toEqual(corpus.componentDescribe[facet]);
  }
});


test("a session reads the committed semantic descriptors of its closure without any component build", async () => {
  const { cacheInternals } = await inference();
  const crate = corpus.crate;
  const targets = await cacheInternals.playgroundSessionTargets([crate + "/Cargo.toml"], workspace,[contribution()]);
  expect(targets["session-draw"].dependsOn).toEqual(["@semio-tech/s-dev-composition:generate"]);
  expect(targets["session-draw"].inputs).toContain("{workspaceRoot}/🌎️hub/🧩️compositions/🖍️draw/🧩️component/🛂️.descriptor.semio");
  const {createTaskGraph} = nxOracle("nx/src/tasks-runner/create-task-graph");
  const graph:any = {nodes:{registry:{name:"registry",type:"lib",data:{name:"registry",root:".",targets:{...Object.fromEntries(Object.entries(targets).map(([name,target])=>{if(target===null||typeof target!=="object"||Array.isArray(target))throw Error("Actual session target must be an object: "+name);return [name,{...target,executor:"nx:run-commands"}];})),generate:{executor:"nx:run-commands",options:{command:"repo ./📜️script.ts generate"}}}}},"@semio-tech/draw-plugin":{name:"@semio-tech/draw-plugin",type:"lib",data:{name:"@semio-tech/draw-plugin",root:crate,targets:{describe:{executor:"nx:run-commands",options:{command:"repo ./📜️script.ts describe"}}}}}},dependencies:{registry:[],"@semio-tech/draw-plugin":[],"@semio-tech/s-dev-composition":[]}};
  graph.nodes["@semio-tech/s-dev-composition"]={name:"@semio-tech/s-dev-composition",type:"lib",data:{name:"@semio-tech/s-dev-composition",root:"✏️s/🧑‍💻dev",targets:{generate:{executor:"nx:run-commands",options:{command:"repo ./📜️script.ts catalog-publication generate"}}}}};
  expect(createTaskGraph(graph,{},["registry"],["session-draw"],undefined,{},false).dependencies["registry:session-draw"]).toEqual(["@semio-tech/s-dev-composition:generate"]);
});

test("browser and native renderer inputs precede consumers in the installed Nx graph", async () => {
  const targets = await preparationTargets();
  const {createTaskGraph}=nxOracle("nx/src/tasks-runner/create-task-graph");
  for(const profile of corpus.profiles){
    for(const renderer of corpus.browserRenderers){
      const name=`prepare-${corpus.variant}-${renderer}-${profile}`,target=targets[name];
      const graph:any={nodes:{consumer:{name:"consumer",type:"lib",data:{name:"consumer",root:".",targets:{[name]:{...target,executor:"nx:run-commands"}}}}},dependencies:{consumer:[]}};
      for(const dependency of target.dependsOn){const split=dependency.lastIndexOf(":"),owner=dependency.slice(0,split),command=dependency.slice(split+1);graph.nodes[owner]??={name:owner,type:"lib",data:{name:owner,root:".",targets:{}}};graph.nodes[owner].data.targets[command]={executor:"nx:run-commands",options:{command:"repo ./📜️script.ts build"}};graph.dependencies[owner]??=[];}
      const prerequisites = createTaskGraph(graph, {}, ["consumer"], [name], undefined, {}, false).dependencies["consumer:" + name];
      expect([...prerequisites].sort()).toEqual([...corpus.browserPrerequisites[renderer as keyof typeof corpus.browserPrerequisites]].sort());
    }
    expect(targets[["serve", corpus.variant, "react", profile].join("-")].dependsOn).toEqual([corpus.sessionProducer]);
    for (const operation of ["run", "smoke"]) {
      const target = targets[[operation, corpus.variant, "native", profile].join("-")];
      expect(target.options.forwardAllArgs).toBe(true);
      expect(target.dependsOn).toEqual([["prepare", corpus.variant, "native", profile].join("-"), corpus.nativeHostProducer]);
    }
  }
  expect(targets[`build-${corpus.variant}-react-release`].dependsOn).toContain(corpus.flowProducer);
}, 30_000);

test("native foreground tasks retain their child failure outcome",async()=>{
  const targets = await preparationTargets();
  for(const profile of corpus.profiles)for(const operation of ["run","smoke"])expect(targets[`${operation}-${corpus.variant}-native-${profile}`].continuous).toBe(corpus.nativeProcessStatus.continuous);
  const execute=nxOracle("nx/src/executors/run-commands/run-commands.impl").default;
  const outcome=await execute({command:`bun -e "process.exit(${corpus.nativeProcessStatus.childFailureExit})"`,cwd:workspace,parallel:false,color:false,__unparsed__:[]},{root:workspace,isVerbose:false});
  expect(outcome.success).toBe(false);
}, 30_000);
