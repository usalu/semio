import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,lstatSync,mkdirSync,readdirSync,readFileSync,readlinkSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {clashKeyV1,clashV1,retiredV1,staleRepublishV1} from "../🟦️.ts";
import {formatHlcV1} from "../../../📒️journal/🟦️.ts";
import {closeReplicaV1,conflictWinnerV1,followOnceV1,openReplicaV1,publishOnceV1,staleEditV1,type ReplicaEventV1} from "../../🟦️.ts";
import {buildFleetV1,checkFleetV1,disposeFleetV1,roundV1,settleV1,type ChaosFleetV1,type ChaosMachineV1} from "../../🧪️tests/🌪️harness.ts";

const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py"),fixturePath=join(import.meta.dirname,"🧫️fixtures","🔣️.json");
const ajv=new Ajv({strict:true,allowUnionTypes:true}),fleets:ChaosFleetV1[]=[];
afterAll(async()=>{for(const fleet of fleets)await disposeFleetV1(fleet);});

const at=(who:ChaosMachineV1,path:string):string=>join(who.options.worktree,...path.split("/"));
const put=(who:ChaosMachineV1,path:string,text:string):void=>{mkdirSync(dirname(at(who,path)),{recursive:true});writeFileSync(at(who,path),text);};
const text=(who:ChaosMachineV1,path:string):string|undefined=>existsSync(at(who,path))?readFileSync(at(who,path),"utf8"):undefined;
const names=(who:ChaosMachineV1,directory:string=""):string[]=>readdirSync(join(who.options.worktree,directory)).filter(name=>name!==".git"&&name!==".cache").sort();
const events=(fleet:ChaosFleetV1,kind:string):ReplicaEventV1[]=>fleet.machines.flatMap(who=>who.events.filter(event=>event.event===kind));
const publish=async(who:ChaosMachineV1):Promise<void>=>{await publishOnceV1(who.replica!);};
const lostFiles=(who:ChaosMachineV1):string[]=>{
 const found:string[]=[],root=join(who.options.worktree,".cache","f","lost"),walk=(directory:string):void=>{
  if(!existsSync(directory))return;
  for(const item of readdirSync(directory,{withFileTypes:true})){const path=join(directory,item.name);if(item.isDirectory())walk(path);else found.push(path.slice(root.length+1));}
 };
 walk(root);return found;
};
async function fleetOf(count:number=3):Promise<ChaosFleetV1>{const fleet=await buildFleetV1(count);fleets.push(fleet);return fleet;}
async function sync(fleet:ChaosFleetV1):Promise<void>{
 expect(await settleV1(fleet,60_000)).toBeGreaterThan(0);
 const violations=await checkFleetV1(fleet);
 expect(violations,violations.join("\n")).toEqual([]);
 expect(events(fleet,"error").map(event=>event.message)).toEqual([]);
}

test("the fixture is valid against the schema",()=>{
 const validate=ajv.compile(schema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
});

test("the structural conflict rule matches the vectors and the independent Python oracle",()=>{
 for(const vector of fixture.vectors)expect(Object.fromEntries(retiredV1(vector.entries)),vector.name).toEqual(vector.retired);
 for(const vector of fixture.stale)expect(staleRepublishV1(vector.events,vector.hlc)??null,vector.name).toEqual(vector.stale);
 expect(clashV1("a/b","a/b")).toBe(false);expect(clashV1("a","a/b")).toBe(true);expect(clashV1("A/b","a/c")).toBe(false);expect(clashKeyV1("STRASSE")).toBe(clashKeyV1("straße"));
 if(python===undefined)return;
 const result=Bun.spawnSync([python,"-I",oracle,"vectors",fixturePath],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
});

test("a local edit wins a conflict only when it is simultaneous with the remote event",()=>{
 const remote=formatHlcV1({ms:1_000_000,counter:0},"g1-m00");
 expect(conflictWinnerV1(1_000_500,remote)).toBe("local");
 expect(conflictWinnerV1(999_000,remote)).toBe("remote");
 expect(conflictWinnerV1(undefined,remote)).toBe("remote");
 expect(conflictWinnerV1(1_000_000+8*3_600_000,remote)).toBe("remote");
 expect(staleEditV1(1_000_000+8*3_600_000,remote)).toBe(true);
 expect(staleEditV1(1_000_500,remote)).toBe(false);
});

test("the rule is order independent",()=>{
 for(const vector of fixture.vectors){
  const shuffled=[...vector.entries].reverse();
  expect(Object.fromEntries(retiredV1(shuffled)),vector.name).toEqual(vector.retired);
 }
 for(const vector of fixture.stale)expect(staleRepublishV1([...vector.events].reverse(),vector.hlc)??null,vector.name).toEqual(vector.stale);
});

posixTest("D1: a case-only rename racing an edit of the old name converges on the newer entry and keeps the other in lost",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 renameSync(at(m0,"b.txt"),at(m0,"B.txt"));
 await publish(m0);
 put(m1,"b.txt","edited concurrently\n");
 await publish(m1);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(names(who).filter(name=>name.toLowerCase()==="b.txt")).toEqual(["b.txt"]);expect(text(who,"b.txt")).toBe("edited concurrently\n");}
 expect(events(fleet,"superseded").length).toBeGreaterThan(0);
 expect(lostFiles(m0).some(file=>file.endsWith("B.txt"))).toBe(true);
},180_000);

posixTest("D1: a case-only rename that is newer than the edit of the old name wins",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m1,"b.txt","edited first\n");
 await publish(m1);
 await Bun.sleep(5);
 renameSync(at(m0,"b.txt"),at(m0,"B.txt"));
 await publish(m0);
 await sync(fleet);
 for(const who of [m0,m1,m2])expect(names(who).filter(name=>name.toLowerCase()==="b.txt")).toEqual(["B.txt"]);
},180_000);

posixTest("D2: a descendant newer than a file replacing its directory wins and the file goes to lost",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 rmSync(at(m0,"dir"),{recursive:true});writeFileSync(at(m0,"dir"),"now a file\n");
 await publish(m0);
 await Bun.sleep(5);
 put(m1,"dir/c.txt","edited inside\n");
 await publish(m1);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(lstatSync(at(who,"dir")).isDirectory()).toBe(true);expect(text(who,"dir/c.txt")).toBe("edited inside\n");}
 expect(lostFiles(m0).some(file=>file.endsWith("/dir"))).toBe(true);
},180_000);

posixTest("D2: a file newer than every descendant retires the descendants",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m1,"dir/new.txt","created inside\n");
 await publish(m1);
 await Bun.sleep(5);
 rmSync(at(m0,"dir"),{recursive:true});writeFileSync(at(m0,"dir"),"now a file\n");
 await publish(m0);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(lstatSync(at(who,"dir")).isFile()).toBe(true);expect(text(who,"dir")).toBe("now a file\n");}
 expect(lostFiles(m1).some(file=>file.endsWith("dir/new.txt"))).toBe(true);
},180_000);

posixTest("a retired entry comes back, also after a restart, when its winner is deleted",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 renameSync(at(m0,"b.txt"),at(m0,"B.txt"));
 await publish(m0);
 await Bun.sleep(5);
 put(m1,"b.txt","edited concurrently\n");
 await publish(m1);
 await sync(fleet);
 const old=m2.replica!;m2.replica=undefined;await closeReplicaV1(old);m2.replica=await openReplicaV1(m2.options,event=>m2.events.push(event));
 rmSync(at(m1,"b.txt"));
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(names(who).filter(name=>name.toLowerCase()==="b.txt")).toEqual(["B.txt"]);expect(text(who,"B.txt")).toBe("bravo\n");}
},240_000);

posixTest("D3: a put below a symlink that the fleet knows retires or follows the structural rule and never writes through the link",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 symlinkSync("dir",at(m0,"ln"));
 await publish(m0);
 await Bun.sleep(5);
 put(m1,"ln/x.txt","below\n");
 await publish(m1);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(lstatSync(at(who,"ln")).isDirectory()).toBe(true);expect(text(who,"ln/x.txt")).toBe("below\n");expect(existsSync(at(who,"dir/x.txt"))).toBe(false);}
},180_000);

posixTest("D3: a symlink that stays local blocks the put below it with a visible reason and the target stays untouched",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 const outside=join(fleet.root,"outside");mkdirSync(outside);
 symlinkSync("../../../../outside",at(m1,"esc"));
 expect(readlinkSync(at(m1,"esc"))).toBe("../../../../outside");
 put(m0,"esc/x.txt","through\n");
 await publish(m0);
 for(let round=0;round<6;round++){await roundV1(m1);await roundV1(m2);await roundV1(m0);}
 expect(existsSync(join(outside,"x.txt"))).toBe(false);
 expect(m1.events.some(event=>event.event==="blocked"&&event.path==="esc/x.txt"&&event.reason==="symlink-in-the-way")).toBe(true);
 expect(m1.replica!.queue.has("esc/x.txt")).toBe(true);
 expect(m1.events.filter(event=>event.event==="error")).toEqual([]);
 expect(text(m2,"esc/x.txt")).toBe("through\n");
},180_000);

posixTest("D3: a symlink cycle produces neither error events nor writes",async()=>{
 const fleet=await fleetOf(),[m0,m1]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 const outside=join(fleet.root,"outside");mkdirSync(outside);
 symlinkSync("../../outside/a",at(m1,"loop1"));symlinkSync(at(m1,"loop1"),join(outside,"a"));
 put(m0,"loop1/x.txt","x\n");
 await publish(m0);
 for(let round=0;round<5;round++){await roundV1(m1);await roundV1(m0);}
 expect(m1.events.filter(event=>event.event==="error")).toEqual([]);
 expect(m1.replica!.queue.has("loop1/x.txt")).toBe(true);
},180_000);

posixTest("D3: a file reached through a symlinked directory is not published under the link path",async()=>{
 const fleet=await fleetOf(),[m0,m1]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 symlinkSync("dir",at(m0,"ln"));
 await publish(m0);
 put(m0,"dir/seen.txt","real\n");
 m0.replica!.hot.noteMany(["ln/seen.txt","dir/seen.txt"],2,Date.now());
 await publishOnceV1(m0.replica!,undefined,["ln/seen.txt","dir/seen.txt"]);
 await sync(fleet);
 expect(m1.replica!.manifest.has("ln/seen.txt")).toBe(false);
 expect(text(m1,"dir/seen.txt")).toBe("real\n");
},180_000);

posixTest("D4: forty deletes and forty symlinks in one directory apply without error events",async()=>{
 const fleet=await fleetOf(),[m0,m1]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 const files=Array.from({length:40},(_,index)=>`p/a${index}.txt`);
 for(const path of files)put(m0,path,"x\n");
 await sync(fleet);
 for(const path of files)rmSync(at(m0,path));
 files.forEach((_,index)=>symlinkSync("../b.txt",at(m0,`p/l${index}`)));
 await publish(m0);
 for(let round=0;round<4;round++)await roundV1(m1);
 await sync(fleet);
 expect(names(m1,"p")).toEqual(files.map((_,index)=>`l${index}`).sort());
},240_000);

posixTest("the 06:14 incident: a replica republishing old content with a fresh hlc does not replace the newer content",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"inc.txt","version one\n");
 await sync(fleet);
 put(m0,"inc.txt","version two\n");
 await sync(fleet);
 put(m2,"inc.txt","version one\n");
 const id=m2.replica!.book.find("inc.txt")!;
 m2.replica!.folds.remove(id);m2.replica!.files.clearId(id);
 await publish(m2);
 expect(m2.replica!.manifest.get("inc.txt")).toMatchObject({operation:"put"});
 await sync(fleet);
 for(const who of [m0,m1,m2])expect(text(who,"inc.txt")).toBe("version two\n");
 expect(events(fleet,"stale-republish").length).toBeGreaterThan(0);
},240_000);

posixTest("the 06:14 incident after a restart of the machines that hold the newer content: the memory of the stale rule survives it",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"inc2.txt","version one\n");
 await sync(fleet);
 put(m0,"inc2.txt","version two\n");
 await sync(fleet);
 for(const who of [m0,m1]){await closeReplicaV1(who.replica!);who.replica=await openReplicaV1(who.options,event=>who.events.push(event));}
 put(m2,"inc2.txt","version one\n");
 const id=m2.replica!.book.find("inc2.txt")!;
 m2.replica!.folds.remove(id);m2.replica!.files.clearId(id);
 await publish(m2);
 await sync(fleet);
 for(const who of [m0,m1,m2])expect(text(who,"inc2.txt")).toBe("version two\n");
 expect(events(fleet,"stale-republish").length).toBeGreaterThan(0);
},240_000);

posixTest("a machine that reads the newer content and the stale republish in one batch ends on the newer content",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"inc3.txt","version one\n");
 await sync(fleet);
 put(m0,"inc3.txt","version two\n");
 await publish(m0);
 await followOnceV1(m2.replica!);
 await roundV1(m2);
 put(m2,"inc3.txt","version one\n");
 const id=m2.replica!.book.find("inc3.txt")!;
 m2.replica!.folds.remove(id);m2.replica!.files.clearId(id);
 await publish(m2);
 await sync(fleet);
 for(const who of [m0,m1,m2])expect(text(who,"inc3.txt")).toBe("version two\n");
},240_000);

posixTest("D2 across case: files that adopt or join wrote into the registers directly are found below a newer file spelled in another case",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"seed9.txt","seed\n");
 await publish(m0);
 await sync(fleet);
 put(m1,"Dir9/x.txt","adopted\n");
 const replica=m1.replica!,id=replica.book.intern("Dir9/x.txt"),hlc=formatHlcV1({ms:1_700_000_000_000,counter:0},"g1-m01"),stat=lstatSync(at(m1,"Dir9/x.txt"));
 const entry={hlc,operation:"put" as const,sha256:"0".repeat(63)+"1",bytes:8,executable:false};
 replica.manifest.set("Dir9/x.txt",entry);replica.files.write(id,{...entry,mtimeMs:stat.mtimeMs,seenMs:Date.now()});replica.folds.add(id);
 put(m0,"dir9","a file\n");
 await publish(m0);
 await followOnceV1(replica);
 await roundV1(m1);
 expect(lstatSync(at(m1,"dir9")).isFile()).toBe(true);
 expect(text(m1,"dir9")).toBe("a file\n");
 expect(existsSync(at(m1,"Dir9/x.txt"))).toBe(false);
 expect(events(fleet,"superseded").some(event=>event.path==="Dir9/x.txt")).toBe(true);
},180_000);

posixTest("a tree of empty directories does not block a file at its path",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 mkdirSync(at(m1,"emp/a/b"),{recursive:true});
 put(m0,"emp","a file\n");
 await publish(m0);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(lstatSync(at(who,"emp")).isFile()).toBe(true);expect(text(who,"emp")).toBe("a file\n");}
 expect(m1.replica!.queue.size).toBe(0);
},180_000);

posixTest("D2 across case: a file newer than a directory that is spelled in another case retires the entries below it",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m1,"Dir2/x.txt","below\n");
 await publish(m1);
 await Bun.sleep(5);
 put(m0,"dir2","a file\n");
 await publish(m0);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(names(who).filter(name=>name.toLowerCase()==="dir2")).toEqual(["dir2"]);expect(text(who,"dir2")).toBe("a file\n");}
 expect(lostFiles(m1).some(file=>file.endsWith("Dir2/x.txt"))).toBe(true);
},180_000);

posixTest("D2 across case: an entry below a directory spelled in another case retires the older file",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"dir3","a file\n");
 await publish(m0);
 await Bun.sleep(5);
 put(m1,"Dir3/x.txt","below\n");
 await publish(m1);
 await sync(fleet);
 for(const who of [m0,m1,m2]){expect(names(who).filter(name=>name.toLowerCase()==="dir3")).toHaveLength(1);expect(text(who,"Dir3/x.txt")).toBe("below\n");}
 expect(lostFiles(m0).some(file=>file.endsWith("/dir3"))).toBe(true);
},180_000);

posixTest("the 02:13 incident: an edit written on a stale view never replaces newer fleet content and is kept in lost",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"stale-base.txt","version one\n");
 await sync(fleet);
 put(m0,"stale-base.txt","version two, written on top of one\n");
 await publish(m0);
 for(const who of [m2]){await closeReplicaV1(who.replica!);who.options={...who.options,concurrentEditMilliseconds:200};who.replica=await openReplicaV1(who.options,event=>who.events.push(event));}
 await Bun.sleep(400);
 const later=new Date(Date.now()+3_600_000);
 put(m2,"stale-base.txt","version one plus a late edit\n");
 utimesSync(at(m2,"stale-base.txt"),later,later);
 const held=await publishOnceV1(m2.replica!);
 expect(held.published).toBe(0);
 expect(m2.replica!.queue.has("stale-base.txt")).toBe(true);
 await sync(fleet);
 for(const who of [m0,m1,m2])expect(text(who,"stale-base.txt")).toBe("version two, written on top of one\n");
 expect(lostFiles(m2).some(file=>file.endsWith("stale-base.txt"))).toBe(true);
 expect(events(fleet,"stale-base").length).toBeGreaterThan(0);
},240_000);

posixTest("a simultaneous edit of the same base still resolves by modification time and keeps the other in lost",async()=>{
 const fleet=await fleetOf(),[m0,m1,m2]=fleet.machines as [ChaosMachineV1,ChaosMachineV1,ChaosMachineV1];
 put(m0,"both.txt","start\n");
 await sync(fleet);
 put(m0,"both.txt","from zero\n");
 await publish(m0);
 put(m2,"both.txt","from two\n");
 await publish(m2);
 await sync(fleet);
 const winner=text(m0,"both.txt");
 expect(["from zero\n","from two\n"]).toContain(winner);
 for(const who of [m1,m2])expect(text(who,"both.txt")).toBe(winner);
},240_000);
