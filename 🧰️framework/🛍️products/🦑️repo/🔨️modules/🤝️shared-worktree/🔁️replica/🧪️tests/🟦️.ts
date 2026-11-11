import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {appendFileSync,chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {blobPathV1,foldEventsV1,formatHlcV1,readEventsV1,readHeadV1,readNewestSnapshotV1,type FleetEventV1,type FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";
import {closeReplicaV1,CompactTableV1,compactV1,conflictWinnerV1,followOnceV1,foldStoreV1,homedOriginsV1,joinV1,JoinRefusedV1,openReplicaV1,publishOnceV1,relayOnceV1,relayV1,ReplicaBookV1,scanDelayV1,ReplicaBusyV1,ReplicaFoldsV1,requireReplicaShapeV1,runV1,seedV1,snapshotV1,statusV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),python=testPythonV1(),scratch:string[]=[];
const oracle=join(import.meta.dirname,"🐍️.py"),fixturePath=join(import.meta.dirname,"🧫️fixtures","🔣️.json"),posixTest=process.platform==="win32"?test.skip:test;
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0};
let clockTick=1_700_000_000;

function machine(name:string,store:string,group="g1"):Machine{return {name,events:[],options:{worktree:temp(name),storeRoot:store,originId:name,groupId:group,...fast}};}
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
function write(who:Machine,path:string,text:string,future:boolean=false):void{
 const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);
 const stamp=future?Math.floor(Date.now()/1000)+3600:clockTick++;
 utimesSync(file,stamp,stamp);
}
const read=(who:Machine,path:string):string|undefined=>existsSync(at(who,path))?readFileSync(at(who,path),"utf8"):undefined;
async function open(who:Machine):Promise<ReplicaV1>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));return who.replica;}
async function shut(who:Machine):Promise<void>{if(who.replica){await closeReplicaV1(who.replica);who.replica=undefined;}}
const publish=(who:Machine)=>publishOnceV1(who.replica!);
const follow=(who:Machine)=>followOnceV1(who.replica!);
async function settle(machines:readonly Machine[]):Promise<void>{for(let round=0;round<3;round++){for(const who of machines)await publish(who);for(const who of machines)await follow(who);}}
async function fleet(names:readonly string[],store:string=temp("store")){
 const machines=names.map(name=>machine(name,store));
 write(machines[0]!,".gitignore",fixture.gitignore);
 await seedV1(machines[0]!.options);
 for(const other of machines.slice(1))await joinV1(other.options);
 for(const who of machines)await open(who);
 return {store,machines};
}
const walk=(directory:string):string[]=>existsSync(directory)?readdirSync(directory,{withFileTypes:true}).flatMap(item=>item.isDirectory()?walk(join(directory,item.name)):[join(directory,item.name)]):[];
const hidden=(path:string)=>path.split("/").some(part=>part===".cache"||part===".git"||part==="ignored"||part.endsWith(".log"));
function shared(who:Machine):Record<string,string>{
 const root=who.options.worktree,rows:Record<string,string>={};
 for(const file of walk(root)){const path=file.slice(root.length+1).split("\\").join("/");if(!hidden(path))rows[path]=lstatSync(file).isSymbolicLink()?`-> ${readlinkSync(file)}`:readFileSync(file,"utf8");}
 return rows;
}
const lostFiles=(who:Machine)=>walk(join(who.options.worktree,".cache","f","lost"));
async function until(condition:()=>boolean,milliseconds=15000):Promise<void>{
 const end=Date.now()+milliseconds;
 while(!condition()){if(Date.now()>end)throw Error("Timed out waiting for the replica");await Bun.sleep(20);}
}
function oracleCompare(store:string,who:Machine):{status:number|null;report:{ok:boolean;drift:{path:string;kind:string}[]}}{
 const result=Bun.spawnSync([python!,"-I",oracle,"compare",store,who.options.worktree,...fixture.ignorePatterns],{stdout:"pipe",stderr:"pipe"});
 return {status:result.exitCode,report:JSON.parse(result.stdout.toString())};
}

type Step={machine:string;op:"write"|"remove"|"publish"|"follow"|"settle";path?:string;text?:string;mtime?:"future"};
type Scenario={name:string;machines:string[];steps:Step[];expect:{tree?:Record<string,string>;absent?:string[];absentOn?:string[];absentFromStore?:string[];converged?:string[];lost?:Record<string,number>;lostTexts?:Record<string,string[]>;keptOn?:string[];silent?:string[]}};

for(const scenario of fixture.scenarios as unknown as Scenario[])test(`scenario ${scenario.name} converges and agrees with the Python oracle`,async()=>{
 const {store,machines}=await fleet(scenario.machines),by=new Map(machines.map(who=>[who.name,who] as const));
 try{
  for(const step of scenario.steps){
   if(step.op==="settle"){await settle(machines);continue;}
   const who=by.get(step.machine)!;
   if(step.op==="write")write(who,step.path!,step.text!,step.mtime==="future");
   else if(step.op==="remove")rmSync(at(who,step.path!));
   else if(step.op==="publish")await publish(who);
   else await follow(who);
  }
  const wanted=scenario.expect;
  for(const [path,text] of Object.entries(wanted.tree??{}))for(const who of machines)expect(read(who,path)).toBe(text);
  for(const path of wanted.absent??[])for(const name of wanted.absentOn??scenario.machines)expect(existsSync(at(by.get(name)!,path))).toBe(false);
  if(wanted.converged!==undefined){const reference=shared(by.get(wanted.converged[0]!)!);for(const name of wanted.converged)expect(shared(by.get(name)!)).toEqual(reference);}
  for(const [name,count] of Object.entries(wanted.lost??{}))expect(lostFiles(by.get(name)!)).toHaveLength(count);
  for(const [name,texts] of Object.entries(wanted.lostTexts??{}))expect(lostFiles(by.get(name)!).map(file=>readFileSync(file,"utf8")).sort()).toEqual([...texts].sort());
  for(const name of wanted.keptOn??[])expect(by.get(name)!.events.some(event=>event.event==="kept")).toBe(true);
  for(const name of wanted.silent??[])expect(await readHeadV1(store,name)).toBeUndefined();
  const journal=walk(store).filter(file=>file.includes("origins")).map(file=>readFileSync(file,"utf8")).join("\n"),names=walk(store).join("\n");
  for(const needle of wanted.absentFromStore??[]){expect(journal).not.toContain(needle);expect(names).not.toContain(needle);}
  if(python)for(const who of machines){const {status,report}=oracleCompare(store,who);expect({machine:who.name,status,drift:report.drift}).toEqual({machine:who.name,status:0,drift:[]});}
 }finally{for(const who of machines)await shut(who);}
},60000);

test("fold vectors give the same manifest in TypeScript and in the Python oracle",()=>{
 for(const vector of fixture.folds){
  const manifest=new Map<string,FleetManifestEntryV1>();
  foldEventsV1(manifest,vector.events as unknown as FleetEventV1[]);
  expect({vector:vector.name,manifest:Object.fromEntries(manifest)}).toEqual({vector:vector.name,manifest:vector.manifest as unknown as Record<string,FleetManifestEntryV1>});
 }
 if(python){const result=Bun.spawnSync([python,"-I",oracle,"fold",fixturePath],{stdout:"pipe",stderr:"pipe"});expect(result.exitCode).toBe(0);}
});

test("conflict vectors give the same winner in TypeScript and in the Python oracle",()=>{
 for(const vector of fixture.conflicts)expect({vector:vector.name,winner:conflictWinnerV1(vector.localMtimeMs??undefined,vector.remoteHlc) as string}).toEqual({vector:vector.name,winner:vector.winner});
 if(python){const result=Bun.spawnSync([python,"-I",oracle,"conflicts",fixturePath],{stdout:"pipe",stderr:"pipe"});expect(result.exitCode).toBe(0);}
});

test("a newer offline edit is published once with a greater hlc, the loser is not echoed and further rounds are silent",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"x.txt","base");await publish(a);await follow(b);
 write(b,"x.txt","b newer",true);write(a,"x.txt","a older");await publish(a);
 const remote=(await readHeadV1(store,"a"))!.hlc;
 expect(await follow(b)).toMatchObject({kept:1,applied:0,queued:0});
 const kept=b.events.find(event=>event.event==="kept")!;
 expect(kept).toMatchObject({path:"x.txt",reason:"newer-than-remote"});
 expect(validator("ProgressEvent")(JSON.parse(JSON.stringify(kept)))).toBe(true);
 const events=(await readEventsV1(store,"b",undefined)).events;
 expect(events).toHaveLength(1);
 expect(events[0]).toMatchObject({operation:"put",path:"x.txt",base:createHash("sha256").update("a older").digest("hex")});
 expect(events[0]!.hlc>remote).toBe(true);
 expect(b.events.some(event=>event.event==="overwritten")).toBe(false);
 const heads=async()=>JSON.stringify([await readHeadV1(store,"a"),await readHeadV1(store,"b")]);
 const before=await heads();
 await settle(machines);await settle(machines);
 expect(await heads()).toBe(before);
 expect(read(a,"x.txt")).toBe("b newer");expect(read(b,"x.txt")).toBe("b newer");
 expect(lostFiles(a)).toHaveLength(0);expect(lostFiles(b)).toHaveLength(0);
});

test("a local delete has no modification time: the remote put wins, the file comes back and nothing is published",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"y.txt","base");await publish(a);await follow(b);
 rmSync(at(b,"y.txt"));write(a,"y.txt","a again");await publish(a);
 expect(await follow(b)).toMatchObject({applied:1,kept:0});
 expect(read(b,"y.txt")).toBe("a again");
 expect(await publish(b)).toMatchObject({published:0,deleted:0});
 expect(await readHeadV1(store,"b")).toBeUndefined();
});

test("a restarted daemon publishes an offline edit that is newer than what the other machine published meanwhile",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"d.txt","base");await publish(a);await follow(b);await shut(b);
 write(a,"d.txt","a while b was down");await publish(a);
 write(b,"d.txt","b offline newer",true);
 const controller=new AbortController(),events:ReplicaEventV1[]=[],running=runV1(b.options,controller.signal,event=>events.push(event));
 await until(()=>events.some(event=>event.event==="kept"));
 await until(()=>existsSync(join(store,"v1","origins","b","head.json")));
 controller.abort();await running;
 await follow(a);
 expect(read(a,"d.txt")).toBe("b offline newer");expect(read(b,"d.txt")).toBe("b offline newer");
 expect(lostFiles(a)).toHaveLength(0);expect(lostFiles(b)).toHaveLength(0);
},60000);

test("the path book interns many unicode paths exactly once across pages and rejects lone surrogates",()=>{
 const book=new ReplicaBookV1(),paths=Array.from({length:12000},(_,index)=>`dir-${index%37}/🧬️ é/${"ü".repeat(index%300)}${index}.txt`);
 const ids=paths.map(path=>book.intern(path));
 expect(new Set(ids).size).toBe(12000);expect(book.count).toBe(12000);
 for(const [index,path] of paths.entries()){expect(book.find(path)).toBe(ids[index]);expect(book.name(ids[index]!)).toBe(path);}
 expect(paths.reduce((sum,path)=>sum+Buffer.byteLength(path),0)).toBeGreaterThan(2*1024*1024);
 expect(book.intern(paths[7]!)).toBe(ids[7]);expect(book.find("absent/path")).toBeUndefined();
 expect(()=>book.intern("bad\ud800.txt")).toThrow("Closed replica path required");
 expect(book.count).toBe(12000);
});

test("the compact tables behave like maps, keep sha256 as 32 bytes and fold case without a copy of every path",()=>{
 const book=new ReplicaBookV1(),files=new CompactTableV1(book,true),manifest=new CompactTableV1(book,false);
 const put={hlc:formatHlcV1({ms:1731000000000,counter:2},"alpha"),operation:"put" as const,sha256:"ab".repeat(32),bytes:5,executable:true};
 const link={hlc:formatHlcV1({ms:1731000000001,counter:0},"beta"),operation:"link" as const,target:"é/target"};
 const gone={hlc:formatHlcV1({ms:1731000000002,counter:0},"alpha"),operation:"delete" as const};
 files.set("é/🧬️.txt",{...put,mtimeMs:12.5,seenMs:20}).set("l",{...link,mtimeMs:3}).set("d",{...gone,mtimeMs:0});
 expect(files.size).toBe(3);expect(files.get("é/🧬️.txt")).toEqual({...put,mtimeMs:12.5,seenMs:20});
 expect(files.get("l")).toEqual({...link,mtimeMs:3});expect(files.get("d")).toEqual({...gone,mtimeMs:0});
 expect(files.get("nope")).toBeUndefined();expect(files.has("d")).toBe(true);
 expect(files.sha.length).toBeGreaterThanOrEqual(3*32);expect(files.count("delete")).toBe(1);
 expect([...files.keys()]).toEqual(["é/🧬️.txt","l","d"]);expect([...files].map(([path])=>path)).toEqual(["é/🧬️.txt","l","d"]);
 files.set("l",{...put,hlc:link.hlc,mtimeMs:1});expect(files.get("l")).toEqual({...put,hlc:link.hlc,mtimeMs:1});expect(files.targets.size).toBe(0);
 expect(files.get("l")?.seenMs).toBeUndefined();
 manifest.copyRow(files,book.find("é/🧬️.txt")!);expect(manifest.get("é/🧬️.txt")).toEqual(put as never);expect(manifest.sameHlcAs(files,book.find("é/🧬️.txt")!)).toBe(true);
 expect(files.delete("l")).toBe(true);expect(files.delete("l")).toBe(false);expect(files.size).toBe(2);
 files.clear();expect(files.size).toBe(0);expect([...files]).toEqual([]);
 expect(()=>manifest.set("bad",{...put,sha256:"zz"})).toThrow("Closed replica entry required");
 expect(()=>manifest.set("bad",{...put,hlc:"1731-1-a"})).toThrow("Closed replica hlc required");
 const folds=new ReplicaFoldsV1(book);
 for(const path of ["Docs/Read.md","docs/other.md"])folds.add(book.intern(path));
 expect(folds.find("DOCS/READ.MD")).toBe("Docs/Read.md");expect(folds.find("docs/read.md")).toBe("Docs/Read.md");expect(folds.find("docs/none.md")).toBeUndefined();
 folds.remove(book.find("Docs/Read.md")!);expect(folds.find("docs/read.md")).toBeUndefined();expect(folds.find("DOCS/OTHER.md")).toBe("docs/other.md");
});

test("a directory that was renamed and recreated under the old name publishes the deletions of the files that left it, even when the watcher reports only a change of the directory",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"keep/one.txt","1");write(a,"keep/sub/two.txt","2");write(a,"keep/three.txt","3");
 await settle([a,b]);
 expect(read(b,"keep/sub/two.txt")).toBe("2");
 renameSync(at(a,"keep"),at(a,"moved"));
 write(a,"keep/three.txt","3");write(a,"keep/four.txt","4");
 await publishOnceV1(a.replica!,undefined,["keep","moved","keep/four.txt"]);
 await follow(b);
 expect(shared(b)).toEqual(shared(a));
 expect(read(b,"keep/one.txt")).toBeUndefined();expect(read(b,"keep/sub/two.txt")).toBeUndefined();
 expect(read(b,"moved/sub/two.txt")).toBe("2");expect(read(b,"keep/four.txt")).toBe("4");
 for(const who of machines)await shut(who);
},60000);

test("a case-only rename is published as one put and one delete, from a watcher event and from a full scan, so that no machine ends without the file (found on Linux and on macOS next to each other)",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine],docs=(who:Machine)=>Object.fromEntries(Object.entries(shared(who)).filter(([path])=>path.startsWith("docs/")));
 write(a,"docs/readme.txt","one");
 await settle([a,b]);
 expect(shared(b)["docs/readme.txt"]).toBe("one");
 renameSync(at(a,"docs/readme.txt"),at(a,"docs/README.txt"));
 const touched=await publishOnceV1(a.replica!,undefined,["docs/readme.txt","docs/README.txt"]);
 expect([touched.published,touched.deleted]).toEqual([1,1]);
 await settle([a,b]);
 expect(docs(a)).toEqual({"docs/README.txt":"one"});
 expect(docs(b)).toEqual({"docs/README.txt":"one"});
 renameSync(at(a,"docs/README.txt"),at(a,"docs/Readme.txt"));
 const walked=await publishOnceV1(a.replica!);
 expect([walked.published,walked.deleted]).toEqual([1,1]);
 await settle([a,b]);
 expect(docs(b)).toEqual({"docs/Readme.txt":"one"});
 expect(oracleCompare(machines[0]!.options.storeRoot,b).report.ok).toBe(true);
 for(const who of machines)await shut(who);
},60000);

test("a store that vanished during a follow queues the work silently instead of reporting every blob as missing",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"late.txt","payload");await publish(a);
 const sha=createHash("sha256").update("payload").digest("hex");
 renameSync(blobPathV1(store,sha),join(store,"blob.away"));
 await follow(b);
 expect(b.events.filter(event=>event.event==="blob-missing")).toHaveLength(1);
 await shut(b);b.events.length=0;
 renameSync(join(store,"v1"),join(store,"away"));
 await open(b);await follow(b);
 expect(b.events.filter(event=>event.event==="blob-missing")).toHaveLength(0);
 renameSync(join(store,"away"),join(store,"v1"));
 renameSync(join(store,"blob.away"),blobPathV1(store,sha));
 await follow(b);
 expect(read(b,"late.txt")).toBe("payload");
 for(const who of machines)await shut(who);
},60000);

test("the safety-net scan interval is spread over 75 to 125 percent so that machines started together do not scan together",()=>{
 expect(scanDelayV1(900,0)).toBe(675_000);
 expect(scanDelayV1(900,0.5)).toBe(900_000);
 expect(scanDelayV1(900,0.999999)).toBeLessThanOrEqual(1_125_000);
 const spread=new Set(Array.from({length:50},()=>Math.floor(scanDelayV1(900)/10_000)));
 expect(spread.size).toBeGreaterThan(10);
});

test("events the own journal holds but the state never recorded (a crash between append and persist) are replayed, so a file that left the disk meanwhile is deleted everywhere",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 await shut(a);
 const cache=join(a.options.worktree,".cache","f"),saved=new Map(["state.json","state.log","cursor.json"].map(name=>[name,existsSync(join(cache,name))?readFileSync(join(cache,name)):undefined] as const));
 await open(a);
 write(a,"crash/x.txt","published, then the daemon dies");write(a,"crash/y.txt","stays");
 await publish(a);await follow(b);
 expect(read(b,"crash/x.txt")).toBe("published, then the daemon dies");
 await shut(a);
 for(const [name,data] of saved)if(data===undefined)rmSync(join(cache,name),{force:true});else writeFileSync(join(cache,name),data);
 rmSync(at(a,"crash/x.txt"));
 await open(a);
 await publish(a);await follow(b);
 expect(read(b,"crash/x.txt")).toBeUndefined();expect(read(b,"crash/y.txt")).toBe("stays");
 expect(shared(b)).toEqual(shared(a));
 for(const who of machines)await shut(who);
},60000);

test("the folds table counts the live paths below every directory, in step with adds and removes",()=>{
 const book=new ReplicaBookV1(),folds=new ReplicaFoldsV1(book);
 const ids=["a/b/one.txt","a/b/two.txt","a/c/three.txt","top.txt"].map(path=>book.intern(path));
 for(const id of ids){folds.add(id);folds.add(id);}
 expect([folds.below("a"),folds.below("a/b"),folds.below("a/c"),folds.below("zzz")]).toEqual([3,2,1,0]);
 folds.remove(ids[0]!);folds.remove(ids[0]!);
 expect([folds.below("a"),folds.below("a/b")]).toEqual([2,1]);
 folds.add(ids[0]!);
 expect(folds.below("a/b")).toBe(2);
});

test("a follow does not echo: the observer publishes nothing for what the follower materialised",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"echo.txt","once");await publish(a);
 const followed=await follow(b);
 expect(followed.applied).toBe(1);expect(read(b,"echo.txt")).toBe("once");
 const again=await publish(b);
 expect(again).toMatchObject({published:0,deleted:0,retry:[]});
 expect(await readHeadV1(store,"b")).toBeUndefined();
 const partial=await publishOnceV1(b.replica!,undefined,["echo.txt"]);
 expect(partial).toMatchObject({published:0,deleted:0});
});

test("state, cursors and clock survive a restart and are closed documents",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"one.txt","1");await publish(a);await follow(b);await shut(b);
 const directory=join(b.options.worktree,".cache","f");
 requireReplicaShapeV1("Machine",JSON.parse(readFileSync(join(directory,"machine.json"),"utf8")),"machine");
 for(const [definition,file] of [["State","state.json"],["Cursor","cursor.json"],["Clock","clock.json"],["Machine","machine.json"]] as const)expect(validator(definition)(JSON.parse(readFileSync(join(directory,file),"utf8")))).toBe(true);
 await open(b);
 expect(await publish(b)).toMatchObject({published:0,deleted:0});
 write(a,"two.txt","2");await publish(a);
 expect(await follow(b)).toMatchObject({events:1,applied:1});
 expect(read(b,"two.txt")).toBe("2");
});

test("the loser of a concurrent edit is kept under lost/<hlc>/<path> and reported as overwritten",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"c.txt","base text");await publish(a);await follow(b);
 write(a,"c.txt","A wins");write(b,"c.txt","B loses");
 await publish(a);await follow(b);
 const overwritten=b.events.find(event=>event.event==="overwritten");
 expect(overwritten?.path).toBe("c.txt");
 expect(readFileSync(join(b.options.worktree,".cache","f",overwritten!.lost!),"utf8")).toBe("B loses");
 expect(read(b,"c.txt")).toBe("A wins");
 expect((await publish(b)).published).toBe(0);
});

test("a .gitignore change is honoured by publish and by the next scan",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,".gitignore",`${fixture.gitignore}*.tmp\n`);await publish(a);await follow(b);
 write(b,"scratch.tmp","local only");
 expect((await publish(b)).published).toBe(0);
 expect(b.events.some(event=>event.event==="ignore-reloaded")).toBe(true);
 expect(await readHeadV1(store,"b")).toBeUndefined();
 write(a,".gitignore",fixture.gitignore);await publish(a);await follow(b);
 expect((await publish(b)).published).toBe(1);
 await follow(a);
 expect(read(a,"scratch.tmp")).toBe("local only");
});

test("a join starts from the newest snapshot, replays the journal after it and refuses an unrelated worktree",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 write(a,"keep.txt","keep");write(a,"gone.txt","gone");write(a,"dir/x.txt","x");await publish(a);
 const snapshot=await snapshotV1(store,"a");
 expect(snapshot?.created).toBe(true);
 expect((await readNewestSnapshotV1(store))?.manifest.size).toBe(4);
 rmSync(at(a,"gone.txt"));write(a,"keep.txt","keep v2");write(a,"late.txt","late");await publish(a);
 const c=machine("c",store),progress:ReplicaEventV1[]=[];
 await joinV1(c.options,undefined,event=>progress.push(event));
 expect(progress.some(event=>event.event==="progress"&&event.stage==="join"&&event.done===event.total)).toBe(true);
 await open(c);
 expect(shared(c)).toEqual(shared(a));
 expect(read(c,"gone.txt")).toBeUndefined();
 await shut(c);
 expect(await verifyV1(c.options)).toMatchObject({ok:true,drift:[]});
 const d=machine("d",store);
 write(d,"keep.txt","something else");
 await expect(joinV1(d.options)).rejects.toBeInstanceOf(JoinRefusedV1);
 write(d,"keep.txt","keep v2");
 const joined=await joinV1(d.options);
 expect(joined.adopted).toBe(1);expect(read(d,"late.txt")).toBe("late");
});

test("compaction snapshots, drops unreferenced old blobs only when asked to (group servers) and leaves a store that can still be joined",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 write(a,"f.txt","first version");await publish(a);write(a,"f.txt","second version, longer");await publish(a);
 const later=()=>Date.now()+60000,blobCount=()=>walk(join(store,"v1","blobs")).length;
 const before=blobCount(),skipped=await compactV1(store,"a",0,undefined,()=>{},later);
 expect(skipped.snapshot).toBeDefined();expect(skipped.blobs).toBe(0);expect(blobCount()).toBe(before);
 const result=await compactV1(store,"a",0,undefined,()=>{},later,{collectBlobs:true});
 expect(result.snapshot).toBeDefined();expect(result.blobs).toBeGreaterThanOrEqual(1);expect(blobCount()).toBe(before-result.blobs);
 const b=machine("b",store);await joinV1(b.options);await open(b);
 expect(read(b,"f.txt")).toBe("second version, longer");
 const kept=await compactV1(store,"a",14);
 expect(kept.blobs).toBe(0);
});

test("relay carries two groups' journals to each other and three machines converge",async()=>{
 const s1=temp("store1"),s2=temp("store2"),a=machine("a",s1,"g1"),b=machine("b",s1,"g1"),c=machine("c",s2,"g2");
 write(a,".gitignore",fixture.gitignore);await seedV1(a.options);await joinV1(b.options);
 const pull=async(from:string,group:string,into:string)=>relayOnceV1(into,from,await homedOriginsV1(from,group));
 await pull(s1,"g1",s2);
 await joinV1(c.options);
 for(const who of [a,b,c])await open(who);
 write(a,"x.txt","from a");await publish(a);
 const first=await pull(s1,"g1",s2);
 expect(first.origins).toBe(1);
 await follow(c);
 expect(read(c,"x.txt")).toBe("from a");
 write(c,"y.txt","from c");await publish(c);
 await pull(s2,"g2",s1);
 await follow(a);await follow(b);
 expect(read(a,"y.txt")).toBe("from c");expect(read(b,"y.txt")).toBe("from c");expect(read(b,"x.txt")).toBe("from a");
 expect(await homedOriginsV1(s2,"g2")).toEqual(["c"]);
 expect(await homedOriginsV1(s2,"g1")).toEqual(["a"]);
 expect(readFileSync(join(s1,"v1","origins","a","segments","000000000000.jsonl"))).toEqual(readFileSync(join(s2,"v1","origins","a","segments","000000000000.jsonl")));
 const again=await pull(s1,"g1",s2);
 expect(again).toMatchObject({origins:0,blobs:0,segments:0,bytes:0});
 expect(shared(a)).toEqual(shared(c));expect(shared(b)).toEqual(shared(c));
 if(python){
  for(const [store,who] of [[s1,a],[s1,b],[s2,c]] as const){const {status,report}=oracleCompare(store,who);expect({machine:who.name,status,drift:report.drift}).toEqual({machine:who.name,status:0,drift:[]});}
 }
 for(const who of [a,b,c])await shut(who);
},60000);

test("a relay that carries twelve origins at once copies identical journals and blobs, also when the origins share blobs",async()=>{
 const peer=temp("peer"),own=temp("own"),names=Array.from({length:12},(_,index)=>`m${index}`);
 const first=machine(names[0]!,peer,"g1");
 write(first,".gitignore",fixture.gitignore);await seedV1(first.options);
 const others=names.slice(1).map(name=>machine(name,peer,"g1"));
 for(const who of others)await joinV1(who.options);
 const all=[first,...others];
 for(const who of all){await open(who);for(let index=0;index<40;index++)write(who,`bulk/${who.name}/f${index}.txt`,index%2===0?"shared by everybody":`own ${who.name} ${index}`);await publish(who);await shut(who);}
 const result=await relayOnceV1(own,peer,await homedOriginsV1(peer,"g1"));
 expect(result.origins).toBe(12);
 for(const name of names){
  const left=join(peer,"v1","origins",name),right=join(own,"v1","origins",name);
  expect(readFileSync(join(right,"head.json"),"utf8")).toBe(readFileSync(join(left,"head.json"),"utf8"));
  expect(readFileSync(join(right,"segments","000000000000.jsonl"))).toEqual(readFileSync(join(left,"segments","000000000000.jsonl")));
 }
 const blobs=(store:string)=>walk(join(store,"v1","blobs")).map(file=>file.slice(store.length)).sort();
 expect(blobs(own)).toEqual(blobs(peer));
 expect(await relayOnceV1(own,peer,await homedOriginsV1(peer,"g1"))).toMatchObject({origins:0,blobs:0,segments:0,bytes:0});
},60000);

test("a relay whose own store vanished pauses and reports it instead of recreating the store as a local directory",async()=>{
 const s1=temp("store1"),s2=temp("store2"),a=machine("a",s1,"g1");
 write(a,".gitignore",fixture.gitignore);await seedV1(a.options);
 await relayOnceV1(s2,s1,await homedOriginsV1(s1,"g1"));
 const controller=new AbortController(),events:ReplicaEventV1[]=[],running=relayV1({ownStore:s2,peers:{g1:s1},pollMilliseconds:10},controller.signal,event=>events.push(event));
 await Bun.sleep(100);
 renameSync(join(s2,"v1"),join(s2,"away"));
 await until(()=>events.some(event=>event.event==="store-unreachable"&&event.path===s2));
 await Bun.sleep(200);
 expect(existsSync(join(s2,"v1"))).toBe(false);
 expect(events.filter(event=>event.event==="store-unreachable")).toHaveLength(1);
 renameSync(join(s2,"away"),join(s2,"v1"));
 await until(()=>events.some(event=>event.event==="store-reachable"&&event.path===s2));
 controller.abort();await running;
},60000);

test("a peer store that vanishes during a relay round never lets journal events overtake their blobs",async()=>{
 const peer=temp("peer"),own=temp("own"),a=machine("a",peer,"g1");
 write(a,".gitignore",fixture.gitignore);
 for(let index=0;index<1500;index++)write(a,`bulk/f${index}.txt`,`content ${index}`);
 await seedV1(a.options);
 setTimeout(()=>{renameSync(join(peer,"v1"),join(peer,"away"));},8);
 const outcome=await relayOnceV1(own,peer,["a"]).then(()=>"done",()=>"failed");
 const referenced=new Set<string>();
 for(const file of walk(join(own,"v1","origins")).filter(file=>file.endsWith(".jsonl")))for(const line of readFileSync(file,"utf8").split("\n"))if(line!=="")referenced.add((JSON.parse(line) as {sha256?:string}).sha256??"");
 referenced.delete("");
 for(const sha of referenced)expect(existsSync(blobPathV1(own,sha))).toBe(true);
 expect(existsSync(join(peer,"away"))).toBe(true);
 renameSync(join(peer,"away"),join(peer,"v1"));
 await relayOnceV1(own,peer,["a"]);
 expect(outcome === "done" || outcome === "failed").toBe(true);
 expect(readFileSync(join(own,"v1","origins","a","segments","000000000000.jsonl"))).toEqual(readFileSync(join(peer,"v1","origins","a","segments","000000000000.jsonl")));
},60000);

test("an unreachable store is reported once, local work continues, and publishing resumes when it returns",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 await shut(a);
 const controller=new AbortController(),events:ReplicaEventV1[]=[],running=runV1(a.options,controller.signal,event=>events.push(event));
 await until(()=>events.some(event=>event.event==="started"));
 renameSync(join(store,"v1"),join(store,"away"));
 await until(()=>events.some(event=>event.event==="store-unreachable"));
 write(a,"offline.txt","written during the outage");
 await Bun.sleep(300);
 expect(events.filter(event=>event.event==="store-unreachable")).toHaveLength(1);
 expect(read(a,"offline.txt")).toBe("written during the outage");
 renameSync(join(store,"away"),join(store,"v1"));
 await until(()=>events.some(event=>event.event==="published"));
 expect(events.some(event=>event.event==="store-reachable")).toBe(true);
 controller.abort();await running;
 expect(readFileSync(join(store,"v1","origins","a","segments","000000000000.jsonl"),"utf8")).toContain("offline.txt");
 const check=validator("ProgressEvent");
 for(const event of events)expect(check(JSON.parse(JSON.stringify(event)))).toBe(true);
 expect(existsSync(join(a.options.worktree,".cache","f","daemon.lock"))).toBe(false);
},60000);

test("the daemon propagates an edit between two running replicas",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 await shut(a);await shut(b);
 const controller=new AbortController(),runs=[runV1(a.options,controller.signal),runV1(b.options,controller.signal)];
 write(a,"live/file.txt","hello from the daemon");
 await until(()=>read(b,"live/file.txt")==="hello from the daemon");
 rmSync(at(a,"live/file.txt"));
 await until(()=>read(b,"live/file.txt")===undefined);
 controller.abort();await Promise.all(runs);
},60000);

test("torn writer state is never read and torn local staging files and stale locks are cleaned up",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"whole.txt","complete");await publish(a);
 const segment=join(store,"v1","origins","a","segments","000000000000.jsonl");
 appendFileSync(segment,'{"version":1,"hlc":"17310');
 await follow(b);
 expect(read(b,"whole.txt")).toBe("complete");
 write(a,"after.txt","after the tear");await publish(a);await follow(b);
 expect(read(b,"after.txt")).toBe("after the tear");
 await shut(b);
 const directory=join(b.options.worktree,".cache","f");
 writeFileSync(join(directory,"tmp","half-written"),"partial");
 const child=Bun.spawnSync([process.execPath,"--version"]);
 writeFileSync(join(directory,"daemon.lock"),JSON.stringify({version:1,pid:child.pid,startedMs:1,originId:"b"}));
 await open(b);
 expect(existsSync(join(directory,"tmp","half-written"))).toBe(false);
 await expect(openReplicaV1(b.options)).rejects.toBeInstanceOf(ReplicaBusyV1);
 expect(JSON.parse(readFileSync(join(directory,"daemon.lock"),"utf8")).pid).toBe(process.pid);
});

test("verify rehashes the tree against state and store and names every drift",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"p.txt","pristine");write(a,"q.txt","quiet");write(a,"r.txt","remote");await publish(a);await follow(b);await shut(a);await shut(b);
 expect(await verifyV1(b.options)).toMatchObject({ok:true,drift:[],truncated:false});
 write(b,"p.txt","tampered");rmSync(at(b,"q.txt"));write(b,"stray.txt","extra");
 const report=await verifyV1(b.options);
 expect(report.ok).toBe(false);
 expect(report.drift.map(item=>[item.path,item.kind])).toEqual(expect.arrayContaining([["p.txt","content"],["q.txt","missing-local"],["stray.txt","extra-local"]]));
 expect(validator("VerifyReport")(JSON.parse(JSON.stringify(report)))).toBe(true);
 if(python){const {status}=oracleCompare(b.options.storeRoot,b);expect(status).toBe(1);}
});

test("opening a replica reclaims the staging files that a kill -9 left, in the store and in .cache/f/tmp, and spares those of a live process",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 await shut(a);
 const child=Bun.spawn(["sleep","30"]),pid=child.pid;
 child.kill(9);await child.exited;
 const tmp=join(store,"v1","tmp"),local=join(a.options.worktree,".cache","f","tmp"),hex="0123456789abcdef";
 mkdirSync(tmp,{recursive:true});mkdirSync(local,{recursive:true});
 for(const name of [`a-${pid}-${hex}`,`a-${hex}`,`a-${process.pid}-${hex}`,`b-${pid}-${hex}`,`relay-${hex}`])writeFileSync(join(tmp,name),"torn");
 writeFileSync(join(local,"torn"),"torn");
 await open(a);
 expect(readdirSync(tmp).sort()).toEqual([`a-${process.pid}-${hex}`,`b-${pid}-${hex}`,`relay-${hex}`].sort());
 expect(readdirSync(local)).toEqual([]);
});

const longDirectory=`${"d".repeat(100)}/${"e".repeat(100)}/${"f".repeat(100)}`;

test("a path longer than the limit is refused with a visible event when scanned and when touched, counted by status --deep and listed by verify",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"fine.txt","fine");write(a,`${longDirectory}/long.txt`,"too long");
 const touched=`${longDirectory}/long.txt`;
 await publishOnceV1(a.replica!,undefined,[touched]);
 expect(a.events.filter(event=>event.event==="refused"&&event.path===touched).map(event=>event.reason)).toEqual(["too-long"]);
 await publish(a);
 expect(a.events.filter(event=>event.event==="refused"&&event.reason==="too-long").length).toBeGreaterThanOrEqual(2);
 await follow(b);
 expect(read(b,"fine.txt")).toBe("fine");expect(read(b,touched)).toBeUndefined();
 expect((await statusV1(a.options)).refused).toBeNull();
 const status=await statusV1(a.options,true);
 expect(status.refused).toBe(1);expect(validator("Status")(JSON.parse(JSON.stringify(status)))).toBe(true);
 await shut(a);await shut(b);
 const report=await verifyV1(a.options);
 expect(report.ok).toBe(true);expect(report.refusedTotal).toBe(1);
 expect(report.refused[0]!.reason).toBe("too-long");expect(touched.startsWith(report.refused[0]!.path)).toBe(true);
 expect(validator("VerifyReport")(JSON.parse(JSON.stringify(report)))).toBe(true);
});

test("status reports the lag per origin, the queues and the preserved losses in a closed shape",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"s.txt","status");await publish(a);
 const behind=await statusV1(b.options,true);
 expect(behind.reachable).toBe(true);expect(behind.daemon).toEqual({running:true,pid:process.pid});
 const lag=behind.origins.find(item=>item.origin==="a")!;
 expect(lag.behindEvents).toBeGreaterThan(0);expect(lag.behindBytes).toBeGreaterThan(0);expect(lag.rejoinRequired).toBe(false);
 expect(behind.queue.outgoing).toBe(0);
 expect(validator("Status")(JSON.parse(JSON.stringify(behind)))).toBe(true);
 await follow(b);
 const caught=await statusV1(b.options);
 expect(caught.origins.find(item=>item.origin==="a")).toMatchObject({behindEvents:0,behindBytes:0});
 expect(caught.queue).toEqual({incoming:0,outgoing:null});expect(caught.lost).toBe(0);
});

posixTest("executable bits and relative symbolic links travel, absolute links never do",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"bin/run.sh","#!/bin/sh\n");chmodSync(at(a,"bin/run.sh"),0o755);
 write(a,"real.txt","target");symlinkSync("real.txt",at(a,"alias.txt"));symlinkSync("/etc/hosts",at(a,"absolute.txt"));
 await publish(a);await follow(b);
 expect(statSync(at(b,"bin/run.sh")).mode&0o111).not.toBe(0);
 expect(readlinkSync(at(b,"alias.txt"))).toBe("real.txt");
 expect(existsSync(at(b,"absolute.txt"))).toBe(false);
 expect(readFileSync(join(store,"v1","origins","a","segments","000000000000.jsonl"),"utf8")).not.toContain("absolute.txt");
 expect(a.events.some(event=>event.event==="refused"&&event.path==="absolute.txt")).toBe(true);
});

test("a path the receiver ignores is reported and never materialised",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(b,".gitignore",`${fixture.gitignore}*.late\n`);
 write(a,"x.late","late");await publish(a);await follow(b);
 expect(read(b,"x.late")).toBeUndefined();
 expect(b.events.some(event=>event.event==="refused"&&event.path==="x.late"&&event.reason==="ignored")).toBe(true);
});

test("a follower whose segments were compacted away recovers from the newest snapshot of that origin",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 write(a,"one.txt","1");write(a,"two.txt","2");await publish(a);
 await compactV1(store,"a",14);
 rmSync(join(store,"v1","origins","a","segments","000000000000.jsonl"));
 expect(await follow(b)).toMatchObject({applied:2});
 expect(read(b,"one.txt")).toBe("1");expect(read(b,"two.txt")).toBe("2");
 expect(b.events.some(event=>event.event==="rejoin-required")).toBe(false);
});

test("relay carries the newest snapshot of a homed origin together with the blobs it references",async()=>{
 const s1=temp("store1"),s2=temp("store2"),a=machine("a",s1,"g1");
 write(a,".gitignore",fixture.gitignore);write(a,"s.txt","snap");await seedV1(a.options);
 await snapshotV1(s1,"a");
 await relayOnceV1(s2,s1,await homedOriginsV1(s1,"g1"));
 const entry=(await readNewestSnapshotV1(s2))?.manifest.get("s.txt");
 expect(entry?.sha256).toBeDefined();
 expect(existsSync(join(s2,"v1","blobs",entry!.sha256!.slice(0,2),entry!.sha256!))).toBe(true);
});

test("a future-dated event is held, earlier events and other origins proceed, and it is applied once the wall clock catches up",async()=>{
 const {store,machines}=await fleet(["a","b","c"]),[a,b,c]=machines as [Machine,Machine,Machine],base=Date.now()+5000,segment=join(store,"v1","origins","a","segments","000000000000.jsonl");
 let wallA=base,wallB=base;
 for(const who of machines)await shut(who);
 a.options={...a.options,now:()=>wallA};b.options={...b.options,now:()=>wallB};
 for(const who of machines)await open(who);
 write(a,"early.txt","early");await publish(a);
 wallA=base+120_000;
 write(a,"late.txt","late");await publish(a);
 write(a,"later.txt","later");await publish(a);
 write(c,"other.txt","other");await publish(c);
 const skews=()=>b.events.filter(event=>event.event==="clock-skew");
 expect(await follow(b)).toMatchObject({events:2,applied:2});
 expect(read(b,"early.txt")).toBe("early");expect(read(b,"other.txt")).toBe("other");
 expect(read(b,"late.txt")).toBeUndefined();expect(read(b,"later.txt")).toBeUndefined();
 expect(skews()).toHaveLength(1);expect(skews()[0]).toMatchObject({origin:"a",path:"late.txt",seconds:120});
 expect(validator("ProgressEvent")(JSON.parse(JSON.stringify(skews()[0])))).toBe(true);
 expect(b.replica!.clock.ms).toBeLessThan(base+120_000);
 const lines=readFileSync(segment,"utf8").split("\n"),stop=lines.findIndex(line=>line.includes('"late.txt"')),before={segment:0,bytes:Buffer.byteLength(`${lines.slice(0,stop).join("\n")}\n`)};
 expect(b.replica!.cursors.a).toEqual(before);
 const folded=await foldStoreV1(store,undefined,()=>{},()=>base);
 expect(folded.manifest.has("early.txt")).toBe(true);expect(folded.manifest.has("late.txt")).toBe(false);expect(folded.cursors.a).toEqual(before);
 wallB=base+30_000;
 await follow(b);await follow(b);
 expect(skews()).toHaveLength(1);expect(read(b,"late.txt")).toBeUndefined();expect(b.replica!.cursors.a).toEqual(before);
 wallB=base+61_000;
 expect(await follow(b)).toMatchObject({events:2,applied:2});
 expect(read(b,"late.txt")).toBe("late");expect(read(b,"later.txt")).toBe("later");
 const head=(await readHeadV1(store,"a"))!;
 expect(b.replica!.cursors.a).toEqual({segment:head.segment,bytes:head.bytes});
 expect(skews()).toHaveLength(1);
});

test("state is a checkpoint plus an append-only log that survives a crash, a torn tail, a stale generation and a rollover",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 const directory=join(b.options.worktree,".cache","f"),state=join(directory,"state.json"),log=join(directory,"state.log"),checkpoint=readFileSync(state,"utf8"),first=(JSON.parse(checkpoint) as {generation:number}).generation;
 write(a,"one.txt","1");write(a,"two.txt","2");await publish(a);await follow(b);
 expect(readFileSync(state,"utf8")).toBe(checkpoint);
 const lines=readFileSync(log,"utf8").trimEnd().split("\n").map(line=>JSON.parse(line) as {kind?:string;path?:string;entry?:{operation?:string}|null});
 for(const line of lines)expect(validator("StateLogLine")(line)).toBe(true);
 expect<unknown>(lines[0]).toEqual({generation:first});
 expect(lines.some(line=>line.kind==="files"&&line.path==="one.txt"&&line.entry?.operation==="put")).toBe(true);
 expect(lines.some(line=>line.kind==="queue"&&line.path==="one.txt"&&line.entry===null)).toBe(true);
 await b.replica!.release();b.replica=undefined;
 appendFileSync(log,'{"kind":"files","path":"torn.txt","entry":{"hlc":"170');
 const crashed=await open(b);
 expect(crashed.files.get("one.txt")?.operation).toBe("put");expect(crashed.files.get("two.txt")?.operation).toBe("put");
 expect(crashed.files.has("torn.txt")).toBe(false);expect(crashed.queue.size).toBe(0);
 expect(readFileSync(log,"utf8").endsWith("}\n")).toBe(true);
 expect(await publish(b)).toMatchObject({published:0,deleted:0});
 await shut(b);
 expect(readFileSync(log,"utf8")).toBe("");
 const generation=(JSON.parse(readFileSync(state,"utf8")) as {generation:number}).generation;
 expect(generation).toBe(first+1);
 const removal=`${JSON.stringify({kind:"files",path:"one.txt",entry:null})}\n`;
 writeFileSync(log,`${JSON.stringify({generation:generation-1})}\n${removal}`);
 const stale=await open(b);
 expect(stale.files.get("one.txt")?.operation).toBe("put");expect(statSync(log).size).toBe(0);
 await shut(b);
 writeFileSync(log,`${JSON.stringify({generation})}\n${removal}`);
 const replayed=await open(b);
 expect(replayed.files.has("one.txt")).toBe(false);expect(replayed.files.get("two.txt")?.operation).toBe("put");
 await replayed.release();b.replica=undefined;
 const filler=Array.from({length:100_000},(_,index)=>JSON.stringify({kind:"files",path:`filler/${"x".repeat(120)}${index}`,entry:null})).join("\n");
 writeFileSync(log,`${JSON.stringify({generation})}\n${filler}\n`);
 await open(b);
 expect(statSync(log).size).toBeGreaterThan(16*1024*1024);
 write(b,"roll.txt","r");
 expect((await publish(b)).published).toBe(1);
 expect(statSync(log).size).toBe(0);
 const rolled=JSON.parse(readFileSync(state,"utf8")) as {generation:number;files:Record<string,unknown>};
 expect(rolled.generation).toBe(generation+1);expect(Object.keys(rolled.files)).toContain("roll.txt");
 await shut(b);
},60000);

test("a same-size edit inside the racy window is detected while an old entry is still trusted by size and mtime",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine],stamp=Math.floor(Date.now()/1000),wall=stamp*1000+500;
 await shut(a);a.options={...a.options,now:()=>wall};await open(a);
 const edit=(path:string,text:string,seconds:number)=>{writeFileSync(at(a,path),text);utimesSync(at(a,path),seconds,seconds);};
 edit("fresh.txt","aaaa",stamp);edit("old.txt","aaaa",1_600_000_000);
 expect((await publish(a)).published).toBe(2);
 edit("fresh.txt","bbbb",stamp);edit("old.txt","bbbb",1_600_000_000);
 expect(await publish(a)).toMatchObject({published:1,touched:0});
 expect(await publish(a)).toMatchObject({published:0,touched:1});
 const paths=(await readEventsV1(store,"a",undefined)).events.map(event=>event.path);
 expect(paths.filter(path=>path==="fresh.txt")).toHaveLength(2);expect(paths.filter(path=>path==="old.txt")).toHaveLength(1);
 await shut(a);
 await open(a);
 expect(a.replica!.files.get("fresh.txt")?.seenMs).toBe(wall);
 expect(a.replica!.files.get("old.txt")?.seenMs).toBe(wall);
});

function prng(seed:number):()=>number{let state=seed>>>0;return ()=>{state=(state+0x6D2B79F5)>>>0;let value=Math.imul(state^state>>>15,1|state);value=value+Math.imul(value^value>>>7,61|value)^value;return ((value^value>>>14)>>>0)/4294967296;};}
const chaosPaths=Array.from({length:30},(_,index)=>index<20?`d${index%5}/n${index%2}/f${index}.txt`:`f${index}.txt`);
function expectedTree(store:string,manifest:Iterable<readonly [string,FleetManifestEntryV1]>):Record<string,string>{
 const rows:Record<string,string>={};
 for(const [path,entry] of manifest)if(entry.operation==="put"&&!hidden(path))rows[path]=readFileSync(blobPathV1(store,entry.sha256!),"utf8");
 return rows;
}
type Stage={store:string;group:string;members:Machine[]};
async function chaos(seed:number,stages:Stage[],options:{kill?:boolean}={}):Promise<void>{
 const random=prng(seed),all=stages.flatMap(stage=>stage.members),left=new Map<Machine,number>(all.map(who=>[who,40])),pick=<T>(items:readonly T[]):T=>items[Math.floor(random()*items.length)]!;
 const pull=async(from:Stage,into:Stage)=>relayOnceV1(into.store,from.store,await homedOriginsV1(from.store,from.group));
 const relays=async()=>{let moved=0;for(const [from,into] of stages.flatMap(one=>stages.filter(other=>other!==one).map(other=>[one,other] as const))){const result=await pull(from,into);moved+=result.origins+result.bytes+result.blobs;}return moved;};
 let serial=0;
 while([...left.values()].some(count=>count>0)){
  const roll=random();
  if(roll<0.5){
   const who=pick(all.filter(one=>left.get(one)!>0)),path=pick(chaosPaths);
   left.set(who,left.get(who)!-1);
   if(existsSync(at(who,path))&&random()<0.35)rmSync(at(who,path));else write(who,path,`${who.name}:${serial++}:${"x".repeat(Math.floor(random()*24))}`);
  }else if(roll<0.72)await publish(pick(all));
  else if(roll<0.94)await follow(pick(all));
  else if(roll<0.97&&stages.length>1)await relays();
  else if(options.kill){const who=pick(all);await who.replica!.release();who.replica=undefined;await open(who);}
 }
 let quiet=false;
 for(let round=0;round<12&&!quiet;round++){
  let activity=0;
  for(const who of all){const result=await publish(who);activity+=result.published+result.deleted+result.retry.length;}
  if(stages.length>1)activity+=await relays();
  for(const who of all){const result=await follow(who);activity+=result.applied+result.queued;}
  quiet=activity===0;
 }
 expect(quiet).toBe(true);
 for(const who of all){const result=await publish(who);expect({machine:who.name,published:result.published,deleted:result.deleted,retry:result.retry}).toEqual({machine:who.name,published:0,deleted:0,retry:[]});}
 if(stages.length>1)expect(await relays()).toBe(0);
 for(const who of all){const result=await follow(who);expect({machine:who.name,applied:result.applied,queued:result.queued}).toEqual({machine:who.name,applied:0,queued:0});}
 for(const stage of stages){
  const folded=(await foldStoreV1(stage.store)).manifest,tree=expectedTree(stage.store,folded);
  for(const who of stage.members)expect({machine:who.name,tree:shared(who)}).toEqual({machine:who.name,tree});
  expect(await verifyV1(stage.members[0]!.options)).toMatchObject({ok:true,drift:[]});
 }
 if(stages.length>1){const [first,...rest]=stages;for(const stage of rest)expect(expectedTree(stage.store,(await foldStoreV1(stage.store)).manifest)).toEqual(expectedTree(first!.store,(await foldStoreV1(first!.store)).manifest));}
}

async function singleStage(names:readonly string[]):Promise<Stage>{
 const {store,machines}=await fleet(names);
 return {store,group:"g1",members:machines};
}

async function twoStages():Promise<Stage[]>{
 const s1=temp("store1"),s2=temp("store2"),first=["a","b","c"].map(name=>machine(name,s1,"g1")),second=["d","e"].map(name=>machine(name,s2,"g2"));
 write(first[0]!,".gitignore",fixture.gitignore);await seedV1(first[0]!.options);
 for(const who of first.slice(1))await joinV1(who.options);
 await relayOnceV1(s2,s1,await homedOriginsV1(s1,"g1"));
 for(const who of second)await joinV1(who.options);
 for(const who of [...first,...second])await open(who);
 return [{store:s1,group:"g1",members:first},{store:s2,group:"g2",members:second}];
}

for(const seed of [1,2,3])test(`five machines on one store make 40 random edits over 30 paths and converge without ping-pong (seed ${seed})`,async()=>{
 const stage=await singleStage(["a","b","c","d","e"]);
 try{await chaos(seed,[stage]);}finally{for(const who of stage.members)await shut(who);}
},120000);

for(const seed of [11,12])test(`five machines on two relayed stores converge in both directions without ping-pong (seed ${seed})`,async()=>{
 const stages=await twoStages();
 try{await chaos(seed,stages);}finally{for(const who of stages.flatMap(stage=>stage.members))await shut(who);}
},120000);

for(const seed of [21,22])test(`machines killed and reopened in the middle of random edits still converge (seed ${seed})`,async()=>{
 const stages=seed===21?[await singleStage(["a","b","c","d","e"])]:await twoStages();
 try{await chaos(seed,stages,{kill:true});}finally{for(const who of stages.flatMap(stage=>stage.members))await shut(who);}
},120000);
