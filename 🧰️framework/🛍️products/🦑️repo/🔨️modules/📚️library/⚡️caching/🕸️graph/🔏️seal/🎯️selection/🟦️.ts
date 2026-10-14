import {createHash} from "node:crypto";
import {lstat,readFile} from "node:fs/promises";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🎯️ Exact selected task ownership, without a workspace-wide orchestration root. */
export interface SelectedGraphScopeV1 {readonly version:1;readonly tasks:readonly string[];readonly roots:readonly string[];readonly files:readonly string[];}
/** 👁️ Live source observer epoch and latest mutation relevant to this scope or global identities. */
export interface SelectedGraphJournalWitnessV1 {readonly version:1;readonly epoch:string;readonly sequence:number;readonly globalIdentity:{readonly paths:readonly string[];readonly sha256:string};readonly selectedMembership:{readonly paths:readonly string[];readonly sha256:string};readonly changes:readonly {readonly path:string;readonly type:"create"|"update"|"delete"}[];}
/** 🔐️ Native implementation details stay behind owned content and live-capture ports. */
export interface SelectedGraphSourcePortsV1 {capture(scope:Readonly<{roots:readonly string[];files:readonly string[]}>):Promise<SelectedGraphJournalWitnessV1>;hashFile(path:string):string|null|Promise<string|null>;}
/** 🧾️ Original selected source bytes and physical ancestry custody. */
export interface SelectedGraphSourceReceiptV1 {readonly version:1;readonly scope:SelectedGraphScopeV1;readonly journal:SelectedGraphJournalWitnessV1;readonly files:readonly {path:string;sha256:string;device:string;inode:string}[];readonly ancestry:readonly {path:string;device:string;inode:string}[];}
const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const pathPattern=/^(?!\/)(?!.*(?:^|\/)\.{1,2}(?:\/|$))[^\\:\u0000]+$/u;
const identities=new Set(["package.json","project.json","📋️project.json","Cargo.toml","go.mod","go.work","go.sum","go.work.sum","pyproject.toml","uv.lock","bun.lock","package-lock.json","yarn.lock","pnpm-lock.yaml","nx.json",".nxignore",".gitignore","bunfig.toml","rust-toolchain.toml"]);
/** 🏛️ Retains every project identity, package resolver and workspace compiler authority. */
export function isGlobalGraphIdentityPathV1(path:string):boolean {return identities.has(path.split("/").at(-1)!)||path===".cargo/config.toml"||path.endsWith("/.cargo/config.toml");}
export function validateSelectedGraphValueV1(kind:string,value:unknown):void {if(validateJsonSchemaSubset({...schema,$ref:`#/$defs/${kind}`},value).length)throw Error(`Selected graph ${kind} refused`);}
function sameJournal(left:SelectedGraphJournalWitnessV1,right:SelectedGraphJournalWitnessV1):boolean{return left.epoch===right.epoch&&left.sequence===right.sequence&&left.globalIdentity.sha256===right.globalIdentity.sha256&&left.selectedMembership.sha256===right.selectedMembership.sha256&&JSON.stringify(left.globalIdentity.paths)===JSON.stringify(right.globalIdentity.paths)&&JSON.stringify(left.selectedMembership.paths)===JSON.stringify(right.selectedMembership.paths);}
async function bounded<T>(rows:readonly T[],visit:(row:T)=>Promise<void>,signal?:AbortSignal):Promise<void>{
 let next=0;await Promise.all(Array.from({length:Math.min(64,rows.length)},async()=>{while(next<rows.length){signal?.throwIfAborted();await visit(rows[next++]!);}}));
}
async function sourceFacts(root:string,paths:readonly string[],signal?:AbortSignal):Promise<Pick<SelectedGraphSourceReceiptV1,"files"|"ancestry">>{
 const sorted=[...new Set(paths)].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),files:SelectedGraphSourceReceiptV1["files"][number][]=[],parents=new Set([""]),ancestry:SelectedGraphSourceReceiptV1["ancestry"][number][]=[];
 for(const path of sorted){if(!pathPattern.test(path)||join(root,path).length>256)throw Error("Selected graph source path refused");const parts=path.split("/");for(let i=1;i<parts.length;i++)parents.add(parts.slice(0,i).join("/"));}
 await bounded([...parents].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),async path=>{const stat=await lstat(join(root,path),{bigint:true});if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Selected graph source ancestry requires original real directories");ancestry.push({path,device:String(stat.dev),inode:String(stat.ino)});},signal);
 await bounded(sorted,async path=>{const absolute=join(root,path),before=await lstat(absolute,{bigint:true});if(!before.isFile()||before.isSymbolicLink())throw Error("Selected graph source requires an original real file");const bytes=await readFile(absolute),after=await lstat(absolute,{bigint:true});if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Selected graph source changed during capture");files.push({path,sha256:hash(bytes),device:String(before.dev),inode:String(before.ino)});},signal);
 await bounded(ancestry,async row=>{const current=await lstat(join(root,row.path),{bigint:true});if(!current.isDirectory()||current.isSymbolicLink()||String(current.dev)!==row.device||String(current.ino)!==row.inode)throw Error("Selected graph original source ancestry changed during capture");},signal);
 return {files:files.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),ancestry:ancestry.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)))};
}
/** ✍️ Creates source custody only when the original graph hashes and live observer agree across capture. */
export async function createSelectedGraphSourceReceiptV1(root:string,scope:SelectedGraphScopeV1,before:SelectedGraphJournalWitnessV1,graphFiles:readonly {file:string;hash:string|null}[],ports:SelectedGraphSourcePortsV1,signal?:AbortSignal):Promise<SelectedGraphSourceReceiptV1>{
 validateSelectedGraphValueV1("ScopeV1",scope);validateSelectedGraphValueV1("JournalWitnessV1",before);if(scope.roots.includes("."))throw Error("Workspace orchestration requires complete graph proof");
 const originals=new Map(graphFiles.map(row=>[row.file,row.hash])),globalPaths=graphFiles.map(row=>row.file).filter(isGlobalGraphIdentityPathV1).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));if(JSON.stringify(globalPaths)!==JSON.stringify(before.globalIdentity.paths))throw Error("Global project identity membership changed before publication");
 for(const path of scope.files)if(!before.selectedMembership.paths.includes(path))throw Error(`Explicit selected source missing from original observer: ${path}`);
 const expected=[...new Set(graphFiles.map(row=>row.file).filter(path=>scope.files.includes(path)||scope.roots.some(owner=>path===owner||path.startsWith(owner+"/"))))].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));if(JSON.stringify(expected)!==JSON.stringify(before.selectedMembership.paths))throw Error("Original selected source membership disagrees with live observer");
 const paths=[...new Set([...before.globalIdentity.paths,...before.selectedMembership.paths])].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));for(const path of paths){signal?.throwIfAborted();if(!originals.has(path)||originals.get(path)===null||await ports.hashFile(join(root,path))!==originals.get(path))throw Error(`Selected graph original source changed: ${path}`);}
 const facts=await sourceFacts(root,paths,signal),after=await ports.capture(scope);validateSelectedGraphValueV1("JournalWitnessV1",after);if(!sameJournal(before,after))throw Error("Live selected graph source journal changed during publication");const receipt={version:1 as const,scope,journal:after,...facts};validateSelectedGraphValueV1("SourceReceiptV1",receipt);return receipt;
}
/** ✅️ A live epoch plus exact source bytes/ancestry refuse relevant mutations and observer loss. */
export async function selectedGraphSourceReceiptHoldsV1(root:string,receipt:SelectedGraphSourceReceiptV1,ports:SelectedGraphSourcePortsV1,signal?:AbortSignal):Promise<boolean>{
 try{validateSelectedGraphValueV1("SourceReceiptV1",receipt);const before=await ports.capture(receipt.scope);validateSelectedGraphValueV1("JournalWitnessV1",before);if(!sameJournal(receipt.journal,before))return false;const current=await sourceFacts(root,receipt.files.map(row=>row.path),signal);if(JSON.stringify(current.files)!==JSON.stringify(receipt.files)||JSON.stringify(current.ancestry)!==JSON.stringify(receipt.ancestry))return false;const after=await ports.capture(receipt.scope);validateSelectedGraphValueV1("JournalWitnessV1",after);return sameJournal(before,after);}catch(error){signal?.throwIfAborted();return false;}
}
