import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {closeReplicaV1,openReplicaV1,publishOnceV1,seedV1} from "../../../../🤝️shared-worktree/🔁️replica/🟦️.ts";
import {appendWalV1,payloadsV1,readCheckpointV1} from "../../../../🤝️shared-worktree/🔁️replica/💾️state/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🌐️remote-cache/🖥️server/🟦️.ts";
import {GRAPH_STATE_POLICY_V1,GraphStateFoldV1,classifyGraphInputV1,graphPlatformV1,graphStateAfterV1,graphStateFrameV1,graphStateOfV1,type GraphStateFrameV1,type SharedFileV1} from "../🟦️.ts";
import {decideGraphEntryV1,expandRootV1,fetchGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryV1,unpackGraphDocumentsV1,packGraphDocumentV1} from "../🛰️exchange/🟦️.ts";
import {readReplicaFilesV1,replicaHealthyV1} from "../📥️replica/🟦️.ts";
import {readTreeFilesV1,sharedPathsV1} from "../🌳️tree/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🪪️ The graph state is proven four ways: classification vectors, an independent Python model of the folds and the digest, properties of the key over random states (it changes exactly when a graph-relevant file changes), and the sources (replica checkpoint, delta log and tree walk) against each other on a real replica. The fleet exchange runs against a real cache server over real sockets. */
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),scratch:string[]=[],ajv=new Ajv({strict:false});
ajv.addSchema(schema);
const validator=(name:string)=>{const validate=ajv.getSchema(`${schema.$id}#/definitions/${name}`);if(!validate)throw Error("Missing definition "+name);return validate;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`graph-state-${label}-`));scratch.push(path);return path;};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const frame:GraphStateFrameV1={kind:"portable-graph",tooling:sha("tooling"),implementation:sha("implementation"),platform:"darwin-arm64",extra:{}};

function random(seed:number):()=>number{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}
const words=["alpha","beta","gamma","delta","🧪️tests","📦️packages","✏️s","🧰️framework"],suffixes=[".rs",".ts",".json",".md",".png",".toml",".lock",".txt",".feature"];
function listing(seed:number,count:number):SharedFileV1[]{
 const next=random(seed),rows:SharedFileV1[]=[],seen=new Set<string>();
 while(rows.length<count){
  const depth=1+Math.floor(next()*4),parts=Array.from({length:depth},()=>words[Math.floor(next()*words.length)]!+Math.floor(next()*40));
  const path=parts.join("/")+suffixes[Math.floor(next()*suffixes.length)]!;
  if(seen.has(path))continue;seen.add(path);rows.push([path,sha(path+":"+Math.floor(next()*1e9))]);
 }
 rows.push(["node_modules/x/y.ts",sha("ignored")],["target/debug/z.rs",sha("ignored2")],[".git/HEAD",sha("git")],["🗑️generated/a.json",sha("generated")]);
 return rows;
}

test("the schema is closed and the policy and a state validate against it",()=>{
 expect(validator("Frame")(frame)).toBe(true);
 expect(validator("Frame")({...frame,kind:"other"})).toBe(false);
 expect(validator("Frame")({...frame,extra:{generated:"zz"}})).toBe(false);
 expect(validator("Frame")({...frame,unknown:1})).toBe(false);
 const state=graphStateOfV1(frame,listing(1,50));
 expect(validator("State")(state)).toBe(true);
 expect(validator("State")({...state,digest:"abc"})).toBe(false);
 expect(validator("Note")({version:1,kind:"nx-graph",digest:state.digest,platform:"darwin-arm64",producer:"m1:1",createdMs:1,documents:[{name:"a.br",hash:sha("a"),size:1}]})).toBe(true);
 expect(validator("Note")({version:1,kind:"nx-graph",digest:state.digest,platform:"darwin-arm64",producer:"m1:1",createdMs:1,documents:[{name:"../a",hash:sha("a"),size:1}]})).toBe(false);
 expect(GRAPH_STATE_POLICY_V1.domain).toBe("semio-graph-state-v1");
 for(const key of Object.keys(schema.definitions.TextCatalogue.const))expect(Object.keys((schema.definitions.TextCatalogue.const as Record<string,Record<string,string>>)[key]!).sort()).toEqual(["de","en"]);
});

test("classification: the walk, the authored extensions and the names the policy lists",()=>{
 const vectors:[string,string][]=[
  ["✏️s/🔌️plugins/📋️project.json","content"],["a/b/c.rs","content"],["a/b/c.ts","content"],["Cargo.lock","content"],["bun.lock","content"],["nx.json","content"],["a/go.mod","content"],["a/📜️script.ts","content"],
  ["README.md","name"],["a/logo.png","name"],["a/b/notes.pdf","name"],["a/c.RS","name"],["Makefile","name"],
  ["node_modules/x/index.ts","ignored"],["a/node_modules/x.json","ignored"],["a/target/debug/x.rs","ignored"],[".git/HEAD","ignored"],[".cache/b/x.json","ignored"],["a/🗑️generated/x.json","ignored"],[".🧬semio/🦑️repo/🎫️tickets/x.md","ignored"],["a/dist/x.js","ignored"],
 ];
 for(const [path,kind] of vectors)expect<unknown>(classifyGraphInputV1(path),path).toBe(kind);
});

test("the folds and the digest equal an independent Python model",()=>{
 const batches=[0,1,2,3,4].map(index=>{
  const rows=listing(100+index,index*80+1),next=random(index);
  return {frame:graphStateFrameV1({...frame,extra:index%2?{generated:sha("g"+index)}:{}}),rows:rows.map(([path,digest])=>[path,classifyGraphInputV1(path),digest] as const),shuffled:rows.sort(()=>next()-0.5)};
 });
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify({batches:batches.map(batch=>({frame:batch.frame,rows:batch.rows}))})),stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const expected=JSON.parse(run.stdout.toString()) as {names:string;content:string;files:number;contentFiles:number;digest:string}[];
 batches.forEach((batch,index)=>{
  const state=graphStateOfV1(batch.frame,batch.shuffled);
  expect({names:state.names,content:state.content,files:state.files,contentFiles:state.contentFiles,digest:state.digest}).toEqual(expected[index]!);
 });
});

test("the key changes exactly when a graph-relevant file changes",()=>{
 const base=listing(7,400),original=graphStateOfV1(frame,base),contentRows=base.filter(([path])=>classifyGraphInputV1(path)==="content"),nameRows=base.filter(([path])=>classifyGraphInputV1(path)==="name"),ignoredRows=base.filter(([path])=>classifyGraphInputV1(path)==="ignored");
 expect(contentRows.length).toBeGreaterThan(50);expect(nameRows.length).toBeGreaterThan(20);expect(ignoredRows.length).toBe(4);
 const change=(edit:(rows:[path:string,sha256:string][])=>void)=>{const rows=base.map((row):[path:string,sha256:string]=>[row[0],row[1]]);edit(rows);return graphStateOfV1(frame,rows).digest;};
 for(const [path] of contentRows.slice(0,40))expect(change(rows=>{rows.find(row=>row[0]===path)![1]=sha("edited "+path);}),"edit "+path).not.toBe(original.digest);
 for(const [path] of nameRows.slice(0,40))expect(change(rows=>{rows.find(row=>row[0]===path)![1]=sha("edited "+path);}),"bytes of a name only file "+path).toBe(original.digest);
 for(const [path] of ignoredRows)expect(change(rows=>{rows.find(row=>row[0]===path)![1]=sha("edited "+path);}),"ignored "+path).toBe(original.digest);
 expect(change(rows=>{rows.push(["alpha1/new.md",sha("n")]);}),"a new name only file changes what a directory listing returns").not.toBe(original.digest);
 expect(change(rows=>{rows.push(["alpha1/new.rs",sha("n")]);})).not.toBe(original.digest);
 expect(change(rows=>{rows.push(["node_modules/q/new.ts",sha("n")]);rows.push([".git/refs/x",sha("n")]);})).toBe(original.digest);
 expect(change(rows=>{rows.splice(rows.findIndex(row=>row[0]===nameRows[0]![0]),1);})).not.toBe(original.digest);
 expect(change(rows=>{rows.splice(rows.findIndex(row=>row[0]===contentRows[0]![0]),1);})).not.toBe(original.digest);
 expect(change(rows=>{rows.reverse();})).toBe(original.digest);
 const moved=contentRows[3]!;
 expect(change(rows=>{rows.find(row=>row[0]===moved[0])![0]=moved[0]+"x";})).not.toBe(original.digest);
 for(const other of [{...frame,platform:"linux-x64"},{...frame,tooling:sha("t2")},{...frame,implementation:sha("i2")},{...frame,kind:"nx-graph" as const},{...frame,extra:{generated:sha("g")}}])expect(graphStateOfV1(other,base).digest).not.toBe(original.digest);
 expect(graphStateOfV1({...frame,extra:{b:sha("1"),a:sha("2")}},base).digest).toBe(graphStateOfV1({...frame,extra:{a:sha("2"),b:sha("1")}},base).digest);
 expect(() => graphStateOfV1(frame,[["a/b.rs","not-a-digest"]])).toThrow();
});

test("the state after a few changes equals the state of the whole new list",()=>{
 const base=listing(11,600),names=new GraphStateFoldV1(),content=new GraphStateFoldV1();
 for(const [path,digest] of base){const kind=classifyGraphInputV1(path);if(kind==="ignored")continue;names.add(path);if(kind==="content")content.add(path,digest);}
 const next=random(5),current=new Map(base),changes:{path:string;before:string|undefined;after:string|undefined}[]=[];
 for(let index=0;index<30;index++){
  const roll=next(),paths=[...current.keys()];
  if(roll<0.4){const path=paths[Math.floor(next()*paths.length)]!;const after=sha("e"+index);changes.push({path,before:current.get(path),after});current.set(path,after);}
  else if(roll<0.7){const path=paths[Math.floor(next()*paths.length)]!;changes.push({path,before:current.get(path),after:undefined});current.delete(path);}
  else{const path=`gamma${index}/added${index}.rs`;changes.push({path,before:undefined,after:sha("a"+index)});current.set(path,sha("a"+index));}
 }
 expect(graphStateAfterV1({frame,names,content},changes).digest).toBe(graphStateOfV1(frame,current).digest);
});

test("the platform tag names the operating system and the architecture",()=>{
 expect(graphPlatformV1("darwin","arm64")).toBe("darwin-arm64");expect(graphPlatformV1("win32","x64")).toBe("win32-x64");expect(graphPlatformV1()).toMatch(/^[a-z0-9]+-[a-z0-9_]+$/u);
});

function repository(files:Record<string,string>):string{
 const root=temp("repo");
 writeFileSync(join(root,".gitignore"),"ignored/\n*.log\n.cache/\n");
 for(const [path,text] of Object.entries(files)){const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);utimesSync(file,1_700_000_000,1_700_000_000);}
 Bun.spawnSync(["git","init","-q"],{cwd:root});
 return root;
}

test("a real replica checkpoint and its delta log give the digests of a tree walk",async()=>{
 const files={"a/📋️project.json":"{\"name\":\"a\"}\n","a/src/main.rs":"fn main(){}\n","a/README.md":"hello\n","b/c.ts":"export const c=1;\n","b/data.bin":"binary","node_modules/x/i.ts":"ignored","ignored/z.rs":"ignored"};
 const root=repository(files),store=temp("store"),options={worktree:root,storeRoot:store,originId:"a",groupId:"g1",pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0};
 await seedV1(options);
 const replica=await openReplicaV1(options);await publishOnceV1(replica);await closeReplicaV1(replica);
 expect(await replicaHealthyV1(root)).toBe(false);
 const state=await readReplicaFilesV1(root);
 expect(state).toBeDefined();
 const tree=await readTreeFilesV1(root,{memoFile:join(root,".cache","f","memo.json")});
 const left=graphStateOfV1(frame,state!.files()),right=graphStateOfV1(frame,tree.files);
 expect(left.digest).toBe(right.digest);expect(left.files).toBe(right.files);expect(left.contentFiles).toBeGreaterThanOrEqual(4);
 expect(state!.get("a/src/main.rs")).toBe(createHash("sha256").update("fn main(){}\n").digest("hex"));
 expect(state!.get("a/missing.rs")).toBeUndefined();
 const again=await readTreeFilesV1(root,{memoFile:join(root,".cache","f","memo.json")});
 expect(again.read).toBe(0);expect(again.memoised).toBe(tree.read);
 const generation=(await readCheckpointV1(join(root,".cache","f","state.ckp"),()=>undefined))!.generation,added=createHash("sha256").update("fn two(){}\n").digest("hex");
 await appendWalV1(join(root,".cache","f","state.wal"),generation,false,[...payloadsV1([{kind:"files",path:"a/src/two.rs",entry:{hlc:"0000000000001-000001-a",operation:"put",sha256:added,bytes:11,executable:false}},{kind:"files",path:"b/c.ts",entry:null}])]);
 const after=await readReplicaFilesV1(root);
 expect(after!.get("a/src/two.rs")).toBe(added);expect(after!.get("b/c.ts")).toBeUndefined();
 const incremental=graphStateAfterV1({frame,names:foldOf(state!.files(),"names"),content:foldOf(state!.files(),"content")},[{path:"a/src/two.rs",before:undefined,after:added},{path:"b/c.ts",before:state!.get("b/c.ts"),after:undefined}]);
 expect(graphStateOfV1(frame,after!.files()).digest).toBe(incremental.digest);
 expect((await sharedPathsV1(root)).sort()).toContain("a/src/main.rs");
},60000);

function foldOf(files:Iterable<SharedFileV1>,which:"names"|"content"):GraphStateFoldV1{
 const fold=new GraphStateFoldV1();
 for(const [path,digest] of files){const kind=classifyGraphInputV1(path);if(kind==="ignored")continue;if(which==="names")fold.add(path);else if(kind==="content")fold.add(path,digest);}
 return fold;
}

test("the replica health gate demands a fresh file, a reachable store, a verified manifest and a live process, and ignores lifetime failure counters",async()=>{
 const root=temp("health"),write=(value:unknown)=>{mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","health.json"),JSON.stringify(value));};
 const healthy={version:1,wallMs:1_000_000,pid:42,store:"ok",storeSinceMs:1,pending:null,failures:[],trust:{scope:"shared-source",verifiedMs:990_000,lostMs:null,lostKind:null}};
 write(healthy);
 expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(true);
 expect(await replicaHealthyV1(root,1_100_000,()=>true)).toBe(false);
 expect(await replicaHealthyV1(root,1_010_000,()=>false)).toBe(false);
 write({...healthy,store:"down"});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
 write({...healthy,failures:[{kind:"blocked",count:126360,lastMs:1}]});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(true);
 write({...healthy,failures:["x"]});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(true);
 write({...healthy,trust:{...healthy.trust,verifiedMs:null}});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
 write({...healthy,trust:{...healthy.trust,lostMs:995_000,lostKind:"unstable"}});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
 write({...healthy,trust:{...healthy.trust,verifiedMs:996_000,lostMs:995_000,lostKind:"unstable"}});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(true);
 write({...healthy,trust:undefined});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
 write({...healthy,wallMs:"x"});expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
 rmSync(join(root,".cache","f","health.json"));expect(await replicaHealthyV1(root,1_010_000,()=>true)).toBe(false);
});

test("neutralizing the root gives the same bytes at every root and expanding restores them",()=>{
 const body=(root:string)=>JSON.stringify({cwd:root,arguments:[`${root}/🧰️framework/x.ts`,"--flag"],env:{CATALOG:`${root}/a:${root}/b`},other:`${root}x/not-the-root`,inside:`/mnt${root}/not-either`});
 const rootA="/Users/Shared/semio",rootB="/srv/semio-two";
 const neutralA=Buffer.from(neutralizeRootV1(Buffer.from(body(rootA)),rootA)).toString(),neutralB=Buffer.from(neutralizeRootV1(Buffer.from(body(rootB)),rootB)).toString();
 expect(neutralA.split(GRAPH_STATE_POLICY_V1.rootToken).length-1).toBe(4);
 expect(neutralA.replaceAll("semio-two","semio")).toBe(neutralA);
 expect(neutralA.replace("x/not-the-root","").length).toBeLessThan(neutralA.length);
 expect(neutralA.includes(`${rootA}x/not-the-root`)).toBe(true);
 expect(neutralA.includes(`/mnt${rootA}/not-either`)).toBe(true);
 expect(neutralB.replace(`${rootB}x/not-the-root`,`${rootA}x/not-the-root`).replace(`/mnt${rootB}/not-either`,`/mnt${rootA}/not-either`)).toBe(neutralA);
 expect(Buffer.from(expandRootV1(Buffer.from(neutralA),rootB)).toString()).toBe(body(rootB).replace(`${rootB}x/not-the-root`,`${rootA}x/not-the-root`).replace(`/mnt${rootB}/not-either`,`/mnt${rootA}/not-either`));
 const windows="C:\\semio",text=JSON.stringify({a:windows+"\\x.ts",b:"C:/semio/y.ts"});
 const neutralW=neutralizeRootV1(Buffer.from(text),windows);
 expect(Buffer.from(neutralW).toString()).not.toContain("semio\\\\x");
 expect(Buffer.from(expandRootV1(neutralW,windows)).toString()).toBe(text);
 expect(Buffer.from(neutralizeRootV1(Buffer.from("plain"),"/r")).toString()).toBe("plain");
});

test("documents pack into blobs of at most 32 MiB and unpack to the same bytes",()=>{
 const bytes=Buffer.from("x".repeat(100000)+Math.random()),blobs=packGraphDocumentV1({name:"graph/project-graph.json",bytes});
 expect(blobs.length).toBe(1);expect(blobs[0]!.name).toBe("graph/project-graph.json.br");
 expect(Buffer.from(unpackGraphDocumentsV1(blobs)[0]!.bytes).equals(bytes)).toBe(true);
 const noise=Buffer.alloc((33<<20)),next=random(3);for(let index=0;index<noise.length;index+=4)noise.writeUInt32LE(Math.floor(next()*4294967296),index);
 const parts=packGraphDocumentV1({name:"big",bytes:noise});
 expect(parts.length).toBeGreaterThanOrEqual(2);expect(parts.every(part=>part.bytes.length<=32<<20)).toBe(true);
 expect(Buffer.from(unpackGraphDocumentsV1(parts)[0]!.bytes).equals(noise)).toBe(true);
 expect(()=>unpackGraphDocumentsV1([{name:"oops",bytes:new Uint8Array(1)}])).toThrow();
});

async function cacheServer(){
 const directory=temp("cache"),server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const client=(token:string,upload=true)=>graphFleetClientV1({endpoint:server.url,instance:"main",token,upload,stateDirectory:temp("breaker")});
 return {server,client};
}
const documents=(label:string)=>[{name:"graph/project-graph.json",bytes:Buffer.from(JSON.stringify({label,root:"⟦semio-root⟧/x"}))},{name:"graph/file-map.json",bytes:Buffer.from(JSON.stringify({files:[label]}))}];

test("a published entry is fetched by another client; a read-only client cannot publish",async()=>{
 const {server,client}=await cacheServer(),state=graphStateOfV1(frame,listing(21,60));
 try{
  const writer=client("w"),reader=client("r",false);
  expect(await fetchGraphEntryV1(reader,state)).toBeUndefined();
  expect(await publishGraphEntryV1({client:reader,state,owner:"reader:1",documents:documents("x")})).toBe(false);
  expect(await fetchGraphEntryV1(reader,state)).toBeUndefined();
  expect(await publishGraphEntryV1({client:writer,state,owner:"writer:1",documents:documents("x")})).toBe(true);
  const found=await fetchGraphEntryV1(reader,state);
  expect(found).toBeDefined();expect(found).not.toBe("fault");
  const entry=found as Exclude<typeof found,undefined|"fault">;
  expect(entry.note.digest).toBe(state.digest);expect(validator("Note")(entry.note)).toBe(true);
  expect(entry.documents.map(document=>document.name).sort()).toEqual(["graph/file-map.json","graph/project-graph.json"]);
  expect(JSON.parse(Buffer.from(entry.documents.find(document=>document.name==="graph/file-map.json")!.bytes).toString())).toEqual({files:["x"]});
  const other=graphStateOfV1({...frame,platform:"linux-x64"},listing(21,60));
  expect(graphEntryKeyV1(other)).not.toBe(graphEntryKeyV1(state));expect(await fetchGraphEntryV1(reader,other)).toBeUndefined();
 }finally{await server.stop();}
},60000);

test("two concurrent cold starts compute once: one holds the claim, the other waits and installs its upload",async()=>{
 const {server,client}=await cacheServer(),state=graphStateOfV1(frame,listing(22,60)),first=client("w"),second=client("w");
 try{
  let computing=0,computed=0;
  const machine=async(who:ReturnType<typeof client>,owner:string)=>{
   const decision=await decideGraphEntryV1({client:who,state,owner});
   if(decision.outcome==="hit")return {owner,outcome:"hit" as const,via:decision.via};
   computing++;computed++;expect(computing).toBe(1);
   await Bun.sleep(600);
   computing--;
   const stored=await publishGraphEntryV1({client:who,state,owner,documents:documents(owner),hold:decision.hold});
   return {owner,outcome:"computed" as const,stored};
  };
  const results=await Promise.all([machine(first,"m1:1"),machine(second,"m2:2")]);
  expect(computed).toBe(1);
  expect(results.filter(result=>result.outcome==="computed").length).toBe(1);
  const hit=results.find(result=>result.outcome==="hit");
  expect(hit).toMatchObject({outcome:"hit",via:"wait"});
  const third=await decideGraphEntryV1({client:client("w"),state,owner:"m3:3"});
  expect(third).toMatchObject({outcome:"hit",via:"cache"});
 }finally{await server.stop();}
},60000);

test("a holder that dies lets the next machine take over; a broken entry and a dead server fall back to computing",async()=>{
 const {server,client}=await cacheServer(),state=graphStateOfV1(frame,listing(23,60));
 try{
  const dead=await decideGraphEntryV1({client:client("w"),state,owner:"dead:1"});
  expect(dead).toMatchObject({outcome:"compute",reason:"granted"});
  if(dead.outcome==="compute")dead.hold?.stop();
  const waiting=decideGraphEntryV1({client:client("w"),state,owner:"alive:2",now:()=>Date.now()});
  const claim=await client("w").claim(graphEntryKeyV1(state),"alive:2",1000);
  expect(["held","granted"]).toContain(claim.state);
  server.claims.release("main","ac",graphEntryKeyV1(state),"dead:1");
  const taken=await waiting;
  expect(taken.outcome).toBe("compute");
  if(taken.outcome==="compute")await taken.hold?.release();
  const denied=await decideGraphEntryV1({client:client("wrong"),state,owner:"denied:3"});
  expect(denied.outcome).toBe("compute");expect(denied.outcome==="compute"&&denied.hold).toBeFalsy();
 }finally{await server.stop();}
 const gone=await decideGraphEntryV1({client:graphFleetClientV1({endpoint:"http://127.0.0.1:9",instance:"main",token:"w",upload:true,stateDirectory:temp("gone")}),state,owner:"nobody:4"});
 expect(gone.outcome).toBe("compute");expect(gone.outcome==="compute"&&gone.hold).toBeFalsy();
 expect(await publishGraphEntryV1({client:graphFleetClientV1({endpoint:"http://127.0.0.1:9",instance:"main",token:"w",upload:true,stateDirectory:temp("gone2")}),state,owner:"nobody:4",documents:documents("x")})).toBe(false);
},60000);

test("the trust object of the graph state schema equals the one the replica daemon writes (same schema, same policy times)",async()=>{
 const trust=(await import("../../../../🤝️shared-worktree/🔏️trust/🧬️schema/🔣️.json",{with:{type:"json"}})).default as {definitions:Record<string,Record<string,unknown>>};
 const own=await import("../🧬️schema/🔣️.json",{with:{type:"json"}}),strip=(value:unknown):unknown=>JSON.parse(JSON.stringify(value,(key,item)=>key==="description"?undefined:key==="required"&&Array.isArray(item)?[...item].sort():item));
 expect(strip(own.default.definitions.Trust)).toEqual(strip(trust.definitions.Trust));
 expect(strip(own.default.definitions.Count)).toEqual(strip(trust.definitions.Count));
 const {trustPolicyV1}=await import("../../../../🤝️shared-worktree/🔏️trust/🟦️.ts");
 expect(own.default.definitions.Policy.const).toMatchObject({healthMaximumAgeMilliseconds:trustPolicyV1.healthMaximumAgeMilliseconds});
});
