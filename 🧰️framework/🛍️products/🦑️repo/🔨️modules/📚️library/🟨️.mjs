import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
export {cacheInternals,libraryBootstrap,nativeSourceWatchPlanV1,discoverBuildProjectsV1,discoverBuildDependenciesV1} from "./🏗️build/🕸️graph/📥️inference/🟨️.mjs";

/** 📌️ Reads the exact portable graph pinned by this owned build invocation. */
function graphPublication(workspaceRoot){
 const path=process.env.SEMIO_BUILD_GRAPH_PATH??join(workspaceRoot,".🧬semio/🦑️repo/⚡️cache/build/graph.json"),source=readFileSync(path,"utf8"),digest=createHash("sha256").update(source).digest("hex");
 if(process.env.SEMIO_BUILD_GRAPH_DIGEST&&digest!==process.env.SEMIO_BUILD_GRAPH_DIGEST)throw Error("Pinned build graph source changed");
 const graph=JSON.parse(source);if(graph.version!==1||!graph.projects||!Array.isArray(graph.dependencies))throw Error("Prepared portable build graph required");
 return {graph,digest};
}

/** 🔌️ Projects only portable process execution onto Nx's internal executor contract. */
function nxTarget(target){
 const {execution,configurations,...properties}=target,{kind,ownerArguments,...options}=execution;
 const result={...properties,executor:kind==="noop"?"nx:noop":ownerArguments?"@semio-tech/repo-lib:owner-command":"nx:run-commands",...(kind==="noop"?{}:{options})};
 if(configurations)result.configurations=Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>{const {execution,...properties}=configuration;return[name,{...properties,...execution}];}));
 return result;
}

/** 📽️ Projects one portable publication without discovering implementation-specific projects. */
export function projectBuildGraphForNxV1(graph){
 const projects=Object.fromEntries(Object.entries(graph.projects).map(([name,project])=>[name,{...project,namedInputs:{...graph.namedInputs,...project.namedInputs},targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,nxTarget(target)]))}]));
 return {projects,externalNodes:graph.externalNodes??{}};
}

/** 🧩️ Makes every backend consume the same complete project publication. */
async function nodes(files,_options,context){
 const {graph}=graphPublication(context.workspaceRoot);
 return [[files.includes("📋️project.json")?"📋️project.json":files[0]??"📋️project.json",projectBuildGraphForNxV1(graph)]];
}

/** 🪢️ Projects portable dependencies without reconstructing a second graph. */
export function createDependencies(_options,context){return graphPublication(context.workspaceRoot).graph.dependencies;}

/** 🧬️ Binds dependency reuse to the exact published portable project and edge authority. */
export function dependencyResolutionAuthority(workspaceRoot,projects){
 const {graph,digest}=graphPublication(workspaceRoot),owners=Object.entries(projects).map(([name,project])=>[name,project.root]).sort(([a],[b])=>a.localeCompare(b));
 for(const[name,root]of owners)if(graph.projects[name]&&graph.projects[name].root!==root)throw Error(`Build project authority differs: ${name}`);
 return createHash("sha256").update(JSON.stringify([resolve(workspaceRoot),digest,owners])).digest("hex");
}

export default {name:"@repo/emoji-project-json",createNodesV2:["📋️project.json",nodes],createDependencies};
