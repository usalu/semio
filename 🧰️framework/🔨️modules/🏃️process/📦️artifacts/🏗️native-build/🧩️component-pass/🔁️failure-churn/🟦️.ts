import {statSync} from "node:fs";
import {isDeepStrictEqual} from "node:util";
import {basename,dirname,isAbsolute,relative,resolve,sep} from "node:path";
import {blake3Hex} from "../../../../../🔏️hash/🟦️.ts";
import {cargoDepInfoChecksumsV1} from "../../🧾️receipt/🟦️.ts";
import {watchCargoSourceDriftV1,type CargoSourceDriftV1} from "../../📥️source/👁️drift/🟦️.ts";
import {cargoLockStaleFailureV1} from "../../../../🦀️cargo-workspace/🟦️.ts";
import policy from "./🔣️.json" with {type:"json"};
import {concurrentEditOfV1,concurrentEditV1,type ConcurrentEditErrorV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🧾️ What a failed Cargo pass depends on: when Cargo started, the files its failing units read (manifests, compiler targets and the sources of their error spans) and the provenance custody's own recheck of every file the build already hashed. */
export type FailedPassInputsV1=Readonly<{startedMs:number;files:readonly string[];recheck:()=>Promise<void>;modifiedMs?:(path:string)=>number|undefined}>;

const defaultModifiedMs=(path:string):number|undefined=>{try{return statSync(path).mtimeMs;}catch{return undefined;}};

/** 📍️ Resolve Cargo's package-version remapping against the package manifest, and ordinary source names against the Cargo workspace. */
export function failedCompilerMessageFilesV1(message:Readonly<{package_id?:unknown;manifest_path?:unknown;target?:Readonly<{src_path?:unknown}>;message?:Readonly<{spans?:readonly Readonly<{file_name?:unknown;is_primary?:unknown}>[]}>}>,cwd:string,cargoWorkspace:string,primarySpansOnly=false):readonly string[]{
 const root=resolve(cwd),manifest=typeof message.manifest_path==="string"&&message.manifest_path?resolve(root,message.manifest_path):undefined;
 const repositoryFile=(path:string)=>{const distance=relative(root,path);return distance!==""&&distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);};
 const files=new Set<string>();
 if(!primarySpansOnly&&manifest&&repositoryFile(manifest))files.add(manifest);
 const target=typeof message.target?.src_path==="string"&&message.target.src_path?resolve(root,message.target.src_path):undefined;
 if(!primarySpansOnly&&target&&repositoryFile(target))files.add(target);
 const identity=typeof message.package_id==="string"?message.package_id.split("#")[1]:undefined;
 const pkg=manifest&&dirname(manifest);
 const prefix=pkg&&identity?(identity.includes("@")?identity.replace("@","-"):basename(pkg)+"-"+identity)+"/":undefined;
 for(const {file_name:name,is_primary:primary} of message.message?.spans??[]){
  if(primarySpansOnly&&primary!==true)continue;
  if(typeof name!=="string"||!name||/^<.*>$/u.test(name))continue;
  const remapped=pkg&&prefix&&name.startsWith(prefix);
  if(manifest&&!repositoryFile(manifest)&&!remapped&&!isAbsolute(name))continue;
  const path=remapped?resolve(pkg,name.slice(prefix.length)):resolve(cargoWorkspace,name);
  if(repositoryFile(path))files.add(path);
 }
 return [...files];
}

/** 🔁️ Decides whether a failed Cargo pass failed because a peer edited its inputs while it ran: a file the custody had hashed that now differs or vanished (its own recheck), or a file a failing unit read that vanished or was modified after Cargo started. Returns the concurrent edit naming that file, which restarts the command (exit 75), or `undefined` when every input is unchanged and the failure is the build's own.
 * @see ../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts */
export async function failedPassConcurrentEditV1(inputs:FailedPassInputsV1):Promise<ConcurrentEditErrorV1|undefined>{
 try{await inputs.recheck();}
 catch(error){const edit=concurrentEditOfV1(error);if(edit)return concurrentEditV1(edit.reason,edit.path,"An input of the failed Cargo pass changed after it was read","build");}
 const modifiedMs=inputs.modifiedMs??defaultModifiedMs;
 for(const path of inputs.files){
  const modified=modifiedMs(path);
  if(modified===undefined)return concurrentEditV1("vanished",path,"An input of the failed Cargo pass vanished after it was read","build");
  if(modified>inputs.startedMs)return concurrentEditV1("bytes",path,"An input of the failed Cargo pass was modified after Cargo started","build");
 }
 return;
}


/** 🧬️ Finds the first bounded byte witness that a dependency of a failed compiler unit used an older source. */
export async function failedDependencyDriftV1(input: Readonly<{
  workspace: string;
  build: string;
  target: string;
  graph: Readonly<{units: readonly Readonly<{pkg_id:string;identity:string;dependencies:readonly Readonly<{index:number}>[]}>[]}>;
  failed: ReadonlyMap<string, ReadonlySet<string>>;
  artifacts: readonly Readonly<{identity:string; filenames?: readonly string[]}>[];
  read: (path: string) => Promise<Uint8Array | undefined>;
}>): Promise<Readonly<{path:string; expected:string; observed:string}> | undefined> {
  const roots = new Set([...input.failed].filter(([, codes]) => policy.dependencyErrorCodes.some(code => codes.has(code))).map(([id]) => id));
  if (!roots.size) return;
  const queued = input.graph.units.flatMap(unit => roots.has(unit.pkg_id) ? unit.dependencies.map(edge => edge.index) : []);
  const visited = new Set<number>(), dependencies = new Set<string>();
  for (let index = 0; index < queued.length; index++) {
    const id = queued[index]!;
    if (visited.has(id)) continue;
    visited.add(id);
    const unit = input.graph.units[id];
    if (!unit) continue;
    if (unit.pkg_id.startsWith("path+file:") && !roots.has(unit.pkg_id)) dependencies.add(unit.identity);
    for (const edge of unit.dependencies) queued.push(edge.index);
  }
  const byIdentity = new Map<string, string[]>();
  for (const row of input.artifacts) {
    if (!dependencies.has(row.identity)) continue;
    const files = byIdentity.get(row.identity) ?? [];
    files.push(...row.filenames ?? []);
    byIdentity.set(row.identity, files);
  }
  const filenames = [...dependencies].flatMap(identity => byIdentity.get(identity) ?? []);
  return failedCompilerArtifactDriftV1({ ...input, filenames });
}





/** 🔬️ Verifies one former physical rustc read against the corresponding bounded live source. */
export async function failedCompilerArtifactDriftV1(input: Readonly<{
  workspace: string; build: string; target: string; filenames: readonly string[];
  read: (path: string) => Promise<Uint8Array | undefined>;
}>): Promise<Readonly<{ path: string; expected: string; observed: string }> | undefined> {
  const inTree = (root: string, path: string): boolean => {
    const difference = relative(root, path);
    return difference !== "" && difference !== ".." && !difference.startsWith(".." + sep) && !isAbsolute(difference);
  };
  const paths = new Set<string>();
    for (const file of input.filenames) {
      if (!isAbsolute(file) || (!inTree(input.build, file) && !inTree(input.target, file))) continue;
      const name = basename(file).replace(/\.(?:rlib|rmeta|dylib|so|dll|lib|wasm|exe)$/u, "");
      if (name === basename(file) && !name.includes("-")) continue;
      paths.add(resolve(dirname(file), name.replace(/^lib/u, "") + ".d"));
      paths.add(resolve(dirname(file), name + ".d"));
    }
  const seenDeps = new Set<string>(), seenChecksums = new Set<string>();
  let reads = 0;
  for (const dep of paths) {
    if (seenDeps.has(dep)) continue;
    seenDeps.add(dep);
    if (reads++ === policy.maximumChecks) return;
    const info = await input.read(dep);
    if (!info) continue;
    for (const checksum of cargoDepInfoChecksumsV1(Buffer.from(info).toString("utf8"))) {
      if (!isAbsolute(checksum.path)) continue;
      const path = resolve(checksum.path), key = JSON.stringify([path, checksum.blake3]);
      if (!inTree(input.workspace, path) || inTree(input.build, path) || inTree(input.target, path) || inTree(resolve(input.workspace, ".cache"), path) || seenChecksums.has(key)) continue;
      seenChecksums.add(key);
      if (reads++ === policy.maximumChecks) return;
      const current = await input.read(path);
      if (current === undefined) continue;
      const observed = blake3Hex(current);
      if (observed !== checksum.blake3) return Object.freeze({path, expected: checksum.blake3, observed});
    }
  }
}

/** 🧾️ Recognizes relevant rustc diagnostic headers without consuming or rewriting Cargo output. */
export function cargoDependencyDiagnosticV1(line: string): boolean {
  const display = line.replace(/\u001b\[[0-9;]*m/gu, "");
  return policy.dependencyErrorCodes.some(code => display.startsWith("error[" + code + "]:"));
}

type ReplacementFitV1=boolean|Readonly<{refusal:string}>;

/** 🔒️ A stale locked graph invalidates the enclosing plan, allowing its owner to repeat lock admission. */
export function classifyCargoGraphObservationFailureV1(error:unknown,manifest:string):unknown {
  if (!cargoLockStaleFailureV1(error)) return error;
  const edit=concurrentEditV1("input",manifest,"Cargo lockfile changed before unit graph observation","build");
  Object.defineProperty(edit,"cause",{value:error,configurable:true,writable:true});
  return edit;
}

/** 🕸️ Admits the locked retry selection between equal Cargo graph observations. */
export async function admitWarmCargoRetryGraphV1<Graph>(request:Readonly<{
  manifest:string;
  suffix:string;
  previous:Graph;
  observe:(suffix:string)=>Promise<Graph>;
  admit:()=>Promise<unknown>;
  replacementFits:(next:Graph)=>ReplacementFitV1|Promise<ReplacementFitV1>;
}>):Promise<Graph> {
  const planned=await request.observe(request.suffix+"-plan");
  await request.admit();
  const admitted=await request.observe(request.suffix);
  const refuse=(reason:string)=>concurrentEditV1("input",request.manifest,reason,"build");
  if (!isDeepStrictEqual(planned,admitted)) throw refuse("Cargo unit graph changed during retry source admission");
  if (!isDeepStrictEqual(request.previous,admitted)) {
    const fit=await request.replacementFits(admitted);
    if (fit!==true) {
      const message="Replacement Cargo unit graph does not fit the held component plan";
      throw refuse(typeof fit==="boolean"?message:message+" ("+fit.refusal+")");
    }
  }
  return admitted;
}

/** 🔄️ A single warm Cargo pass can retry before publishing any linked component. */
export function retryableWarmComponentPassV1(generation: string, passes: readonly Readonly<{ packages: readonly unknown[] }>[]): boolean {
  return generation === "warm" && passes.length === 1 && (passes[0]?.packages.length ?? 0) > 0;
}

type WarmDriftRetryV1 = (drift:CargoSourceDriftV1)=>boolean;

/** 👁️ Retires only an unlinked owned child with a new drift witness and a remaining warm retry. */
export function watchWarmCargoAttemptV1(input:Readonly<{
  sources:ReadonlyMap<string,string>;
  startedMs:number;
  signal:AbortSignal;
  mayRetry?:WarmDriftRetryV1;
  active:()=>boolean;
  stopChild:()=>void;
}>,watch=watchCargoSourceDriftV1):Readonly<{drift:()=>CargoSourceDriftV1|undefined;close:()=>Promise<void>}> {
  let observed:CargoSourceDriftV1|undefined;
  const sources={*[Symbol.iterator](){for(const [path,digest] of input.sources)yield {path,digest};}};
  const close=input.mayRetry?watch(sources,input.startedMs,mismatch=>{
    if(observed||input.signal.aborted||!input.active()||!input.mayRetry?.(mismatch))return false;
    observed=mismatch;
    input.stopChild();
    return true;
  }):async()=>{};
  return {drift:()=>observed,close};
}

/** 🔁️ Retries a retired pass only after a new dependency-byte witness and a successful graph admission. */
export async function runCargoPassAttemptsV1<T>(
  compile:(suffix:string,mayRetry?:WarmDriftRetryV1)=>Promise<T>,
  diagnose:(outcome:T)=>Promise<Readonly<{path:string;expected:string;observed:string}>|undefined>,
  admit:(drift:Readonly<{path:string;expected:string;observed:string}>,suffix:string)=>Promise<void>,
):Promise<T> {
  const recovered = new Set<string>();
  const witness = (drift:CargoSourceDriftV1) => JSON.stringify([drift.path,drift.expected,drift.observed]);
  const mayRetry = (drift:CargoSourceDriftV1) => !recovered.has(witness(drift));
  let outcome = await compile("",policy.retrySuffixes.length?mayRetry:undefined);
  for (const [index,suffix] of policy.retrySuffixes.entries()) {
    const drift = await diagnose(outcome);
    if (!drift || !mayRetry(drift)) break;
    recovered.add(witness(drift));
    await admit(drift,suffix);
    outcome = await compile(suffix,index+1<policy.retrySuffixes.length?mayRetry:undefined);
  }
  return outcome;
}
