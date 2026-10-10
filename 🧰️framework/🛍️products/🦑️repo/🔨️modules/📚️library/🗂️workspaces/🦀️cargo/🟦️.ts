import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "./🛠️preparation/📦️storage/🟦️.ts";
export {repositoryCargoPreparationStorageV1} from "./🛠️preparation/📦️storage/🟦️.ts";
import {createHash} from "node:crypto";
import {runOwnedCommand} from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {admitCargoPreparationObservationV1,assertCargoPreparationObservationCurrentV1,cargoPreparationProgramSourcesV1,type CargoPreparationObservationV1} from "./🛠️preparation/🧾️custody/🟦️.ts";
import custodySchema from "./🛠️preparation/🧾️custody/🧬️schema/🔣️.json";
import invocationSchema from "./🧬️schema/🏃️invocation/🔣️.json";
import { existsSync, lstatSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import {createRequire} from "node:module";
const {workspaceTestingCollectionRoot: collectionRoot}:{workspaceTestingCollectionRoot:(root:string,directory:string)=>string|undefined}=createRequire(import.meta.url)("../🟨️.cjs");
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type CargoWorkspaceContributionV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; exclude: readonly string[] }>;
export type CargoWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; contribution: CargoWorkspaceContributionV1; memberManifests: readonly string[] }>;
export type CargoWorkspacePackage = Readonly<{ directory: string; manifest: string; name: string; workspace: string }>;
const slash = (value: string): string => value.replaceAll("\\", "/");
const pathPattern = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !isAbsolute(value) && !/^[A-Za-z]:\//u.test(value) && !value.includes("\\") && !value.split("/").includes("..");
const object = (value: unknown): Record<string, any> => { if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Cargo workspace requires an object"); return value as Record<string, any>; };

/** 🧬️ Admits the language-neutral physical membership contract without optional owner identities. */
export function parseCargoWorkspaceContribution(value: unknown): CargoWorkspaceContributionV1 {
  const row = object(value);
  if (Object.keys(row).some(k => !["schemaVersion", "members", "exclude"].includes(k)) || row.schemaVersion !== 1 || !Array.isArray(row.members) || !row.members.length || !Array.isArray(row.exclude)) throw new Error("Invalid Cargo workspace contribution");
  for (const values of [row.members, row.exclude]) if (values.some(v => !pathPattern(v)) || new Set(values).size !== values.length) throw new Error("Invalid Cargo workspace member patterns");
  return { schemaVersion: 1, members: [...row.members], exclude: [...row.exclude] };
}

export type CargoWorkspaceProgressV1=Readonly<{operation:"state"|"presence"|"list"|"text"|"recipe";path:string;completedOperations:number}>;
export interface CargoWorkspaceControlV1 {readonly signal:AbortSignal;advance(progress:CargoWorkspaceProgressV1):Promise<void>;}
export interface CargoPreparationControlV1 extends CargoWorkspaceControlV1 {childBudget(requestedMilliseconds:number):number;}
type CargoPhysicalOperationV1=Readonly<{operation:"state"|"presence"|"list"|"text";path:string;root:string}>|Readonly<{operation:"recipe";path:string;root:string;args:string[];cwd:string;sourceSha256:string;environment:Record<string,string|undefined>}>;
type CargoPhysicalPlanV1<T>=Generator<CargoPhysicalOperationV1,T,any>;
/** 🛡️ Rechecks current directory custody immediately before one physical operation. */
function directory(root:string,path:string):void {
 const full=resolve(root,path),local=slash(relative(root,full));if(local.startsWith("../")||isAbsolute(local))throw Error(`Cargo workspace path escapes repository: ${path}`);
 let current=resolve(root);for(const part of ["",...local.split("/").filter(Boolean)]){if(part)current=join(current,part);const info=lstatSync(current);if(info.isSymbolicLink())throw Error(`Cargo workspace path contains a symlink: ${path}`);if(!info.isDirectory())throw Error(`Cargo workspace path is not a directory: ${path}`);}
}
/** 🧾️ Guards actual recipe source and working directory immediately before child acquisition. */
function recipeCurrent(step:Extract<CargoPhysicalOperationV1,{operation:"recipe"}>):void {directory(step.root,step.cwd);if(createHash("sha256").update(readFileSync(physical(step.root,step.path))).digest("hex")!==step.sourceSha256)throw Error("Cargo recipe source changed before acquisition");}
const operate=(step:CargoPhysicalOperationV1):any=>{
 if(step.operation==="recipe"){recipeCurrent(step);const result=Bun.spawnSync([process.execPath,step.path,...step.args],{cwd:step.cwd,env:step.environment,stdout:"pipe",stderr:"inherit",timeout:30_000});if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw Error(`Cargo owner preparation failed: ${step.path} (${result.exitCode})`);return;}
 if(step.operation==="list"){directory(step.root,step.path);return readdirSync(step.path,{withFileTypes:true});}
 directory(step.root,step.path===resolve(step.root)?step.root:dirname(step.path));
 if(step.operation==="state")return lstatSync(step.path);
 if(step.operation==="presence"){try{const info=lstatSync(step.path);if(info.isSymbolicLink())throw Error(`Cargo workspace path contains a symlink: ${step.path}`);return true;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}}
 return readFileSync(physical(step.root,step.path),"utf8");
};
/** 🏃️ Drains the same physical operation plan for synchronous repository consumers. */
function drain<T>(plan:CargoPhysicalPlanV1<T>):T {
 let step=plan.next();try{while(!step.done)step=plan.next(operate(step.value));return step.value;}finally{plan.return(undefined as T);}
}
/** 🎛️ Preserves original cancellation across every reported physical operation. */
async function controlled<T>(plan:CargoPhysicalPlanV1<T>,control:CargoWorkspaceControlV1):Promise<T> {
 control.signal.throwIfAborted();let count=0,step=plan.next();
 try{while(!step.done){control.signal.throwIfAborted();const operation=step.value;let value:any;if(operation.operation==="recipe"){if(!("childBudget" in control)||typeof control.childBudget!=="function")throw Error("Original Cargo preparation child budget required");const budget=control.childBudget(30_000);recipeCurrent(operation);try{await runOwnedCommand(process.execPath,[operation.path,...operation.args],operation.cwd,"cargo:owner-preparation",budget,{env:operation.environment,stdout:"stderr",signal:control.signal,onProgress:line=>console.error(line)});}catch(error){control.signal.throwIfAborted();throw error;}}else value=operate(operation);control.signal.throwIfAborted();await control.advance({operation:operation.operation,path:operation.path,completedOperations:++count});control.signal.throwIfAborted();step=plan.next(value);}return step.value;}finally{plan.return(undefined as T);}
}
/** 📁️ Refuses escaped or symlink-backed manifest ancestry before reading owner authority. */
function* physicalPlan(root:string,path:string):CargoPhysicalPlanV1<string> {
 const full=resolve(root,path),rel=slash(relative(root,full));if(rel.startsWith("../")||isAbsolute(rel))throw Error(`Cargo workspace path escapes repository: ${path}`);
 let current=resolve(root),info:any;
 for(const part of rel.split("/").filter(Boolean)){current=join(current,part);info=yield {operation:"state",path:current,root};if(info.isSymbolicLink())throw Error(`Cargo workspace path contains a symlink: ${path}`);}
 if(!info || !info.isFile())throw Error(`Cargo manifest is not a regular file: ${path}`);return full;
}
const physical=(root:string,path:string):string=>drain(physicalPlan(root,path));
function* readPlan(root:string,path:string):CargoPhysicalPlanV1<Record<string,any>> {
 const full=yield* physicalPlan(root,path);return object(Bun.TOML.parse(yield {operation:"text",path:full,root}));
}
const read=(root:string,path:string):Record<string,any>=>drain(readPlan(root,path));
const scope = (directory: string, workspace: Record<string, any>): CargoWorkspaceScope => {
  const admission = workspace.metadata?.semio?.repository;
  if (admission !== undefined && (Object.keys(object(admission)).some(k => !["schema-version", "owner-manifests", "member-manifests", "exclude-patterns"].includes(k)) || admission["schema-version"] !== 1)) throw new Error(`Unknown repository workspace admission at ${directory}`);
  const memberManifests = admission?.["member-manifests"];
  if (!Array.isArray(memberManifests) || !memberManifests.length || memberManifests.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(memberManifests).size !== memberManifests.length) throw new Error(`Invalid native member manifest authority: ${directory}`);
  const prefix = directory === "." ? "" : `${directory}/`;
  return { directory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests, contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: admission["exclude-patterns"] }) };
};

/** 🗂️ Discovers only present, explicitly authored repository workspaces through physical Cargo documents. */
export function discoverCargoWorkspaces(root:string):readonly CargoWorkspaceScope[] {return drain(discoverPlan(root));}
/** 🎛️ Discovers current production owner authority through the original controlled physical plan. */
export function discoverCargoWorkspacesControlledV1(root:string,control:CargoWorkspaceControlV1):Promise<readonly CargoWorkspaceScope[]> {return controlled(discoverPlan(root),control);}
function* discoverPlan(root:string):CargoPhysicalPlanV1<readonly CargoWorkspaceScope[]> {
  const rootDocument = yield* readPlan(root, "Cargo.toml");
  const patterns = rootDocument.workspace?.metadata?.semio?.repository?.["owner-manifests"] ?? [];
  if (!Array.isArray(patterns) || patterns.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(patterns).size !== patterns.length) throw new Error("Invalid repository workspace owner patterns");
  const files = ["Cargo.toml"], matchers=patterns.map(pattern=>new Bun.Glob(pattern)), opaque=new Set(["node_modules","target","dist","build","🤖️generated","🗑️generated","coverage","temp","compose"]);
  const prefixes=patterns.map(pattern=>pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u,""));
  const walk=function*(directory:string):CargoPhysicalPlanV1<void>{
    if(collectionRoot(root,join(root,directory)))return;
    if(directory && !prefixes.some(prefix=>!prefix || directory===prefix || directory.startsWith(prefix+"/") || prefix.startsWith(directory+"/")))return;
    const depth=directory?directory.split("/").length:0;
    if(!patterns.some(pattern=>pattern.split("/").includes("**") || depth<pattern.split("/").length))return;
    for(const entry of (yield {operation:"list",path:join(root,directory),root})){
      if(entry.name.startsWith(".") || opaque.has(entry.name) || entry.isSymbolicLink())continue;
      const path=directory?directory+"/"+entry.name:entry.name;
      if(entry.isDirectory())yield* walk(path);
      else if(entry.isFile() && entry.name==="Cargo.toml" && matchers.some(pattern=>pattern.match(path)))files.push(path);
    }
  };
  yield* walk("");
  const found: CargoWorkspaceScope[] = [];
  for (const path of files.sort()) {
    if (path.split("/").some(p => p.startsWith(".") || ["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "temp", "compose"].includes(p))) continue;
    const document = yield* readPlan(root, path);
    if (path === "Cargo.toml" || document.workspace?.metadata?.semio?.repository !== undefined) found.push(scope(slash(dirname(path)), object(document.workspace)));
  }
  if (!found.some(row => row.directory === ".")) throw new Error("Root Cargo workspace lacks authored repository admission");
  return found.sort((a, b) => a.manifest.localeCompare(b.manifest));
}

/** 🦀️ Expands each admitted Cargo-native pattern against current physical manifests and rejects absent exact owners. */
export function cargoWorkspaceMembers(root:string,owner:CargoWorkspaceScope):readonly CargoWorkspacePackage[] {return drain(membersPlan(root,owner));}
/** 🎛️ Reads current production member authority through one cancellable physical plan. */
export function cargoWorkspaceMembersControlledV1(root:string,owner:CargoWorkspaceScope,control:CargoWorkspaceControlV1):Promise<readonly CargoWorkspacePackage[]> {return controlled(membersPlan(root,owner),control);}
function* membersPlan(root:string,owner:CargoWorkspaceScope):CargoPhysicalPlanV1<readonly CargoWorkspacePackage[]> {
  const documents = new Map<string, Record<string, any>>(), scopes = new Map<string, CargoWorkspaceScope>();
  const readDocument=function*(path:string):CargoPhysicalPlanV1<Record<string,any>>{const current=documents.get(path);if(current)return current;const document=yield* readPlan(root,path);documents.set(path,document);return document;};
  const cwd = resolve(root, owner.directory), paths = new Set<string>();
  const patterns = owner.memberManifests.map(pattern => new Bun.Glob(pattern));
  const leaves = owner.memberManifests.map(pattern => new Bun.Glob(pattern.slice(0, -"/Cargo.toml".length)));
  const prefixes = owner.memberManifests.map(pattern => pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u, ""));
  const excluded = owner.contribution.exclude.map(pattern => new Bun.Glob(pattern));
  const excludedRoots = owner.contribution.exclude.map(pattern => pattern.endsWith("/**") ? new Bun.Glob(pattern.slice(0, -3)) : undefined);
  const opaque = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "🗑️generated"]);
  const walk=function*(directory:string):CargoPhysicalPlanV1<void>{
    if(collectionRoot(root,join(cwd,directory)))return;
    if (directory && !prefixes.some(prefix => !prefix || directory.startsWith(prefix + "/") || prefix === directory || prefix.startsWith(directory + "/"))) return;
    if (directory && leaves.some(pattern => pattern.match(directory))) {
      const manifest = join(cwd, directory, "Cargo.toml");
      if((yield {operation:"presence",path:manifest,root}) && (yield* readDocument(slash(relative(root,manifest)))).package!==undefined){yield* physicalPlan(root,slash(relative(root,manifest)));paths.add(slash(relative(root,manifest)));return;}
    }
    for(const entry of (yield {operation:"list",path:join(cwd,directory),root})){
      const name=entry.name;
      if(!entry.isDirectory() && (!entry.isFile() || name!=="Cargo.toml"))continue;
      const local=directory?`${directory}/${name}`:name;
      if(name.startsWith(".") || opaque.has(name) || excluded.some((pattern,index)=>pattern.match(local) || pattern.match(`${local}/`) || excludedRoots[index]?.match(local)))continue;
      if(entry.isDirectory())yield* walk(local);
      else if(local!=="Cargo.toml" && patterns.some(pattern=>pattern.match(local)) && !excluded.some(pattern=>pattern.match(slash(dirname(local)))))paths.add(slash(relative(root,join(cwd,local))));
    }
  };
  if (patterns.some(pattern => pattern.match("Cargo.toml")) && !excluded.some(pattern => pattern.match(".") || pattern.match("./") || pattern.match("Cargo.toml"))) {
    const manifest = slash(relative(root, join(cwd, "Cargo.toml")));
    if((yield* readDocument(manifest)).package!==undefined)paths.add(manifest);
  }
  yield* walk("");
  const owned:string[]=[];
  for(const manifest of paths)if((yield* readDocument(manifest)).package!==undefined && (yield* selectPlan(root,manifest,readDocument,scopes)).directory===owner.directory)owned.push(manifest);
  if(!owned.length)throw Error(`Cargo workspace has no current source-bound members: ${owner.manifest}`);
  const names=new Set<string>(),result:CargoWorkspacePackage[]=[];
  for(const manifest of owned.sort()){
   const document=yield* readDocument(manifest),name=document.package?.name;
   if(typeof name!=="string" || !name || names.has(name))throw Error(`Cargo workspace member has a missing or duplicate name: ${manifest}`);
   names.add(name);result.push({directory:slash(dirname(manifest)),manifest,name,workspace:owner.directory});
  }
  return result;
}

/** 🧭️ Selects the actual nearest Cargo workspace for a physical package, including isolated native test workspaces. */
export function cargoWorkspaceForManifest(root: string, manifest: string): CargoWorkspaceScope {
  return selectCargoWorkspace(root, manifest, path => read(root, path), new Map());
}

/** 🏎️ Binds native profiles to their selected workspace or the repository-wide configuration. */
export function cargoNextestConfiguration(root:string,directory:string):string {
 const repository=resolve(root),owner=resolve(repository,directory);
 const local=slash(relative(repository,owner));
 if(local.startsWith("../")||isAbsolute(local))throw Error(`Cargo test configuration escapes repository: ${directory}`);
 const localPath=join(owner,".config/nextest.toml"),path=existsSync(localPath)?localPath:join(repository,".config/nextest.toml");
 if(!existsSync(path))throw Error(`Selected Cargo test configuration is missing: ${directory}`);
 return physical(repository,relative(repository,path));
}

/** 📇️ Reuses parsed authority only within the caller's current physical membership scan. */
function selectCargoWorkspace(root:string,manifest:string,readDocument:(path:string)=>Record<string,any>,scopes:Map<string,CargoWorkspaceScope>):CargoWorkspaceScope {
 return drain(selectPlan(root,manifest,function*(path){return readDocument(path);},scopes));
}
function* selectPlan(root:string,manifest:string,readDocument:(path:string)=>CargoPhysicalPlanV1<Record<string,any>>,scopes:Map<string,CargoWorkspaceScope>):CargoPhysicalPlanV1<CargoWorkspaceScope> {
  const full=yield* physicalPlan(root,manifest),packageRow=(yield* readDocument(manifest)).package;
  let directory = packageRow?.workspace ? resolve(dirname(full), packageRow.workspace) : dirname(full);
  while (true) {
    const path = join(directory, "Cargo.toml"), relativePath = slash(relative(root, path));
    if (relativePath.startsWith("../") || isAbsolute(relativePath)) throw new Error(`Cargo workspace escapes repository: ${manifest}`);
    if(yield {operation:"presence",path,root}){
      const admitted = scopes.get(relativePath);
      if (admitted) return admitted;
      const workspace=(yield* readDocument(relativePath)).workspace;
      if (workspace) {
        const relativeDirectory = slash(relative(root, directory)) || ".", prefix = relativeDirectory === "." ? "" : `${relativeDirectory}/`;
        const selected = workspace.metadata?.semio?.repository !== undefined ? scope(relativeDirectory, object(workspace)) : { directory: relativeDirectory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests: (workspace.members ?? ["."]).map((path: string) => `${path}/Cargo.toml`), contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members ?? ["."], exclude: workspace.exclude ?? [] }) };
        scopes.set(relativePath, selected);
        return selected;
      }
    }
    if (directory === resolve(root)) throw new Error(`Cargo package has no workspace: ${manifest}`);
    directory = dirname(directory);
  }
}

/** 📦️ Inventories all current repository member identities without returning stale removed owners. */
export function cargoRepositoryPackages(root: string): readonly CargoWorkspacePackage[] {
  const packages = discoverCargoWorkspaces(root).flatMap(owner => cargoWorkspaceMembers(root, owner));
  const names = new Set<string>();
  for (const row of packages) { if (names.has(row.name)) throw new Error(`Duplicate repository Cargo package: ${row.name}`); names.add(row.name); }
  return packages;
}

/** 📦️ Binds one current repository inventory to an ordered group of named packages. */
export function cargoRepositoryPackageSelections(root: string, names: readonly string[]): readonly CargoWorkspacePackage[] {
  if (!names.length) return [];
  const packages = new Map(cargoRepositoryPackages(root).map(row => [row.name, row]));
  return names.map(name => { const row = packages.get(name); if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`); return row; });
}

/** 🎯️ Binds a named current repository package to its physical manifest and selected native workspace. */
export function cargoRepositoryPackage(root: string, name: string): CargoWorkspacePackage {
  const row = cargoRepositoryPackages(root).find(row => row.name === name);
  if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`);
  return row;
}

/** 📐️ Checks declared native member patterns independently of physical package discovery. */
export function cargoWorkspaceDeclaresMemberV1(source: string, directory: string): boolean {
  const workspace = object(object(Bun.TOML.parse(source)).workspace);
  const contribution = parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: workspace.exclude ?? [] });
  return contribution.members.some(pattern => new Bun.Glob(pattern).match(directory)) && !contribution.exclude.some(pattern => new Bun.Glob(pattern).match(directory));
}

/** 🎛️ Binds package selectors to one current physical workspace and preserves explicit Cargo manifest control. */
export function selectedCargoArguments(root: string, args: readonly string[]): string[] {
  if (args.some(arg => arg === "--manifest-path" || arg.startsWith("--manifest-path="))) return [...args];
  const names: string[] = [];
  for (let index = 1; index < args.length; index++) {
    const arg = args[index]!;
    if (arg === "--") break;
    if (arg === "-p" || arg === "--package") { const value = args[++index]; if (!value) throw new Error("Cargo package selector requires a current name"); names.push(value); }
    else if (arg.startsWith("--package=")) names.push(arg.slice(10));
  }
  if (!names.length) return [...args];
  const packages = cargoRepositoryPackages(root), rows = names.map(name => { const row = packages.find(row => row.name === name); if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`); return row; });
  if (new Set(rows.map(row => row.workspace)).size !== 1) throw new Error("Cargo command selects packages from different workspaces; invoke each exact owner separately");
  return [args[0]!, "--manifest-path", join(root, rows[0]!.manifest), ...args.slice(1)];
}

/** 📣️ Publishes only the admitted current member array, preserving every other Cargo declaration. */
export function publishCargoWorkspaceMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write"): boolean {
  return publishMembership(root, owner, mode, discoverCargoWorkspaces(root));
}

/** 📣️ Publishes one freshly admitted repository inventory without rediscovering every child for each owner. */
export function publishCargoWorkspaceMemberships(root: string, mode: "check" | "write"): readonly Readonly<{ owner: CargoWorkspaceScope; changed: boolean }>[] {
  const owners = discoverCargoWorkspaces(root);
  return owners.map(owner => ({ owner, changed: publishMembership(root, owner, mode, owners) }));
}

/** 📣️ Applies the current admitted native child boundaries to one guarded owner publication. */
function publishMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write", owners: readonly CargoWorkspaceScope[]): boolean {
  const path=physical(root,owner.manifest),source=readFileSync(path,"utf8"),document=object(Bun.TOML.parse(source));
  const members=cargoWorkspaceMembers(root,owner).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))) || ".");
  const children=owners.filter(row=>row.directory!==owner.directory && (owner.directory==="." || row.directory.startsWith(owner.directory+"/"))).map(row=>slash(relative(resolve(root,owner.directory),resolve(root,row.directory))));
  const exclude=[...new Set([...owner.contribution.exclude,...children])].sort();
  if(JSON.stringify(document.workspace.members)===JSON.stringify(members) && JSON.stringify(document.workspace.exclude??[])===JSON.stringify(exclude))return false;
  if(mode==="check")throw Error(`Cargo source membership is stale: ${owner.manifest}`);
  let after=source;
  for(const [name,values] of [["members",members],["exclude",exclude]] as const){
    const heading=/^\[workspace\]\r?$/m.exec(after);if(!heading)throw Error(`Cargo workspace table is absent: ${owner.manifest}`);
    const bodyStart=heading.index+heading[0].length,next=/^\[/m.exec(after.slice(bodyStart)),section=after.slice(bodyStart,next?bodyStart+next.index:after.length),field=new RegExp("^"+name+"\\s*=\\s*\\[","m").exec(section);
    const replacement=name+" = [\n"+values.map(value=>"    "+JSON.stringify(value)+",").join("\n")+"\n]";
    if(!field){after=after.slice(0,bodyStart)+"\n"+replacement+after.slice(bodyStart);continue;}
    const start=bodyStart+field.index;let end=start+field[0].length,quote="",escaped=false;
    for(;end<after.length;end++){const character=after[end]!;if(quote){if(escaped)escaped=false;else if(quote==='"' && character==="\\")escaped=true;else if(character===quote)quote="";}else if(character==='"' || character==="'")quote=character;else if(character==="]"){end++;break;}}
    if(end>=after.length)throw Error(`Unterminated Cargo workspace array: ${owner.manifest}`);
    after=after.slice(0,start)+replacement+after.slice(end);
  }
  if(readFileSync(physical(root,owner.manifest),"utf8")!==source)throw Error(`Cargo workspace changed during member discovery: ${owner.manifest}`);
  writeFileSync(path,after);return true;
}

export type CargoPreparationV1 = Readonly<{ script: string; command: readonly string[] }>;
export type CargoPreparationDiagnosticPhaseV1 = "lease-wait" | "lease" | "discovery" | "inventory" | "closure" | "recipe" | "finished" | "publication";
/** 🕰️ Emits opt-in preparation observations separately from machine-readable stdout. */
export function logCargoPreparationDiagnosticV1(phase: CargoPreparationDiagnosticPhaseV1, owner: string, elapsedMs: number, items: number): void {
 if(process.env.SEMIO_CARGO_PREPARATION_TIMING==="1")console.error(`[DEBUG] cargo-preparation ${JSON.stringify({phase,owner,elapsedMs,items})}`);
}
/** 🧬️ Admits an owner-authored executable recipe with literal argv and no implicit producer identity. */
export function parseCargoPreparation(value: unknown): CargoPreparationV1 {
 const row=object(value);if(Object.keys(row).some(k=>!["script","command"].includes(k)) || (typeof row.script!=="string" || !row.script || isAbsolute(row.script) || /^[A-Za-z]:\//.test(row.script) || row.script.includes("\\")) || row.script.split("/").at(-1)!=="📜️script.ts" || !Array.isArray(row.command) || !row.command.length || row.command.some((v:unknown)=>typeof v!=="string" || !v || v.includes("\0")))throw new Error("Invalid Cargo preparation recipe");return {script:row.script,command:[...row.command]};
}
/** 🔗️ Selects only native scopes reached by the current workspace's authored local dependency paths. */
export type CargoPreparationResultV1=Readonly<{manifests:readonly string[];owners:readonly string[];recipes:readonly CargoPreparationObservationV1[]}>;
export function prepareCargoOwners(root:string,selected:CargoWorkspaceScope,names:readonly string[]=[],observationDirectory?:string):CargoPreparationResultV1 {return drain(preparationPlan(root,selected,names,observationDirectory));}
/** 🎚️ Keeps selected dependency closure and real recipe execution under the original caller control. */
export function prepareCargoOwnersControlledV1(root:string,selected:CargoWorkspaceScope,control:CargoPreparationControlV1,names:readonly string[]=[],observationDirectory?:string):Promise<CargoPreparationResultV1> {return controlled(preparationPlan(root,selected,names,observationDirectory),control);}
function* preparationPlan(root:string,selected:CargoWorkspaceScope,names:readonly string[],observationDirectory?:string):CargoPhysicalPlanV1<CargoPreparationResultV1> {
 const timed=process.env.SEMIO_CARGO_PREPARATION_TIMING==="1",started=timed?performance.now():0;
 const diagnostic=(phase:CargoPreparationDiagnosticPhaseV1,owner:string,start:number,items:number):void=>{if(timed)logCargoPreparationDiagnosticV1(phase,owner,performance.now()-start,items);};
 const recipes:CargoPreparationObservationV1[]=[];
 let scopes=(yield* discoverPlan(root));const nativeScopes=new Map(scopes.map(scope=>[scope.manifest,scope])), inventories=new Map<string,readonly CargoWorkspacePackage[]>(), documents=new Map<string,Record<string,any>>(), executed=new Set<string>(),observedManifests=new Set<string>(),observedOwners=new Set<string>();
 diagnostic("discovery",selected.manifest,started,scopes.length);
 const document=function*(manifest:string):CargoPhysicalPlanV1<Record<string,any>>{observedManifests.add(manifest);let value=documents.get(manifest);if(!value){value=yield* readPlan(root,manifest);documents.set(manifest,value);}return value;};
 const members=function*(owner:CargoWorkspaceScope):CargoPhysicalPlanV1<readonly CargoWorkspacePackage[]>{observedOwners.add(owner.directory);let value=inventories.get(owner.directory);if(!value){const start=timed?performance.now():0;value=yield* membersPlan(root,owner);inventories.set(owner.directory,value);diagnostic("inventory",owner.manifest,start,value.length);}return value;};
 const binding=function*(manifest:string):CargoPhysicalPlanV1<{owner:CargoWorkspaceScope;pkg:CargoWorkspacePackage;current:Record<string,any>}>{const native=yield* selectPlan(root,manifest,document,nativeScopes),owner=scopes.find(scope=>scope.manifest===native.manifest);if(!owner)throw Error(`Unknown selected Cargo scope: ${manifest}`);const pkg=(yield* members(owner)).find(pkg=>pkg.manifest===manifest);if(!pkg)throw Error(`Cargo dependency lacks authored member authority: ${manifest}`);return{owner,pkg,current:yield* document(manifest)};};
 const localDependencies=function*(manifest:string,development:boolean):CargoPhysicalPlanV1<string[]>{
 const {owner,pkg,current}=yield* binding(manifest),authority=(yield* document(owner.manifest)).workspace,groups=[current,...Object.values(current.target??{})] as Record<string,any>[],paths:string[]=[];
 for(const group of groups)for(const kind of development?["dependencies","dev-dependencies","build-dependencies"]:["dependencies","build-dependencies"])for(const [alias,value] of Object.entries(group[kind]??{})){
 const entry=value as any;if(!entry || typeof entry!=="object")continue;const dependency=entry.workspace?authority.dependencies?.[alias]:entry;
 if(!dependency)throw Error(`Missing inherited Cargo dependency: ${manifest} ${alias}`);if(typeof dependency.path!=="string")continue;
 const path=resolve(root,entry.workspace?owner.directory:pkg.directory,dependency.path,"Cargo.toml"),local=slash(relative(root,path));
 if(local.startsWith("../") || isAbsolute(local))throw Error(`Cargo dependency escapes preparation authority: ${alias}`);yield* physicalPlan(root,local);paths.push(local);
 }return paths;};
 const initial=yield* members(selected),initialRoots=new Set(initial.map(pkg=>pkg.manifest));let selectedPackages=[...initial];
 if(names.some(name=>!initial.some(pkg=>pkg.name===name))){const reachable=new Set(initialRoots);for(const manifest of reachable)for(const dependency of (yield* localDependencies(manifest,initialRoots.has(manifest))))reachable.add(dependency);selectedPackages=[];for(const manifest of reachable)selectedPackages.push((yield* binding(manifest)).pkg);}
 const roots=new Set((names.length?names.map(name=>{const candidates=selectedPackages.filter(pkg=>pkg.name===name);if(candidates.length!==1)throw Error(`Unknown or ambiguous selected Cargo package: ${selected.manifest} ${name}`);return candidates[0]!;}):initial).map(pkg=>pkg.manifest)),pending=new Set(roots);
 const visiting=new Set<string>(),finished=new Set<string>();
 const visit=function*(manifest:string):CargoPhysicalPlanV1<void>{
 if(finished.has(manifest)||visiting.has(manifest))return;visiting.add(manifest);
 const start=timed?performance.now():0,{owner,pkg}=yield* binding(manifest);if(!names.length&&owner.directory===selected.directory)roots.add(manifest);
 while(true){const dependencies=yield* localDependencies(manifest,roots.has(manifest));for(const dependency of dependencies)pending.add(dependency);const unfinished=dependencies.filter(path=>!finished.has(path)&&!visiting.has(path));if(!unfinished.length)break;for(const dependency of unfinished)yield* visit(dependency);}
 diagnostic("closure",manifest,start,pending.size);
 const value=(yield* document(manifest)).package?.metadata?.semio?.preparation;if(value===undefined){visiting.delete(manifest);finished.add(manifest);return;}
 const recipe=parseCargoPreparation(value),script=yield* physicalPlan(root,slash(relative(root,resolve(root,pkg.directory,recipe.script)))),key=JSON.stringify([script,...recipe.command]);
 if(slash(relative(resolve(root,owner.directory),script)).startsWith("../"))throw Error(`Cargo preparation leaves its owner workspace: ${manifest}`);if(executed.has(key)){visiting.delete(manifest);finished.add(manifest);return;}executed.add(key);
 console.error(`[cargo-preparation] ${manifest} ${recipe.command.join(" ")}`);
 const recipeStart=timed?performance.now():0;
 const observation=observationDirectory?join(observationDirectory,createHash("sha256").update(key).digest("hex")+".json"):undefined, before=createHash("sha256").update(readFileSync(physical(root,script))).digest("hex"), program=observation?cargoPreparationProgramSourcesV1(root,script):undefined;
 yield {operation:"recipe",path:script,root,args:[...recipe.command],cwd:resolve(root,pkg.directory),sourceSha256:before,environment:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_ACTIVE:script,...(observation?{SEMIO_CARGO_PREPARATION_OBSERVATION:observation}:{})}};
 if(observation){const captured=admitCargoPreparationObservationV1(JSON.parse(readFileSync(observation,"utf8")));if(captured.root!==root||captured.sourceRoot!==program!.sourceRoot||captured.script!==script||JSON.stringify(captured.command)!==JSON.stringify(recipe.command)||captured.sources.find(input=>input.path===script)?.sha256!==before)throw Error("Preparation observation does not bind its original producer");assertCargoPreparationObservationCurrentV1(captured);assertCargoPreparationObservationCurrentV1({...captured,sources:program!.sources,resolverInputs:program!.resolverInputs,inputs:program!.inputs,outputs:[],resolutions:program!.resolutions});const merged={...captured,resolverInputs:program!.resolverInputs,resolutions:program!.resolutions,sources:[...new Map([...program!.sources,...captured.sources].map(input=>[input.path,input])).values()],inputs:[...new Map([...program!.inputs,...captured.inputs].map(input=>[JSON.stringify([input.path,input.kind]),input])).values()]};assertCargoPreparationObservationCurrentV1(merged);recipes.push(merged);}
 diagnostic("recipe",manifest,recipeStart,1);
 documents.clear();inventories.clear();scopes=(yield* discoverPlan(root));nativeScopes.clear();for(const scope of scopes)nativeScopes.set(scope.manifest,scope);
 for(const dependency of (yield* localDependencies(manifest,roots.has(manifest)))){pending.add(dependency);yield* visit(dependency);}
 visiting.delete(manifest);finished.add(manifest);
 };
 for(const manifest of pending)yield* visit(manifest);
 while(true){
 documents.clear();inventories.clear();scopes=(yield* discoverPlan(root));nativeScopes.clear();for(const scope of scopes)nativeScopes.set(scope.manifest,scope);
 if(!names.length)for(const pkg of (yield* members(yield* selectPlan(root,selected.manifest,document,nativeScopes))))roots.add(pkg.manifest);
 const closure=new Set(roots);for(const manifest of closure)for(const dependency of (yield* localDependencies(manifest,roots.has(manifest))))closure.add(dependency);
 const before=executed.size;for(const manifest of closure){pending.add(manifest);yield* visit(manifest);}if(executed.size!==before)continue;
 for(const manifest of closure){const {owner,pkg,current}=yield* binding(manifest),value=current.package?.metadata?.semio?.preparation;if(value===undefined)continue;const recipe=parseCargoPreparation(value),script=yield* physicalPlan(root,slash(relative(root,resolve(root,pkg.directory,recipe.script)))),key=JSON.stringify([script,...recipe.command]);if(slash(relative(resolve(root,owner.directory),script)).startsWith("../")||!executed.has(key))throw Error(`Prepared recipe closure changed: ${manifest}`);}
 break;
 }
 for(const recipe of recipes)assertCargoPreparationObservationCurrentV1(recipe);
 diagnostic("finished",selected.manifest,started,pending.size);
 return {manifests:[...new Set([...observedManifests,...scopes.map(scope=>scope.manifest)])],owners:[...observedOwners],recipes};
}

export type CargoPreparationSelectionV1=Readonly<{manifest:string;packages:readonly string[]}>;
/** 🧭️ Admits a portable current manifest and distinct declared package roots. */
export function parseCargoPreparationSelectionV1(value:unknown):CargoPreparationSelectionV1 {
 const row=object(value);if(Object.keys(row).some(key=>!Object.keys(custodySchema.$defs.CargoPreparationSelectionV1.properties).includes(key))||!pathPattern(row.manifest)||!/(^|\/)Cargo\.toml$/.test(row.manifest)||!Array.isArray(row.packages)||new Set(row.packages).size!==row.packages.length||row.packages.some(name=>typeof name!=="string"||!name||name.includes("\0")))throw Error("Invalid Cargo preparation selection");return{manifest:row.manifest,packages:[...row.packages]};
}

/** 🦀️ Uses the owned schema authority for native operations that consume current package inputs. */
export function cargoCommandRequiresOwnerPreparationV1(command: string): boolean {
  return invocationSchema.allOf[0]!.if.properties.command.enum.includes(command);
}

/** 🧭️ Resolves the manifest selected by the repository's existing Cargo invocation grammar. */
export function cargoInvocationManifestV1(args: readonly string[], cwd: string): string {
  const index = args.indexOf("--manifest-path"), inline = args.find(arg => arg.startsWith("--manifest-path="));
  const selected = index >= 0 ? args[index+1] : inline?.slice(16);
  return selected ? resolve(cwd, selected) : join(cwd, "Cargo.toml");
}

export type CargoPreparationInvocationV1=Readonly<{command:string;args:readonly string[];cwd:string;environment:Readonly<Record<string,string>>}>;
/** 🧭️ Selects the current owned preparation launch without executing or borrowing a completed receipt. */
export function cargoWorkspacePreparationInvocationV1(storage:CargoPreparationStorageV1,root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string,string|undefined>>):CargoPreparationInvocationV1|undefined {
  parseCargoPreparationStorageV1(storage);if (!cargoCommandRequiresOwnerPreparationV1(args[0] ?? "")) return;
  const path = cargoInvocationManifestV1(args, cwd);
  if (environment.SEMIO_CARGO_PREPARATION_ACTIVE) throw new Error(`Cargo recursion in owner preparation: ${environment.SEMIO_CARGO_PREPARATION_ACTIVE}`);
  const owner = cargoWorkspaceForManifest(root, slash(relative(root, path))), source = read(root, owner.manifest), names:string[]=[];
  for(let cursor=1;cursor<args.length && args[cursor]!=="--";cursor++){const arg=args[cursor]!;if(arg==="-p" || arg==="--package"){const name=args[++cursor];if(!name)throw Error("Cargo package selector requires a current name");names.push(name);}else if(arg.startsWith("--package="))names.push(arg.slice(10));else if(arg.startsWith("-p") && arg.length>2)names.push(arg.slice(2));}
  const selectedPackage=read(root,slash(relative(root,path))).package?.name;
  if(!names.length && !args.includes("--workspace") && typeof selectedPackage==="string")names.push(selectedPackage);
  if (source.workspace?.metadata?.semio?.repository === undefined)return;
  const selection=parseCargoPreparationSelectionV1({manifest:owner.manifest,packages:names}),env:Record<string,string>={};for(const [key,value] of Object.entries(environment))if(value!==undefined)env[key]=value;env.NX_WORKSPACE_ROOT=root;env.SEMIO_CARGO_PREPARATION_STORAGE=JSON.stringify(storage);
  return{command:process.execPath,args:[fileURLToPath(new URL("./🛠️preparation/📜️script.ts",import.meta.url)),"synchronize","--manifest",selection.manifest,...selection.packages.flatMap(name=>["--package",name])],cwd:root,environment:env};
}

/** 🛠️ Executes the selected native workspace launch through the synchronous Cargo boundary. */
export function prepareCargoWorkspaceInvocation(storage:CargoPreparationStorageV1,root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string,string|undefined>>): void {
 const request=cargoWorkspacePreparationInvocationV1(storage,root,args,cwd,environment);if(!request)return;
 const result=Bun.spawnSync([request.command,...request.args],{cwd:request.cwd,env:request.environment,stdout:"pipe",stderr:"inherit"});if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw Error(`Selected Cargo preparation failed: ${request.args[3]} (${result.exitCode})`);
}
