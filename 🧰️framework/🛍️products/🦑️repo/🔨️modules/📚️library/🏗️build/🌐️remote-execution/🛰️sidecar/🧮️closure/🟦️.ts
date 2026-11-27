import {createHash} from "node:crypto";
import {existsSync,readFileSync,statSync} from "node:fs";
import {dirname,resolve} from "node:path";

/** 🔎️ The relative module specifiers a TypeScript or JavaScript source imports (static, re-exported, and literal dynamic imports) with whether each is dynamic; bare and builtin specifiers are not returned. */
export function sidecarImportsV1(source:string):readonly Readonly<{specifier:string;dynamic:boolean}>[]{
 const found=new Map<string,boolean>();
 for(const item of new Bun.Transpiler({loader:"ts"}).scan(source).imports){
  if(!item.path.startsWith("./")&&!item.path.startsWith("../"))continue;
  const dynamic=item.kind==="dynamic-import";
  found.set(item.path,(found.get(item.path)??true)&&dynamic);
 }
 return [...found].map(([specifier,dynamic])=>({specifier,dynamic}));
}

/** 🔇️ Whether a file of the closure only carries words for people (the text catalogues below a `📖️texts` folder, edited by everyone every few seconds): a role shows its old words until it restarts for other reasons, so such a file never makes a role restart. */
export const sidecarClosureIgnoredV1=(path:string):boolean=>/[\\/]📖️texts[\\/]/u.test(path);

/** 🧩️ What a closure is made of: each file below the root with the SHA-256 of its bytes. */
export type SidecarClosureV1=Readonly<{files:ReadonlyMap<string,string>;digest:string}>;

export type Known=Readonly<{stamp:string;sha256:string;imports:readonly Readonly<{specifier:string;dynamic:boolean}>[]}>;

/** 💾️ Where a watch keeps what it hashed and scanned between processes, keyed by absolute path and file stamp (size and modification time): a short-lived reader such as `repo fleet status` then hashes only the files that changed since the last look instead of the whole closure. A missing or damaged record is no record. */
export type SidecarClosureMemoryV1=Readonly<{read():Readonly<Record<string,Known>>|undefined;write(records:Readonly<Record<string,Known>>):void}>;

/** 🧬️ A watch over the code one sidecar actually runs: the closure of its entry modules over relative imports, hashed by content. `staticOnly` modules contribute their static imports only (the dispatcher of the roles names every role by a dynamic import, which must not make the other roles part of this one). `changed()` is true only when a byte of a file in the closure, or the set of files in it, differs from the closure at creation: a touched, rewritten-identically or unrelated file never counts. The digest names each file by its path below `root` with `/` separators, so the same files give the same digest on Windows, Linux and macOS and `repo fleet status` can compare the code of every worker with the tree's. Cheap: a file is re-read only when its size or modification time moved. */
export function createSidecarClosureWatchV1(entries:readonly string[],staticOnly:readonly string[],root:string,memory?:SidecarClosureMemoryV1):Readonly<{baseline:string;closure():SidecarClosureV1;changed():boolean;changes():readonly string[]}>{
 const known=new Map<string,Known>(),skip=new Set(staticOnly);let dirty=false;
 if(memory!==undefined)try{for(const [path,row] of Object.entries(memory.read()??{}))known.set(path,row);}catch{}
 const load=(path:string):Known|undefined=>{
  let stamp:string;try{const facts=statSync(path);stamp=`${facts.size}:${facts.mtimeMs}`;}catch{known.delete(path);return undefined;}
  const have=known.get(path);if(have?.stamp===stamp)return have;
  try{
   const bytes=readFileSync(path),sha256=createHash("sha256").update(bytes).digest("hex");
   const imports=have?.sha256===sha256?have.imports:/\.(?:ts|tsx|mts|js|mjs)$/u.test(path)?sidecarImportsV1(bytes.toString("utf8")):[];
   const next:Known={stamp,sha256,imports};known.set(path,next);dirty=true;return next;
  }catch{known.delete(path);return undefined;}
 };
 const resolveOf=(from:string,specifier:string):string|undefined=>{const path=resolve(dirname(from),specifier);return existsSync(path)?path:undefined;};
 const build=():SidecarClosureV1=>{
  const files=new Map<string,string>(),stack=[...entries];
  while(stack.length>0){
   const path=stack.pop()!;if(files.has(path))continue;
   const item=load(path);if(item===undefined)continue;
   files.set(path,item.sha256);
   for(const row of item.imports){if(row.dynamic&&skip.has(path))continue;const next=resolveOf(path,row.specifier);if(next!==undefined&&!files.has(next))stack.push(next);}
  }
  const lines=[...files].filter(([path])=>!sidecarClosureIgnoredV1(path)).map(([path,sha256])=>`${(path.startsWith(root)?path.slice(root.length):path).replaceAll("\\","/")} ${sha256}`).sort();
  return {files,digest:createHash("sha256").update(lines.join("\n")).digest("hex")};
 };
 const first=build();
 if(memory!==undefined&&dirty)try{const used:Record<string,Known>={};for(const path of first.files.keys())used[path]=known.get(path)!;memory.write(used);}catch{}
 const relative=(path:string):string=>path.startsWith(root)?path.slice(root.length).replace(/^[\\/]+/u,""):path;
 const changes=():readonly string[]=>{
  const now=build().files,names:string[]=[];
  for(const [path,sha256] of now)if(!sidecarClosureIgnoredV1(path)&&first.files.get(path)!==sha256)names.push(relative(path));
  for(const path of first.files.keys())if(!sidecarClosureIgnoredV1(path)&&!now.has(path))names.push(relative(path));
  return names.sort();
 };
 return Object.freeze({baseline:first.digest,closure:build,changed:()=>build().digest!==first.digest,changes});
}

/** ⏳️ When and how fast a restart is due: see `../🧬️schema/🔣️.json`. */
export type SidecarRestartPolicyV1=Readonly<{quietMilliseconds:number;maximumDeferralMilliseconds:number}>;

/** ⚖️ The verdict of one look at the closure: `same` as at start, `deferred` (edited, the restart waits) or `due` (quiet for long enough, or deferred for the maximum). */
export type SidecarRestartVerdictV1=Readonly<{state:"same"|"deferred"|"due";deferredMilliseconds:number;reason:"quiet"|"maximum"|null}>;

/** ⏳️ Debounces the restart of a sidecar: it is due when the closure digest has not moved for `quietMilliseconds` or when the closure has differed from `baseline` for `maximumDeferralMilliseconds`; an edit that is reverted (the digest equals the baseline again) cancels the wait. Pure: the caller feeds the digest, the clock is injectable. */
export function createSidecarRestartDebounceV1(policy:SidecarRestartPolicyV1,now:()=>number=Date.now):Readonly<{observe(digest:string,baseline:string):SidecarRestartVerdictV1}>{
 let first:number|undefined,last=0,seen:string|undefined;
 return Object.freeze({observe(digest:string,baseline:string):SidecarRestartVerdictV1{
  const at=now();
  if(digest===baseline){first=undefined;seen=undefined;return {state:"same",deferredMilliseconds:0,reason:null};}
  if(first===undefined)first=at;
  if(digest!==seen){seen=digest;last=at;}
  const deferredMilliseconds=at-first;
  if(at-last>=policy.quietMilliseconds)return {state:"due",deferredMilliseconds,reason:"quiet"};
  if(deferredMilliseconds>=policy.maximumDeferralMilliseconds)return {state:"due",deferredMilliseconds,reason:"maximum"};
  return {state:"deferred",deferredMilliseconds,reason:null};
 }});
}
