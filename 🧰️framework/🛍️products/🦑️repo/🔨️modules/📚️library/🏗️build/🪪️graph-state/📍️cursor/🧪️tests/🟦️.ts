import {afterAll,describe,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {brotliCompressSync,brotliDecompressSync} from "node:zlib";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {journalGraphSeamsV1} from "../../../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";
import {GRAPH_CURSOR_POLICY_V1,composeJournalPortsV1,decodeSnapshotV1,encodeSnapshotV1,graphCursorTextV1,graphJournalPortV1,manifestJournalPortV1,manifestTrustV1,verifiedReplicaFilesV1,snapshotOfFilesV1,snapshotRowsV1,snapshotStoreV1,type JournalPortV1} from "../🟦️.ts";
import {createVerdictMemoV1,GRAPH_VERDICT_POLICY_V1,verdictCodeDigestV1,verdictKeyV1} from "../🧠️verdicts/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📍️ The manifest cursor is proven four ways: the instance and the rows equal an independent Python model, the stored form round trips and refuses a tampered one, the port answers like the watcher journal does, and a property over random edits of a real tree: the rows a cursor read leaves unchecked are exactly rows that still hold, so the stale set of the cursor equals the stale set of the full pass. */
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),scratch:string[]=[],ajv=new Ajv({strict:false});
ajv.addSchema(schema);
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(realpathSync(tmpdir()),`cursor-${label}-`));scratch.push(path);return path;};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const ask=(request:unknown):any=>{const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(request))});if(run.exitCode!==0)throw Error(run.stderr.toString());return JSON.parse(run.stdout.toString());};
function random(seed:number):()=>number{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}
const words=["alpha","beta","gamma","🧪️tests","📦️packages","✏️s","🧰️framework","a b"],suffixes=[".rs",".ts",".json",".md",".png",".toml",".lock",".txt"];
function listing(seed:number,count:number):[string,string][]{
 const next=random(seed),rows:[string,string][]=[],seen=new Set<string>();
 while(rows.length<count){
  const depth=1+Math.floor(next()*4),parts:string[]=[];
  for(let at=0;at<depth;at++)parts.push(words[Math.floor(next()*words.length)]!);
  const path=parts.join("/")+"/f"+Math.floor(next()*40)+suffixes[Math.floor(next()*suffixes.length)]!;
  if(seen.has(path))continue;seen.add(path);rows.push([path,sha(path+Math.floor(next()*3))]);
 }
 rows.push(["node_modules/x/index.js",sha("n")],[".cache/b/z.json",sha("c")]);
 return rows;
}
const policy=GRAPH_CURSOR_POLICY_V1;

describe("the snapshot",()=>{
 test("its instance equals the Python model over 20 random states and ignored paths never enter",()=>{
  for(let seed=1;seed<=20;seed++){
   const files=listing(seed,5+seed*7),snapshot=snapshotOfFilesV1(files);
   expect(snapshot.paths.some(path=>path.startsWith("node_modules/")||path.startsWith(".cache/"))).toBe(false);
   if(python){
    const rows=snapshot.paths.map((path,at)=>[path,snapshot.shas[at]]);
    expect(snapshot.instance).toBe(ask({command:"instance",rows}));
   }
   expect(snapshot.shas.every(entry=>entry.length===policy.shaLength)).toBe(true);
  }
 });
 test("order and duplicates of the input do not move the instance, one byte of one file does",()=>{
  const files=listing(3,60),shuffled=[...files].reverse(),base=snapshotOfFilesV1(files).instance;
  expect(snapshotOfFilesV1(shuffled).instance).toBe(base);
  const contentAt=files.findIndex(([path])=>path.endsWith(".ts")||path.endsWith(".rs")||path.endsWith(".json")||path.endsWith(".toml"));
  const edited=files.map((row,at)=>at===contentAt?[row[0],sha("other")] as [string,string]:row);
  expect(snapshotOfFilesV1(edited).instance).not.toBe(base);
 });
 test("the stored form round trips, validates against the schema, and a tampered or renamed one is refused",()=>{
  const snapshot=snapshotOfFilesV1(listing(5,80)),bytes=encodeSnapshotV1(snapshot),back=decodeSnapshotV1(bytes,snapshot.instance);
  expect(back).toEqual(snapshot);
  const header=ajv.getSchema(`${schema.$id}#/definitions/SnapshotHeader`)!;
  expect(header({version:1,instance:snapshot.instance,files:snapshot.paths.length})).toBe(true);
  expect(header({version:1,instance:"xyz",files:1})).toBe(false);
  expect(()=>decodeSnapshotV1(bytes,"0".repeat(16))).toThrow();
  expect(()=>decodeSnapshotV1(Buffer.from("not brotli"))).toThrow();
  expect(()=>snapshotOfFilesV1([["a\tb.ts",sha("x")]])).toThrow();
 });
 test("the rows between two states equal the Python model over 30 random edits",()=>{
  for(let seed=1;seed<=30;seed++){
   const before=listing(seed,40+seed),next=random(seed*977),after=before.filter(()=>next()>0.08).map(row=>next()<0.1?[row[0],sha("edit"+next())] as [string,string]:row);
   const extra=listing(seed+500,Math.floor(next()*8));for(const row of extra)if(!after.some(other=>other[0]===row[0]))after.push(row);
   const left=snapshotOfFilesV1(before),right=snapshotOfFilesV1(after),rows=snapshotRowsV1(left,right);
   if(python){
    const expected=ask({command:"diff",before:left.paths.map((path,at)=>[path,left.shas[at]]),after:right.paths.map((path,at)=>[path,right.shas[at]])});
    expect(rows.map(row=>JSON.stringify(row)).sort()).toEqual(expected.map((row:unknown)=>JSON.stringify(row)).sort());
   }
  }
 });
 test("the message catalogue has both languages for every key",()=>{
  const table=schema.definitions.TextCatalogue.const;
  for(const key of Object.keys(table) as (keyof typeof table)[]){expect(graphCursorTextV1(key,"en",{reason:"r",count:3}).length).toBeGreaterThan(5);expect(graphCursorTextV1(key,"de",{reason:"r",count:3}).length).toBeGreaterThan(5);}
 });
});

describe("the port",()=>{
 const source=(files:()=>[string,string][])=>async()=>({files:files()});
 test("a cursor read of an unchanged state is empty, a changed state names exactly the changed paths and survives a new process",async()=>{
  const root=temp("port"),store=snapshotStoreV1(root);let files=listing(7,50);
  const first=manifestJournalPortV1({root,store,source:source(()=>files)}),cursor=(await first.cursor())!;
  expect(cursor.instance).toMatch(/^[0-9a-f]{16}$/u);expect(await first.owns(cursor.instance)).toBe(true);
  const same=manifestJournalPortV1({root,store,source:source(()=>files)}),answer=await same.since(cursor);
  expect(answer).toMatchObject({complete:true,rows:[]});
  files=files.map((row,at)=>at===3?[row[0],sha("edited")] as [string,string]:row).filter((_,at)=>at!==7);files.push(["🧪️tests/new/added.ts",sha("new")]);
  const later=manifestJournalPortV1({root,store,source:source(()=>files)}),changed=await later.since(cursor);
  expect(changed.complete).toBe(true);
  if(changed.complete){
   const paths=changed.rows.filter(row=>row[1]===0).map(row=>row[2]);
   expect(paths).toContain("🧪️tests/new/added.ts");expect(changed.cursor.instance).not.toBe(cursor.instance);
   expect(changed.rows.some(row=>row[0]==="a"&&row[1]===1&&row[2]==="🧪️tests/new")).toBe(true);
  }
  const pass=later.excluded();
  expect(pass("node_modules/x/index.js")).toBe(true);expect(pass("never/seen.ts")).toBe(false);expect(pass("")).toBe(false);expect(pass("🧪️tests/new/added.ts")).toBe(false);
  writeFileSync(join(root,".gitignore"),"**/🤖️generated/\n**/🤖️generated.*\ndist\n");
  const ignoring=manifestJournalPortV1({root,store,source:source(()=>files)});await ignoring.since(cursor);
  const apart=ignoring.excluded();
  expect(apart("a/b/🤖️generated/x.ts")).toBe(true);expect(apart("a/b/🤖️generated.ts")).toBe(true);expect(apart("dist/out.js")).toBe(true);expect(apart("a/b/real.ts")).toBe(false);expect(apart("a/b/🤖️generatedx.ts")).toBe(false);
 });
 test("an unknown cursor, an unusable source and an overflow are incomplete answers with a reason, never an exception",async()=>{
  const root=temp("incomplete"),files=listing(9,30);
  const port=manifestJournalPortV1({root,source:source(()=>files),maximumRows:2});
  expect(await port.since({instance:"f".repeat(16),seq:0})).toMatchObject({complete:false,reason:"unrecorded"});
  expect(port.lastReason()).toBe("manifestSnapshot");
  const cursor=(await manifestJournalPortV1({root,source:source(()=>files)}).cursor())!;
  const moved=files.map(row=>[row[0],sha("x"+row[0])] as [string,string]);
  const overflow=await manifestJournalPortV1({root,source:source(()=>moved),maximumRows:2}).since(cursor);
  expect(overflow).toMatchObject({complete:false,count:2});
  const dead=manifestJournalPortV1({root:temp("dead"),source:async()=>({reason:"manifestDaemon" as const})});
  expect(await dead.cursor()).toBeUndefined();expect(dead.lastReason()).toBe("manifestDaemon");
  expect(await dead.since(cursor)).toMatchObject({complete:false});
 });
 test("the composition serves from the manifest while it can and from the watcher otherwise, and says why once",async()=>{
  const root=temp("compose"),told:string[]=[],watcher:JournalPortV1={cursor:async()=>({instance:"a".repeat(16),seq:4}),since:async()=>({complete:true,cursor:{instance:"a".repeat(16),seq:5},rows:[["m",0,"w"]]}),excluded:()=>()=>false};
  let usable=true;const files=listing(11,20);
  const port=graphJournalPortV1(root,()=>watcher,reason=>told.push(reason),{source:async()=>usable?{files}:{reason:"manifestDaemon"}});
  const cursor=(await port.cursor())!;
  expect(port.watches).toBe("governed");expect(cursor.instance).not.toBe("a".repeat(16));
  expect(await port.since(cursor)).toMatchObject({complete:true,rows:[]});
  expect(await port.since({instance:"a".repeat(16),seq:4})).toMatchObject({rows:[["m",0,"w"]]});
  expect(port.watches).toBe("all");
  usable=false;const other=graphJournalPortV1(root,()=>watcher,reason=>told.push(reason),{source:async()=>({reason:"manifestDaemon" as const})});
  expect(await other.cursor()).toMatchObject({seq:4});expect(told).toEqual(["manifestDaemon"]);
  expect(composeJournalPortsV1([watcher]).watches).toBe("all");
 });
 test("the daemon gate: a quiet daemon is trusted, one with failures is verified, stale, dead, unreachable or unreadable health is unusable",async()=>{
  const root=temp("health");mkdirSync(join(root,".cache","f"),{recursive:true});
  const write=(value:unknown)=>writeFileSync(join(root,".cache","f","health.json"),JSON.stringify(value));
  const now=1_000_000,trust=(verifiedMs:number|null,lostMs:number|null=null,lostKind:string|null=null)=>({scope:"shared-source",verifiedMs,lostMs,lostKind}),base={wallMs:now-1000,pid:1,store:"ok",failures:[]as {kind:string;count:number;lastMs:number}[],trust:trust(now-5000)};
  write(base);expect(await manifestTrustV1(root,now,()=>true)).toBe("trusted");
  write({...base,failures:[{kind:"blocked",count:126360,lastMs:now-3_600_000},{kind:"refused",count:9,lastMs:now-7_200_000},{kind:"attest",count:4,lastMs:now-90_000}]});expect(await manifestTrustV1(root,now,()=>true)).toBe("trusted");
  write({...base,trust:trust(null)});expect(await manifestTrustV1(root,now,()=>true)).toBe("verify");
  write({...base,trust:trust(now-5000,now-2000,"unstable")});expect(await manifestTrustV1(root,now,()=>true)).toBe("verify");
  write({...base,trust:trust(now-5000,now-9000,"blocked")});expect(await manifestTrustV1(root,now,()=>true)).toBe("trusted");
  write({...base,trust:trust(now-600_001)});expect(await manifestTrustV1(root,now,()=>true)).toBe("verify");
  write({...base,trust:{scope:"everything",verifiedMs:now}});expect(await manifestTrustV1(root,now,()=>true)).toBe("verify");
  const {trust:_,...bare}=base;write(bare);expect(await manifestTrustV1(root,now,()=>true)).toBe("verify");
  write({...base,wallMs:now-60000});expect(await manifestTrustV1(root,now,()=>true)).toBe("unusable");
  write(base);expect(await manifestTrustV1(root,now,()=>false)).toBe("unusable");
  write({...base,store:"lost"});expect(await manifestTrustV1(root,now,()=>true)).toBe("unusable");
  expect(await manifestTrustV1(temp("none"),now,()=>true)).toBe("unusable");
 });
 test("a manifest believed only as far as the tree agrees: unchanged keeps its digest without a read, a changed, young, gone or unreadable file does not",async()=>{
  const root=temp("verify"),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
  const names=["a/one.ts","a/two.ts","b/three.json","b/four.rs","c/five.toml","node_modules/p/x.js"];
  for(const name of names)write(name,"v1 "+name);
  const entry=(name:string,racy=false):[string,string,number,number,boolean]=>{const status=statSync(join(root,name));return[name,sha("v1 "+name),status.size,status.mtimeMs,racy];};
  const entries=names.map(name=>entry(name));
  const reads:string[]=[];const read=async(path:string)=>{reads.push(path);return readFileSync(path);};
  const unchanged=await verifiedReplicaFilesV1(root,entries,{read});
  expect(reads).toEqual([]);expect(unchanged.map(row=>row[0])).toEqual(names.filter(name=>!name.startsWith("node_modules/")));
  write("a/one.ts","v2 changed");utimesSync(join(root,"b/three.json"),new Date(1e6),new Date(1e6));rmSync(join(root,"c/five.toml"));
  const racy=entries.map(row=>row[0]==="b/four.rs"?[row[0],row[1],row[2],row[3],true] as typeof row:row);
  const later=await verifiedReplicaFilesV1(root,racy,{read});
  const byPath=new Map(later);
  expect(byPath.get("a/one.ts")).toBe(sha("v2 changed"));expect(byPath.get("b/three.json")).toBe(sha("v1 b/three.json"));expect(byPath.get("b/four.rs")).toBe(sha("v1 b/four.rs"));
  expect(byPath.has("c/five.toml")).toBe(false);expect(byPath.get("a/two.ts")).toBe(sha("v1 a/two.ts"));
  expect(reads.map(path=>path.slice(root.length+1)).sort()).toEqual(["a/one.ts","b/four.rs","b/three.json"]);
  if(python){const check=Bun.spawnSync([python,"-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",join(root,"a/one.ts")]);expect<unknown>(check.stdout.toString().trim()).toBe(byPath.get("a/one.ts"));}
  const failing=await verifiedReplicaFilesV1(root,[entry("a/two.ts")].map(row=>[row[0],row[1],row[2]+1,row[3],false] as typeof row),{read:async()=>{throw Error("EACCES");}});
  const once=failing[0]![1],again=await verifiedReplicaFilesV1(root,[entry("a/two.ts")].map(row=>[row[0],row[1],row[2]+1,row[3],false] as typeof row),{read:async()=>{throw Error("EACCES");}});
  expect(once).toMatch(/^[0-9a-f]{64}$/u);expect(once).not.toBe(sha("v1 a/two.ts"));expect(again[0]![1]).not.toBe(once);
 });
 test("the store keeps the newest snapshots and never the one in use",async()=>{
  const root=temp("prune"),store=snapshotStoreV1(root,undefined,3),instances:string[]=[];
  for(let seed=1;seed<=6;seed++){const snapshot=snapshotOfFilesV1(listing(seed,10));instances.push(snapshot.instance);await store.putBytes(snapshot.instance,encodeSnapshotV1(snapshot));const path=join(store.directory,snapshot.instance+policy.extension);utimesSync(path,new Date(1000*seed),new Date(1000*seed));}
  await store.prune(instances[0]!);
  const names=readdirSync(store.directory).filter(name=>name.endsWith(policy.extension)).sort();
  expect(names).toEqual([instances[0],instances[3],instances[4],instances[5]].map(instance=>instance+policy.extension).sort());
  expect(await store.has(instances[1]!)).toBe(false);expect(await store.get(instances[5]!)).toBeDefined();
 });
});

describe("the levels of doubt",()=>{
 test("a dirty path is 1, a region the manifest cannot see is 2 (the caller may vouch for it by stamps), a clean shared path is 0, and holds still checks both kinds",async()=>{
  const root=temp("levels");writeFileSync(join(root,".gitignore"),"**/🤖️generated/\n");mkdirSync(join(root,".cache"),{recursive:true});
  const files=listing(21,30),at=(path:string)=>join(root,path);
  const store=snapshotStoreV1(root),cursors:Record<string,any>={},memory={get:(key:string)=>cursors[key],set:(key:string,value:any)=>{cursors[key]=value;}};
  let state=files;const port=graphJournalPortV1(root,()=>{throw Error("no watcher");},()=>{},{store,source:async()=>({files:state})});
  const checked:string[][]=[],seams=journalGraphSeamsV1({root,journal:port as never,store:memory as never,ports:{} as never,digest:()=>"d",membership:async()=>({digest:"x",frontier:{version:1,capturedAt:0,directories:[],members:[]},changes:[]}),holds:async rows=>{checked.push(rows.map(row=>row.path));return true;}});
  seams.scope(true);await seams.membership(undefined);seams.commit({generation:1,owners:["o"]} as never);
  const edited=files[0]![0];state=files.map((row,index)=>index===0?[row[0],sha("changed")] as [string,string]:row);
  const next=journalGraphSeamsV1({root,journal:port as never,store:memory as never,ports:{} as never,digest:()=>"d",membership:async()=>({digest:"x",frontier:{version:1,capturedAt:0,directories:[],members:[]},changes:[]}),holds:async rows=>{checked.push(rows.map(row=>row.path));return true;}});
  next.scope(true);await next.membership({manifest:{generation:1,membership:"x"},frontier:{version:1,capturedAt:0,directories:[],members:[]}});
  const rows=[{kind:"file",path:at(edited),fact:{}},{kind:"file",path:at(files[5]![0]),fact:{}},{kind:"file",path:at("🤖️generated/x.ts"),fact:{}},{kind:"readdir",path:at("node_modules/p"),fact:[]},{kind:"fileExists",path:at("absent/but/shared.ts"),fact:false},{kind:"digest",path:"/outside/etc",fact:{}}];
  expect([...next.doubtful(rows as never)!]).toEqual([1,0,2,2,0,1]);
  expect(await next.holds(rows as never)).toBe(true);
  expect(checked.at(-1)).toEqual([at(edited),at("🤖️generated/x.ts"),at("node_modules/p"),"/outside/etc"]);
 });
});

describe("the property: a cursor read never vouches for a row that no longer holds",()=>{
 const rootFiles=(root:string)=>{
  const out:[string,string][]=[];
  const walk=(directory:string,prefix:string)=>{for(const name of readdirSync(join(root,directory)).sort()){
   const path=prefix===""?name:prefix+"/"+name,full=join(root,path),status=statSync(full);
   if(status.isDirectory()){if(name!=="node_modules"&&name!==".cache"&&name!=="🤖️generated")walk(path,path);}else out.push([path,sha(readFileSync(full,"utf8"))]);
  }};
  walk("","");return out;
 };
 type Row={kind:string;path:string;fact:any};
 const holdsRow=(root:string,row:Row):boolean=>{
  const full=row.path;
  if(row.kind==="file"||row.kind==="analysis")return existsSync(full)&&statSync(full).isFile()?sha(readFileSync(full,"utf8"))===row.fact.sha256:row.fact.sha256===null;
  if(row.kind==="fileExists")return existsSync(full)===row.fact;
  if(row.kind==="readdir")return existsSync(full)?JSON.stringify(readdirSync(full).sort())===JSON.stringify(row.fact):row.fact===null;
  return true;
 };
 const record=(root:string,files:string[],directories:string[]):Row[]=>{
  const rows:Row[]=[],at=(path:string)=>path===""?root:join(root,path),digest=(path:string)=>existsSync(at(path))?sha(readFileSync(at(path),"utf8")):null;
  for(const path of files)rows.push({kind:"file",path:at(path),fact:{sha256:digest(path)}},{kind:"analysis",path:at(path),fact:{sha256:digest(path)}},{kind:"fileExists",path:at(path),fact:existsSync(at(path))});
  for(const directory of directories)rows.push({kind:"readdir",path:at(directory),fact:existsSync(at(directory))?readdirSync(at(directory)).sort():null});
  rows.push({kind:"fileExists",path:at("a/missing.ts"),fact:false},{kind:"fileExists",path:at("🤖️generated/x.ts"),fact:existsSync(at("🤖️generated/x.ts"))},{kind:"file",path:at("node_modules/p/i.js"),fact:{sha256:digest("node_modules/p/i.js")}},{kind:"file",path:at("🤖️generated/x.ts"),fact:{sha256:digest("🤖️generated/x.ts")}});
  return rows;
 };
 for(const seed of [1,2,3,4,5,6,7,8]) test(`seed ${seed}: 25 random edits, the cursor stale set equals the full pass stale set`,async()=>{
  const root=temp("prop"),next=random(seed*7919),put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
  const dirs=["a","a/b","c","c/d","lib","🤖️generated"];writeFileSync(join(root,".gitignore"),"**/🤖️generated/\n");
  for(let at=0;at<40;at++)put(`${dirs[Math.floor(next()*5)]}/f${at}${[".ts",".json",".md",".rs"][at%4]}`,"v"+at);
  put("node_modules/p/i.js","dep");put("🤖️generated/x.ts","gen");mkdirSync(join(root,".cache"),{recursive:true});
  const store=snapshotStoreV1(root),fake:Record<string,{instance:string;seq:number}>={},cursors={get:(key:string)=>fake[key] as any,set:(key:string,cursor:any)=>{fake[key]=cursor;}};
  const port=graphJournalPortV1(root,()=>{throw Error("the watcher must not be asked");},()=>{},{store,source:async()=>({files:rootFiles(root)})});
  const scan=async(previous:any)=>({digest:"scan",frontier:previous?.frontier??{version:1,capturedAt:0,directories:[],members:[]},changes:[]});
  const rowsAt=()=>record(root,rootFiles(root).map(row=>row[0]),["","a","a/b","c","c/d","lib"]);
  let rows=rowsAt(),generation=1,vouched=0,total=0;
  const seams0=journalGraphSeamsV1({root,journal:port as never,store:cursors as never,ports:{} as never,digest:()=>"d",membership:scan,holds:async()=>true});
  seams0.scope(true);await seams0.membership(undefined);seams0.commit({generation,owners:["o"]} as never);
  for(let step=0;step<25;step++){
   const kind=Math.floor(next()*7),files=rootFiles(root).map(row=>row[0]).filter(path=>path!==".gitignore"),pick=files[Math.floor(next()*files.length)]!;
   if(kind===0)put(pick,"edit"+step);
   else if(kind===1&&files.length>8)rmSync(join(root,pick));
   else if(kind===2)put(`${dirs[Math.floor(next()*5)]}/n${step}.ts`,"new"+step);
   else if(kind===3)put(`d${step}/deep/n${step}.json`,"nested"+step);
   else if(kind===4)put("🤖️generated/x.ts","gen"+step);
   else if(kind===5)put("node_modules/p/i.js","dep"+step);
   else if(kind===6)put(pick,readFileSync(join(root,pick),"utf8"));
   if(step%5===4){
    const seams=journalGraphSeamsV1({root,journal:port as never,store:cursors as never,ports:{} as never,digest:()=>"d",membership:scan,holds:async()=>true});
    seams.scope(true);
    await seams.membership({manifest:{generation,membership:"scan"},frontier:{version:1,capturedAt:0,directories:[],members:[]}});
    const doubtful=seams.doubtful(rows)!;
    expect(doubtful).toBeDefined();
    const full=new Set<number>(),viaCursor=new Set<number>();
    rows.forEach((row,at)=>{const holds=holdsRow(root,row);if(!holds)full.add(at);if(!holds&&doubtful[at]!==0)viaCursor.add(at);if(doubtful[at]===0)vouched++;total++;});
    expect([...viaCursor].sort((a,b)=>a-b)).toEqual([...full].sort((a,b)=>a-b));
    seams.commit({generation:generation+1,owners:["o"]} as never);generation++;rows=rowsAt();
   }
  }
  expect(vouched/total).toBeGreaterThan(0.4);
 });
});

describe("the remembered verdicts",()=>{
 const source=(files:()=>[string,string][])=>async()=>({files:files()});
 const memoryOf=(store:ReturnType<typeof snapshotStoreV1>)=>{const directory=join(store.directory,"..","verdicts"),names=existsSync(directory)?readdirSync(directory).filter(name=>name.endsWith(GRAPH_VERDICT_POLICY_V1.extension)):[];return{directory,names,file:names[0]===undefined?undefined:join(directory,names[0])};};
 const probes=(seed:number,count:number)=>{const next=random(seed),rows:string[]=[];for(let at=0;at<count;at++){const depth=1+Math.floor(next()*5),parts:string[]=[];for(let part=0;part<depth;part++)parts.push([...words,"build","dist","target","x.log","node_modules",".cache"][Math.floor(next()*(words.length+6))]!);rows.push(parts.join("/")+(next()<0.5?"/f"+Math.floor(next()*9)+suffixes[Math.floor(next()*suffixes.length)]!:""));}return rows;};
 test("a judgement made by one process is read by the next, answers equal a port without memory over 3000 paths, and a path of the snapshot is never excluded",async()=>{
  const root=temp("verdicts"),store=snapshotStoreV1(root),files=listing(21,80);writeFileSync(join(root,".gitignore"),"build/\n*.log\ntarget\n");
  const first=manifestJournalPortV1({root,store,source:source(()=>files)}),cursor=(await first.cursor())!;await first.since(cursor);
  const paths=[...probes(3,3000),...files.map(row=>row[0])],judge=first.excluded(),firstAnswers=paths.map(path=>judge(path));
  first.flushVerdicts();
  const{file}=memoryOf(store);expect(file).toBeDefined();
  const second=manifestJournalPortV1({root,store,source:source(()=>files)});await second.since(cursor);
  const apart=manifestJournalPortV1({root,store:snapshotStoreV1(temp("verdicts-bare")),source:source(()=>files)});await apart.since((await apart.cursor())!);
  const again=second.excluded(),bare=apart.excluded();
  expect(paths.map(path=>again(path))).toEqual(firstAnswers);expect(paths.map(path=>bare(path))).toEqual(firstAnswers);
  for(const path of snapshotOfFilesV1(files).paths)expect(again(path)).toBe(false);
  expect(firstAnswers.some(Boolean)&&firstAnswers.some(answer=>!answer)).toBe(true);
 });
 test("a verdict is read from the memory (a doctored entry is believed), but a rule edit or another code digest gives an empty memory",async()=>{
  const root=temp("verdicts-key"),store=snapshotStoreV1(root),files=listing(22,40);writeFileSync(join(root,".gitignore"),"build/\n");
  const first=manifestJournalPortV1({root,store,source:source(()=>files)});await first.since((await first.cursor())!);
  const judge=first.excluded();expect(judge("zz/probe/a.ts")).toBe(false);first.flushVerdicts();
  const{file}=memoryOf(store);expect(file).toBeDefined();
  const text=brotliDecompressSync(readFileSync(file!)).toString("utf8");expect(text).toContain("\n0zz/probe/a.ts");
  writeFileSync(file!,brotliCompressSync(Buffer.from(text.replace("\n0zz/probe/a.ts","\n1zz/probe/a.ts"))));
  const believing=manifestJournalPortV1({root,store,source:source(()=>files)});await believing.since((await believing.cursor())!);
  expect(believing.excluded()("zz/probe/a.ts")).toBe(true);
  writeFileSync(join(root,".gitignore"),"build/\nzz/\n");
  const edited=manifestJournalPortV1({root,store,source:source(()=>files)});await edited.since((await edited.cursor())!);
  expect(edited.excluded()("zz/probe/a.ts")).toBe(true);expect(edited.excluded()("yy/probe/a.ts")).toBe(false);
  writeFileSync(join(root,".gitignore"),"build/\n");
  const restored=manifestJournalPortV1({root,store,source:source(()=>files)});await restored.since((await restored.cursor())!);
  expect(restored.excluded()("zz/probe/a.ts")).toBe(true);
  const key=verdictKeyV1({ignoreText:"build/\n",code:"a"}),other=verdictKeyV1({ignoreText:"build/\n",code:"b"});
  expect(key).toMatch(/^[0-9a-f]{32}$/u);expect(other).not.toBe(key);expect(verdictKeyV1({ignoreText:"build/\n",code:"a"})).toBe(key);
 });
 test("a damaged, foreign or oversized memory is empty, a small addition does not rewrite an existing file, a missing code file keeps no memory, and old keys are pruned",()=>{
  const directory=temp("verdicts-unit"),key=verdictKeyV1({ignoreText:"",code:"c"}),file=join(directory,key+GRAPH_VERDICT_POLICY_V1.extension);
  const memory=createVerdictMemoV1({directory,key});expect(memory.get("a")).toBeUndefined();memory.set("a",true);memory.set("b",false);memory.set("a",false);
  expect(memory.size()).toBe(2);expect(memory.pending()).toBe(2);memory.save();expect(memory.pending()).toBe(0);
  const text=brotliDecompressSync(readFileSync(file)).toString("utf8");expect(text).toBe(GRAPH_VERDICT_POLICY_V1.header+"\nkey\t"+key+"\n1a\n0b\n");
  const reread=createVerdictMemoV1({directory,key});expect(reread.get("a")).toBe(true);expect(reread.get("b")).toBe(false);
  const before=statSync(file).mtimeMs;reread.set("c",true);reread.save();expect(statSync(file).mtimeMs).toBe(before);expect(reread.pending()).toBe(1);
  for(let at=0;at<GRAPH_VERDICT_POLICY_V1.saveMinimum;at++)reread.set("p"+at,at%2===0);
  reread.save();expect(createVerdictMemoV1({directory,key}).get("p3")).toBe(false);expect(createVerdictMemoV1({directory,key}).get("p4")).toBe(true);
  writeFileSync(file,Buffer.from("not brotli"));expect(createVerdictMemoV1({directory,key}).get("a")).toBeUndefined();
  writeFileSync(file,brotliCompressSync(Buffer.from(GRAPH_VERDICT_POLICY_V1.header+"\nkey\t"+"0".repeat(32)+"\n1a\n")));expect(createVerdictMemoV1({directory,key}).get("a")).toBeUndefined();
  writeFileSync(file,brotliCompressSync(Buffer.from("other header\nkey\t"+key+"\n1a\n")));expect(createVerdictMemoV1({directory,key}).get("a")).toBeUndefined();
  const nothing=createVerdictMemoV1({directory,key:undefined});nothing.set("a",true);nothing.save();expect(nothing.get("a")).toBeUndefined();expect(verdictCodeDigestV1([join(directory,"missing.ts")])).toBeUndefined();
  for(let at=0;at<4;at++){const other=createVerdictMemoV1({directory,key:verdictKeyV1({ignoreText:String(at),code:"c"})});other.set("x",true);other.save();utimesSync(join(directory,verdictKeyV1({ignoreText:String(at),code:"c"})+GRAPH_VERDICT_POLICY_V1.extension),new Date(2_000_000_000_000+at*1000),new Date(2_000_000_000_000+at*1000));}
  expect(readdirSync(directory).filter(name=>name.endsWith(GRAPH_VERDICT_POLICY_V1.extension)).length).toBeLessThanOrEqual(GRAPH_VERDICT_POLICY_V1.kept+1);
 });
});
