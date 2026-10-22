import{closeSync,fstatSync,lstatSync,openSync,readFileSync,readSync,readdirSync,renameSync,unlinkSync,writeFileSync,writeSync,type BigIntStats}from"node:fs";
import{join,relative,sep}from"node:path";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";
import{runClosureReadCohortsV1}from"../../📡️session/🟦️.ts";
import{physicalSourceOriginV1}from"../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import{MEMBER_INDEX_FILE_V1,appendMemberRowsV1,compactMemberIndexV1,encodeMemberRowV1,fullMemberRowV1,memberHashV1,memberGroupOfPathV1,memberKindOfV1,memberStampV1,parseMemberIndexV1,proveGroupV1,readMemberRowV1,type MemberRowV1}from"../🟨️.mjs";
import closures from"../../🧬️schema/🔣️.json"with{type:"json"};
import protocol from"../🧬️schema/🔣️.json"with{type:"json"};

const P=closures.definitions.Protocol.const,M=protocol.definitions.Protocol.const,fs={openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync}as never,hex=/^[a-f0-9]{64}$/u,worker=fileURLToPath(new URL("../../👷️read.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)));

/** 🎚️ What a scan needs from its caller: whether to stop, and where to report progress. */
export type MemberScanControlV1=Readonly<{cancelled():boolean;progress(label:string,done:number,total:number):void|Promise<void>}>;
/** 🧾️ One leaf the closure read child opens on its own, so its answer is that leaf's own hash, references and links. */
export type MemberReadRequestV1=Readonly<{path:string;stamp:string;kind:string;bytes:number;index?:Readonly<{path:string;stamp:string;bytes:number;hash:string}>}>;
/** 📬️ The closure read child's answer for one leaf. */
export type MemberReadAnswerV1=Readonly<{version:1;rows:readonly Readonly<{path:string;hash:string;expandedBytes:number;publicationGraph:string|null}>[];references:readonly string[];links:readonly Readonly<{path:string;missing:"block"|"forbidden"}>[]}>;
/** 🔌️ What indexing needs from the repository: the per-namespace writer lease (never waited for) and the closure read child, which opens each requested leaf as a request of its own and reports every answer to `accept`. */
export type MemberIndexPortsV1=Readonly<{lease(kind:"selection"|"dataset"|"canonical",identity:string):Promise<Readonly<{release():void}>|undefined>;read(requests:readonly MemberReadRequestV1[],accept:(request:MemberReadRequestV1,answer:MemberReadAnswerV1)=>void):Promise<void>}>;
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
