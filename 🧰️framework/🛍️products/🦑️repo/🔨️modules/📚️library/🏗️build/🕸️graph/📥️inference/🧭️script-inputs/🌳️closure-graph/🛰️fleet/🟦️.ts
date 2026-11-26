/** 🛰️ The module closure graph of a machine as ONE fleet object per graph: the chunks of the stored graph (`../`, digest addressed) as blobs and the head as the result of a key derived from the SHA-256 of the head text, declared kind `graph` (the protected class of the cache server, never taken by unit output pressure), in a slot per scope so a newer graph supersedes the older one after the grace. A machine that fetches it verifies its own tree by digests instead of walking it: a node is adopted only when the bytes of its file on this machine have the SHA-256 the node recorded (a crate or sources node when its manifest and every file leaf it names do, a glob when it matches the same names here), adopted nodes are stamped for this machine, and every other node is left out, so the next pass visits exactly those.
 * @see ../🟨️.mjs
 * @see ../../../../../🪪️graph-state/🛰️exchange/🟦️.ts `graphRetentionManifestV1` */
import{createHash}from"node:crypto";
import{mkdir,readFile,rm,writeFile}from"node:fs/promises";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{REAPI_V1,decodeProtobufV1,encodeProtobufV1}from"../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import{graphRetentionManifestV1,type GraphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{CLOSURE_GRAPH_POLICY_V1,createClosureGraphV1,loadClosureGraphV1,saveClosureGraphV1,type ClosureGraphNodeV1,type ClosureGraphV1}from"../🟨️.mjs";
import{globNodesV1,leafContentHoldsV1,pathStampV1,workspacePathV1}from"../🍃️leaves/🟨️.mjs";

const policy=CLOSURE_GRAPH_POLICY_V1.fleet,sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),EXTENSION=".json.br";
type Steps=Readonly<{signal?:AbortSignal;progress?:(step:{done:number;total:number})=>unknown}>;

/** 🗝️ The key of the fleet object of one stored graph: the domain and the SHA-256 of its head text, so equal graphs are one object and a head names its graph by that digest. */
export const closureGraphFleetKeyV1=(head:string):string=>sha(JSON.stringify([policy.domain,"graph",head]));
/** 🪢️ The retention slot of the graphs of one scope (a workspace and platform): a newer graph of the slot supersedes the older ones after the class grace. */
export const closureGraphFleetSlotV1=(scope:string):string=>sha(JSON.stringify([policy.domain,"slot",scope]));

/** ⬆️ Offers the graph stored in `directory` (`head.json` and `c/`) to the fleet: the chunk blobs the cache lacks, the retention manifest, then the result under the key of its head. An object the fleet already holds is not written again; a machine without upload rights or an unreachable cache never throws. Answers whether it is stored, its head digest and the requests made. */
export async function offerClosureGraphV1(input:Readonly<{client:GraphFleetClientV1;directory:string;scope:string}>&Steps):Promise<Readonly<{stored:boolean;head?:string;chunks:number;uploaded:number;requests:number;reason?:string}>>{
 const{client,directory}=input;let requests=0,uploaded=0;
 try{
  if(!client.settings.upload)return{stored:false,chunks:0,uploaded,requests,reason:"no-upload"};
  if(!client.usable())return{stored:false,chunks:0,uploaded,requests,reason:"breaker"};
  const text=await readFile(join(directory,"head.json"),"utf8"),head=sha(text),chunks=((parsed:{chunks:(string|null)[];pool:string|null})=>[...parsed.chunks,parsed.pool])(JSON.parse(text)).filter((digest):digest is string=>digest!==null),key=closureGraphFleetKeyV1(head);
  requests++;const held=await client.getResult(key,input.signal);
  if(held instanceof Uint8Array&&sha(Buffer.from(decodeProtobufV1(REAPI_V1.actionResult,held).stdout_raw as Uint8Array))===head)return{stored:true,head,chunks:chunks.length,uploaded,requests,reason:"current"};
  const files:{path:string;digest:{hash:string;size_bytes:number}}[]=[];
  for(const[at,digest]of chunks.entries()){
   input.signal?.throwIfAborted();
   const bytes=await readFile(join(directory,"c",digest+EXTENSION)),hash=sha(bytes);
   requests++;if(await client.hasBlob(hash)!==true){requests++;const put=await client.putBlob(hash,bytes);if(!put.stored)return{stored:false,head,chunks:chunks.length,uploaded,requests,reason:"blob: "+put.refusal.text.slice(0,100)};uploaded++;}
   files.push({path:"c/"+digest+EXTENSION,digest:{hash,size_bytes:bytes.length}});
   await input.progress?.({done:at+1,total:chunks.length});
  }
  const kept=graphRetentionManifestV1();
  requests++;if(await client.hasBlob(kept.hash)!==true){requests++;const put=await client.putBlob(kept.hash,kept.bytes);if(!put.stored)return{stored:false,head,chunks:chunks.length,uploaded,requests,reason:"blob: "+put.refusal.text.slice(0,100)};}
  requests++;const put=await client.putResult(key,encodeProtobufV1(REAPI_V1.actionResult,{output_files:[...files,{path:policy.retentionManifest,digest:{hash:kept.hash,size_bytes:kept.bytes.length}}],stdout_raw:Buffer.from(text)}),closureGraphFleetSlotV1(input.scope));
  return put.stored?{stored:true,head,chunks:chunks.length,uploaded,requests}:{stored:false,head,chunks:chunks.length,uploaded,requests,reason:"head: "+put.refusal.text.slice(0,100)};
 }catch(error){input.signal?.throwIfAborted();return{stored:false,chunks:0,uploaded,requests,reason:"failed: "+String((error as Error)?.message??error).slice(0,120)};}
}

/** ⬇️ Fetches the graph named by `head` into `directory` (a fresh directory: its `head.json` and `c/`), every blob verified against its hash (each chunk against the digest in its name when it is loaded), the chunks already in `known` (the machine's own `c/`) copied instead of downloaded. `"miss"` when the fleet has no such graph, `"fault"` when it cannot be read whole. */
export async function fetchClosureGraphV1(input:Readonly<{client:GraphFleetClientV1;head:string;directory:string;known?:string}>&Steps):Promise<Readonly<{outcome:"fetched";chunks:number;downloaded:number;requests:number}>|"miss"|"fault">{
 const{client,head,directory}=input;let requests=1,downloaded=0;
 const result=await client.getResult(closureGraphFleetKeyV1(head),input.signal);
 if(result==="miss")return"miss";
 if(!(result instanceof Uint8Array))return"fault";
 try{
  const decoded=decodeProtobufV1(REAPI_V1.actionResult,result),text=Buffer.from(decoded.stdout_raw as Uint8Array).toString("utf8");
  if(sha(text)!==head)return"fault";
  const files=((decoded.output_files??[])as{path:string;digest:{hash:string;size_bytes:number|string}}[]).filter(file=>file.path!==policy.retentionManifest),chunks=(JSON.parse(text).chunks as(string|null)[]).filter((digest):digest is string=>digest!==null);
  if(files.length!==chunks.length||files.some((file,at)=>file.path!=="c/"+chunks[at]+EXTENSION))return"fault";
  await mkdir(join(directory,"c"),{recursive:true});
  for(const[at,file]of files.entries()){
   input.signal?.throwIfAborted();
   const target=join(directory,file.path),local=input.known===undefined?undefined:await readFile(join(input.known,file.path)).catch(()=>undefined);
   let bytes:Uint8Array|undefined=local!==undefined&&sha(local)===file.digest.hash?local:undefined;
   if(bytes===undefined){requests++;bytes=await client.getBlob(file.digest.hash,Number(file.digest.size_bytes),input.signal);if(bytes===undefined||sha(bytes)!==file.digest.hash)return"fault";downloaded++;}
   await writeFile(target,bytes);
   await input.progress?.({done:at+1,total:files.length});
  }
  await writeFile(join(directory,"head.json"),text);
  return{outcome:"fetched",chunks:files.length,downloaded,requests};
 }catch(error){input.signal?.throwIfAborted();return"fault";}
}

const fileSha=(path:string):string|null=>{try{return sha(readFileSync(path));}catch{return null;}};

/** 🪪️ The nodes of a graph another machine made that this machine's tree confirms by digests, re-stamped for this machine: a module, resource, missing or skipped node when the bytes of its file have the recorded SHA-256 (none for a missing file), a file leaf when its bytes do, a glob when it matches the same names and leaves here, a crate or sources node when every leaf it names is confirmed and its manifest has the recorded bytes. Nothing is parsed or walked; a node that is not confirmed is left out (the next pass visits it). `progress` is awaited every 1,024 nodes. */
export async function confirmedClosureGraphNodesV1(graph:ClosureGraphV1,workspaceRoot:string,steps:Steps={}):Promise<Readonly<{nodes:ClosureGraphNodeV1[];confirmed:number;refused:number}>>{
 const verdicts=new Map<string,boolean>(),out:ClosureGraphNodeV1[]=[],records=[...graph.records()];let refused=0;
 const leafHolds=(key:string):boolean=>{let held=verdicts.get(key);if(held===undefined){const record=graph.get(key);held=record!==undefined&&record.kind==="file"&&leafContentHoldsV1(workspaceRoot,record);verdicts.set(key,held);}return held;};
 for(const[at,record]of records.entries()){
  if(at%1024===1023){steps.signal?.throwIfAborted();await steps.progress?.({done:at+1,total:records.length});}
  const extra=(record.extra??{})as Record<string,unknown>,local=record.name.startsWith("{workspaceRoot}/")?workspacePathV1(workspaceRoot,record.name):undefined;
  let next:ClosureGraphNodeV1|undefined;
  if(record.kind==="file"){if(leafHolds(record.key)&&local!==undefined)next={...record,extra:{...extra,stamp:pathStampV1(local)??null}};}
  else if(record.kind==="glob"){const[,base,pattern]=JSON.parse(record.key) as[string,string,string],again=globNodesV1(workspaceRoot,base,pattern)[0]!;if(again.digest===record.digest&&JSON.stringify(again.edges)===JSON.stringify(record.edges)&&record.edges.every(leafHolds))next=again;}
  else if(record.kind==="crate"||record.kind==="sources"){
   const manifest=record.kind==="crate"&&local!==undefined?fileSha(join(local,"Cargo.toml")):undefined;
   if(record.edges.filter(edge=>JSON.parse(edge)[0]==="file").every(leafHolds)&&(record.kind==="sources"||manifest!==null&&manifest===extra.sha256)){const names=((extra.stamps??[])as[string,string|null][]).map(([name])=>name);next={...record,extra:{...extra,stamps:names.map(name=>[name,pathStampV1(workspacePathV1(workspaceRoot,name))??null])}};}
  }
  else if(local!==undefined&&typeof extra.sha256==="string"||record.kind==="missing"){const now=local===undefined?null:fileSha(local);if(now===(record.kind==="missing"?null:extra.sha256)){const stamp=local===undefined?undefined:pathStampV1(local);if(stamp!==undefined)next={...record,extra:{...extra,witness:null,adopted:stamp}};}}
  if(next===undefined)refused++;else out.push(next);
 }
 return{nodes:out,confirmed:out.length,refused};
}

/** 🛰️ Adopts the graph another machine offered for this tree: fetched into a scratch directory beside the machine's graph (`<directory>/f/<head>`), confirmed node by node ({@link confirmedClosureGraphNodesV1}) and put into `graph` (the machine's own graph object) without replacing a node this machine already holds, the scratch directory removed. Answers what was fetched, confirmed and refused. */
export async function adoptClosureGraphV1(input:Readonly<{client:GraphFleetClientV1;head:string;directory:string;workspaceRoot:string;graph:ClosureGraphV1}>&Steps):Promise<Readonly<{outcome:"adopted"|"miss"|"fault";confirmed:number;refused:number;downloaded:number;requests:number}>>{
 const scratch=join(input.directory,"f",input.head);
 await rm(scratch,{recursive:true,force:true});
 try{
  const fetched=await fetchClosureGraphV1({client:input.client,head:input.head,directory:scratch,known:input.directory,signal:input.signal,progress:input.progress});
  if(typeof fetched==="string")return{outcome:fetched,confirmed:0,refused:0,downloaded:0,requests:1};
  const graph=(await loadClosureGraphV1(scratch,{signal:input.signal}))!,{nodes,confirmed,refused}=await confirmedClosureGraphNodesV1(graph,input.workspaceRoot,input);
  input.graph.update(nodes.filter(node=>!input.graph.has(node.key)));
  return{outcome:"adopted",confirmed,refused,downloaded:fetched.downloaded,requests:fetched.requests};
 }finally{await rm(scratch,{recursive:true,force:true});}
}

/** 🌳️ The graph exchange of one checkout for a selection read (`../../../../📥️load/🤝️selected-exchange`): `offer` saves this process's changed graphs into the machine's graph directory and offers that graph (answers its head digest, undefined when it is not stored), `adopt` takes the graph a head names into the machine's graph by digests and saves it, so every later process of the machine starts from it. */
export function machineClosureGraphExchangeV1(input:Readonly<{client:GraphFleetClientV1;workspaceRoot:string;scope:string}>):Readonly<{offer():Promise<string|undefined>;adopt(head:string):Promise<Readonly<{outcome:"adopted"|"miss"|"fault";confirmed:number;refused:number;downloaded:number;requests:number}>>}>{
 return{
  async offer(){const scripts=await import("../../🟨️.mjs");await scripts.flushClosureGraphsV1();const offered=await offerClosureGraphV1({client:input.client,directory:scripts.closureGraphDirectoryV1(input.workspaceRoot),scope:input.scope});return offered.stored?offered.head:undefined;},
  async adopt(head){
   const scripts=await import("../../🟨️.mjs"),directory=scripts.closureGraphDirectoryV1(input.workspaceRoot),graph=await loadClosureGraphV1(directory).catch(()=>undefined)??createClosureGraphV1();
   if((graph as{head?:string}).head===head)return{outcome:"adopted",confirmed:0,refused:0,downloaded:0,requests:0};
   const adopted=await adoptClosureGraphV1({client:input.client,head,directory,workspaceRoot:input.workspaceRoot,graph});
   if(adopted.confirmed>0)await saveClosureGraphV1(graph,directory);
   return adopted;
  },
 };
}
