import sourceSchema from "../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { lstat, opendir } from "node:fs/promises";
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative,resolve,isAbsolute } from "node:path";

/** 🪶️ Writes generated text only when its bytes changed, preserving no-op prerequisite mtimes. */
export function writeGeneratedFileIfChanged(path: string, content: string): boolean {
  if (existsSync(path)) {
    const metadata = lstatSync(path);
    if (!metadata.isFile() || metadata.isSymbolicLink()) throw new Error(`Invalid generated file: ${path}`);
    if (readFileSync(path).equals(Buffer.from(content,"utf8"))) return false;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content, "utf8");
  return true;
}

/** 🌱️ Makes sure the generated file at `path` exists, empty when it is new, and says whether it was new: a native
 * exporter whose own freshness test compiles its published mirror in (`include_str!`) builds on a checkout that has no
 * mirror yet. */
export function seedGeneratedFile(path: string): boolean {
  if (existsSync(path)) return false;
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, "", "utf8");
  return true;
}

export type ArtifactInventoryBoundsV1=Readonly<Partial<{maximumFiles:number;maximumBytes:number;maximumEntries:number;maximumDepth:number}>>;
/** 🗂️ Enumerates finite regular file DATA one entry at a time under the canonical publication bounds. */
export async function collectArtifactFiles(root:string,signal?:AbortSignal,bounds:ArtifactInventoryBoundsV1={}):Promise<ReadonlyMap<string,string>>{
 const defaults={maximumFiles:sourceSchema.properties.maximumFiles.const,maximumBytes:sourceSchema.properties.maximumBytes.const,maximumEntries:sourceSchema.properties.maximumEntries.const,maximumDepth:sourceSchema.properties.maximumDepth.const};for(const key of Object.keys(bounds)){if(!Object.hasOwn(defaults,key))throw Error("Closed artifact inventory bounds required");const name=key as keyof typeof defaults,value=bounds[name];if(!Number.isSafeInteger(value)||value!<0||value!>defaults[name])throw Error("Inventory cannot exceed canonical finite publication bounds");}const policy={...defaults,...bounds};signal?.throwIfAborted();if(!isAbsolute(root)||resolve(root)!==root||root.length>256)throw Error("Canonical bounded artifact root required");const metadata=await lstat(root,{bigint:true});if(!metadata.isDirectory()||metadata.isSymbolicLink())throw Error("Regular artifact root required");const pending=[{path:root,depth:0}],files=new Map<string,string>();let entries=0,bytes=0;
 while(pending.length){signal?.throwIfAborted();const directory=pending.pop();if(!directory)throw Error("Actual pending inventory directory required");const before=await lstat(directory.path,{bigint:true});if(!before.isDirectory()||before.isSymbolicLink())throw Error("Regular inventory directory required");const handle=await opendir(directory.path,{bufferSize:1});for await(const entry of handle){signal?.throwIfAborted();if(++entries>policy.maximumEntries)throw Error("Artifact inventory entry bound exceeded");const path=join(directory.path,entry.name),name=relative(root,path).replaceAll("\\","/");if(path.length>256||validateJsonSchemaSubset(sourceSchema.definitions.SourceName,name).length)throw Error("Canonical finite artifact name required");const facts=await lstat(path,{bigint:true});if(facts.isDirectory()&&!facts.isSymbolicLink()){if(directory.depth>=policy.maximumDepth)throw Error("Artifact inventory depth bound exceeded");pending.push({path,depth:directory.depth+1});}else{if(!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1n)throw Error("Single-link regular artifact file required");if(files.size>=policy.maximumFiles||facts.size>BigInt(policy.maximumBytes-bytes))throw Error("Artifact inventory file/byte bound exceeded");bytes+=Number(facts.size);files.set(name,path);}}
  const after=await lstat(directory.path,{bigint:true});if(!after.isDirectory()||after.isSymbolicLink()||after.dev!==before.dev||after.ino!==before.ino)throw Error("Inventory directory identity changed");
 }const final=await lstat(root,{bigint:true});if(!final.isDirectory()||final.isSymbolicLink()||final.dev!==metadata.dev||final.ino!==metadata.ino)throw Error("Inventory root identity changed");return new Map([...files].sort(([a],[b])=>a<b?-1:a>b?1:0));
}
