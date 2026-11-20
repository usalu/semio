import {statSync} from "node:fs";
import {basename,dirname,isAbsolute,relative,resolve,sep} from "node:path";
import {blake3Hex} from "../../../../../🔏️hash/🟦️.ts";
import {cargoDepInfoChecksumsV1} from "../../🧾️receipt/🟦️.ts";
import policy from "./🔣️.json" with {type:"json"};
import {concurrentEditOfV1,concurrentEditV1,type ConcurrentEditErrorV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🧾️ What a failed Cargo pass depends on: when Cargo started, the files its failing units read (manifests and the sources of their error spans) and the provenance custody's own recheck of every file the build already hashed. */
export type FailedPassInputsV1=Readonly<{startedMs:number;files:readonly string[];recheck:()=>Promise<void>;modifiedMs?:(path:string)=>number|undefined}>;

const defaultModifiedMs=(path:string):number|undefined=>{try{return statSync(path).mtimeMs;}catch{return undefined;}};

/** 📍️ Resolve Cargo's package-version remapping against the package manifest, and ordinary source names against the workspace. */
export function failedCompilerMessageFilesV1(message:Readonly<{package_id?:unknown;manifest_path?:unknown;message?:Readonly<{spans?:readonly Readonly<{file_name?:unknown}>[]}>}>,cwd:string):readonly string[]{
 const root=resolve(cwd),manifest=typeof message.manifest_path==="string"&&message.manifest_path?resolve(root,message.manifest_path):undefined;
 const workspaceFile=(path:string)=>{const distance=relative(root,path);return distance!==""&&distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);};
 const files=new Set<string>();
 if(manifest&&workspaceFile(manifest))files.add(manifest);
 const identity=typeof message.package_id==="string"?message.package_id.split("#")[1]:undefined;
 const pkg=manifest&&dirname(manifest);
 const prefix=pkg&&identity?(identity.includes("@")?identity.replace("@","-"):basename(pkg)+"-"+identity)+"/":undefined;
 for(const {file_name:name} of message.message?.spans??[]){
  if(typeof name!=="string"||!name||/^<.*>$/u.test(name))continue;
  const remapped=pkg&&prefix&&name.startsWith(prefix);
  if(manifest&&!workspaceFile(manifest)&&!remapped&&!isAbsolute(name))continue;
  const path=remapped?resolve(pkg,name.slice(prefix.length)):resolve(root,name);
  if(workspaceFile(path))files.add(path);
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
  const inTree = (root: string, path: string): boolean => {
    const difference = relative(root, path);
    return difference !== "" && difference !== ".." && !difference.startsWith(".." + sep) && !isAbsolute(difference);
  };
  const artifacts = new Map<string, Set<string>>();
  for (const row of input.artifacts) {
    if (!dependencies.has(row.identity)) continue;
    const paths = artifacts.get(row.identity) ?? new Set<string>();
    for (const file of row.filenames ?? []) {
      if (!isAbsolute(file) || (!inTree(input.build, file) && !inTree(input.target, file))) continue;
      const name = basename(file).replace(/\.(?:rlib|rmeta|dylib|so|dll|lib|wasm|exe)$/u, "");
      if (name === basename(file) && !name.includes("-")) continue;
      paths.add(resolve(dirname(file), name.replace(/^lib/u, "") + ".d"));
      paths.add(resolve(dirname(file), name + ".d"));
    }
    artifacts.set(row.identity, paths);
  }
  const seenDeps = new Set<string>(), seenChecksums = new Set<string>();
  let reads = 0;
  for (const identity of dependencies) for (const dep of artifacts.get(identity) ?? []) {
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


/** 🔁️ Gives one fully retired failed invocation the opportunity to recover in its original lane. */
export async function runCargoPassAttemptsV1<T>(compile:(suffix:string)=>Promise<T>, planRetry:(first:T)=>Promise<boolean>):Promise<T> {
  const first = await compile("");
  return policy.maximumAttempts === 2 && await planRetry(first) ? compile("-retry") : first;
}
