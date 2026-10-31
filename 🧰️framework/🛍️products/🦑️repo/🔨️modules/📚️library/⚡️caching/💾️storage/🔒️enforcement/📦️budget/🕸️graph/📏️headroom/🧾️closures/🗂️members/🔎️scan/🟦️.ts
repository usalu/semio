import{closeSync,fstatSync,lstatSync,openSync,readFileSync,readSync,readdirSync,renameSync,unlinkSync,writeFileSync,writeSync,type BigIntStats}from"node:fs";
import{join,relative,sep}from"node:path";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";
import{runClosureReadCohortsV1}from"../../📡️session/🟦️.ts";
import{physicalSourceOriginV1}from"../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import{MEMBER_INDEX_FILE_V1,appendMemberRowsV1,compactMemberIndexV1,encodeMemberRowV1,fullMemberRowV1,memberHashV1,memberGroupOfPathV1,memberKindOfV1,memberGroupProtectedV1,memberStampV1,parseMemberIndexV1,proveGroupV1,readMemberRowV1,type MemberRowV1}from"../🟨️.mjs";
import graphStorageSchema from"../../../../../../../../../🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"with{type:"json"};
import closures from"../../🧬️schema/🔣️.json"with{type:"json"};
import protocol from"../🧬️schema/🔣️.json"with{type:"json"};

const P=closures.definitions.Protocol.const,M=protocol.definitions.Protocol.const,fs={openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync}as never,hex=/^[a-f0-9]{64}$/u,worker=fileURLToPath(new URL("../../👷️closure-read-worker/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)));

/** 🎚️ What a scan needs from its caller: whether to stop, and where to report progress. */
export type MemberScanControlV1=Readonly<{cancelled():boolean;progress(label:string,done:number,total:number):void|Promise<void>}>;
/** 🧾️ One leaf the closure read child opens on its own, so its answer is that leaf's own hash, references and links. */
export type MemberReadRequestV1=Readonly<{path:string;stamp:string;kind:string;bytes:number;index?:Readonly<{path:string;stamp:string;bytes:number;hash:string}>}>;
/** 📬️ The closure read child's answer for one leaf. */
export type MemberReadAnswerV1=Readonly<{version:1;rows:readonly Readonly<{path:string;hash:string;expandedBytes:number;publicationGraph:string|null}>[];references:readonly string[];links:readonly Readonly<{path:string;missing:"block"|"forbidden"}>[]}>;
/** 🔌️ What indexing needs from the repository: the per-namespace writer lease (never waited for) and the closure read child, which opens each requested leaf as a request of its own and reports every answer to `accept`. */
export type MemberIndexPortsV1=Readonly<{lease(kind:"selection"|"dataset"|"canonical",identity:string,urgent?:boolean):Promise<Readonly<{release():void}>|undefined>;read(requests:readonly MemberReadRequestV1[],accept:(request:MemberReadRequestV1,answer:MemberReadAnswerV1)=>void):Promise<void>}>;
/** 📊️ What one indexing pass did. */
export type MemberIndexReportV1=Readonly<{groups:number;skipped:number;busy:number;indexed:number;refused:number;compacted:number;cancelled:boolean}>;
/** 📈️ What one deep verification found; mismatches list at most one hundred names. */
export type MemberVerifyReportV1=Readonly<{groups:number;verified:number;bytes:number;mismatched:readonly string[];mismatches:number;stale:number;unindexed:number;vanished:number;closures:number;closureMismatched:readonly string[];closureMismatches:number;cancelled:boolean}>;

type Leaf={name:string;path:string;kind:string;bytes:number;stamp:string;proof:string};
const absent=(error:unknown)=>(error as NodeJS.ErrnoException)?.code==="ENOENT";
const stamp7=(row:BigIntStats)=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");

const directoryOf=(group:string)=>group==="canonical"?"inference":group==="generations"?"generations":"inference/"+group;
const present=(path:string)=>{try{return lstatSync(path).isDirectory();}catch(error){if(absent(error))return false;throw error;}};

/** 🗂️ Lists the groups of an image root: the shared `canonical` namespace and the content-addressed `generations` first (they decide which closure inventories stay referenced), then `selections/<id>` and `contexts/<id>`, sorted. */
export function listBuildGraphMemberGroupsV1(root:string):readonly string[]{
 const names:string[]=[];
 for(const kind of["selections","contexts"]){let ids:string[]=[];try{ids=readdirSync(join(root,"inference",kind));}catch(error){if(!absent(error))throw error;}for(const id of ids)if(hex.test(id))names.push(kind+"/"+id);}
 return[...(present(join(root,"inference"))?["canonical"]:[]),...(present(join(root,"generations"))?["generations"]:[]),...names.sort()];
}

function leavesOf(root:string,group:string):Leaf[]{
 const base=directoryOf(group),directory=join(root,...base.split("/")),leaves:Leaf[]=[],pending=[""];
 while(pending.length){
  const at=pending.pop()!;let entries;try{entries=readdirSync(join(directory,at),{withFileTypes:true});}catch(error){if(absent(error))continue;throw error;}
  for(const entry of entries){
   const name=at?at+"/"+entry.name:entry.name;
   if(entry.isDirectory()){if(group==="canonical"&&!at&&(entry.name==="selections"||entry.name==="contexts"))continue;pending.push(name);continue;}
   if(!entry.isFile()||name===MEMBER_INDEX_FILE_V1)continue;
   const imageName=base+"/"+name,kind=memberKindOfV1(imageName);if(!kind||kind==="member-index"||kind==="generation-pointer"||memberGroupOfPathV1(imageName)?.group!==group)continue;
   const path=join(directory,name);let row:BigIntStats;try{row=lstatSync(path,{bigint:true});}catch(error){if(absent(error))continue;throw error;}
   if(!row.isFile()||row.nlink!==1n||row.size>BigInt(P.maximumFileBytes))continue;
   leaves.push({name,path,kind,bytes:Number(row.size),stamp:memberStampV1(row),proof:stamp7(row)});
  }
 }
 return leaves.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0);
}

function indexText(root:string,group:string):string{try{return readFileSync(join(root,...directoryOf(group).split("/"),MEMBER_INDEX_FILE_V1),"utf8");}catch(error){if(absent(error))return"";throw error;}}

const turn=()=>new Promise<void>(resolve=>setImmediate(resolve));

async function hashFile(path:string,control:MemberScanControlV1):Promise<string|undefined>{
 const fd=openSync(path,"r"),digest=createHash("sha256"),chunk=Buffer.alloc(1<<20);let failed=false;
 try{for(let chunks=0;;chunks++){if(chunks%16===15){await turn();}if(control.cancelled())return undefined;const count=readSync(fd,chunk,0,chunk.length,null);if(!count)return digest.digest("hex");digest.update(chunk.subarray(0,count));}}catch(error){failed=true;throw error;}finally{try{closeSync(fd);}catch(error){if(!failed)throw error;}}
}

/** 🔬️ Re-hashes every indexed leaf of every group and every content-addressed closure inventory, reports what disagrees with its recorded hash or name, and honours cancellation between files; the one place a leaf is read again after it was written. */
export async function verifyBuildGraphMembersDeepV1(root:string,control:MemberScanControlV1):Promise<MemberVerifyReportV1>{
 const groups=listBuildGraphMemberGroupsV1(root),mismatched:string[]=[],closureMismatched:string[]=[];
 let files=0,verified=0,bytes=0,mismatches=0,stale=0,unindexed=0,vanished=0,closures=0,closureMismatches=0,cancelled=false;
 const finish=():MemberVerifyReportV1=>Object.freeze({groups:groups.length,verified,bytes,mismatched,mismatches,stale,unindexed,vanished,closures,closureMismatched,closureMismatches,cancelled});
 for(let at=0;at<groups.length;at++){
  if(control.cancelled()){cancelled=true;return finish();}
  await control.progress(groups[at]!,at,groups.length);await turn();
  const group=groups[at]!,index=parseMemberIndexV1(indexText(root,group),group);
  for(const leaf of leavesOf(root,group)){
   if(++files%256===0){await turn();if(control.cancelled()){cancelled=true;return finish();}}
   const row=index.valid?index.rows.get(leaf.name):undefined;
   if(!row){unindexed++;continue;}
   if(row.k!==leaf.kind||row.b!==leaf.bytes||row.s!==leaf.stamp){stale++;continue;}
   let digest:string|undefined;try{digest=await hashFile(leaf.path,control);}catch(error){if(absent(error)){vanished++;continue;}throw error;}
   if(digest===undefined){cancelled=true;return finish();}
   verified++;bytes+=leaf.bytes;
   if(digest!==row.h){mismatches++;if(mismatched.length<100)mismatched.push(group+"/"+leaf.name);}
  }
 }
 for(const namespace of P.namespaces){
  let names:string[]=[];try{names=readdirSync(join(root,namespace)).sort();}catch(error){if(!absent(error))throw error;}
  for(let at=0;at<names.length;at++){
   const name=names[at]!;
   if(at%512===0){await turn();if(control.cancelled()){cancelled=true;return finish();}await control.progress(namespace,at,names.length);}
   if(!/^[a-f0-9]{64}\.json$/u.test(name))continue;
   let digest:string|undefined;try{digest=await hashFile(join(root,namespace,name),control);}catch(error){if(absent(error))continue;throw error;}
   if(digest===undefined){cancelled=true;return finish();}
   closures++;
   if(digest!==name.slice(0,64)){closureMismatches++;if(closureMismatched.length<100)closureMismatched.push(namespace+"/"+name);}
  }
 }
 return finish();
}

type Request=MemberReadRequestV1;
const READ_CHUNK_V1=64;

function packet(root:string,files:readonly Request[]):string{return JSON.stringify({files,imageRoot:root,remainingBytes:P.maximumProofBytes,root:process.cwd()});}

async function rowsFor(root:string,group:string,pending:readonly Leaf[],leaves:readonly Leaf[],ports:MemberIndexPortsV1,control:MemberScanControlV1,counters:{refused:number}):Promise<MemberRowV1[]>{
 const directory=join(root,...directoryOf(group).split("/")),rows:MemberRowV1[]=[],requests:Request[]=[],byPath=new Map(leaves.map(leaf=>[leaf.path,leaf]));
 const full=(leaf:Leaf,hash:string,expanded:number,graph:string|null,references:readonly string[],links:readonly{p:string;m?:"block"}[])=>{const row=readMemberRowV1(JSON.parse(encodeMemberRowV1(fullMemberRowV1({n:leaf.name,b:leaf.bytes,h:hash,k:leaf.kind as never,s:leaf.stamp,x:expanded,g:graph,r:references,l:links}))));if(row)rows.push(row);else counters.refused++;};
 for(const leaf of pending){
  if(leaf.kind==="observation"){try{const bytes=readFileSync(leaf.path);if(bytes.length===leaf.bytes)full(leaf,memberHashV1(bytes),bytes.length,null,[],[]);else counters.refused++;}catch(error){if(!absent(error))throw error;}continue;}
  if(leaf.kind==="index"){try{const bytes=readFileSync(leaf.path);if(bytes.length===leaf.bytes&&bytes.length<=P.maximumIndexBytes)full(leaf,memberHashV1(bytes),bytes.length,null,[],[]);else counters.refused++;}catch(error){if(!absent(error))throw error;}continue;}
  if(leaf.kind==="pack"){
   const idx=byPath.get(leaf.path.slice(0,-5)+".idx");if(!idx){counters.refused++;continue;}
   let hash:string;try{const bytes=readFileSync(idx.path);if(bytes.length!==idx.bytes||bytes.length>P.maximumIndexBytes)throw Error("Bounded closure index required");hash=memberHashV1(bytes);}catch(error){if(absent(error)){counters.refused++;continue;}throw error;}
   requests.push({path:leaf.path,stamp:leaf.proof,kind:leaf.kind,bytes:leaf.bytes,index:{path:idx.path,stamp:idx.proof,bytes:idx.bytes,hash}});continue;
  }
  requests.push({path:leaf.path,stamp:leaf.proof,kind:leaf.kind,bytes:leaf.bytes});
 }
 const take=(request:Request,answer:MemberReadAnswerV1)=>{const leaf=byPath.get(request.path)!,value=answer.rows[0]!;full(leaf,value.hash,value.expandedBytes,value.publicationGraph,answer.references,answer.links.map(link=>link.missing==="block"?{p:relative(directory,link.path).split(sep).join("/"),m:"block" as const}:{p:relative(directory,link.path).split(sep).join("/")}));};
 for(let from=0;from<requests.length;from+=READ_CHUNK_V1){
  if(control.cancelled())break;
  const chunk=requests.slice(from,from+READ_CHUNK_V1),done=new Set<string>();
  try{await ports.read(chunk,(request,answer)=>{take(request,answer);done.add(request.path);});}
  catch{for(const request of chunk){if(done.has(request.path))continue;if(control.cancelled())break;try{await ports.read([request],take);}catch{counters.refused++;}}}
 }
 return rows;
}

/** 📇️ Indexes the leaves no member index vouches for yet and upgrades light rows to full ones, group by group. Leaves are immutable once published, so they are read without any lease (a leaf replaced meanwhile fails the child's stamp check and is simply indexed again next pass, and a row for an old stamp can never vouch for new bytes); the group's non-blocking writer lease is held only for the appends and for compacting an index that carries more dead rows than live leaves, so a writer waits for milliseconds, never for a read. A group whose lease is taken is counted busy and left for the next pass; a leaf the closure read child refuses is counted, never fatal. */
export async function indexBuildGraphMembersV1(root:string,ports:MemberIndexPortsV1,control:MemberScanControlV1):Promise<MemberIndexReportV1>{
 const groups=listBuildGraphMemberGroupsV1(root),counters={refused:0};let skipped=0,busy=0,indexed=0,compacted=0,cancelled=false,seen=0;
 for(let at=0;at<groups.length;at++){
  if(control.cancelled()){cancelled=true;break;}
  const group=groups[at]!,[kind,identity]=group.split("/") as [string,string],target=join(root,...directoryOf(group).split("/"),MEMBER_INDEX_FILE_V1);
  await control.progress(group,at,groups.length);await turn();
  seen++;
  const leaves=leavesOf(root,group),index=parseMemberIndexV1(indexText(root,group),group),proven=proveGroupV1(index,leaves.map(leaf=>({name:leaf.name,kind:leaf.kind,bytes:leaf.bytes,stamp:leaf.stamp}))),pending=leaves.filter((_,position)=>proven.members[position]!.state!=="full");
  if(!pending.length&&!index.rejected&&!index.tornTail&&index.count<=2*leaves.length+M.compactionSlackRows){skipped++;continue;}
  const rows=pending.length?await rowsFor(root,group,pending,leaves,ports,control,counters):[];
  if(control.cancelled()){cancelled=true;break;}
  const lease=await ports.lease(kind==="contexts"?"dataset":kind==="selections"?"selection":"canonical",identity??"");if(!lease){busy++;continue;}
  try{
   try{unlinkSync(target+".tmp");}catch(error){if(!absent(error))throw error;}
   for(let from=0;from<rows.length;){let to=from,size=0;while(to<rows.length&&size+encodeMemberRowV1(rows[to]!).length+1<M.maximumAppendBytes/2){size+=encodeMemberRowV1(rows[to]!).length+1;to++;}if(to===from)to++;appendMemberRowsV1(fs,target,group,rows.slice(from,to));from=to;}
   indexed+=rows.length;
   const after=parseMemberIndexV1(indexText(root,group),group),current=leavesOf(root,group);
   if(after.valid&&(after.count>2*current.length+M.compactionSlackRows||after.rejected||after.tornTail)){
    const live=new Map(current.map(leaf=>[leaf.name,leaf.stamp])),temporary=target+".tmp";
    writeFileSync(temporary,compactMemberIndexV1(after,group,live));renameSync(temporary,target);compacted++;
   }
  }finally{lease.release();}
 }
 return Object.freeze({groups:seen,skipped,busy,indexed,refused:counters.refused,compacted,cancelled});
}

/** 🪢️ The writer lease port of a sweep: a group is first tried without waiting; a group whose blocks already exceed the inventory limit (`urgent`) then queues for the writer with a bounded wait, because a store whose inventory is exhausted refuses every command and the shared tickets of an overlapping cohort would otherwise starve a no-wait attempt for ever. The queued exclusive ticket holds nothing while it waits and makes later cohort arrivals wait behind it; a wait that times out is a busy group, as before. */
export function createSweepLeasePortV1(input:Readonly<{acquire(kind:"selection"|"dataset"|"canonical",identity:string):Promise<Readonly<{release():void}>|undefined>;acquireQueued(kind:"selection"|"dataset"|"canonical",identity:string,timeoutMs:number):Promise<Readonly<{release():void}>|undefined>;waitMilliseconds:number}>):MemberIndexPortsV1["lease"]{
 return async(kind,identity,urgent=false)=>(await input.acquire(kind,identity))??(urgent?await input.acquireQueued(kind,identity,input.waitMilliseconds):undefined);
}

export type SupersedeReportV1=Readonly<{groups:number;busy:number;skipped:number;headers:number;removed:number;bytes:number;cancelled:boolean}>;
const recordDirectoriesV1:readonly Readonly<{name:string;pattern?:RegExp}>[]=Object.freeze([{name:"np"},{name:"ip"},{name:"rr"},{name:"inference",pattern:/^(?:emoji-project-json|test-cases)\.json$/u}]);

/** 🧹️ Removes, group by group, the observation blocks that no record header of the group names any more, each group under its non-blocking writer lease (a group a builder writes is skipped as busy): every re-inference writes new blocks and the superseded ones are never read again, so without this the finite inventory of the graph store fills up and no graph command can run. Reports progress per record directory and stops at the next checkpoint when cancelled; `dryRun` only counts. */
export async function supersedeBuildGraphObservationBlocksV1(root:string,ports:Pick<MemberIndexPortsV1,"lease">,control:MemberScanControlV1,{dryRun=false,minimumBlocks=0,blocksPerHeader=0,graceMilliseconds,now}:Readonly<{dryRun?:boolean;minimumBlocks?:number;blocksPerHeader?:number;graceMilliseconds?:number;now?:number}>={}):Promise<SupersedeReportV1>{
 const{sweepSupersededObservationBlocksV1}=await import("../../../../../../../../../🏗️build/👁️observation/🟨️.mjs"),groups=listBuildGraphMemberGroupsV1(root).filter(group=>group.startsWith("selections/")||group.startsWith("contexts/"));
 let busy=0,skipped=0,headers=0,removed=0,bytes=0,cancelled=false,seen=0;
 for(let at=0;at<groups.length;at++){
  if(control.cancelled()){cancelled=true;break;}
  const group=groups[at]!,[kind,identity]=group.split("/") as [string,string],blocks=recordDirectoriesV1.reduce((sum,directory)=>{try{return sum+readdirSync(join(root,"inference",...group.split("/"),directory.name,"ob")).length;}catch{return sum;}},0),lease=await ports.lease(kind==="contexts"?"dataset":"selection",identity,minimumBlocks>0&&blocks>=minimumBlocks);
  if(!lease){busy++;continue;}
  seen++;
  try{
   for(const directory of recordDirectoriesV1){
    if(control.cancelled()){cancelled=true;break;}
    const path=join(root,"inference",...group.split("/"),directory.name);if(!present(path))continue;
    const report=await sweepSupersededObservationBlocksV1(path,{...(directory.pattern?{headerPattern:directory.pattern}:{}),dryRun,minimumBlocks,blocksPerHeader,...(graceMilliseconds!==undefined?{graceMilliseconds}:{}),...(now!==undefined?{now}:{}),progress:row=>control.progress(group+"/"+directory.name+":"+row.phase,row.done,row.total)});
    if(report.skipped)skipped++;headers+=report.headers;removed+=report.removed;bytes+=report.bytes;
   }
  }finally{lease.release();}
  if(cancelled)break;
 }
 return Object.freeze({groups:seen,busy,skipped,headers,removed,bytes,cancelled});
}

/** 📃️ What one closure sweep did: `unvouched` counts passes that refused to remove anything because a leaf's references could not be proven, `busy` the attempts that found a group writer taken. */
export type ClosureSupersedeReportV1=Readonly<{groups:number;unvouched:number;busy:number;candidates:number;removed:number;bytes:number;cancelled:boolean}>;
const closurePrefixV1:Readonly<Record<string,string>>=Object.freeze({"build-input-closures":"source:","build-lock-closures":"lock:"});

/** 💀️ Whether a read refusal says the leaf can never be decoded, so no reader can follow it to a closure: its observation blocks are gone, or its payload is no complete compressed stream (the child's inflation check failed with a truncated or corrupt stream error, which stays so; a memory or limit error says nothing about the bytes; a leaf is only judged once it is older than the grace). */
const deadLeafV1=(error:unknown)=>{const worker=(error as {closureReadFailure?:{worker?:{failure?:{message?:unknown;code?:unknown};closurePayloadRefusal?:{check?:unknown}|null}}}|undefined)?.closureReadFailure?.worker,code=worker?.failure?.code;return worker?.failure?.message==="Authenticated observation closure block required"||((code==="Z_BUF_ERROR"||code==="Z_DATA_ERROR")&&worker?.closurePayloadRefusal?.check==="inflation");};
/** 🧠️ What a sweep learned about a leaf by its path and physical stamp: the references it names, or null for a dead leaf; a leaf is read once per sweep, not once per pass. */
/** 🔑️ One writer lease of a sweep: the namespace kind and its identity; groups that share a namespace (the shared one and the generations) share one lease, so it is taken once. */
type LeaseKeyV1=readonly ["selection"|"dataset"|"canonical",string];
type LeafKnowledgeV1=Map<string,readonly string[]|null>;

/** 🪢️ Unions the closure references of every group: indexed leaves vouch for themselves, an observation block is never read on its own (the closure read child authenticates a block only through the record header that names it, and that header's references already include the block's),  the leaves without a full row are read once more by the closure read child (a dead leaf older than the grace names nothing); more than `maximumPending` of them, or one the child refuses for another reason, means the references are unproven. */
async function referencedClosuresV1(root:string,groups:readonly string[],ports:Pick<MemberIndexPortsV1,"read">,control:MemberScanControlV1,maximumPending:number,known:LeafKnowledgeV1,settledBefore:number):Promise<Set<string>|undefined>{
 const referenced=new Set<string>();
 for(const group of groups){
  if(control.cancelled())return undefined;
  const leaves=leavesOf(root,group),proven=proveGroupV1(parseMemberIndexV1(indexText(root,group),group),leaves.map(leaf=>({name:leaf.name,kind:leaf.kind,bytes:leaf.bytes,stamp:leaf.stamp})));
  for(const reference of proven.references)referenced.add(reference);
  const pending=leaves.filter((_,at)=>proven.members[at]!.state!=="full"),unread:Leaf[]=[];
  if(pending.length>maximumPending)return undefined;
  for(const leaf of pending){
   const cached=known.get(leaf.path+"\0"+leaf.proof);
   if(cached!==undefined){for(const reference of cached??[])referenced.add(reference);continue;}
   if(leaf.kind==="observation"){known.set(leaf.path+"\0"+leaf.proof,null);continue;}
   unread.push(leaf);
  }
  if(unread.length){
   const dead=new Set<string>(),read:MemberIndexPortsV1["read"]=async(requests,accept)=>{try{await ports.read(requests,accept);}catch(error){if(requests.length===1&&deadLeafV1(error)&&Number(BigInt(unread.find(leaf=>leaf.path===requests[0]!.path)!.stamp.split(":")[2]!)/1000000n)<=settledBefore){dead.add(requests[0]!.path);return;}throw error;}},counters={refused:0},rows=await rowsFor(root,group,unread,leaves,{lease:async()=>undefined,read},control,counters);
   if(counters.refused||rows.length+dead.size!==unread.length)return undefined;
   for(const leaf of unread)if(dead.has(leaf.path))known.set(leaf.path+"\0"+leaf.proof,null);
   for(const row of rows){const leaf=unread.find(candidate=>candidate.name===row.n)!;known.set(leaf.path+"\0"+leaf.proof,row.r??[]);for(const reference of row.r??[])referenced.add(reference);}
  }
 }
 return referenced;
}

/** 🏗️ Removes, under the shared hold of the graph image, the closure inventories no leaf of any group names: a leaf's references are proven by its index row (or read once more), an inventory younger than the grace may belong to a leaf still being written, and each batch is removed while every group's non-blocking writer lease is held and the references are proven again, so no writer can start naming an inventory that is being removed. Nothing is removed while any reference is unproven or a writer is busy. */
export async function supersedeBuildGraphClosuresV1(root:string,ports:Pick<MemberIndexPortsV1,"lease"|"read">&Readonly<{queue?(kind:"selection"|"dataset"|"canonical",identity:string):Promise<Readonly<{release():void}>|undefined>}>,control:MemberScanControlV1,{dryRun=false,graceMilliseconds=3600000,now=Date.now(),maximumPending=65536,batch=256,attempts=8}:Readonly<{dryRun?:boolean;graceMilliseconds?:number;now?:number;maximumPending?:number;batch?:number;attempts?:number}>={}):Promise<ClosureSupersedeReportV1>{
 const groups=listBuildGraphMemberGroupsV1(root),known:LeafKnowledgeV1=new Map(),keys=[...new Map(groups.map(group=>{const[kind,identity]=group.split("/") as [string,string],key:LeaseKeyV1=[kind==="contexts"?"dataset":kind==="selections"?"selection":"canonical",identity??""];return[key.join("\0"),key] as const;})).values()];let unvouched=0,busy=0,removed=0,bytes=0,cancelled=false,candidates=0;
 const done=()=>Object.freeze({groups:groups.length,unvouched,busy,candidates,removed,bytes,cancelled});
 const collect=(referenced:ReadonlySet<string>)=>{
  const rows:{path:string;key:string;allocated:number;proof:string}[]=[];
  for(const[namespace,prefix]of Object.entries(closurePrefixV1)){
   let names:string[]=[];try{names=readdirSync(join(root,namespace));}catch(error){if(!absent(error))throw error;}
   for(const name of names){
    if(!/^[a-f0-9]{64}\.json$/u.test(name)||referenced.has(prefix+name.slice(0,64)))continue;
    const path=join(root,namespace,name);let row:BigIntStats;try{row=lstatSync(path,{bigint:true});}catch(error){if(absent(error))continue;throw error;}
    if(!row.isFile()||row.nlink!==1n||now-Number(row.mtimeNs/1000000n)<graceMilliseconds)continue;
    rows.push({path,key:prefix+name.slice(0,64),allocated:Number(row.blocks)*512,proof:stamp7(row)});
   }
  }
  return rows;
 };
 await control.progress("closures:references",0,groups.length);
 const first=await referencedClosuresV1(root,groups,ports,control,maximumPending,known,now-graceMilliseconds);
 if(!first){if(control.cancelled())cancelled=true;else unvouched++;return done();}
 const list=collect(first);candidates=list.length;
 if(dryRun){removed=list.length;bytes=list.reduce((sum,row)=>sum+row.allocated,0);return done();}
 for(let from=0;from<list.length;from+=batch){
  if(control.cancelled()){cancelled=true;break;}
  await control.progress("closures:remove",from,list.length);await turn();
  let held:Array<{release():void}>|undefined;
  for(let attempt=0;attempt<attempts&&!held;attempt++){
   if(control.cancelled()){cancelled=true;break;}
   const taken:Array<{release():void}>=[];let blocked:LeaseKeyV1|undefined;
   for(const key of keys){const lease=await ports.lease(key[0],key[1]);if(!lease){blocked=key;break;}taken.push(lease);}
   if(blocked===undefined){held=taken;break;}
   for(const lease of taken)lease.release();busy++;
   if(ports.queue){
    const first=await ports.queue(blocked[0],blocked[1]);
    if(first){
     const rest:Array<{release():void}>=[first];let all=true;
     for(const key of keys){if(key===blocked)continue;const lease=await ports.lease(key[0],key[1]);if(!lease){all=false;break;}rest.push(lease);}
     if(all){held=rest;break;}
     for(const lease of rest)lease.release();
    }
   }else if(attempt+1<attempts)await new Promise<void>(resolve=>setTimeout(resolve,500));
  }
  if(!held)break;
  try{
   const referenced=await referencedClosuresV1(root,groups,ports,control,maximumPending,known,now-graceMilliseconds);
   if(!referenced){if(control.cancelled())cancelled=true;else unvouched++;break;}
   for(const row of list.slice(from,from+batch)){
    if(referenced.has(row.key))continue;
    let current:BigIntStats;try{current=lstatSync(row.path,{bigint:true});}catch(error){if(absent(error))continue;throw error;}
    if(stamp7(current)!==row.proof)continue;
    try{unlinkSync(row.path);}catch(error){if(absent(error))continue;throw error;}
    removed++;bytes+=row.allocated;
   }
  }finally{for(const lease of held)lease.release();}
 }
 return done();
}

/** 📉️ What one least-recently-used pass over the optional graph cache did: its bytes before and after (the logical size of every optional leaf, the measure of the optional ledger) against the ledger's fixed limit and the pass's target, and how many groups were retired, busy (their writer is taken), in use (an optional leaf written within the protected idle time) or excluded (the caller's own namespace). */
export type OptionalLruReportV1=Readonly<{usedBytes:number;maximumBytes:number;targetBytes:number;remainingBytes:number;groups:number;retiredGroups:number;retiredFiles:number;retiredBytes:number;busy:number;protected:number;excluded:number;cancelled:boolean}>;
type OptionalLeafV1=Readonly<{name:string;path:string;bytes:number;rank:number}>;
const optionalLeafPatternV1=new RegExp(graphStorageSchema.definitions.OptionalQuota.definitions.OptionalPath.const,"u");
const optionalDirectoryOfV1=(root:string,group:string)=>join(root,"inference",...(group==="canonical"?[]:group.split("/")));

/** 🗃️ Lists the optional leaves of one group (index files, receipts and publications never match the optional pattern), records before the packs they index and observation blocks last, so a pass cut short never leaves a record naming a removed block. */
function optionalLeavesOfV1(root:string,group:string):{leaves:OptionalLeafV1[];newest:number}{
 const directory=optionalDirectoryOfV1(root,group),prefix=group==="canonical"?"":group+"/",leaves:OptionalLeafV1[]=[],pending=[""];let newest=0;
 while(pending.length){
  const at=pending.pop()!;let entries;try{entries=readdirSync(join(directory,at),{withFileTypes:true});}catch(error){if(absent(error))continue;throw error;}
  for(const entry of entries){
   const name=at?at+"/"+entry.name:entry.name;
   if(entry.isDirectory()){if(group==="canonical"&&!at&&(entry.name==="selections"||entry.name==="contexts"))continue;pending.push(name);continue;}
   if(!entry.isFile()||!optionalLeafPatternV1.test(prefix+name))continue;
   const path=join(directory,name);let row;try{row=lstatSync(path);}catch(error){if(absent(error))continue;throw error;}
   if(!row.isFile()||row.nlink!==1)continue;newest=Math.max(newest,row.mtimeMs);
   leaves.push({name,path,bytes:row.size,rank:/(?:^|\/)ob\//u.test(name)?2:/\.pack(?:\.[a-f0-9]{8}\.tmp)?$/u.test(name)?1:0});
  }
 }
 return{leaves:leaves.sort((left,right)=>left.rank-right.rank||(left.name<right.name?-1:left.name>right.name?1:0)),newest};
}

/** ♻️ Retires optional graph cache groups least recently written first until the optional bytes are at most `targetBytes` (the ledger's low-water mark unless given), so an insertion into a full cache stores its records instead of being declined. Optional leaves are recomputable accelerators that no generation, pin or mandatory publication names; a group is retired only under its non-blocking writer lease (every reader and writer of a namespace holds it, so a busy group is skipped, never waited for), never when it is the caller's own (`exclude`) and never while an optional leaf of it is younger than the protected idle time. Its member index is compacted to the leaves that remain. Reports progress per group and stops at the next group when cancelled; `dryRun` only plans. */
export async function retireBuildGraphOptionalGroupsLruV1(root:string,ports:Pick<MemberIndexPortsV1,"lease">,control:MemberScanControlV1,{targetBytes=graphStorageSchema.definitions.OptionalQuota.definitions.DefaultPolicy.const.lowWaterBytes,dryRun=false,exclude=[],protectedMilliseconds=M.groupProtectedIdleMilliseconds,now=Date.now()}:Readonly<{targetBytes?:number;dryRun?:boolean;exclude?:readonly string[];protectedMilliseconds?:number;now?:number}>):Promise<OptionalLruReportV1>{
 if(!Number.isSafeInteger(targetBytes)||targetBytes<0)throw Error("exact-optional-graph-target-bytes-required");
 const groups=["canonical",...listBuildGraphMemberGroupsV1(root).filter(group=>group.startsWith("selections/")||group.startsWith("contexts/"))].map(group=>{const{leaves,newest}=optionalLeavesOfV1(root,group);return{group,newest,bytes:leaves.reduce((sum,leaf)=>sum+leaf.bytes,0)};}).filter(row=>row.bytes>0).sort((left,right)=>left.newest-right.newest||(left.group<right.group?-1:left.group>right.group?1:0));
 const usedBytes=groups.reduce((sum,row)=>sum+row.bytes,0);let remaining=usedBytes,retiredGroups=0,retiredFiles=0,retiredBytes=0,busy=0,protectedGroups=0,excluded=0,cancelled=false;
 for(let at=0;at<groups.length&&remaining>targetBytes;at++){
  if(control.cancelled()){cancelled=true;break;}
  const row=groups[at]!;await control.progress(row.group,at,groups.length);await turn();
  if(exclude.includes(row.group)){excluded++;continue;}
  if(memberGroupProtectedV1(row.newest,now,protectedMilliseconds)){protectedGroups++;continue;}
  if(dryRun){retiredGroups++;retiredBytes+=row.bytes;remaining-=row.bytes;continue;}
  const[kind,identity]=row.group.split("/") as [string,string|undefined],lease=await ports.lease(kind==="contexts"?"dataset":kind==="selections"?"selection":"canonical",identity??"");
  if(!lease){busy++;continue;}
  try{
   const{leaves,newest}=optionalLeavesOfV1(root,row.group);
   if(memberGroupProtectedV1(newest,Date.now(),protectedMilliseconds)){protectedGroups++;continue;}
   let bytes=0;for(const leaf of leaves){try{unlinkSync(leaf.path);}catch(error){if(absent(error))continue;throw error;}retiredFiles++;bytes+=leaf.bytes;}
   const target=join(root,...directoryOf(row.group).split("/"),MEMBER_INDEX_FILE_V1),index=parseMemberIndexV1(indexText(root,row.group),row.group);
   if(index.valid&&index.count){const live=new Map(leavesOf(root,row.group).map(leaf=>[leaf.name,leaf.stamp])),temporary=target+".tmp";writeFileSync(temporary,compactMemberIndexV1(index,row.group,live));renameSync(temporary,target);}
   retiredGroups++;retiredBytes+=bytes;remaining-=row.bytes;
  }finally{lease.release();}
 }
 return Object.freeze({usedBytes,maximumBytes:graphStorageSchema.definitions.OptionalQuota.definitions.DefaultPolicy.const.maximumBytes,targetBytes,remainingBytes:Math.max(0,remaining),groups:groups.length,retiredGroups,retiredFiles,retiredBytes,busy,protected:protectedGroups,excluded,cancelled});
}

/** 🧒️ Binds the closure read child: the captured worker, heap and wire bounds the closure proof uses, one cohort of requests per child under the caller's lease control, each leaf its own request so that its answer is its own. */
export function closureReadChildPortV1(root:string,control:import("../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl):MemberIndexPortsV1["read"]{
 return async(requests,accept)=>{
  await runClosureReadCohortsV1(requests.map(request=>[request]),worker,root,control,async rows=>packet(root,rows),async(text,rows)=>{
   const answer=JSON.parse(text) as MemberReadAnswerV1,request=rows[0]!;
   if(!answer||answer.version!==1||!Array.isArray(answer.rows)||answer.rows.length!==1||answer.rows[0]!.path!==request.path||!Array.isArray(answer.references)||!Array.isArray(answer.links))throw Error("Closed closure read result required");
   accept(request,answer);
  },async()=>{throw Error("Deferred closure read refused");});
 };
}
