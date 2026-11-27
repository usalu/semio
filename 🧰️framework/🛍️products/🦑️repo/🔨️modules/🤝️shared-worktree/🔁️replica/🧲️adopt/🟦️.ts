import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {appendFileSync,createReadStream,existsSync,readFileSync} from "node:fs";
import {copyFile,lstat,mkdir,readdir,readFile,readlink,rename,unlink,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {blobPathV1,listOriginsV1,mergeClockV1,readHeadV1,storePathsV1} from "../../📒️journal/🟦️.ts";
import {compileIgnoreV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../../🚧️boundary/🟦️.ts";
import {diffTreeV1,gitStateV1,indexRealmV1,readTreeV1,requestRebaseV1,resolveGitV1,updateIndexV1,writeTreeV1} from "../../🌿️git/🟦️.ts";
import {normaliseNamesV1} from "../🥾️bootstrap/🟦️.ts";
import {applyQueue,closeReplicaV1,CompactTableV1,enqueueId,filesKey,queueKey,foldInto,foldStoreV1,guarded,localPathsV1,openReplicaV1,persist,publishOnceV1,settle,storeReachableV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaSinkV1,type ReplicaV1} from "../🟦️.ts";

/** 🧲️ What one local path becomes when an existing repository is adopted into the fleet, decided from the content and never from a modification time: `same` is adopted as it is, `materialise` is missing locally and comes from the store, `publish` is local work at a path the fleet does not hold (or deleted while the repository's base never had it), `edit` is local work on top of a fleet version that has not changed since the repository's base and is published as a new edit, `stale` is an older version of what the fleet holds or a file the fleet deleted since, so the fleet wins and nothing is published, `conflict` was changed here and in the fleet since the base, so the fleet's version stays and the local one is kept in `lost/`, `nested` lies inside a nested repository and is left alone. */
export type AdoptCategoryV1="same"|"materialise"|"publish"|"edit"|"stale"|"conflict"|"nested";

/** 🪞️ One local file or link as the classification sees it: `sha256` and, where the volume can represent it, the `executable` bit are the identity of a file, `target` of a link; `mtimeMs` is shown to people and never decides. */
export type AdoptLocalV1=Readonly<{kind:"file"|"link";sha256?:string;executable?:boolean;target?:string;mtimeMs:number}>;

/** 🛰️ The fleet's winning entry for the same path; `executable` is the bit of a put. */
export type AdoptRemoteV1=Readonly<{operation:"put"|"link"|"delete";sha256?:string;executable?:boolean;target?:string;hlc:string}>;

/** 🧾️ What is known about the local content of one path besides the fleet's winner: `seen` names the history in which the fleet had this very content before (`journal`: some event of the path left it; `git`: the path held it in a commit of the fleet's history), `base` relates it to the repository's own base (the content at its HEAD: `absent` no such path, `equal` unchanged, `differs` edited, `unknown` there is no commit to compare with) and `fleetSince` tells whether the fleet's state of the path (a delete counts as absent) differs from that base, `null` while the base is unknown. */
export type AdoptEvidenceV1=Readonly<{seen:"journal"|"git"|null;base:"absent"|"equal"|"differs"|"unknown";fleetSince:boolean|null}>;

/** 🔎️ The reason of a decision, shown beside it: `journal`, `git` and `base` explain a stale file, `fleet-unchanged` an edit, `both-changed`, `deleted-by-fleet` and `no-base` a conflict. */
export type AdoptReasonV1="journal"|"git"|"base"|"fleet-unchanged"|"both-changed"|"deleted-by-fleet"|"no-base";

const NO_EVIDENCE:AdoptEvidenceV1={seen:null,base:"unknown",fleetSince:null};

const sameContent=(local:AdoptLocalV1,remote:AdoptRemoteV1):boolean=>remote.operation==="put"?local.kind==="file"&&local.sha256!==undefined&&local.sha256===remote.sha256&&(local.executable===undefined||remote.executable===undefined||local.executable===remote.executable):remote.operation==="link"&&local.kind==="link"&&local.target===remote.target;

/** ⚖️ Classifies one path from the local fact, the fleet's winner and the evidence; `undefined` means nothing to do (absent on both sides, or deleted by the fleet and absent locally). The order is the rule: equal content is `same` (equal bytes whose executable bit differs are not equal content: the bit is decided by the rest of the rule like any other difference, and the evidence is then about bytes and bit together); content the fleet had before is `stale`; content unchanged since the repository's base is `stale` when the fleet holds something else; at a path the fleet does not hold at all the local file is `publish`; otherwise it is local work, an `edit` (or `publish` over a delete) when the fleet's state equals the base and a `conflict` when both sides moved or the base is unknown. */
export function classifyV1(local:AdoptLocalV1|undefined,remote:AdoptRemoteV1|undefined,evidence:AdoptEvidenceV1=NO_EVIDENCE):AdoptCategoryV1|undefined{
 if(local===undefined)return remote===undefined||remote.operation==="delete"?undefined:"materialise";
 if(remote!==undefined&&sameContent(local,remote))return "same";
 if(evidence.seen!==null)return "stale";
 if(remote===undefined)return "publish";
 if(evidence.base==="equal")return "stale";
 if(evidence.fleetSince===false)return remote.operation==="delete"?"publish":"edit";
 return "conflict";
}

/** 🗒️ The reason that goes with the category of `classifyV1`; `undefined` for the categories that need none. */
export function reasonV1(category:AdoptCategoryV1|undefined,remote:AdoptRemoteV1|undefined,evidence:AdoptEvidenceV1=NO_EVIDENCE):AdoptReasonV1|undefined{
 if(category==="stale")return evidence.seen??"base";
 if(category==="edit")return "fleet-unchanged";
 if(category==="conflict")return remote?.operation==="delete"?"deleted-by-fleet":evidence.base==="unknown"?"no-base":"both-changed";
 return undefined;
}

/** 🎛️ The inputs of an adoption: `dryRun` plans and changes nothing, `force` adopts a repository without common history, `sample` is the number of example paths per category, `fault` is called after every finished step (tests throw from it to simulate a crash). */
export type AdoptOptionsV1=Readonly<{dryRun:boolean;force:boolean;sample:number;fault?:(step:AdoptStepV1)=>void}>;

/** 🪜️ The steps of an adoption, in order; the marker `.cache/f/adopt.json` names the last finished one and a rerun continues after it. */
export type AdoptStepV1="planned"|"saved"|"objects"|"files"|"applied"|"published"|"staged"|"reported";

/** 🌿️ How the fleet's branch tip and the local one relate: equal, the fleet is ahead (the local tip is an ancestor), the local repository is ahead, they diverged, they share no commit, or `unknown` while the objects of the fleet tip are not in the local repository yet. */
export type AdoptRelationV1="none"|"equal"|"fleet-ahead"|"local-ahead"|"diverged"|"unrelated"|"unknown";

/** 🔖️ One path that needs a look: what became of it (`edit`, `stale` or `conflict`), why, the identity of the local content and of the fleet's (`null` for a delete), whether the bytes are equal and only the executable bit differs (`modeOnly`), the wall time inside the fleet's hlc and the local modification time (both only shown to people, they decide nothing). */
export type AdoptDecisionV1=Readonly<{path:string;category:"edit"|"stale"|"conflict";reason:AdoptReasonV1;localSha256:string;fleetSha256:string|null;modeOnly:boolean;remoteWallMs:number;localMtimeMs:number}>;

/** 🗺️ The plan of an adoption: the names that are not NFC and are renamed to NFC before anything is compared (`renamed`, the new paths; a dry run reads them under their NFC names and renames nothing), counts and sample paths per category, every path that is an edit, stale or a conflict with its reason, the nested repositories, the git relation and the size of the evidence that decided (`events` read from the journals, `commits` of the fleet's history searched). */
export type AdoptPlanV1=Readonly<{version:1;dryRun:boolean;renamed:readonly string[];counts:Readonly<Record<AdoptCategoryV1,number>>;samples:Readonly<Record<AdoptCategoryV1,readonly string[]>>;decisions:readonly AdoptDecisionV1[];nested:readonly string[];git:Readonly<{fleetTip:string|null;localTip:string|null;relation:AdoptRelationV1;stagedPaths:number|null}>;evidence:Readonly<{events:number;commits:number}>}>;

/** 📒️ The report of a finished adoption, shaped by `🧬️schema/🔣️.json`: the paths renamed to NFC first (`renamed`), the plan counts, the local work that was published (`published` new paths, `edited` edits), the stale files (the fleet won, `lost` names the kept local bytes), every conflict (the fleet's version stays, the local one is kept at `lost`), the stage paths that changed and what stayed pending. */
export type AdoptReportV1=Readonly<{version:1;stamp:string;originId:string;renamed:readonly string[];counts:Readonly<Record<AdoptCategoryV1,number>>;published:readonly string[];edited:readonly string[];stale:readonly Readonly<{path:string;lost:string;reason:AdoptReasonV1}>[];conflicts:readonly Readonly<{path:string;lost:string;reason:AdoptReasonV1;local:string;fleet:string|null}>[];linearised:readonly Readonly<{path:string;winner:string;loser:string;result:string}>[];stage:Readonly<{lost:string|null;changed:number;paths:readonly string[]}>;pending:readonly string[];nested:readonly string[];issues:readonly string[]}>;

/** 🏁️ What an adoption did or, for a dry run, would do. */
export type AdoptResultV1=Readonly<{plan:AdoptPlanV1;report:AdoptReportV1|undefined;files:number;adopted:number;queued:number}>;

/** ⛔️ Raised when an adoption must not start: `already-joined` (the worktree has local state) or `no-common-history` (the repositories share no commit). */
export class AdoptRefusedV1 extends Error{
 readonly reason:"already-joined"|"no-common-history";
 constructor(reason:"already-joined"|"no-common-history"){super(`Adoption refused: ${reason}`);this.name="AdoptRefusedV1";this.reason=reason;}
}

const STEPS:readonly AdoptStepV1[]=["planned","saved","objects","files","applied","published","staged","reported"];
const CATEGORIES:readonly AdoptCategoryV1[]=["same","materialise","publish","edit","stale","conflict","nested"];
const oid=/^[0-9a-f]{40}(?:[0-9a-f]{24})?$/u;
const wallOf=(hlc:string):number=>Number(hlc.slice(0,13));
const empty=():Record<AdoptCategoryV1,number>=>({same:0,materialise:0,publish:0,edit:0,stale:0,conflict:0,nested:0});
const emptySamples=():Record<AdoptCategoryV1,string[]>=>({same:[],materialise:[],publish:[],edit:[],stale:[],conflict:[],nested:[]});

async function runGit(worktree:string,args:readonly string[],signal?:AbortSignal,input?:string):Promise<Readonly<{code:number;out:string}>|null>{
 const file=await resolveGitV1();
 if(file===null||signal?.aborted)return null;
 return new Promise(done=>{
  try{
   const environment:Record<string,string|undefined>={...process.env,GIT_DIR:join(worktree,".git"),GIT_TERMINAL_PROMPT:"0",GIT_OPTIONAL_LOCKS:"0",LC_ALL:"C"};
   for(const name of ["GIT_WORK_TREE","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_INDEX_FILE"])delete environment[name];
   const child=spawn(file,["-c","core.fsmonitor=false",...args],{cwd:worktree,env:environment as NodeJS.ProcessEnv,stdio:[input===undefined?"ignore":"pipe","pipe","ignore"],windowsHide:true,signal}),chunks:Buffer[]=[];
   if(input!==undefined){child.stdin!.on("error",()=>{});child.stdin!.end(input);}
   child.stdout!.on("data",(chunk:Buffer)=>{chunks.push(chunk);});
   child.on("error",()=>{done(null);});
   child.on("close",code=>{done({code:code??1,out:Buffer.concat(chunks).toString("utf8").trim()});});
  }catch{done(null);}
 });
}

async function relationOf(worktree:string,fleetTip:string|null,localTip:string|null,signal?:AbortSignal):Promise<AdoptRelationV1>{
 if(fleetTip===null||localTip===null)return "none";
 if(fleetTip===localTip)return "equal";
 const known=await runGit(worktree,["cat-file","-e",`${fleetTip}^{commit}`],signal);
 if(known===null||known.code!==0)return "unknown";
 const forward=await runGit(worktree,["merge-base","--is-ancestor",localTip,fleetTip],signal);
 if(forward?.code===0)return "fleet-ahead";
 const back=await runGit(worktree,["merge-base","--is-ancestor",fleetTip,localTip],signal);
 if(back?.code===0)return "local-ahead";
 const base=await runGit(worktree,["merge-base",localTip,fleetTip],signal);
 return base!==null&&base.code===0&&oid.test(base.out)?"diverged":"unrelated";
}

async function fleetTipOf(manifest:CompactTableV1,storeRoot:string):Promise<string|null>{
 const text=async(path:string):Promise<string|undefined>=>{
  const id=manifest.idOf(path);
  return id===undefined||manifest.operationAt(id)!=="put"?undefined:readFile(blobPathV1(storeRoot,manifest.shaAt(id)),"utf8").catch(()=>undefined);
 };
 const head=(await text(".git/HEAD"))?.trim();
 if(head===undefined)return null;
 if(oid.test(head))return head;
 if(!head.startsWith("ref: "))return null;
 const ref=head.slice(5),loose=(await text(`.git/${ref}`))?.trim();
 if(loose!==undefined&&oid.test(loose))return loose;
 const packed=(await text(".git/packed-refs"))?.split("\n").find(line=>line.endsWith(` ${ref}`))?.split(" ")[0];
 return packed!==undefined&&oid.test(packed)?packed:null;
}

type Move=Readonly<{path:string;category:"stale"|"conflict";reason:AdoptReasonV1;sha256:string;fleet:string|null}>;
type Survey=Readonly<{plan:AdoptPlanV1;same:readonly Readonly<{path:string;mtimeMs:number}>[];publish:readonly string[];edit:readonly string[];moves:readonly Move[];nestedRoots:ReadonlySet<string>;present:ReadonlySet<string>}>;

async function nestedRootsOf(worktree:string,directories:readonly string[],concurrency:number,disk:(path:string)=>string,signal?:AbortSignal):Promise<ReadonlySet<string>>{
 const roots=new Set<string>();
 let next=0;
 await Promise.all(Array.from({length:Math.min(concurrency,Math.max(1,directories.length))},async()=>{
  while(next<directories.length){
   signal?.throwIfAborted();
   const directory=directories[next++]!;
   if(directory!==""&&await lstat(join(worktree,...disk(directory).split("/"),".git")).then(()=>true,()=>false))roots.add(directory);
  }
 }));
 return roots;
}

const insideNested=(roots:ReadonlySet<string>,path:string):boolean=>{
 if(roots.size===0)return false;
 for(let cut=path.indexOf("/");cut>0;cut=path.indexOf("/",cut+1))if(roots.has(path.slice(0,cut)))return true;
 return false;
};

type Pending={entry:FleetScanEntryV1;remote:AdoptRemoteV1|undefined;key:string;mark:string;goid:string;local:AdoptLocalV1;seen:"journal"|"git"|null;base:AdoptEvidenceV1["base"];baseId:string|undefined};
const zeros=/^0+$/u;
const bit=(executable:boolean|undefined):string=>executable===true?"+x":"";
const ordered=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const gitHash=(format:string,size:number):ReturnType<typeof createHash>=>createHash(format).update(`blob ${size}\0`);

async function identityOfFile(file:string,size:number,format:string):Promise<Readonly<{sha256:string;goid:string}>>{
 const sha=createHash("sha256"),blob=gitHash(format,size);
 for await(const chunk of createReadStream(file)){sha.update(chunk);blob.update(chunk);}
 return {sha256:sha.digest("hex"),goid:blob.digest("hex")};
}

async function each<T>(items:readonly T[],concurrency:number,signal:AbortSignal|undefined,job:(item:T)=>Promise<void>):Promise<void>{
 let next=0;
 await Promise.all(Array.from({length:Math.min(Math.max(1,concurrency),Math.max(1,items.length))},async()=>{
  while(next<items.length){signal?.throwIfAborted();await job(items[next++]!);}
 }));
}

async function historyV1(storeRoot:string,wanted:ReadonlySet<string>,modeless:boolean,signal:AbortSignal|undefined,sink:ReplicaSinkV1):Promise<Readonly<{history:Map<string,Set<string>>;events:number}>>{
 const history=new Map<string,Set<string>>(),directory=storePathsV1(storeRoot).origins;
 let events=0,segments=0;
 const origins=await listOriginsV1(storeRoot),heads=await Promise.all(origins.map(origin=>readHeadV1(storeRoot,origin)));
 const total=heads.reduce((sum,head)=>sum+(head===undefined?0:head.segment+1),0);
 for(const [index,origin] of origins.entries()){
  const head=heads[index];
  if(head===undefined)continue;
  for(let segment=0;segment<=head.segment;segment++){
   signal?.throwIfAborted();
   const data=await readFile(join(directory,origin,"segments",`${String(segment).padStart(12,"0")}.jsonl`)).catch(()=>undefined);
   if(++segments%10===0)sink({version:1,event:"progress",stage:"adopt-history",done:segments,total});
   if(data===undefined)continue;
   for(const line of data.toString("utf8",0,segment===head.segment?Math.min(head.bytes,data.length):data.length).split("\n")){
    if(line==="")continue;
    let event:{operation?:string;path?:unknown;sha256?:unknown;executable?:unknown;target?:unknown};
    try{event=JSON.parse(line) as typeof event;}catch{continue;}
    events++;
    if(typeof event.path!=="string"||!wanted.has(event.path))continue;
    const key=event.operation==="put"&&typeof event.sha256==="string"?event.sha256+bit(!modeless&&event.executable===true):event.operation==="link"&&typeof event.target==="string"?`link:${event.target}`:undefined;
    if(key===undefined)continue;
    let known=history.get(event.path);
    if(known===undefined){known=new Set();history.set(event.path,known);}
    known.add(key);
   }
  }
 }
 return {history,events};
}

async function forkOf(worktree:string,localTip:string,tips:readonly string[],signal?:AbortSignal):Promise<string|null>{
 if(tips.length===0)return localTip;
 const own=await runGit(worktree,["rev-list","--first-parent","--stdin"],signal,`${localTip}\n${tips.map(tip=>`^${tip}`).join("\n")}\n`);
 if(own===null||own.code!==0)return null;
 if(own.out==="")return localTip;
 const parent=await runGit(worktree,["rev-parse","--verify","-q",`${own.out.split("\n").at(-1)!}^`],signal);
 return parent!==null&&parent.code===0&&oid.test(parent.out)?parent.out:null;
}

async function baseOf(worktree:string,commit:string,paths:ReadonlySet<string>,modeless:boolean,signal?:AbortSignal):Promise<Map<string,string>|null>{
 const listing=await runGit(worktree,["ls-tree","-r","-z","--full-tree",commit],signal);
 if(listing===null||listing.code!==0)return null;
 const base=new Map<string,string>();
 for(const row of listing.out.split("\0")){
  const tab=row.indexOf("\t");
  if(tab<0)continue;
  const path=row.slice(tab+1),[mode,type,id]=row.slice(0,tab).split(" ");
  if(type==="blob"&&id!==undefined&&paths.has(path))base.set(path,id+bit(!modeless&&mode==="100755"));
 }
 return base;
}

async function fleetTipsV1(worktree:string,storeRoot:string,refs:ReadonlyMap<string,ReadonlySet<string>>,fleetTip:string|null,signal?:AbortSignal):Promise<readonly string[]>{
 const found=new Set<string>(fleetTip===null?[]:[fleetTip]);
 for(const shas of refs.values()){
  for(const sha of shas){
   if(found.size>=20000)break;
   const text=await readFile(blobPathV1(storeRoot,sha.slice(0,64)),"utf8").catch(()=>"");
   for(const line of text.split("\n")){const match=/^([0-9a-f]{40}(?:[0-9a-f]{24})?)(?:\s|$)/u.exec(line.trim());if(match!==null)found.add(match[1]!);}
  }
 }
 if(found.size===0)return [];
 const checked=await runGit(worktree,["cat-file","--batch-check"],signal,`${[...found].join("\n")}\n`);
 return checked===null||checked.code!==0?[]:checked.out.split("\n").filter(line=>/^[0-9a-f]+ commit /u.test(line)).map(line=>line.slice(0,line.indexOf(" ")));
}

async function ancestorBlobsV1(worktree:string,tips:readonly string[],paths:readonly string[],modeless:boolean,signal?:AbortSignal):Promise<Map<string,Set<string>>>{
 const found=new Map<string,Set<string>>();
 if(tips.length===0)return found;
 const input=`${tips.join("\n")}\n`;
 for(let start=0;start<paths.length;){
  let end=start,size=0;
  while(end<paths.length&&(end===start||size+paths[end]!.length<16000)){size+=paths[end]!.length+1;end++;}
  const log=await runGit(worktree,["--literal-pathspecs","log","--no-renames","--raw","--no-abbrev","-z","--format=","--stdin","--",...paths.slice(start,end)],signal,input);
  start=end;
  if(log===null||log.code!==0)continue;
  const tokens=log.out.split("\0");
  for(let index=0;index<tokens.length;){
   const token=tokens[index]!;
   if(!token.startsWith(":")||index+1>=tokens.length){index++;continue;}
   const fields=token.slice(1).split(" "),path=tokens[index+1]!;
   index+=2;
   let known=found.get(path);
   if(known===undefined){known=new Set();found.set(path,known);}
   for(const [mode,id] of [[fields[0],fields[2]],[fields[1],fields[3]]])if(id!==undefined&&!zeros.test(id))known.add(id+bit(!modeless&&mode==="100755"));
  }
 }
 return found;
}

const fleetKey=(remote:AdoptRemoteV1|undefined):string|null=>remote?.operation==="put"?remote.sha256!:remote?.operation==="link"?`link:${remote.target}`:null;

async function fleetIdentityV1(storeRoot:string,remote:AdoptRemoteV1|undefined,format:string,modeless:boolean):Promise<string|null>{
 if(remote===undefined||remote.operation==="delete")return null;
 if(remote.operation==="link")return gitHash(format,Buffer.byteLength(remote.target!)).update(remote.target!).digest("hex");
 const data=await readFile(blobPathV1(storeRoot,remote.sha256!)).catch(()=>undefined);
 return data===undefined?"":gitHash(format,data.length).update(data).digest("hex")+bit(!modeless&&remote.executable);
}

async function aliasesV1(worktree:string,ignore:FleetIgnoreV1,refused:readonly Readonly<{path:string;reason:string}>[],signal:AbortSignal|undefined,visit:(entry:FleetScanEntryV1)=>void,listed:(path:string)=>void):Promise<readonly (readonly [string,string])[]>{
 const found:[string,string][]=[],todo=refused.filter(item=>item.reason==="not-nfc").map(item=>[item.path.normalize("NFC"),item.path] as [string,string]);
 for(let next=0;next<todo.length;next++){
  signal?.throwIfAborted();
  const [clean,real]=todo[next]!,file=join(worktree,...real.split("/"));
  if(refusePathV1(clean)!==undefined||(await readdir(dirname(file)).catch(()=>[] as string[])).includes(clean.slice(clean.lastIndexOf("/")+1)))continue;
  const info=await lstat(file).catch(()=>undefined);
  if(info===undefined)continue;
  found.push([clean,real]);
  if(info.isDirectory()){
   const inner=await scanSharedV1(file,{digest:ignore.digest,ignores:(path,isDirectory)=>ignore.ignores(`${clean}/${path}`,isDirectory),child:(path,isDirectory)=>(ignore.child??ignore.ignores)(`${clean}/${path}`,isDirectory)},signal,entry=>{if(!entry.path.startsWith(".git/"))visit({...entry,path:`${clean}/${entry.path}`});},path=>{listed(path===""?clean:`${clean}/${path}`);});
   for(const item of inner.refused)if(item.reason==="not-nfc")todo.push([`${clean}/${item.path.normalize("NFC")}`,`${real}/${item.path}`]);
  }else if(info.isSymbolicLink()){
   const raw=await readlink(file),target=process.platform==="win32"?raw.replaceAll("\\","/"):raw;
   if(linkRefusalV1(clean,target)===undefined)visit({path:clean,kind:"link",bytes:info.size,mtimeMs:info.mtimeMs,executable:false,target});
  }else if(info.isFile())visit({path:clean,kind:"file",bytes:info.size,mtimeMs:info.mtimeMs,executable:process.platform!=="win32"&&(info.mode&0o111)!==0});
 }
 return found;
}

const resolver=(aliases:readonly (readonly [string,string])[]):((path:string)=>string)=>aliases.length===0?path=>path:path=>{
 let best:readonly [string,string]|undefined;
 for(const pair of aliases)if((path===pair[0]||path.startsWith(`${pair[0]}/`))&&(best===undefined||pair[0].length>best[0].length))best=pair;
 return best===undefined?path:best[1]+path.slice(best[0].length);
};

async function survey(worktree:string,storeRoot:string,manifest:CompactTableV1,ignore:FleetIgnoreV1,sample:number,concurrency:number,modeless:boolean,sink:ReplicaSinkV1,signal?:AbortSignal):Promise<Survey>{
 const found:FleetScanEntryV1[]=[],directories:string[]=[];
 const scan=await scanSharedV1(worktree,ignore,signal,entry=>{found.push(entry);},path=>{directories.push(path);});
 const aliases=await aliasesV1(worktree,ignore,scan.refused,signal,entry=>{found.push(entry);},path=>{directories.push(path);}),disk=resolver(aliases),file=(path:string):string=>join(worktree,...disk(path).split("/"));
 const hashed=(item:Pending,identity:Readonly<{sha256:string;goid:string}>):Pending=>{
  const executable=modeless?undefined:item.entry.executable;
  return {...item,key:identity.sha256,mark:identity.sha256+bit(executable),goid:identity.goid+bit(executable),local:{kind:"file",mtimeMs:item.entry.mtimeMs,sha256:identity.sha256,...(executable===undefined?{}:{executable})}};
 };
 const nestedRoots=await nestedRootsOf(worktree,directories,concurrency,disk,signal),counts=empty(),samples=emptySamples(),decisions:AdoptDecisionV1[]=[],same:Array<{path:string;mtimeMs:number}>=[],publish:string[]=[],edit:string[]=[],moves:Move[]=[],present=new Set<string>(),pending:Pending[]=[],unhashed:Pending[]=[];
 const note=(category:AdoptCategoryV1,path:string):void=>{counts[category]++;if(samples[category].length<sample)samples[category].push(path);};
 const repository=await lstat(join(worktree,".git","HEAD")).then(()=>true,()=>false),head=repository?await runGit(worktree,["rev-parse","--verify","-q","HEAD^{commit}"],signal):null,localTip=head!==null&&head.code===0&&oid.test(head.out)?head.out:null;
 const format=repository&&(await runGit(worktree,["rev-parse","--show-object-format"],signal))?.out==="sha256"?"sha256":"sha1";
 let done=0;
 await each(found,concurrency,signal,async entry=>{
  present.add(entry.path);
  if(insideNested(nestedRoots,entry.path)){note("nested",entry.path);return;}
  if(entry.path.startsWith(".git/"))return;
  const id=manifest.idOf(entry.path),operation=id===undefined?undefined:manifest.operationAt(id);
  const remote:AdoptRemoteV1|undefined=id===undefined||operation===undefined?undefined:{operation,hlc:manifest.hlcAt(id),...(operation==="put"?{sha256:manifest.shaAt(id),executable:manifest.executableAt(id)}:operation==="link"?{target:manifest.targets.get(id)}:{})};
  if(entry.kind==="link"){
   const local:AdoptLocalV1={kind:"link",target:entry.target,mtimeMs:entry.mtimeMs};
   if(remote!==undefined&&sameContent(local,remote)){note("same",entry.path);same.push({path:entry.path,mtimeMs:entry.mtimeMs});return;}
   pending.push({entry,remote,key:`link:${entry.target}`,mark:`link:${entry.target}`,goid:gitHash(format,Buffer.byteLength(entry.target!)).update(entry.target!).digest("hex"),local,seen:null,base:"unknown",baseId:undefined});
  }else{
   const item:Pending={entry,remote,key:"",mark:"",goid:"",local:{kind:"file",mtimeMs:entry.mtimeMs},seen:null,base:"unknown",baseId:undefined};
   if(remote?.operation==="put"&&entry.bytes===manifest.bytes[id!]){
    const identity=await identityOfFile(file(entry.path),entry.bytes,format).catch(()=>undefined);
    if(identity===undefined)return;
    if(identity.sha256===remote.sha256&&(modeless||entry.executable===remote.executable)){note("same",entry.path);same.push({path:entry.path,mtimeMs:entry.mtimeMs});}
    else pending.push(hashed(item,identity));
   }else unhashed.push(item);
  }
  if(++done%2000===0)sink({version:1,event:"progress",stage:"adopt-plan",done,total:found.length});
 });
 await each(unhashed,concurrency,signal,async item=>{
  const identity=await identityOfFile(file(item.entry.path),item.entry.bytes,format).catch(()=>undefined);
  if(identity===undefined)return;
  pending.push(hashed(item,identity));
 });
 pending.sort((left,right)=>left.entry.path<right.entry.path?-1:left.entry.path>right.entry.path?1:0);
 const fleetTip=await fleetTipOf(manifest,storeRoot);
 const refPaths=pending.length===0?[]:[...manifest.ids()].map(id=>manifest.book.name(id)).filter(path=>path===".git/HEAD"||path===".git/packed-refs"||path.startsWith(".git/refs/"));
 const {history,events}=pending.length===0?{history:new Map<string,Set<string>>(),events:0}:await historyV1(storeRoot,new Set([...pending.map(item=>item.entry.path),...refPaths]),modeless,signal,sink);
 const tips=pending.length===0?[]:await fleetTipsV1(worktree,storeRoot,new Map(refPaths.map(path=>[path,history.get(path)??new Set<string>()])),fleetTip,signal);
 const fork=pending.length===0||localTip===null?null:await forkOf(worktree,localTip,tips,signal),base=fork===null?null:await baseOf(worktree,fork,new Set(pending.map(item=>item.entry.path)),modeless,signal);
 for(const item of pending){
  item.seen=history.get(item.entry.path)?.has(item.mark)?"journal":null;
  item.baseId=base?.get(item.entry.path);
  item.base=base===null?"unknown":item.baseId===undefined?"absent":item.baseId===item.goid?"equal":"differs";
 }
 const searched=pending.filter(item=>item.seen===null&&!(item.remote!==undefined&&item.base==="equal"));
 if(searched.length>0&&searched.length<=50000){
  const blobs=await ancestorBlobsV1(worktree,tips,searched.map(item=>item.entry.path),modeless,signal);
  for(const item of searched)if(blobs.get(item.entry.path)?.has(item.goid))item.seen="git";
 }
 for(const item of pending){
  const {entry,remote}=item;
  let fleetSince:boolean|null=null;
  if(item.seen===null&&item.base!=="equal"&&item.base!=="unknown")fleetSince=(await fleetIdentityV1(storeRoot,remote,format,modeless))!==(item.baseId??null);
  const evidence:AdoptEvidenceV1={seen:item.seen,base:item.base,fleetSince},category=classifyV1(item.local,remote,evidence)!,reason=reasonV1(category,remote,evidence);
  note(category,entry.path);
  if(category==="publish")publish.push(entry.path);
  else if(category==="edit"){edit.push(entry.path);decisions.push({path:entry.path,category,reason:reason!,localSha256:item.key,fleetSha256:fleetKey(remote),modeOnly:fleetKey(remote)===item.key,remoteWallMs:wallOf(remote!.hlc),localMtimeMs:entry.mtimeMs});}
  else if(category==="stale"||category==="conflict"){
   decisions.push({path:entry.path,category,reason:reason!,localSha256:item.key,fleetSha256:fleetKey(remote),modeOnly:fleetKey(remote)===item.key,remoteWallMs:remote===undefined?0:wallOf(remote.hlc),localMtimeMs:entry.mtimeMs});
   moves.push({path:entry.path,category,reason:reason!,sha256:item.key,fleet:fleetKey(remote)});
  }
 }
 for(const id of manifest.ids()){
  const path=manifest.book.name(id),operation=manifest.operationAt(id);
  if(operation==="delete"||present.has(path)||indexRealmV1(path)||insideNested(nestedRoots,path))continue;
  if(!sharedV1(path,false,ignore)||refusePathV1(path)!==undefined)continue;
  note("materialise",path);
 }
 decisions.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
 const staged=repository?await runGit(worktree,["diff","--cached","--name-only"],signal):null;
 const relation=await relationOf(worktree,fleetTip,localTip,signal);
 sink({version:1,event:"progress",stage:"adopt-plan",done:found.length,total:found.length});
 const plan:AdoptPlanV1={version:1,dryRun:false,renamed:aliases.map(pair=>pair[0]).sort(ordered),counts,samples,decisions,nested:[...nestedRoots].sort(),git:{fleetTip,localTip,relation,stagedPaths:staged===null||staged.code!==0?null:staged.out===""?0:staged.out.split("\n").length},evidence:{events,commits:tips.length}};
 return {plan,same,publish,edit,moves,nestedRoots,present};
}

async function ignoreOf(worktree:string,manifest:CompactTableV1,storeRoot:string):Promise<Readonly<{ignore:FleetIgnoreV1;localText:string}>>{
 const localText=await readFile(join(worktree,".gitignore"),"utf8").catch(()=>""),rules=manifest.get(".gitignore"),fleet=compileIgnoreV1(rules?.operation==="put"?await readFile(blobPathV1(storeRoot,rules.sha256!),"utf8").catch(()=>""):""),mine=compileIgnoreV1(localText);
 return {localText,ignore:{digest:fleet.digest,ignores:(path,isDirectory)=>fleet.ignores(path,isDirectory)||mine.ignores(path,isDirectory),child:(path,isDirectory)=>(fleet.child??fleet.ignores)(path,isDirectory)||(mine.child??mine.ignores)(path,isDirectory)}};
}

/** 🧭️ Plans the adoption of the repository at `worktree` into the store without changing anything: no state file, no lock, no object, no marker is written, no name is renamed (a name that is not NFC is read under its NFC name and listed in `renamed`), and the local and the fleet's ignore rules both exclude paths from it. */
export async function planAdoptV1(options:ReplicaOptionsV1,adopt:Pick<AdoptOptionsV1,"sample">,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<AdoptPlanV1>{
 const settings=settle(options);
 if(!await storeReachableV1(settings.storeRoot))throw Error(`Replica store not found: ${settings.storeRoot}`);
 const folded=await foldStoreV1(settings.storeRoot,signal,sink,settings.now),{ignore}=await ignoreOf(settings.worktree,folded.manifest,settings.storeRoot);
 const result=await survey(settings.worktree,settings.storeRoot,folded.manifest,guarded(ignore),adopt.sample,settings.concurrency,settings.modeless,sink,signal);
 return {...result.plan,dryRun:true};
}

type Marker={version:1;step:"begin"|AdoptStepV1;stamp:string;startedMs:number;force:boolean;fleetTip:string|null;preTree:string|null;stageLost:string|null;counts:Record<AdoptCategoryV1,number>;nested:string[]};
type Decided=Readonly<{publish:readonly string[];edit:readonly string[];moves:readonly Move[]}>;
const markerFile=(worktree:string):string=>localPathsV1(worktree).directory+"/adopt.json";
const decidedFile=(worktree:string):string=>`${markerFile(worktree)}.decided`;
const lostOf=(stamp:string,move:Pick<Move,"category"|"path">):string=>`lost/adopt-${stamp}/${move.category}/${move.path}`;
async function readDecided(worktree:string):Promise<Decided>{
 try{return JSON.parse(await readFile(decidedFile(worktree),"utf8")) as Decided;}catch{return {publish:[],edit:[],moves:[]};}
}
const journalFile=(worktree:string):string=>localPathsV1(worktree).directory+"/adopt-journal.jsonl";

async function writeMarker(worktree:string,marker:Marker):Promise<void>{
 const file=markerFile(worktree),temporary=`${file}.tmp`;
 await writeFile(temporary,JSON.stringify(marker));
 await rename(temporary,file);
}

/** 📖️ Reads the adoption marker of a worktree (`.cache/f/adopt.json`): the last finished step, or `undefined` when no adoption ran there. */
export async function readAdoptStepV1(worktree:string):Promise<"begin"|AdoptStepV1|undefined>{
 try{return (JSON.parse(await readFile(markerFile(worktree),"utf8")) as Marker).step;}catch{return undefined;}
}

const stampOf=(wall:number):string=>new Date(wall).toISOString().replace(/[:.]/gu,"-");

function report(marker:Marker,originId:string,decided:Decided,journal:string,changed:readonly string[],pending:readonly string[]):AdoptReportV1{
 const linearised:{path:string;winner:string;loser:string;result:string}[]=[],issues:string[]=[],renamed:string[]=[];
 const lines=existsSync(journal)?readFileSync(journal,"utf8").split("\n").filter(line=>line!==""):[];
 const seen=new Set<string>();
 for(const line of lines){
  if(seen.has(line))continue;
  seen.add(line);
  const event=JSON.parse(line) as ReplicaEventV1,path=event.path??"";
  if(event.event==="overwritten"&&event.path===undefined)continue;
  if(event.event==="linearised")linearised.push({path,winner:event.winner??"",loser:event.loser??"",result:event.result??""});
  else if(event.reason==="renamed-to-nfc")renamed.push(path);
  else issues.push(`${event.event} ${path} ${event.reason??event.message??""}`.trim());
 }
 const stale=decided.moves.filter(move=>move.category==="stale").map(move=>({path:move.path,lost:lostOf(marker.stamp,move),reason:move.reason})),conflicts=decided.moves.filter(move=>move.category==="conflict").map(move=>({path:move.path,lost:lostOf(marker.stamp,move),reason:move.reason,local:move.sha256,fleet:move.fleet}));
 return {version:1,stamp:marker.stamp,originId,renamed:renamed.sort(ordered),counts:marker.counts,published:[...decided.publish].sort(),edited:[...decided.edit].sort(),stale:stale.sort((left,right)=>left.path<right.path?-1:1),conflicts:conflicts.sort((left,right)=>left.path<right.path?-1:1),linearised,stage:{lost:marker.stageLost,changed:changed.length,paths:changed.slice(0,1000)},pending,nested:marker.nested,issues};
}

const markdownOf=(value:AdoptReportV1):string=>{
 const list=(title:string,items:readonly string[]):string=>items.length===0?"":`\n## ${title} (${items.length})\n\n${items.slice(0,500).map(item=>`- ${item}`).join("\n")}${items.length>500?`\n- ... ${items.length-500} more in the JSON report`:""}\n`;
 return `# Adoption report ${value.stamp} of ${value.originId}\n\n${CATEGORIES.map(category=>`- ${category}: ${value.counts[category]}`).join("\n")}\n${list("Names that were not NFC (macOS tar extracts them like that), renamed to NFC before anything was compared",value.renamed)}${list("Conflicts: changed here and in the fleet, the fleet's version stays, the local version is kept in lost/",value.conflicts.map(item=>`${item.path} (${item.reason}) local ${item.local.slice(0,12)}, fleet ${item.fleet===null?"deleted":item.fleet.slice(0,12)} -> ${item.lost}`))}${list("Stale: older versions of fleet files or files the fleet deleted, the fleet wins, local bytes in lost/",value.stale.map(item=>`${item.path} (${item.reason}) -> ${item.lost}`))}${list("Edited here on top of the fleet's version, published as edits",value.edited)}${list("Only here, published as new work",value.published)}${list("Branches linearised",value.linearised.map(item=>`${item.path}: ${item.loser.slice(0,12)} on top of ${item.winner.slice(0,12)} = ${item.result.slice(0,12)}`))}\n## Stage\n\nPrevious local index: ${value.stage.lost??"none"}\nStaged paths whose state changed: ${value.stage.changed}\n${value.stage.paths.slice(0,500).map(path=>`- ${path}`).join("\n")}\n${list("Pending (still queued)",value.pending)}${list("Nested repositories left alone",value.nested)}${list("Issues",value.issues)}`;
};

/** 🧳️ Adopts the existing repository at the worktree into the fleet, loss-free, content-based and resumable: the store is folded; names that are not NFC are renamed to NFC first, as `bootstrap-settle` does (`normaliseNamesV1`), so that a tree extracted by macOS `tar` is compared under the fleet's names; identical files are adopted (equal bytes whose executable bit differs from the fleet's are a difference like any other and follow the same rule), missing ones are materialised, a file that differs is decided by its content against the fleet's history and the repository's own base and never by a modification time (a version the fleet had before, a file the fleet deleted since, or a file unchanged since the base is `stale`: the fleet wins and the local bytes go to `.cache/f/lost/adopt-<stamp>/stale/`; local work on a fleet version that did not move since the base, and local-only files, are published; a file changed on both sides since the base is a `conflict`: the fleet's version stays and the local one goes to `.cache/f/lost/adopt-<stamp>/conflict/` and into the report); ignored files and nested repositories are never touched; the git objects are united first and a repository without a common commit is refused unless `force`, branch refs follow the linearisation of concurrent commits, the previous local index is copied to `lost/` and replaced by the fleet's stage. A report (`lost/adopt-<stamp>.json` and `.md`) lists every difference. `adopt.dryRun` only plans. After every step the marker `.cache/f/adopt.json` is written, so a crash continues where it stopped. */
export async function adoptV1(options:ReplicaOptionsV1,adopt:AdoptOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<AdoptResultV1>{
 if(adopt.dryRun)return {plan:await planAdoptV1(options,adopt,signal,sink),report:undefined,files:0,adopted:0,queued:0};
 const settings=settle(options),{worktree,storeRoot}=settings,paths=localPathsV1(worktree);
 if(!await storeReachableV1(storeRoot))throw Error(`Replica store not found: ${storeRoot}`);
 await mkdir(paths.directory,{recursive:true});
 const journal=journalFile(worktree);
 let current:ReplicaV1|undefined;
 const tap:ReplicaSinkV1=event=>{
  if(["overwritten","kept","linearised","refused","blocked","skipped","error"].includes(event.event)){
   const id=event.path===undefined?undefined:current?.book.find(event.path);
   appendFileSync(journal,`${JSON.stringify({...event,version:undefined,operation:id===undefined?undefined:current?.manifest.operationAt(id)})}\n`);
  }
  sink(event);
 };
 const replica=await openReplicaV1(options,tap);
 current=replica;
 try{
  let marker:Marker|undefined;
  try{marker=JSON.parse(await readFile(markerFile(worktree),"utf8")) as Marker;}catch{marker=undefined;}
  if(marker===undefined){
   if(replica.files.size>0)throw new AdoptRefusedV1("already-joined");
   marker={version:1,step:"begin",stamp:stampOf(settings.now()),startedMs:settings.now(),force:adopt.force,fleetTip:null,preTree:null,stageLost:null,counts:empty(),nested:[]};
   await writeMarker(worktree,marker);
  }
  const reached=(step:AdoptStepV1):boolean=>STEPS.indexOf(step)<=STEPS.indexOf(marker!.step as AdoptStepV1);
  const finish=async(step:AdoptStepV1):Promise<void>=>{marker={...marker!,step};await writeMarker(worktree,marker);adopt.fault?.(step);};
  const {manifest,files,book}=replica;
  let plan:Survey|undefined;
  if(!reached("files")){
   const folded=await foldInto(storeRoot,manifest,signal,sink,settings.now);
   Object.assign(replica.cursors,folded.cursors);
   if(folded.hlc!==""){try{replica.clock=mergeClockV1(replica.clock,folded.hlc,settings.now());}catch{}}
   const {ignore,localText}=await ignoreOf(worktree,manifest,storeRoot);
   replica.ignore=guarded(ignore);replica.ignoreHash=createHash("sha256").update(localText).digest("hex");
   await normaliseNamesV1(worktree,replica.ignore,signal,tap,"adopt");
   plan=await survey(worktree,storeRoot,manifest,replica.ignore,adopt.sample,settings.concurrency,settings.modeless,sink,signal);
   if(!reached("planned")){
    marker={...marker,fleetTip:plan.plan.git.fleetTip,counts:plan.plan.counts as Record<AdoptCategoryV1,number>,nested:[...plan.plan.nested]};
    if(!existsSync(decidedFile(worktree)))await writeFile(decidedFile(worktree),JSON.stringify({publish:plan.publish,edit:plan.edit,moves:plan.moves}));
    await finish("planned");
   }
  }
  if(!reached("saved")){
   const index=join(worktree,".git","index"),stageLost=join(paths.lost,`adopt-${marker.stamp}`,"index");
   let preTree:string|null=null,kept:string|null=null;
   if(await lstat(index).then(()=>true,()=>false)){
    await mkdir(dirname(stageLost),{recursive:true});
    await copyFile(index,stageLost);kept=`lost/adopt-${marker.stamp}/index`;
    preTree=await writeTreeV1(worktree,{signal});
   }
   await liftRefs(replica,marker.stamp,signal);
   marker={...marker,preTree,stageLost:kept};
   await finish("saved");
  }
  const everything=plan;
  if(!reached("objects")){
   for(const row of everything!.same){
    const id=book.find(row.path);
    if(id===undefined||!manifest.live(id))continue;
    files.write(id,{...manifest.entryAt(id),mtimeMs:row.mtimeMs,seenMs:settings.now()});replica.folds.add(id);replica.changed.add(filesKey(id));
   }
   const state=await gitStateV1(worktree,signal);
   if(state.present&&manifest.has(".git/HEAD")){
    for(const id of manifest.ids()){
     const path=book.name(id);
     if(path.startsWith(".git/objects/")&&!files.live(id)&&manifest.operationAt(id)!=="delete")enqueueId(replica,id);
    }
    for(const id of replica.queue.snapshot())if(!book.name(id).startsWith(".git/objects/")){replica.queue.removeId(id);replica.changed.add(queueKey(id));}
    replica.dirty=true;await persist(replica,true);
    await drain(replica,signal);
    const tip=await fleetTipOf(manifest,storeRoot),local=state.commit,relation=await relationOf(worktree,tip,local,signal);
    if(relation==="unrelated"&&!marker.force&&!adopt.force)throw new AdoptRefusedV1("no-common-history");
   }
   await finish("objects");
  }
  if(!reached("files")){
   await decide(replica,marker.stamp,await readDecided(worktree),signal);
   for(const id of manifest.ids()){
    if(files.live(id)||replica.queue.hasId(id))continue;
    const path=book.name(id);
    if(!indexRealmV1(path)&&insideNested(everything!.nestedRoots,path))continue;
    if(manifest.operationAt(id)==="delete"&&!everything!.present.has(path)){files.write(id,{...manifest.entryAt(id),mtimeMs:0});replica.changed.add(filesKey(id));}
    else enqueueId(replica,id);
   }
   replica.dirty=true;await persist(replica,true);
   await finish("files");
  }
  if(!reached("applied")){await drain(replica,signal);await finish("applied");}
  if(!reached("published")){
   const decided=await readDecided(worktree),targets=[...new Set([...decided.publish,...decided.edit,...await gitPathsV1(worktree,files)])];
   for(let start=0;start<targets.length;start+=5000){signal?.throwIfAborted();await publishOnceV1(replica,signal,targets.slice(start,start+5000));}
   await persist(replica,true);await finish("published");
  }
  let changed:readonly string[]=[];
  if(!reached("staged")){
   let post=await writeTreeV1(worktree,{signal});
   post=await restage(replica,marker,post,signal);
   if(marker.preTree!==null&&post!==null&&marker.preTree!==post)changed=((await diffTreeV1(worktree,marker.preTree,post,signal))??[]).map(change=>change.path);
   await writeFile(`${journal}.stage`,JSON.stringify(changed));
   await finish("staged");
  }else changed=JSON.parse(await readFile(`${journal}.stage`,"utf8").catch(()=>"[]")) as string[];
  const pending=[...replica.queue.keys()].filter(path=>!indexRealmV1(path)),done=report(marker,settings.originId,await readDecided(worktree),journal,changed,pending),base=join(paths.lost,`adopt-${marker.stamp}`);
  await mkdir(base,{recursive:true});
  await writeFile(`${base}.json`,JSON.stringify(done));
  await writeFile(`${base}.md`,markdownOf(done));
  await finish("reported");
  return {plan:{...(plan?.plan??emptyPlan(marker)),renamed:done.renamed,dryRun:false},report:done,files:replica.files.size+replica.queue.size,adopted:marker.counts.same,queued:replica.queue.size};
 }finally{await closeReplicaV1(replica);}
}

async function setAside(replica:ReplicaV1,stamp:string,moves:readonly Move[],signal?:AbortSignal):Promise<void>{
 const {worktree}=replica.settings;
 for(const move of moves){
  signal?.throwIfAborted();
  const target=join(worktree,...move.path.split("/")),info=await lstat(target).catch(()=>undefined);
  if(info===undefined||info.isDirectory())continue;
  const copy=join(localPathsV1(worktree).directory,...lostOf(stamp,move).split("/"));
  await mkdir(dirname(copy),{recursive:true});
  if(info.isSymbolicLink())await writeFile(copy,await readlink(target));else await copyFile(target,copy);
  await unlink(target);
 }
}

const GIT_FLEET_WINS=(path:string):boolean=>path===".git/HEAD"||path===".git/packed-refs"||path.startsWith(".git/refs/")&&!path.startsWith(".git/refs/heads/")&&!path.startsWith(".git/refs/adopt/");

async function setAsideGit(replica:ReplicaV1,stamp:string,signal?:AbortSignal):Promise<void>{
 const {worktree}=replica.settings,{manifest,book}=replica;
 for(const id of manifest.ids()){
  const path=book.name(id);
  if(manifest.operationAt(id)!=="put"||!GIT_FLEET_WINS(path))continue;
  signal?.throwIfAborted();
  const target=join(worktree,...path.split("/")),info=await lstat(target).catch(()=>undefined);
  if(info===undefined||!info.isFile()||(await hashFileV1(target)).sha256===manifest.shaAt(id))continue;
  const copy=join(localPathsV1(worktree).directory,"lost",`adopt-${stamp}`,"git",...path.split("/"));
  await mkdir(dirname(copy),{recursive:true});
  await copyFile(target,copy);
  await unlink(target);
 }
}

async function liftRefs(replica:ReplicaV1,stamp:string,signal?:AbortSignal):Promise<void>{
 const {worktree,storeRoot}=replica.settings,{manifest,book}=replica;
 const listing=await runGit(worktree,["for-each-ref","--format=%(refname) %(objectname)"],signal);
 if(listing===null||listing.code!==0)return;
 const text=async(path:string):Promise<string>=>{const id=manifest.idOf(path);return id===undefined||manifest.operationAt(id)!=="put"?"":readFile(blobPathV1(storeRoot,manifest.shaAt(id)),"utf8").catch(()=>"");};
 const fleet=new Map<string,string>(),gone=new Set<string>();
 for(const id of manifest.ids()){
  const path=book.name(id);
  if(!path.startsWith(".git/refs/"))continue;
  if(manifest.operationAt(id)==="delete"){gone.add(path.slice(5));continue;}
  const value=(await text(path)).trim();
  if(oid.test(value))fleet.set(path.slice(5),value);
 }
 for(const line of (await text(".git/packed-refs")).split("\n")){const [value,name]=line.split(" ");if(oid.test(value??"")&&name?.startsWith("refs/")===true&&!fleet.has(name))fleet.set(name,value!);}
 const lift=async(name:string,value:string):Promise<void>=>{
  const file=join(worktree,".git",...name.split("/"));
  if(await lstat(file).then(()=>true,()=>false))return;
  await mkdir(dirname(file),{recursive:true});
  await writeFile(file,`${value}\n`);
 };
 for(const row of listing.out.split("\n")){
  const [name,value]=row.split(" ");
  if(name===undefined||value===undefined||!name.startsWith("refs/")||name.startsWith("refs/remotes/")||name.startsWith("refs/adopt/")||name==="refs/stash")continue;
  const mine=fleet.get(name);
  if(mine===value)continue;
  if(mine===undefined){if(!gone.has(name))await lift(name,value);}
  else if(name.startsWith("refs/heads/"))await lift(name,value);
  else await lift(`refs/adopt/${stamp}/${name.slice(5).replaceAll("/","-")}`,value);
 }
 const stashes=await runGit(worktree,["stash","list","--format=%H"],signal);
 if(stashes!==null&&stashes.code===0&&stashes.out!=="")for(const [index,hash] of stashes.out.split("\n").slice(0,1000).entries())if(oid.test(hash))await lift(`refs/adopt/${stamp}/stash-${index}`,hash);
}

async function decide(replica:ReplicaV1,stamp:string,decided:Decided,signal?:AbortSignal):Promise<void>{
 await setAside(replica,stamp,decided.moves,signal);
 await setAsideGit(replica,stamp,signal);
 for(const path of [...decided.publish,...decided.edit]){
  const id=replica.manifest.idOf(path);
  if(id===undefined)continue;
  replica.files.write(id,{...replica.manifest.entryAt(id),mtimeMs:0});replica.changed.add(filesKey(id));
  replica.queue.removeId(id);replica.changed.add(queueKey(id));
 }
 replica.dirty=true;await persist(replica,true);
}

async function drain(replica:ReplicaV1,signal?:AbortSignal):Promise<void>{
 let stalled=0;
 for(let round=0;round<80&&replica.queue.size>0&&stalled<8;round++){
  const before=replica.queue.size;
  await applyQueue(replica,signal,"adopt");
  await persist(replica,true);
  stalled=replica.queue.size<before?0:stalled+1;
  if(replica.queue.size>0)await new Promise<void>(done=>{setTimeout(done,stalled===0?0:150);});
 }
}


const emptyPlan=(marker:Marker):AdoptPlanV1=>({version:1,dryRun:false,renamed:[],counts:marker.counts,samples:emptySamples(),decisions:[],nested:marker.nested,git:{fleetTip:null,localTip:null,relation:"none",stagedPaths:null},evidence:{events:0,commits:0}});

async function gitPathsV1(worktree:string,files:CompactTableV1):Promise<readonly string[]>{
 const found=[".git/HEAD",".git/packed-refs"];
 const walk=async(directory:string,every:boolean):Promise<void>=>{
  for(const item of await readdir(join(worktree,...directory.split("/")),{withFileTypes:true}).catch(()=>[])){
   const path=`${directory}/${item.name}`;
   if(item.isDirectory())await walk(path,every);
   else if(every||!files.has(path))found.push(path);
  }
 };
 await walk(".git/refs",true);
 await walk(".git/objects",false);
 return found;
}

async function restage(replica:ReplicaV1,marker:Marker,post:string|null,signal?:AbortSignal):Promise<string|null>{
 const {worktree}=replica.settings,state=await gitStateV1(worktree,signal);
 if(!state.present||marker.fleetTip===null||state.commit===null||state.commit===marker.fleetTip||post===null)return post;
 const tip=await runGit(worktree,["rev-parse",`${marker.fleetTip}^{tree}`],signal),head=await runGit(worktree,["rev-parse","HEAD^{tree}"],signal);
 if(tip===null||head===null||!oid.test(tip.out)||!oid.test(head.out))return post;
 const delta=await diffTreeV1(worktree,tip.out,post,signal);
 if(delta===null||!await readTreeV1(worktree,head.out,{signal})||!await updateIndexV1(worktree,delta,{signal,hash:head.out.length}))return post;
 const next=await writeTreeV1(worktree,{signal});
 if(next===null||next===post)return post;
 await requestRebaseV1(worktree);
 await publishOnceV1(replica,signal,[".git/index"]);
 return next;
}
