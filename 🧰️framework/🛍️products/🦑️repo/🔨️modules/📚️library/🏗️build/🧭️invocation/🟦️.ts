import {buildBackendNativeNamespacesV1} from "../🔌️backends/🟦️.ts";
import {buildBackendDefaultIdV1} from "../🔌️backends/🟦️.ts";
import {requireBuildBackendIdV1} from "../🔌️backends/🟦️.ts";
import { canonicalBuildTargetV1 } from "../🎯️targets/🟦️.ts";
import {buildOperationArgumentsV1} from "../⚙️operations/🔤️options/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{extractGraphPolicyV1}from"../🕸️graph/🕰️generations/🔌️load/🟨️.mjs";
import dependencySchema from "../🧬️schema/📥️dependencies.json" with { type: "json" };
import authoringSchema from "../🧬️schema/📥️authoring.json" with { type: "json" };
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readFileSync,statSync} from "node:fs";
import {join,resolve,isAbsolute} from "node:path";
import {checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildInvocationGraphSelectionV1,readBuildOwnerRegistryV1,type BuildGraphSelectionV1} from "../🕸️graph/🚪️admission/🟦️.ts";
import type { BuildGraphV1 } from "../🕸️graph/📋️model/🟦️.ts";
import {resolveTestLevel} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {semioBuildParallelFlag} from "../../🏃️process/🌿️environment/🟦️.ts";
import {buildModeEnvironment,readBuildMode} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {fixedTargetBuildMode} from "../🎚️mode/🟨️.mjs";
import policy from "../🔣️policy.json" with { type: "json" };
import projectSchema from "../🧬️schema/📋️project/🔣️.json" with { type: "json" };
import {readWorkerSwitchV1} from "../👷️workers/🟦️.ts";
import {declaredCapabilitiesV1} from "../📥️caller/🛰️service/🟦️.ts";

/** 📖️ Admits public help before storage while preserving an opaque child argument tail. */
export function buildInvocationRequestsHelpV1(args:readonly string[]):boolean {
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary);return head.length===0||head.some(argument=>argument==="--help"||argument==="-h");
}
const testTaxonomyV1=new Map<string,Readonly<{identity:string;value:Readonly<{testPhases?:readonly string[];testLevellessPhases?:readonly string[]}>}>>();

/** 🧊️ Reads the two test phase lists of the taxonomy once per file identity, so the several selections of one invocation do not parse the whole taxonomy again; a replaced or edited file has another identity and is read anew. */
function readTestTaxonomyV1(root:string):Readonly<{testPhases?:readonly string[];testLevellessPhases?:readonly string[]}> {
  const path=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),row=statSync(path,{bigint:true}),identity=[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":"),prior=testTaxonomyV1.get(path);
  if(prior?.identity===identity)return prior.value;
  const taxonomy=JSON.parse(readFileSync(path,"utf8")) as {testPhases?:readonly string[];testLevellessPhases?:readonly string[]},value=Object.freeze({testPhases:taxonomy.testPhases,testLevellessPhases:taxonomy.testLevellessPhases});
  testTaxonomyV1.set(path,{identity,value});
  return value;
}
/** 🧭️ Resolves public selections before shared task graph admission. */
export function resolveBuildSelectionV1(root:string,segments: string[]): { args: string[]; env: NodeJS.ProcessEnv; watch?: string } {
  if (segments[0] !== "run") return { args: segments, env: {} };
  const delimiter = segments.indexOf("--");
  const selected = delimiter < 0 ? [] : segments.slice(delimiter + 1);
  const options = segments.slice(2, delimiter < 0 ? undefined : delimiter);
  const target = segments[1];
  const print = target?.match(/^@semio-tech\/print:(build|watch)(?:-(.*))?$/);
  if (print) {
    const catalog = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8")) as { documents: { id: string }[] };
    const suffix = print[2], viz = suffix === "viz" || selected[0] === "viz";
    const names = selected[0] === "viz" ? selected.slice(1) : selected;
    const ids = suffix && suffix !== "viz" ? [suffix] : names.map(name => viz && !name.startsWith("viz-") ? `viz-${name}` : name);
    if (suffix && suffix !== "viz" && selected.length) throw new Error("A Print document target accepts no compiler arguments");
    for (const id of ids) if (!catalog.documents.some(document => document.id === id)) throw new Error(`Unknown Print document: ${id}`);
    if (print[1] === "watch" && ids.length > 1) throw new Error("Select one Print document or a complete collection to watch");
    const targets = ids.length ? [...new Set(ids)].map(id => `${print[1]}-${id}`) : [`${print[1]}${viz ? "-viz" : ""}`];
    const args = targets.length === 1 ? ["run", `@semio-tech/print:${targets[0]}`, ...options] : ["run-many", "--projects=@semio-tech/print", `--targets=${targets.join(",")}`, ...semioBuildParallelFlag(), ...options];
    return { args, env: {}, ...(print[1] === "watch" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/print:${targets[0]!.replace(/^watch/, "build")}` } : {}) };
  }
  const report = target?.match(/^@semio-tech\/mit-bestand-bericht:(build|watch)(?:-(.*))?$/);
  if (report) {
    const catalog = JSON.parse(readFileSync(join(root, "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🔣️.json"), "utf8")) as { documents: { id: string }[] };
    if (selected.length) throw new Error("Report targets accept no compiler arguments");
    if (report[2] && !catalog.documents.some(document => document.id === report[2])) throw new Error(`Unknown report document: ${report[2]}`);
    return { args: ["run", target!, ...options], env: {}, ...(report[1] === "watch" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: target!.replace(":watch", ":build") } : {}) };
  }
  const demonstrator = target?.match(/^@semio-tech\/mit-bestand-demonstrator:(prepare-(prototype|release)|prepare|build|activate-prototype|activate-release|serve|dev|prepare-e2e|serve-e2e|test-e2e)$/);
  if (demonstrator) {
    if (selected.length) throw new Error("Demonstrator targets accept no compiler arguments");
    return { args: segments, env: { SEMIO_RENDERER: "react" }, ...(demonstrator[1] === "dev" && !options.some(argument => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: "@semio-tech/mit-bestand-demonstrator:activate-prototype" } : {}) };
  }
  const nativeRuntime = target?.match(/^@semio-tech\/framework-os-dev:(prepare|run|smoke)-(.+)-native-(prototype|release)$/);
  if (nativeRuntime) return { args: segments, env: { SEMIO_PLUGIN: nativeRuntime[2]!, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: nativeRuntime[3] === "release" ? "release" : "prototype" } };
  const preparation = target?.match(/^@semio-tech\/framework-os-dev:(prepare|activate|serve|dev|build)-(.+)-(react|wgpu)-(prototype|release)$/);
  if (preparation) return { args: segments, env: { SEMIO_BUILD_MODE: preparation[4] === "release" ? "release" : "prototype", SEMIO_PLUGIN: preparation[2]!, SEMIO_RENDERER: preparation[3]! }, ...(preparation[1] === "dev" && !options.some((argument) => /^--(?:graph|help)(?:=|$)/.test(argument)) ? { watch: `@semio-tech/framework-os-dev:activate-${preparation[2]}-${preparation[3]}-${preparation[4]}` } : {}) };
  if (target === "workspace:setup" && selected.length) {
    const command = selected[0] === "deps" ? `deps-${selected[1]}` : selected[0] === "prepare" ? "prepare" : `setup-${selected[0]}`;
    return { args: ["run", `workspace:${command}`, ...options], env: {} };
  }
  if (target === "workspace:bench" || target === "@semio-tech/framework-os-dev:bench") {
    if (selected[0] !== "plugins") throw new Error("Select bench plugins through repo");
    let renderer = "native";
    const args: string[] = [];
    for (let index = 1; index < selected.length; index++) {
      const argument = selected[index]!;
      if (argument === "--renderer") renderer = selected[++index]??"";
      else if (argument.startsWith("--renderer=")) renderer = argument.slice(11);
      else args.push(argument);
    }
    if (!["native", "react", "wgpu"].includes(renderer)) throw new Error(`Unknown benchmark renderer: ${renderer}`);
    return { args: ["run", `@semio-tech/framework-os-dev:bench-plugins-${renderer}`, ...options, ...(args.length ? ["--", ...args] : [])], env: {} };
  }
  if (target === "workspace:cpp" && selected.length) {
    const [command, ...args] = selected;
    if (!["setup", "configure", "build", "test", "all"].includes(command??"")) throw new Error(`Unknown CMake operation: ${command}`);
    return { args: ["run", command === "all" ? "workspace:cpp" : `workspace:cpp-${command}`, ...options, ...(args.length ? ["--", ...args] : [])], env: {} };
  }
  if (target === "workspace:test") {
    const { level, rest } = resolveTestLevel(selected);
    if (!rest.length) return { args: ["run", "workspace:test", ...options, "--", level], env: { SEMIO_TEST_LEVEL: level } };
    if (rest[0] === "repo-client" || rest[0] === "repo-mcp") return { args: ["run", `${rest[0] === "repo-client" ? "@semio-tech/repo-client" : "repo-mcp"}:test`, ...options, "--", level, ...rest.slice(1)], env: { SEMIO_TEST_LEVEL: level } };
    const taxonomy = readTestTaxonomyV1(root);
    if (((taxonomy.testPhases ?? []) as string[]).includes(rest[0]??"" )) return { args: ["run", `@semio-tech/repo-test-domain:test-${rest[0]}`, ...options, "--", ...(((taxonomy.testLevellessPhases ?? []) as string[]).includes(rest[0]??"" ) ? [] : [level]), ...rest.slice(1)], env: { SEMIO_TEST_LEVEL: level } };
  }
  if (target === "workspace:lint" && selected[0] === "repo") return { args: ["run", "workspace:lint-repo", ...options, ...(selected.length > 1 ? ["--", ...selected.slice(1)] : [])], env: {} };
  if (target === "workspace:build" && selected.length) {
    const resolved = selected[0] === "storybook" ? "workspace:build-storybook" : canonicalBuildTargetV1(selected[0]!);
    return { args: ["run", resolved, ...options, ...(selected.length > 1 ? ["--", ...selected.slice(1)] : [])], env: {} };
  }
  return { args: segments, env: {} };
}
type Selection = { args: string[]; env: NodeJS.ProcessEnv; watch?: string };

/** 📑️ Carries only authored compiler intent from one exact selected owner declaration. */
export interface SelectedBuildTargetModeV1 { readonly project:string;readonly target:string;readonly fixed?:"prototype"|"release";readonly default?:"prototype"|"release" }

const buildOwnerRegistries=new Map<string,{readonly signature:string;readonly registry:ReturnType<typeof readBuildOwnerRegistryV1>}>();
function readBuildFileV1(path:string):string{return readFileSync(path,"utf8");}

/** 🗄️ Reads and validates the build owner registry once per file state: the key is size, modification time and inode, and a file written in the last two seconds is never trusted, so a resident process pays the parse once and every edit is seen. */
function readBuildOwnerRegistryOnceV1(root:string,read:(path:string)=>string):ReturnType<typeof readBuildOwnerRegistryV1>{
 const path=join(root,"📋️build-owners.json");let signature:string|undefined;
 if(read===readBuildFileV1)try{const status=statSync(path);if(Date.now()-status.mtimeMs>2000)signature=status.size+":"+status.mtimeMs+":"+status.ino;}catch{}
 const hit=signature===undefined?undefined:buildOwnerRegistries.get(path);if(hit!==undefined&&hit.signature===signature)return hit.registry;
 const registry=readBuildOwnerRegistryV1(JSON.parse(read(path)));if(signature!==undefined)buildOwnerRegistries.set(path,{signature,registry});return registry;
}

/** 🗂️ Reads one exact canonical run declaration before issuing original compiler authority. */
export function readSelectedBuildTargetModesV1(root:string,args:readonly string[],read:(path:string)=>string=readBuildFileV1):readonly SelectedBuildTargetModeV1[]{
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary);if(head[0]!=="run"||head.some(value=>value==="--help"||value==="-h"))return [];
 const [project,target]=head[1]?.split(":")??[];if(!project||!target||/[*?{}\[\]!]/u.test(project+target))return [];
 const registry=readBuildOwnerRegistryOnceV1(root,read),owner=registry.owners.find(row=>row.name===project);if(!owner||!owner.manifest.endsWith("/📋️project.json")&&owner.manifest!=="📋️project.json")return [];
 const manifest=JSON.parse(read(join(root,owner.manifest)));if(!manifest||manifest.name!==owner.name)throw Error("Canonical selected build owner declaration required: "+project);if(!manifest.targets||!Object.hasOwn(manifest.targets,target))return [];
 const row=manifest.targets[target],declaration=row.metadata?.semio?.buildMode;if(declaration!==undefined&&validateJsonSchemaSubset({$defs:projectSchema.$defs,$ref:"#/$defs/BuildModeDeclaration"},declaration).length)throw Error("Closed selected target build mode required: "+project+":"+target);
 return Object.freeze([Object.freeze({project,target,...(declaration?.fixed===undefined?{}:{fixed:readBuildMode(declaration.fixed)}),...(row.defaultConfiguration===undefined?{}:{default:readBuildMode(row.defaultConfiguration)})})]);
}


/** 🗣️ States a refused build mode disagreement in English, then German, naming the fixed-mode targets that pin a mode. */
export function buildModeConflictMessageV1(values: readonly string[], pinned: readonly string[]): string {
  const list = values.join(", "), pins = pinned.join(", ");
  return `Conflicting build modes: ${list}${pins ? ` (fixed by target: ${pins})` : ""}; select one mode with --configuration, -c, a :<configuration> suffix or SEMIO_BUILD_MODE\nWidersprüchliche Build-Modi: ${list}${pins ? ` (fest durch Target: ${pins})` : ""}; wähle genau einen Modus mit --configuration, -c, dem Suffix :<configuration> oder SEMIO_BUILD_MODE`;
}

/** 🎯️ Resolves mode options before the task graph and daemon receive the environment. */
export function resolveBuildModeV1(selection: Selection, environment: NodeJS.ProcessEnv, declarations:readonly SelectedBuildTargetModeV1[]=[]): Selection {
  const delimiter = selection.args.indexOf("--"), head = selection.args.slice(0, delimiter < 0 ? undefined : delimiter), tail = delimiter < 0 ? [] : selection.args.slice(delimiter);
  if (!["run", "run-many", "affected", "cargo"].includes(head[0]??"")) return selection;
  const configurations: string[] = [], args: string[] = [];
  for (let index = 0; index < head.length; index++) {
    const argument = head[index]!;
    if (argument === "--configuration" || argument === "-c") {const value=head[++index];if(!value||value.startsWith("-"))throw Error("Explicit compiler configuration value required");configurations.push(value);}
    else if (/^(?:--configuration|-c)=/.test(argument)) configurations.push(argument.slice(argument.indexOf("=") + 1));
    else args.push(argument);
  }
  if (args[0] === "run" && args[1]?.split(":").length === 3) {
    const parts = args[1].split(":"); configurations.push(parts.pop()!); args[1] = parts.join(":");
  }
  const targets = args[0] === "run" ? [args[1]?.split(":")[1] ?? ""] : args.flatMap((argument, index) => /^(?:--targets?|-t)=/.test(argument) ? argument.slice(argument.indexOf("=") + 1).split(",") : ["--target", "--targets", "-t"].includes(argument) ? (args[index + 1] ?? "").split(",") : []);
  const fixed = [...targets.map(fixedTargetBuildMode),...declarations.map(row=>row.fixed)].filter((value): value is "prototype" | "release" => value !== undefined);
  const authored = selection.env.SEMIO_BUILD_MODE;
  const inherited = environment.SEMIO_BUILD_MODE;
  const explicit = configurations.map(value => readBuildMode(value));
  const values = [...explicit, ...fixed, ...(authored === undefined ? [] : [readBuildMode(authored)]), ...(inherited === undefined ? [] : [readBuildMode(inherited)])];
  if(!values.length)values.push(...declarations.flatMap(row=>row.default===undefined?[]:[row.default]));
  const mode = values[0] ?? "prototype";
  if (values.some(value => value !== mode)) throw new Error(buildModeConflictMessageV1(values, [...targets.flatMap(target => fixedTargetBuildMode(target) === undefined ? [] : [`${target}=${fixedTargetBuildMode(target)}`]),...declarations.flatMap(row=>row.fixed===undefined?[]:[`${row.project}:${row.target}=${row.fixed}`]) ]));
  const compilerArguments = tail.slice(1);
  if (targets.some(target => /^(?:build|wasm|native|component|cpp|package|extension-package|bundle)(?:-|$)/.test(target)) && compilerArguments.some(argument => ["--release", "--dist", "--dev", "--prototype", "--mode", "--build-mode"].includes(argument) || /^(?:--mode|--build-mode|--profile)=/.test(argument))) throw new Error("Select the build prototype or release configuration before --");
  return { ...selection, args: selection.args, env: { ...selection.env, ...buildModeEnvironment(mode, {}) } };
}

/** 🛠️ Selects a configured implementation without consuming task arguments after the separator. */
export function parseBuildInvocationV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):{tool:string;args:string[];workers?:"on"|"off";graphPolicy?:"fresh"|"current"} {
  let tool=environment.SEMIO_BUILD_TOOL??buildBackendDefaultIdV1(),workers=environment.SEMIO_BUILD_WORKERS;
  const graph=extractGraphPolicyV1(args,environment),selected:string[]=[];
  args=graph.args;
  for(let index=0;index<args.length;index++){
    const argument=args[index]!;
    if(argument==="--"){selected.push(...args.slice(index));break;}
    if(argument==="--tool"){tool=args[++index]??"";continue;}
    if(argument.startsWith("--tool=")){tool=argument.slice(7);continue;}
    if(argument==="--workers"){workers=args[++index]??"";continue;}
    if(argument.startsWith("--workers=")){workers=argument.slice(10);continue;}
    if(argument==="--service"){index++;continue;}
    if(argument.startsWith("--service=")){continue;}
    selected.push(argument);
  }
  requireBuildBackendIdV1(tool);
  readWorkerSwitchV1(workers);
  return {tool,args:buildOperationArgumentsV1(selected),...(graph.explicit?{graphPolicy:graph.policy}:{}),...(workers?{workers:workers as "on"|"off"}:{})};
}

/** 🔌️ Translates portable CLI and environment options only at the Nx implementation boundary; `--graph` is the portable alias of `--dry-run` and always names stdout or a file. */
export function nxBuildArgumentsV1(args:readonly string[]):string[] {
  const aliases:Record<string,string>={"--skip-cache":"--skip-nx-cache","--skip-remote-cache":"--skip-remote-cache","--exclude-dependencies":"--excludeTaskDependencies","--exclude-task-dependencies":"--excludeTaskDependencies","--output":"--output-style","--bail":"--nx-bail","--dry-run":"--graph=stdout","--include-global-files":"--includeGlobalWorkspaceFiles"};
  const translated:string[]=[];
  for(let index=0;index<args.length;index++){
    const argument=args[index]!;
    if(argument==="--"){translated.push(...args.slice(index));break;}
    const separator=argument.indexOf("="),name=separator<0?argument:argument.slice(0,separator);
    if(name==="--dry-run"){if(!argument.endsWith("=false"))translated.push("--graph=stdout");continue;}
    if(name==="--graph"){
      const value=separator<0?undefined:argument.slice(separator+1),next=args[index+1];
      if(value==="false")continue;
      if(value===undefined&&next!==undefined&&next!=="--"&&!next.startsWith("-")){translated.push(`--graph=${next}`);index++;continue;}
      translated.push(`--graph=${value===undefined||value==="true"?"stdout":value}`);continue;
    }
    translated.push((aliases[name]??name)+(separator<0?"":argument.slice(separator)));
  }
  return translated;
}

/** 🔨️ Resolves the public selection and compiler configuration before any backend prepares. */
export function resolveBuildInvocationV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Selection { const selected=resolveBuildSelectionV1(root,[...args]);return resolveBuildModeV1(selected,environment,readSelectedBuildTargetModesV1(root,selected.args)); }

/** 🧬️ Resolves an original declared caller without deriving authority from ambient task fields. */
export function readBuildGraphAncestorSelectionV1(root:string,value:unknown):BuildGraphSelectionV1|undefined{
 const caller=(value===null||typeof value!=="object"?value:declaredCapabilitiesV1(value)) as {workspaceRoot?:unknown;native?:unknown;arguments?:string[];selection?:string[];launches?:string[][];buildMode?:string;tool?:string}|undefined;
 if(!caller||caller.native===undefined&&caller.selection===undefined)return undefined;
 if(validateJsonSchemaSubset(authoringSchema,caller).length||typeof caller.workspaceRoot!=="string"||!isAbsolute(caller.workspaceRoot)||resolve(caller.workspaceRoot)!==resolve(root))throw Error("Original nested build caller authority required");
 const namespace=caller.arguments![0];if(!["repo",...buildBackendNativeNamespacesV1()].includes(namespace!)||caller.selection![0]!==namespace||!caller.launches!.some(launch=>JSON.stringify(launch)===JSON.stringify(caller.selection)))throw Error("Original nested build launch authority required");
 const selection=(args:readonly string[])=>{if(args[0]!==namespace)throw Error("Original nested build namespace changed");const parsed=parseBuildInvocationV1(args.slice(1),{SEMIO_BUILD_TOOL:caller.tool??buildBackendDefaultIdV1()});return buildInvocationGraphSelectionV1(resolveBuildInvocationV1(root,parsed.args,{SEMIO_BUILD_MODE:caller.buildMode}).args);},ancestor=selection(caller.arguments!);
 if(!caller.launches!.some(launch=>launch[0]===namespace&&JSON.stringify(selection(launch))===JSON.stringify(ancestor)))throw Error("Original nested build request is undeclared");
 return ancestor;
}



/** 📥️ An explicit dependency selection enters its original owner before any installed graph tool. */
export type BuildDependencyInvocationV1=Readonly<{mode:"sync"|"lock"|"ensure";selection:Readonly<{kind:"all"}>|Readonly<{kind:"scope";owner:string}>}>;

/** 🗝️ Admits only the schema-defined whole or scoped installer operation, or the selection-free ensure request. */
export function readBuildDependencyInvocationV1(args:readonly string[]):BuildDependencyInvocationV1 {
  const [command,mode,selection,owner]=args,value=mode==="ensure"?(args.length===2?{mode,selection:{kind:"all"}}:undefined):selection==="--all"&&args.length===3?{mode,selection:{kind:"all"}}:selection==="--scope"&&args.length===4?{mode,selection:{kind:"scope",owner}}:undefined;
  if(command!=="dependencies"||value===undefined||validateJsonSchemaSubset(dependencySchema,value).length)throw Error("Expected repo dependencies sync|lock --all|--scope <owner>, or repo dependencies ensure");
  return value as BuildDependencyInvocationV1;
}
