/** 🕰 Contract: the journal decides what is looked at. Dirty sets equal an independent Python computation and validate against the schema with jsonschema; advancing a frontier from a dirty set equals a full walk, the stat-diff of the two frontiers and the Python walker, and costs the paths it names; the seams trust what the journal proves untouched, announce every verify epoch and move the verified cursor only after the generation was verified. */
import {afterAll,describe,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createFrontierV1,diffFrontiersV1} from "../../../🕸️graph/🕰️generations/🧭️frontier/🟨️.mjs";
import {walkFrontierV1} from "../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {GRAPH_HOOKS_POLICY_V1,advanceFrontierByDirtySetV1,derivedDirtySetV1,dirtyIndexV1,dirtySetFromRowsV1,renderGraphHooksNoticeV1} from "../🟨️.mjs";
import {journalGraphSeamsV1,relativeFactPathV1} from "../🔌️host/🟨️.mjs";
import {fileCursorStoreV1,installFingerprintV1,renderJournalNoticeV1} from "../📖️direct/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),oracle=join(here,"🐍️oracle","🐍️.py"),fixturePath=join(here,"🧫️fixtures","🔣️.json"),fixtures=JSON.parse(readFileSync(fixturePath,"utf8"));
const windows=process.platform==="win32",sandboxes:string[]=[];
afterAll(()=>sandboxes.forEach(path=>rmSync(path,{recursive:true,force:true})));
const sandbox=(prefix:string)=>{const path=mkdtempSync(join(realpathSync(tmpdir()),prefix));sandboxes.push(path);return path;};
const python3=(...args:string[])=>{const out=Bun.spawnSync([python!,"-I",oracle,...args]);if(out.exitCode!==0)throw Error(out.stderr.toString());return JSON.parse(out.stdout.toString());};

const counting=(root:string)=>{
 const calls={stat:0,list:0};
 return{calls,ports:{
  list:async(directory:string)=>{calls.list++;try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:entry.isSymbolicLink()?"other" as const:entry.isDirectory()?"directory" as const:entry.isFile()?"file" as const:"other" as const}));}catch{return undefined;}},
  stat:async(path:string)=>{calls.stat++;try{const status=await lstat(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
  descend:(name:string)=>name!=="node_modules"&&name!=="dist",
  member:(path:string)=>{const name=path.slice(path.lastIndexOf("/")+1);return name==="project.json"||name==="Cargo.toml"||name.endsWith(".feature");},
 }};
};
const build=(root:string,files:Record<string,string>)=>{for(const[path,content]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}};
const normal=(frontier:{directories:readonly (readonly [string,number,number])[];members:readonly (readonly [string,number])[]})=>({directories:[...frontier.directories].map(row=>[...row]).sort((a,b)=>String(a[0])<String(b[0])?-1:1),members:[...frontier.members].map(row=>[...row]).sort((a,b)=>String(a[0])<String(b[0])?-1:1)});
const names=(frontier:{directories:readonly (readonly [string,...unknown[]])[];members:readonly (readonly [string,number])[]})=>({directories:frontier.directories.map(row=>row[0]).sort(),members:[...frontier.members].map(row=>[row[0],row[1]]).sort((a,b)=>String(a[0])<String(b[0])?-1:1)});
const apply=(root:string,op:any)=>{
 const at=(path:string)=>join(root,...path.split("/"));
 if(op.op==="write"){mkdirSync(dirname(at(op.path)),{recursive:true});writeFileSync(at(op.path),op.content);}
 else if(op.op==="rm")rmSync(at(op.path));
 else if(op.op==="rmdir")rmSync(at(op.path),{recursive:true});
 else if(op.op==="mv"){mkdirSync(dirname(at(op.to)),{recursive:true});renameSync(at(op.from),at(op.to));}
 else if(op.op==="chmod")chmodSync(at(op.path),op.mode);
 else if(op.op==="mkdir")mkdirSync(at(op.path),{recursive:true});
};
const cursor={instance:"0123456789abcdef",seq:7};

describe("dirty sets from journal rows",()=>{
 const expected=python?python3("dirty",fixturePath):[];
 for(const[index,item]of fixtures.dirty.entries())test(item.name+": equals the Python computation, validates against the schema and answers the probes",()=>{
  const set=dirtySetFromRowsV1(item.rows,cursor);
  expect({exact:set.exact,subtrees:set.subtrees}).toEqual(python?{exact:expected[index].exact,subtrees:expected[index].subtrees}:{exact:set.exact,subtrees:set.subtrees});
  expect(set).toMatchObject({complete:true,overflow:false,cursor});
  if(python){
  }
  const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/DirtySetV1",definitions:schema.definitions});
  expect(validate(set)).toBe(true);expect(validate({...set,extra:1})).toBe(false);expect(validate({...set,exact:[...set.exact,"/abs"]})).toBe(false);
  const index_=dirtyIndexV1(set);
  for(const[path,doubtful]of item.probes)expect([item.name,path,index_.has(path)]).toEqual([item.name,path,doubtful]);
 });
 test("an overflowing answer is an incomplete dirty set and the advance refuses it",async()=>{
  const set=dirtySetFromRowsV1([["m",0,"a"]],cursor,{overflow:true});
  expect(set).toMatchObject({complete:false,overflow:true,exact:[],subtrees:[]});
  expect(await advanceFrontierByDirtySetV1({frontier:{version:1,capturedAt:1,directories:[],members:[]},set,...counting("/nowhere").ports})).toBeUndefined();
  expect(await advanceFrontierByDirtySetV1({frontier:{version:1,capturedAt:1,directories:[],members:[]},set:dirtySetFromRowsV1([["a",1,""]],cursor),...counting("/nowhere").ports})).toBeUndefined();
 });
 test("the policy declares both languages for every notice",()=>{
  for(const language of["en","de"])for(const notice of["unrecorded","outside"])expect(renderGraphHooksNoticeV1(notice,language,{generation:3})).not.toContain("{");
  expect(()=>renderGraphHooksNoticeV1("unrecorded","fr")).toThrow();
  expect(renderJournalNoticeV1({reason:"lost",path:"a",detail:"x"},"de")).toContain("graph:");
  expect(Object.keys(GRAPH_HOOKS_POLICY_V1.texts)).toEqual(["en","de"]);
 });
});

describe("journal contract documents",()=>{
 const ajv=new Ajv({strict:false}),check=(name:string,value:unknown)=>ajv.compile({$ref:"#/definitions/"+name,definitions:schema.definitions})(value);
 test("queries, replies and opens validate and refuse foreign shapes",()=>{
  expect(check("JournalQueryV1",{sinceCursor:cursor})).toBe(true);expect(check("JournalQueryV1",{sinceCursor:{instance:"x",seq:1}})).toBe(false);
  expect(check("JournalReplyV1",{complete:true,cursor,rows:[["m",0,"a/b.ts"]]})).toBe(true);expect(check("JournalReplyV1",{complete:true,cursor,rows:[["x",0,"a"]]})).toBe(false);
  expect(check("JournalReplyV1",{complete:false,reason:"lost",path:"a",detail:"must-scan"})).toBe(true);expect(check("JournalReplyV1",{complete:false})).toBe(false);
  expect(check("JournalOpenV1",{root:"/w",excludes:{rooted:[".git"],names:["node_modules"]},exclusionVersion:1})).toBe(true);expect(check("JournalOpenV1",{root:"/w",excludes:{rooted:[],names:[]},exclusionVersion:0})).toBe(false);
 });
});

describe("handing the dirty set to the derived DAG",()=>{
 test("paths are the named paths, their directories and the subtree roots; an incomplete set is overflow",()=>{
  expect(derivedDirtySetV1(dirtySetFromRowsV1([["m",0,"a/b.ts"],["a",1,"c/d"]],cursor),"rev1")).toEqual({revision:"rev1",paths:["a","a/b.ts","c","c/d"],overflow:false});
  expect(derivedDirtySetV1(dirtySetFromRowsV1([],cursor))).toEqual({paths:[],overflow:false});
  expect(derivedDirtySetV1(dirtySetFromRowsV1([["m",0,"a"]],cursor,{overflow:true}))).toEqual({paths:[],overflow:true});
  expect(Object.keys(derivedDirtySetV1(dirtySetFromRowsV1([["m",0,"x"]],cursor),"r")).sort()).toEqual(["overflow","paths","revision"]);
 });
});

describe("advancing a frontier from a dirty set",()=>{
 for(const scenario of fixtures.scenarios)test(scenario.name+": equals a full walk, the frontier diff and the Python walker",async()=>{
  if(scenario.posix&&windows)return;
  const root=sandbox("dirty-advance-");build(root,fixtures.tree);
  if(!windows)symlinkSync(root,join(root,"a","loop"));
  const{ports,calls}=counting(root),before={version:1 as const,capturedAt:Date.now(),...await walkFrontierV1(ports)},start=createFrontierV1(before);
  for(const op of scenario.ops)apply(root,op);
  calls.stat=0;calls.list=0;
  const advanced=await advanceFrontierByDirtySetV1({frontier:start,set:dirtySetFromRowsV1(scenario.rows,cursor),...ports});
  const fresh=createFrontierV1({...await walkFrontierV1(counting(root).ports),capturedAt:0});
  expect(advanced).toBeDefined();
  if(scenario.maximumChecked!==undefined)expect(advanced!.checked).toBeLessThanOrEqual(scenario.maximumChecked);
  if(scenario.unchanged){expect("unchanged" in advanced!).toBe(true);expect(normal(start)).toEqual(normal(fresh));return;}
  const after=advanced as Exclude<typeof advanced,{unchanged:true}|undefined>;
  expect(normal(after)).toEqual(normal(fresh));
  expect(after.changes).toEqual(diffFrontiersV1(start,{...fresh,capturedAt:0}));
  if(python)expect(names(after)).toEqual(python3("walk",root));
 });
 test("one edit among 3000 files costs its own paths: no listing, at most 4 stat calls, untouched entries are never read",async()=>{
  const root=sandbox("dirty-cost-"),files:Record<string,string>={};
  for(let d=0;d<60;d++)for(let f=0;f<50;f++)files[`p${d}/m${f}/project.json`]="{}";
  build(root,files);
  const{ports,calls}=counting(root),frontier=createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)});
  expect(frontier.members.length).toBe(3000);
  writeFileSync(join(root,"p7/m9/project.json"),"{\"edited\":true}");
  calls.stat=0;calls.list=0;
  const advanced=await advanceFrontierByDirtySetV1({frontier,set:dirtySetFromRowsV1([["m",0,"p7/m9/project.json"]],cursor),...ports});
  expect(advanced).toMatchObject({unchanged:true});expect(calls.list).toBe(0);expect(calls.stat).toBeLessThanOrEqual(4);expect(calls.stat).toBe((advanced as {checked:number}).checked);
  writeFileSync(join(root,"p7/m9/new.feature"),"n");
  calls.stat=0;
  const added:any=await advanceFrontierByDirtySetV1({frontier,set:dirtySetFromRowsV1([["a",0,"p7/m9/new.feature"]],cursor),...ports});
  expect(added.changes.map((c:any)=>c.kind+" "+c.path)).toContain("added p7/m9/new.feature");expect(calls.stat).toBeLessThanOrEqual(4);expect(calls.list).toBe(0);
 });
 test("cancellation stops an advance that walks a new subtree",async()=>{
  const root=sandbox("dirty-cancel-");build(root,{"a/project.json":"{}"});
  const{ports}=counting(root),frontier=createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)});
  build(root,{"a/new/x/project.json":"{}"});
  const controller=new AbortController();controller.abort(Error("stop"));
  await expect(advanceFrontierByDirtySetV1({frontier,set:dirtySetFromRowsV1([["a",1,"a/new"]],cursor),...ports,signal:controller.signal})).rejects.toThrow("stop");
 });
 test("seeded random edit sequences advance to exactly what a walk answers, batch after batch",async()=>{
  if(windows)return;
  for(const seed of[5,17,29]){
   const root=sandbox("dirty-random-");build(root,fixtures.tree);
   let state=seed;const random=()=>{state=(state*1664525+1013904223)>>>0;return state/4294967296;};
   const pick=<T,>(items:T[])=>items[Math.floor(random()*items.length)]!;
   const{ports}=counting(root);let frontier=createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)});
   const dirs=async()=>{const out=[""];const visit=async(directory:string)=>{for(const entry of await readdir(join(root,directory),{withFileTypes:true}))if(entry.isDirectory()){const path=directory?directory+"/"+entry.name:entry.name;out.push(path);await visit(path);}};await visit("");return out;};
   const leaves=["project.json","Cargo.toml","x.feature","other.txt"],dirNames=["a","b","c","node_modules","dist","d"];
   for(let batch=0;batch<40;batch++){
    const rows:any[]=[];
    for(let step=0;step<1+Math.floor(random()*4);step++){
     const here=pick(await dirs()),base=here?join(root,...here.split("/")):root,at=(name:string)=>here?here+"/"+name:name,kind=random();
     if(kind<0.35){const name=pick(leaves);const existed=existsSync(join(base,name));writeFileSync(join(base,name),"w"+batch+step);rows.push([existed?"m":"a",0,at(name)]);}
     else if(kind<0.5){const name=pick(dirNames)+"-n"+batch+step;mkdirSync(join(base,name,"s"),{recursive:true});writeFileSync(join(base,name,"s",pick(leaves)),"z");rows.push(["a",1,at(name)]);}
     else if(kind<0.7){const entries=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.name!=="loop");if(entries.length){const entry=pick(entries);rmSync(join(base,entry.name),{recursive:true});rows.push(["r",entry.isDirectory()?1:0,at(entry.name)]);}}
     else if(kind<0.85){const entries=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.name!=="loop");if(entries.length){const entry=pick(entries),to=entry.name+"-mv"+batch+step;renameSync(join(base,entry.name),join(base,to));rows.push(["n",entry.isDirectory()?1:0,at(entry.name)],["n",entry.isDirectory()?1:0,at(to)]);}}
     else{const name=pick(leaves);if(existsSync(join(base,name))){chmodSync(join(base,name),pick([0o644,0o755]));rows.push(["m",0,at(name)]);}}
    }
    const advanced:any=await advanceFrontierByDirtySetV1({frontier,set:dirtySetFromRowsV1(rows,cursor),...ports}),fresh=createFrontierV1({...await walkFrontierV1(counting(root).ports),capturedAt:0});
    expect(advanced).toBeDefined();
    const next=advanced.unchanged?frontier:createFrontierV1({...advanced,capturedAt:Date.now()});
    expect(normal(next)).toEqual(normal(fresh));
    if(!advanced.unchanged)expect(advanced.changes).toEqual(diffFrontiersV1(frontier,{...fresh,capturedAt:0}));
    frontier=next;
   }
   if(python)expect(names(frontier)).toEqual(python3("walk",root));
  }
 },60000);
});

const fakeJournal=(rows:{log:any[]}=({log:[]}))=>{
 const state={head:5,answers:[] as any[],cursors:0,since:0,lastFrom:undefined as any,excluded:(path:string)=>path.startsWith("ignored/")||path.startsWith(".cache/")||path.split("/").includes("node_modules")};
 const port={
  async cursor(){state.cursors++;return{instance:"0123456789abcdef",seq:state.head};},
  async since(from:any){state.since++;state.lastFrom=from;return state.answers.length?state.answers.shift():{complete:true,cursor:{instance:"0123456789abcdef",seq:state.head},rows:[]};},
  excluded:()=>state.excluded,
 };
 return{state,port};
};
const memoryStore=()=>{const map=new Map<string,any>();return{map,get:(key:string)=>map.get(key),set:(key:string,value:any,carry?:any)=>{map.set(key,carry?{...value,carry}:value);}};};

describe("host seams",()=>{
 const setup=async()=>{
  const root=sandbox("dirty-seams-");build(root,fixtures.tree);
  const{ports,calls}=counting(root),journal=fakeJournal(),store=memoryStore(),notices:any[]=[],scans={membership:0,holds:[] as string[][]},adopted:string[][]=[];
  const scan=async(previous:any)=>{scans.membership++;const frontier=createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)});return{digest:"digest-"+scans.membership,frontier,changes:[]};};
  const seams=journalGraphSeamsV1({root,journal:journal.port,store,ports,digest:members=>"members-"+members.length,membership:scan,holds:async rows=>{scans.holds.push(rows.map(row=>row.path));return true;},adopt:files=>adopted.push(files),notify:notice=>notices.push(notice)});
  return{root,ports,calls,journal,store,notices,scans,seams,adopted};
 };
 const abs=(root:string,path:string)=>root+"/"+path;
 test("the first generation scans (cold), commits its cursor once verified, and the next run is answered by the journal without a scan",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);
  expect(s.scans.membership).toBe(1);expect(s.notices).toEqual([]);
  s.seams.commit({generation:4,owners:["a"],covered:undefined});
  expect(s.store.map.get("g4")).toEqual({instance:"0123456789abcdef",seq:5});
  s.journal.state.head=9;s.calls.stat=0;s.calls.list=0;
  const second=await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
  expect(s.scans.membership).toBe(1);expect(second.digest).toBe(first.digest);expect(second.frontier).toBe(first.frontier);expect(second.changes).toEqual([]);
  expect(s.calls.stat).toBe(0);expect(s.journal.state.lastFrom).toEqual({instance:"0123456789abcdef",seq:5});expect(s.adopted.length).toBe(1);
 });
 test("a content edit is doubtful for the fact checks, everything else is trusted without being looked at, and the cursor stays until the generation is verified",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  const previous={manifest:{generation:4,membership:first.digest},frontier:first.frontier};
  s.journal.state.head=11;s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:11},rows:[["m",0,"a/x/src/main.ts"]]});
  writeFileSync(join(s.root,"a/x/src/main.ts"),"edited");
  await s.seams.membership(previous);
  const rows=[
   {kind:"file",path:abs(s.root,"a/x/src/main.ts"),fact:{}},{kind:"file",path:abs(s.root,"a/y/Cargo.toml"),fact:{}},{kind:"readdir",path:abs(s.root,"a/x/src"),fact:{}},{kind:"readdir",path:abs(s.root,"a/y"),fact:{}},
   {kind:"exists",path:abs(s.root,"b/z.feature"),fact:true},{kind:"file",path:"/usr/lib/outside.js",fact:{}},{kind:"resolve",path:abs(s.root,"b/z.feature"),fact:"x"},{kind:"file",path:abs(s.root,"ignored/q.ts"),fact:{}},
  ];
  expect(await s.seams.holds(rows)).toBe(true);
  expect(s.scans.holds).toEqual([[abs(s.root,"a/x/src/main.ts"),abs(s.root,"a/x/src"),"/usr/lib/outside.js",abs(s.root,"b/z.feature"),abs(s.root,"ignored/q.ts")]]);
  expect(s.seams.stats).toMatchObject({advanced:1,trusted:3,doubtful:5});
  s.journal.state.head=20;s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:20},rows:[["m",0,"a/x/src/main.ts"]]});
  await s.seams.membership(previous);
  expect(s.journal.state.lastFrom).toEqual({instance:"0123456789abcdef",seq:5});
  s.scans.holds.length=0;await s.seams.holds([rows[0]!,rows[1]!]);expect(s.scans.holds).toEqual([[abs(s.root,"a/x/src/main.ts")]]);
  s.seams.commit({generation:5,owners:["a"],covered:new Set(["a"])});
  expect(s.store.map.get("g5")).toEqual({instance:"0123456789abcdef",seq:20});
 });
 test("a scoped load does not move the verified cursor",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);
  s.seams.commit({generation:4,owners:["a","b"],covered:new Set(["a"])});
  expect(s.store.map.size).toBe(0);
  s.seams.commit({generation:4,owners:["a","b"],covered:new Set(["a","b"])});
  expect(s.store.map.size).toBe(1);expect(first.digest).toBe("digest-1");
 });
 test("membership changes carry config flags and rebuild the digest, directory-only changes keep it",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);s.seams.commit({generation:1,owners:[],covered:undefined});
  const previous={manifest:{generation:1,membership:first.digest},frontier:first.frontier};
  writeFileSync(join(s.root,"a/x/Cargo.toml"),"c");
  s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:6},rows:[["a",0,"a/x/Cargo.toml"]]});
  const live=await s.seams.membership(previous);
  expect(live.changes).toContainEqual({path:"a/x/Cargo.toml",kind:"added",directory:false,config:true});
  expect(live.digest).toBe("members-"+live.frontier.members.length);expect(s.scans.membership).toBe(1);
  mkdirSync(join(s.root,"b/empty"));
  s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:7},rows:[["a",1,"b/empty"]]});
  const dirOnly=await s.seams.membership({manifest:{generation:1,membership:first.digest},frontier:first.frontier});
  expect(dirOnly.digest).toBe(first.digest);expect(dirOnly.changes.every((change:any)=>change.directory)).toBe(true);
 });
 test("every verify epoch is announced with its reason and runs exactly one scan; a lost journal recovers after one scan",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);
  const previous={manifest:{generation:4,membership:first.digest},frontier:first.frontier};
  const second=await s.seams.membership(previous);
  expect(s.notices).toEqual([{reason:"unrecorded",generation:4}]);expect(s.scans.membership).toBe(2);
  expect(await s.seams.holds([{kind:"file",path:abs(s.root,"b/z.feature"),fact:{}}])).toBe(true);expect(s.scans.holds).toEqual([[abs(s.root,"b/z.feature")]]);
  s.seams.commit({generation:4,owners:[],covered:undefined});
  const recorded={manifest:{generation:4,membership:second.digest},frontier:second.frontier};
  s.journal.state.answers.push({complete:false,reason:"lost",path:"a",detail:"must-scan"});
  await s.seams.membership(recorded);
  expect(s.notices.at(-1)).toMatchObject({reason:"lost",path:"a",detail:"must-scan",generation:4});expect(s.scans.membership).toBe(3);
  s.seams.commit({generation:4,owners:[],covered:undefined});
  const scans=s.scans.membership;
  await s.seams.membership({manifest:{generation:4,membership:"digest-3"},frontier:second.frontier});
  expect(s.scans.membership).toBe(scans);expect(s.notices.length).toBe(2);expect(s.seams.stats.verified).toBe(2);
 });
 test("an advance the journal cannot explain, a disabled journal and an unreachable cursor each announce and scan",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);s.seams.commit({generation:2,owners:[],covered:undefined});
  s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:9},rows:[["a",1,""]]});
  await s.seams.membership({manifest:{generation:2,membership:first.digest},frontier:first.frontier});
  expect(s.notices.at(-1)).toMatchObject({reason:"outside"});expect(s.scans.membership).toBe(2);
  const quiet=journalGraphSeamsV1({root:s.root,journal:{cursor:async()=>undefined,since:async()=>({complete:false,reason:"disabled"}),excluded:()=>()=>false},store:memoryStore(),ports:s.ports,digest:()=>"d",membership:async()=>({digest:"x",frontier:first.frontier,changes:[]}),holds:async()=>false,notify:notice=>s.notices.push(notice)});
  const cold=await quiet.membership(undefined);quiet.commit({generation:1,owners:[],covered:undefined});
  expect(await quiet.holds([{kind:"file",path:"/x",fact:{}}])).toBe(false);
  await quiet.membership({manifest:{generation:1,membership:cold.digest},frontier:first.frontier});
  expect(s.notices.at(-1)).toMatchObject({reason:"unrecorded"});
 });
 test("resolve rows of absolute workspace specifiers are vouched by the journal, bare specifiers and outside results never",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:11},rows:[["m",0,"a/y/Cargo.toml"],["a",0,"b/created.feature"]]});
  await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
  const resolve=(spec:string,fact:unknown)=>({kind:"resolve",path:JSON.stringify([abs(s.root,"a/x/src/main.ts"),spec]),fact});
  const rows=[resolve(abs(s.root,"top/project.json"),abs(s.root,"top/project.json")),resolve(abs(s.root,"a/y/Cargo.toml"),abs(s.root,"a/y/Cargo.toml")),resolve(abs(s.root,"b/other"),false),resolve(abs(s.root,"b/created"),false),resolve("typescript",abs(s.root,"top/project.json")),resolve(abs(s.root,"top/project.json"),"/usr/lib/x.js"),{kind:"realpath",path:abs(s.root,"top/project.json"),fact:abs(s.root,"top/project.json")},{kind:"typescriptRealpath",path:abs(s.root,"a/y/Cargo.toml"),fact:abs(s.root,"a/y/Cargo.toml")}];
  expect(await s.seams.holds(rows as never)).toBe(true);
  expect(s.scans.holds.at(-1)).toEqual([rows[1],rows[2],rows[3],rows[4],rows[5],rows[7]].map(row=>row.path));
 });
 test("fact paths map to workspace-relative paths on both separators and never outside the workspace",()=>{
  expect(relativeFactPathV1("/w/r","/w/r/a/b.ts")).toBe("a/b.ts");expect(relativeFactPathV1("/w/r/","/w/r")).toBe("");expect(relativeFactPathV1("/w/r","/w/rx/a")).toBeUndefined();
  expect(relativeFactPathV1("C:\\w\\r","C:\\w\\r\\a\\b.ts")).toBe("a/b.ts");expect(relativeFactPathV1("/w/r","/etc/hosts")).toBeUndefined();
 });
});

describe("resuming a verify epoch",()=>{
 const setup2=async()=>{
  const root=sandbox("dirty-resume-");build(root,fixtures.tree);
  const{ports}=counting(root),journal=fakeJournal(),store=memoryStore(),notices:any[]=[],scans={membership:0,holds:[] as string[][]};
  const scan=async()=>{scans.membership++;const frontier=createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)});return{digest:"d"+scans.membership,frontier,changes:[{path:"a/x/new.ts",kind:"added",directory:false},{path:"b/dir",kind:"added",directory:true}]};};
  const failing=new Set<string>();
  const seams=journalGraphSeamsV1({root,journal:journal.port,store,ports,digest:members=>"m"+members.length,membership:scan,holds:async rows=>{scans.holds.push(rows.map(row=>row.path));return !rows.some(row=>failing.has(row.path));},notify:notice=>notices.push(notice)});
  return{root,ports,journal,store,notices,scans,seams,failing};
 };
 test("a full epoch commits at the end of the fact verification with the owed paths, a killed scan resumes from it, and a sealed load clears the carry",async()=>{
  const s=await setup2();
  const first=await s.seams.membership(undefined);s.seams.commit({generation:1,owners:["a"],covered:undefined});
  const previous={manifest:{generation:7,membership:first.digest},frontier:first.frontier};
  s.seams.scope(true);
  await s.seams.membership(previous);
  expect(s.notices.at(-1)).toMatchObject({reason:"unrecorded"});
  s.failing.add(s.root+"/a/x/src/main.ts");
  expect(await s.seams.holds([{kind:"file",path:s.root+"/a/x/src/main.ts",fact:{}}])).toBe(false);
  s.seams.factsVerified();
  const entry=s.store.map.get("g7");
  expect(entry).toMatchObject({instance:"0123456789abcdef",seq:5});
  expect(entry.carry.exact).toEqual(expect.arrayContaining(["a/x/src/main.ts","a/x/new.ts","b/dir"]));expect(entry.carry.subtrees).toEqual(["b/dir"]);
  const scans=s.scans.membership;s.scans.holds.length=0;s.failing.clear();
  const resumed=await s.seams.membership(previous);
  expect(s.scans.membership).toBe(scans);expect(s.notices.length).toBe(1);expect(s.seams.stats.resumed).toBe(1);
  await s.seams.holds([{kind:"file",path:s.root+"/a/x/src/main.ts",fact:{}},{kind:"file",path:s.root+"/a/y/Cargo.toml",fact:{}}]);
  expect(s.scans.holds).toEqual([[s.root+"/a/x/src/main.ts"]]);
  void resumed;
  s.seams.commit({generation:8,owners:["a"],covered:undefined});
  expect(s.store.map.get("g8").carry).toBeUndefined();
 });
 test("an incomplete scan commits nothing; a scoped load carries the cursor and the carry to its new generation; a generation built from another finds the cursor of its base",async()=>{
  const s=await setup2();
  const first=await s.seams.membership(undefined);
  const previous={manifest:{generation:3,membership:first.digest},frontier:first.frontier};
  s.seams.scope(false);await s.seams.membership(previous);s.seams.factsVerified();
  expect(s.store.map.size).toBe(0);
  s.store.set("g3",{instance:"0123456789abcdef",seq:2},{exact:["q"],subtrees:[]});
  s.journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:6},rows:[]});
  s.seams.scope(false);
  await s.seams.membership({manifest:{generation:4,membership:first.digest,built:{kind:"delta",base:3}},frontier:first.frontier});
  expect(s.journal.state.lastFrom).toEqual({instance:"0123456789abcdef",seq:2});
  s.seams.commit({generation:4,owners:["a","b"],covered:new Set(["a"])});
  expect(s.store.map.get("g4")).toEqual({instance:"0123456789abcdef",seq:2,carry:{exact:["q"],subtrees:[]}});
 });
 test("a scan with more owed paths than the bound commits nothing",async()=>{
  const s=await setup2();
  const first=await s.seams.membership(undefined);
  const seams=journalGraphSeamsV1({root:s.root,journal:s.journal.port,store:s.store,ports:s.ports,digest:()=>"d",maximumCarry:1,membership:async()=>({digest:"x",frontier:first.frontier,changes:[{path:"a",kind:"added",directory:false},{path:"b/c",kind:"added",directory:false}]}),holds:async()=>true});
  seams.scope(true);await seams.membership({manifest:{generation:1,membership:"d"},frontier:first.frontier});seams.factsVerified();
  expect(s.store.map.size).toBe(0);
 });
});

describe("dependency install receipt",()=>{
 const run=async(entryInstall:string|undefined,currentInstall:string|undefined,rows:any[],dirtyRows:any[]=[])=>{
  const root=sandbox("dirty-install-");build(root,fixtures.tree);
  const{ports}=counting(root),journal=fakeJournal(),store=memoryStore(),checked:string[]=[];
  const seams=journalGraphSeamsV1({root,journal:journal.port,store,ports,digest:()=>"d",install:async()=>currentInstall,membership:async()=>({digest:"d1",frontier:createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(ports)}),changes:[]}),holds:async list=>{checked.push(...list.map((row:any)=>row.path));return true;}});
  const first=await seams.membership(undefined);store.set("g1",{instance:"0123456789abcdef",seq:1,...(entryInstall?{install:entryInstall}:{})} as never);
  journal.state.answers.push({complete:true,cursor:{instance:"0123456789abcdef",seq:2},rows:dirtyRows});
  await seams.membership({manifest:{generation:1,membership:first.digest},frontier:first.frontier});
  await seams.holds(rows.map(row=>({...row,path:row.path.replace("@ROOT",root),fact:typeof row.fact==="string"?row.fact.replace("@ROOT",root):row.fact})));
  return{checked:checked.map(path=>path.replaceAll(root,"@ROOT")),seams,root,store};
 };
 const f="a".repeat(64);
 const bare=(spec:string,fact:string)=>({kind:"resolve",path:JSON.stringify(["@ROOT/a/x/src/main.ts",spec]),fact});
 const rows=[bare("typescript","@ROOT/.cache/b/d/m/j/id1/node_modules/typescript/lib/typescript.js"),{kind:"exists",path:"@ROOT/node_modules/q/package.json",fact:true},{kind:"lstat",path:"@ROOT/.cache/b/d/m/j/id1/node_modules/q",fact:"directory"},bare("@scope/workspace","@ROOT/top/project.json"),{kind:"file",path:"@ROOT/a/x/src/main.ts",fact:{}}];
 test("with an unchanged install fingerprint, bare resolutions and node_modules rows are vouched; workspace results still follow the dirty set",async()=>{
  const result=await run(f,f,rows);
  expect(result.checked).toEqual([]);
 });
 test("a moved fingerprint, a missing recorded one or none at all makes them all doubtful again",async()=>{
  for(const[entry,current]of[[f,"b".repeat(64)],[undefined,f],[f,undefined]] as const){
   const result=await run(entry,current,rows);
   expect(result.checked.sort()).toEqual(["@ROOT/.cache/b/d/m/j/id1/node_modules/q","@ROOT/node_modules/q/package.json",JSON.stringify(["@ROOT/a/x/src/main.ts","@scope/workspace"]),JSON.stringify(["@ROOT/a/x/src/main.ts","typescript"])].sort());
  }
 });
 test("a configuration file on the way, or a dirty workspace result, doubts a bare resolution even with a matching fingerprint",async()=>{
  const config=await run(f,f,rows,[["m",0,"a/tsconfig.json"]]);
  expect(config.checked.sort()).toEqual([JSON.stringify(["@ROOT/a/x/src/main.ts","@scope/workspace"]),JSON.stringify(["@ROOT/a/x/src/main.ts","typescript"])].sort());
  const moved=await run(f,f,rows,[["m",0,"top/project.json"]]);
  expect(moved.checked).toEqual([JSON.stringify(["@ROOT/a/x/src/main.ts","@scope/workspace"])]);
 });
 test("the fingerprint is the install root and the lockfile digest and nothing else",()=>{
  const base=sandbox("dirty-fingerprint-"),one=join(base,"install-1"),two=join(base,"install-2");mkdirSync(one);mkdirSync(two);mkdirSync(join(base,"w"));
  expect(installFingerprintV1(join(base,"w"))).toBeUndefined();
  writeFileSync(join(base,"w","bun.lock"),"lock-1");
  expect(installFingerprintV1(join(base,"w"))).toBeUndefined();
  symlinkSync(one,join(base,"w","node_modules"),"dir");
  const first=installFingerprintV1(join(base,"w"));expect(first).toMatch(/^[0-9a-f]{64}$/u);expect(installFingerprintV1(join(base,"w"))).toBe(first);
  writeFileSync(join(base,"w","other.txt"),"x");writeFileSync(join(one,"inside"),"x");expect(installFingerprintV1(join(base,"w"))).toBe(first);
  writeFileSync(join(base,"w","bun.lock"),"lock-2");const second=installFingerprintV1(join(base,"w"));expect(second).not.toBe(first);
  rmSync(join(base,"w","node_modules"));symlinkSync(two,join(base,"w","node_modules"),"dir");expect(installFingerprintV1(join(base,"w"))).not.toBe(second);
 });
});

describe("cursor store",()=>{
 test("keeps the newest entries, survives torn and foreign files and writes atomically",()=>{
  const directory=sandbox("dirty-store-"),store=fileCursorStoreV1(directory,3);
  expect(store.get("a")).toBeUndefined();
  for(let index=0;index<5;index++)store.set("g"+index,{instance:"0123456789abcdef",seq:index});
  expect(store.get("g0")).toBeUndefined();expect(store.get("g1")).toBeUndefined();expect(store.get("g4")).toEqual({instance:"0123456789abcdef",seq:4});
  store.set("g3",{instance:"fedcba9876543210",seq:99});expect(store.get("g3")).toEqual({instance:"fedcba9876543210",seq:99});
  const file=join(directory,GRAPH_HOOKS_POLICY_V1.cursorFile);
  writeFileSync(file,"{torn");expect(store.get("g4")).toBeUndefined();
  writeFileSync(file,JSON.stringify({version:2,entries:[["g4","0123456789abcdef",1]]}));expect(store.get("g4")).toBeUndefined();
  writeFileSync(file,JSON.stringify({version:1,entries:[["g4","nothex",1],["g5","0123456789abcdef",-1]]}));expect(store.get("g4")).toBeUndefined();expect(store.get("g5")).toBeUndefined();
  store.set("c1",{instance:"0123456789abcdef",seq:3},{exact:["a/b"],subtrees:["d"]});expect(store.get("c1")).toEqual({instance:"0123456789abcdef",seq:3,carry:{exact:["a/b"],subtrees:["d"]}});
  writeFileSync(file,JSON.stringify({version:1,entries:[["c2","0123456789abcdef",1,{exact:[1],subtrees:[]}]]}));expect(store.get("c2")).toBeUndefined();
  store.set("g6",{instance:"0123456789abcdef",seq:1});expect(store.get("g6")).toEqual({instance:"0123456789abcdef",seq:1});
  expect(readdirSyncNames(directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 });
});
function readdirSyncNames(directory:string):string[]{return require("node:fs").readdirSync(directory);}
