/** 🏃️ A synthetic multi-project workspace with its own tiny inference (nodes, import edges, an environment of owner roots and package names), the ports a generation store needs and a child entry point so several operating system processes can share one store. The same model is implemented independently in `🐍️oracle.py`. */
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {lstat,mkdir,readFile,readdir,rm,writeFile,utimes} from "node:fs/promises";
import {basename,dirname,join,posix} from "node:path";
import {renderGenerationMessageV1} from "../../🔌️load/🟨️.mjs";
import {createGraphPhaseClockV1} from "../../../⏱️phases/🟨️.mjs";
import {canonicalJsonV1,createFileLeasePortV1,createFrontierV1,createGenerationStoreV1,diffFrontiersV1,type FrontierV1,type GenerationComputeResultV1,type GenerationComputeSpecV1,type GenerationMembershipV1,type GenerationRowV1,type GenerationStoreV1} from "../../🟨️.mjs";

export const IDENTITY=createHash("sha256").update("synthetic-identity-v1").digest("hex");
export const IMPLEMENTATION=createHash("sha256").update("synthetic-implementation-v1").digest("hex");
const sha=(text:string|Uint8Array)=>createHash("sha256").update(text).digest("hex");

export type Op={op:"write";path:string;content:string}|{op:"delete";path:string}|{op:"rename";from:string;to:string};

export async function applyOps(dir:string,ops:readonly Op[]):Promise<void>{
 for(const op of ops){
  if(op.op==="write"){await mkdir(dirname(join(dir,op.path)),{recursive:true});await writeFile(join(dir,op.path),op.content);}
  else if(op.op==="delete")await rm(join(dir,op.path),{force:true,recursive:true});
  else{const{rename}=await import("node:fs/promises");await mkdir(dirname(join(dir,op.to)),{recursive:true});await rename(join(dir,op.from),join(dir,op.to));}
 }
}

async function walk(dir:string,relative:string,files:string[],dirs:string[]):Promise<void>{
 let entries;try{entries=await readdir(join(dir,relative),{withFileTypes:true});}catch{return;}
 dirs.push(relative);
 for(const entry of entries.sort((a,b)=>a.name<b.name?-1:1)){
  const path=relative?relative+"/"+entry.name:entry.name;
  if(entry.isDirectory())await walk(dir,path,files,dirs);else if(entry.isFile())files.push(path);
 }
}

type Owner={name:string;root:string;config:string;package?:string;uses:string[];broken?:boolean};

async function readOwners(dir:string):Promise<{owners:Owner[];rows:GenerationRowV1[]}>{
 const files:string[]=[],dirs:string[]=[];await walk(dir,"pkg",files,dirs);
 const owners:Owner[]=[],rows:GenerationRowV1[]=[];
 for(const file of files.filter(file=>basename(file)==="project.json")){
  const text=await readFile(join(dir,file),"utf8"),json=JSON.parse(text);rows.push({kind:"field",path:file,fact:{name:json.name,package:json.package??null}});
  owners.push({name:json.name,root:dirname(file),config:file,...(json.package?{package:json.package}:{}),...(json.broken?{broken:true}:{}),uses:json.uses??[]});
 }
 return{owners:owners.sort((a,b)=>a.name<b.name?-1:1),rows};
}

const fileRow=(path:string,bytes:Uint8Array|undefined):GenerationRowV1=>({kind:"file",path,fact:bytes?{sha256:sha(bytes),size:bytes.length}:{error:"ENOENT"}});

export function environmentDigest(owners:readonly Owner[]):string{
 return sha(canonicalJsonV1({owners:owners.map(owner=>[owner.name,owner.root]).sort((a,b)=>a[0]<b[0]?-1:1),packages:owners.filter(owner=>owner.package).map(owner=>[owner.package,owner.name]).sort((a,b)=>a[0]!<b[0]!?-1:1)}));
}

async function inferOwner(dir:string,owner:Owner,owners:readonly Owner[]):Promise<{name:string;root:string;kind:"project"|"unavailable";node:unknown;edges:unknown[];targets:string[];config:string;rows:GenerationRowV1[]}>{
 const rows:GenerationRowV1[]=[],config=await readFile(join(dir,owner.config));rows.push(fileRow(owner.config,config));
 if(owner.broken)return{name:owner.name,root:owner.root,kind:"unavailable",node:{root:owner.root,project:owner.name,source:owner.config,sha256:sha(config),reason:"broken"},edges:[],targets:[],config:owner.config,rows};
 const sourceDir=owner.root+"/src";let names:string[]|null=null;
 try{names=(await readdir(join(dir,sourceDir))).sort();}catch{}
 rows.push({kind:"readdir",path:sourceDir,fact:names});
 const sources:{path:string;sha256:string}[]=[],edges=new Map<string,unknown>(),byPackage=new Map(owners.filter(row=>row.package).map(row=>[row.package!,row.name])),byRoot=[...owners].sort((a,b)=>b.root.length-a.root.length);
 for(const name of(names??[]).filter(name=>name.endsWith(".ts"))){
  const path=sourceDir+"/"+name,bytes=await readFile(join(dir,path));rows.push(fileRow(path,bytes));sources.push({path,sha256:sha(bytes)});
  for(const line of bytes.toString("utf8").split("\n")){
   const match=/^import "(.*)";$/u.exec(line);if(!match)continue;const spec=match[1]!;let target:string|undefined;
   if(spec.startsWith("@syn/"))target=byPackage.get(spec);
   else if(spec.startsWith(".")){const resolved=posix.normalize(posix.join(posix.dirname(path),spec));target=byRoot.find(row=>resolved.startsWith(row.root+"/"))?.name;}
   if(target&&target!==owner.name){const edge={source:owner.name,sourceFile:path,target,type:"static"};edges.set(canonicalJsonV1(edge),edge);}
  }
 }
 const uses:{path:string;sha256:string|null}[]=[];
 for(const path of owner.uses){let bytes:Buffer|undefined;try{bytes=await readFile(join(dir,path));}catch{}rows.push(fileRow(path,bytes));uses.push({path,sha256:bytes?sha(bytes):null});}
 return{name:owner.name,root:owner.root,kind:"project",node:{name:owner.name,root:owner.root,package:owner.package??null,sources,uses},edges:[...edges.values()].sort((a,b)=>canonicalJsonV1(a)<canonicalJsonV1(b)?-1:1),targets:["build"],config:owner.config,rows};
}

/** 🧪️ The compute port of the synthetic workspace: refreshes exactly the owners the store names and, when the environment moved, the edges of every other owner. */
export function createCompute(dir:string,log:{full:string[];edges:string[];cold:boolean;envStale:boolean;calls:number}[],ownerDelayMs=0){
 return async(spec:GenerationComputeSpecV1,signal?:AbortSignal):Promise<GenerationComputeResultV1>=>{
  signal?.throwIfAborted();
  const{owners,rows:envRows}=await readOwners(dir),envDigest=environmentDigest(owners);
  const roots=new Set([...spec.owners.map(name=>spec.stubs[name]!.root),...spec.discover.map(path=>dirname(path))]);
  const full=spec.cold?owners:owners.filter(owner=>roots.has(owner.root)),fullNames=new Set(full.map(owner=>owner.name)),results:GenerationComputeResultV1["owners"]=[];
  const entry={full:full.map(owner=>owner.name).sort(),edges:[] as string[],cold:spec.cold,envStale:spec.envStale,calls:1};
  for(const owner of full){signal?.throwIfAborted();if(ownerDelayMs)await new Promise(resolve=>setTimeout(resolve,ownerDelayMs));results.push(await inferOwner(dir,owner,owners));}
  if(!spec.cold&&envDigest!==spec.previousEnv){
   for(const owner of owners)if(!fullNames.has(owner.name)){signal?.throwIfAborted();const inferred=await inferOwner(dir,owner,owners);results.push({name:owner.name,partial:"edges",edges:inferred.edges});entry.edges.push(owner.name);}
  }
  log.push(entry);
  return{owners:results,env:{digest:envDigest,rows:envRows},global:{version:1,externalNodes:{},edges:[],unavailable:[]}};
 };
}

/** 🚪️ The row check of the synthetic workspace: a row holds when its fact is what the tree shows now. */
export function createHolds(dir:string){
 const observe=async(row:GenerationRowV1):Promise<unknown>=>{
  if(row.kind==="file"){try{const bytes=await readFile(join(dir,row.path));return{sha256:sha(bytes),size:bytes.length};}catch{return{error:"ENOENT"};}}
  if(row.kind==="readdir"){try{return(await readdir(join(dir,row.path))).sort();}catch{return null;}}
  if(row.kind==="field"){try{const json=JSON.parse(await readFile(join(dir,row.path),"utf8"));return{name:json.name,package:json.package??null};}catch{return{error:"ENOENT"};}}
  throw Error("Unknown synthetic row kind: "+row.kind);
 };
 return async(rows:readonly GenerationRowV1[],signal?:AbortSignal)=>{for(const row of rows){signal?.throwIfAborted();if(canonicalJsonV1(await observe(row))!==canonicalJsonV1(row.fact))return false;}return true;};
}

/** 🗺️ The membership port: members are the project files and sources, directories carry their entry stamps; a previous frontier turns into a change list. */
export function createMembership(dir:string,counter?:{walks:number}){
 return async(previous:{frontier:FrontierV1}|undefined):Promise<GenerationMembershipV1>=>{
  counter&&counter.walks++;
  const capturedAt=Date.now(),files:string[]=[],dirs:string[]=[],directories:[string,number,number][]=[],members:[string,number][]=[];
  await walk(dir,"pkg",files,dirs);
  for(const path of dirs){const status=await lstat(join(dir,path));directories.push([path,status.mtimeMs,status.ino]);}
  for(const path of files)if(path.endsWith("/project.json")||path.endsWith(".ts")){const status=await lstat(join(dir,path));members.push([path,status.mode]);}
  const frontier=createFrontierV1({directories,members,capturedAt}),digest=sha(canonicalJsonV1(frontier.members));
  const changes=previous?diffFrontiersV1(previous.frontier,frontier).map(change=>({...change,...(change.path.endsWith("/project.json")?{config:true}:{})})):[];
  return{digest,frontier,changes};
 };
}

export interface World{dir:string;store:GenerationStoreV1;log:ReturnType<typeof createLog>;holdsLog:{resource:string;heldMs:number;waitedMs:number;endedAt:number}[];refresh:(options?:{policy?:"fresh"|"current";only?:string[];signal?:AbortSignal;progress?:(event:unknown)=>void;bypassBuilder?:boolean})=>ReturnType<GenerationStoreV1["refresh"]>;refreshClosure:(roots:string[],options?:{policy?:"fresh"|"current";signal?:AbortSignal;progress?:(event:unknown)=>void})=>ReturnType<GenerationStoreV1["refreshClosure"]>;graph:(current:Awaited<ReturnType<GenerationStoreV1["refresh"]>>["current"],names?:string[])=>Promise<unknown>}
const createLog=()=>[] as Parameters<typeof createCompute>[1];

export function createWorld(dir:string,options:{store?:string;identity?:string;lease?:boolean;holdsWrap?:(holds:ReturnType<typeof createHolds>)=>ReturnType<typeof createHolds>;computeWrap?:(compute:ReturnType<typeof createCompute>)=>ReturnType<typeof createCompute>;sharedHoldsLog?:World["holdsLog"];sharedLog?:World["log"];now?:()=>number;ownerDelayMs?:number}={}):World{
 const log=options.sharedLog??createLog(),holdsLog:World["holdsLog"]=options.sharedHoldsLog??[],root=options.store??join(dir,".gen"),holds=options.holdsWrap?options.holdsWrap(createHolds(dir)):createHolds(dir),identity=options.identity??IDENTITY;
 const lease=options.lease===false?undefined:createFileLeasePortV1(join(dir,".leases"),{onHold:row=>holdsLog.push({...row,endedAt:Date.now()})});
 const store=createGenerationStoreV1({root,lease,holds,resources:{facts:"repository-graph-facts:synthetic",builder:"repository-graph-build:synthetic"},...(options.now?{now:options.now}:{})});
 const compute=options.computeWrap?options.computeWrap(createCompute(dir,log,options.ownerDelayMs)):createCompute(dir,log,options.ownerDelayMs),membership=createMembership(dir);
 return{
  dir,store,log,holdsLog,
  refresh:(opts={})=>store.refresh({identity,implementation:IMPLEMENTATION,policy:opts.policy,only:opts.only?new Set(opts.only):undefined,membership,compute,signal:opts.signal,progress:opts.progress as never,...(opts.bypassBuilder?{builder:body=>body()}:{})}),
  refreshClosure:(roots,opts={})=>store.refreshClosure({roots,identity,implementation:IMPLEMENTATION,policy:opts.policy,membership,compute,signal:opts.signal,progress:opts.progress as never}),
  graph:async(current,names)=>{const graph=await store.assemble(current,{names});const{generation,membership:_member,...rest}=graph;return JSON.parse(canonicalJsonV1(rest));},
 };
}

/** 🧠️ An in-memory file port for the laws that need thousands of owners without a disk. */
export function createMemoryFilePort(){
 const files=new Map<string,{bytes:Uint8Array;modifiedMs:number}>();
 return{files,port:{
  async read(path:string){return files.get(path)?.bytes;},
  async write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}){if(options?.ifAbsent&&files.has(path))return false;files.set(path,{bytes:Uint8Array.from(bytes),modifiedMs:Date.now()});return true;},
  async names(directory:string){return[...files.keys()].filter(path=>dirname(path)===directory).map(path=>basename(path));},
  async list(directory:string){return[...files.entries()].filter(([path])=>dirname(path)===directory).map(([path,row])=>({name:basename(path),modifiedMs:row.modifiedMs}));},
  async remove(path:string){files.delete(path);},
 }};
}

/** 📁️ The files of the initial synthetic workspace described by a `{path: content}` table. */
export async function seed(dir:string,files:Record<string,string>):Promise<void>{await applyOps(dir,Object.entries(files).map(([path,content])=>({op:"write" as const,path,content})));}

/** 🧪️ Generates a workspace of `count` owners in a ring of imports: owner i imports owner i+1 by package and owner i+2 by relative path. */
export function ringFiles(count:number,prefix="p"):Record<string,string>{
 const files:Record<string,string>={"shared/common.json":"{\"v\":1}"};
 for(let i=0;i<count;i++){
  const name=`${prefix}${i}`,next=`${prefix}${(i+1)%count}`,after=`${prefix}${(i+2)%count}`;
  files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:i%3===0?["shared/common.json"]:[]});
  files[`pkg/${name}/src/index.ts`]=`import "@syn/${next}";\nimport "../../${after}/src/index";\nexport const v${i}=${i};\n`;
 }
 return files;
}

/** 🧵️ Child process entry: `bun world.ts child <dir> <store> <role> <count>`; a writer edits its own owner `count` times and refreshes after each edit, a reader refreshes `count` times, both print one JSON line per refresh. */
if(import.meta.main&&process.argv[2]==="child"){
 const[,,,dir,storeRoot,role,countText,tag]=process.argv,count=Number(countText),world=createWorld(dir!,{store:storeRoot});
 const lines:unknown[]=[];
 for(let step=0;step<count;step++){
  if(role==="writer"){await writeFile(join(dir!,`pkg/${tag}/src/index.ts`),`import "@syn/p1";\nexport const edit${tag}${step}=${step};\n`);}
  const result=await world.refresh({policy:role==="reader-current"?"current":"fresh"});
  lines.push({generation:result.current.manifest?.generation,served:result.served,computed:result.computed,built:result.built,graph:await world.graph(result.current)});
 }
 await writeFile(join(dir!,`.child-${role}-${tag}.json`),JSON.stringify(lines));
}


/** 🧩️ The partial inference of the synthetic workspace in repository terms (what `discoverPart` returns): owners with their facts, the edges of every processed owner, the environment, and the facts of the whole capture. */
export function createPartDiscover(dir:string,log:{full:string[];edges:string[];cold:boolean}[]=[],ownerDelayMs=0){
 return async(spec:GenerationComputeSpecV1,signal?:AbortSignal)=>{
  signal?.throwIfAborted();
  const{owners,rows:envRows}=await readOwners(dir),envDigest=environmentDigest(owners);
  const roots=new Set([...spec.owners.map(name=>spec.stubs[name]!.root),...spec.discover.map(path=>dirname(path))]);
  const full=spec.cold?owners:owners.filter(owner=>roots.has(owner.root)),fullNames=new Set(full.map(owner=>owner.name));
  const inferred=[];for(const owner of full){signal?.throwIfAborted();if(ownerDelayMs)await new Promise(resolve=>setTimeout(resolve,ownerDelayMs));inferred.push(await inferOwner(dir,owner,owners));}
  const edgeOnly:{name:string}[]=[],extra:Awaited<ReturnType<typeof inferOwner>>[]=[];
  if(!spec.cold&&envDigest!==spec.previousEnv)for(const owner of owners)if(!fullNames.has(owner.name)){signal?.throwIfAborted();extra.push(await inferOwner(dir,owner,owners));edgeOnly.push({name:owner.name});}
  log.push({full:inferred.map(row=>row.name).sort(),edges:edgeOnly.map(row=>row.name).sort(),cold:spec.cold});
  const capture=[...inferred,...extra].flatMap(row=>row.rows);
  return{
   owners:inferred.map(row=>({name:row.name,root:row.root,config:row.config,...(row.kind==="unavailable"?{unavailable:row.node}:{node:row.node}),nodeRows:row.rows})),
   edgeOnly,edges:[...inferred,...extra].flatMap(row=>row.edges) as never[],externalNodes:{},parseUnavailable:[],
   rows:capture,env:{digest:envDigest,rows:envRows},replaceTags:[],
  };
 };
}

/** 🏛️ A repository host over the synthetic workspace: custodies are recorded, a builder custody is exclusive and shared readers never wait for it. */
export function createMockHost(dir:string,options:{signal?:AbortSignal;environment?:Record<string,string>;gate?:()=>Promise<void>;identity?:string;implementationFiles?:string[];ownerDelayMs?:number}={}){
 const events:string[]=[],said:{key:string;text:string}[]=[],published:string[]=[],expanded:string[][]=[],log:{full:string[];edges:string[];cold:boolean}[]=[],holdsLog:{resource:string;heldMs:number;waitedMs:number;endedAt:number}[]=[];
 let building=Promise.resolve();
 const implementationRows:GenerationRowV1[]=(options.implementationFiles??[]).map(path=>{const bytes=readFileSync(join(dir,path));return{kind:"file",path,fact:{sha256:sha(bytes),size:bytes.length}};});
 const lease=createFileLeasePortV1(join(dir,".leases"),{onHold:row=>holdsLog.push({...row,endedAt:Date.now()})}),membership=createMembership(dir),discover=createPartDiscover(dir,log,options.ownerDelayMs),holds=createHolds(dir);
 const host={
  workspaceRoot:dir,namedInputs:{default:[]},registryDigest:sha("registry"),signal:options.signal,verbose:true,
  async identity(){return options.identity??IDENTITY;},
  async implementationRows(){return implementationRows;},
  async withShared<T>(body:(store:unknown,control:unknown)=>Promise<T>){events.push("shared");return body({root:join(dir,".gen")},{published});},
  async withBuilder<T>(body:(store:unknown,control:unknown)=>Promise<T>){
   events.push("builder:wait");const previous=building;let release!:()=>void;building=new Promise<void>(resolve=>{release=resolve;});await previous;events.push("builder:held");
   try{await options.gate?.();return await body({root:join(dir,".gen")},{published});}finally{events.push("builder:released");release();}
  },
  storeOptions:(store:{root:string})=>({root:store.root,lease,holds,resources:{facts:"repository-graph-facts:mock"}}),
  membership,discoverPart:discover,
  async expand(projects:Record<string,unknown>){expanded.push(Object.keys(projects).sort());return projects;},
  readGraph:(value:unknown)=>value,
  clock:(control:{published:string[]})=>createGraphPhaseClockV1({publish:resource=>{control.published.push(resource);}}),
  say:(key:string,fields:Record<string,string|number>)=>{said.push({key,text:renderGenerationMessageV1(key as never,fields,options.environment??{})});},
 };
 return{host,events,said,published,expanded,log,holdsLog};
}
