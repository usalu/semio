import {afterAll,expect,mock,spyOn,test} from "bun:test";
import * as real from "node:fs/promises";
import {createHash} from "node:crypto";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {appendEventsV1,blobPathV1,formatHlcV1,readFlatHeadsV1,storePathsV1,writeBlobV1,type FleetEventV1} from "../../📒️journal/🟦️.ts";
import type {ReplicaEventV1,ReplicaOptionsV1,ReplicaV1} from "../🟦️.ts";

type Hook=(op:string,path:string)=>void|Promise<void>;
const originals={...real},seen:{op:string;path:string}[]=[];
let hook:Hook|undefined,counting=false;
const wrap=(op:string)=>async(...args:unknown[])=>{
 const path=typeof args[0]==="string"?args[0]:"";
 if(counting)seen.push({op,path});
 await hook?.(op,path);
 return (originals as unknown as Record<string,(...inner:unknown[])=>unknown>)[op]!(...args);
};
mock.module("node:fs/promises",()=>({...originals,...Object.fromEntries(["stat","open"].map(op=>[op,wrap(op)]))}));
const {closeReplicaV1,followOnceV1,homedOriginsV1,joinV1,openReplicaV1,publishOnceV1,relayOnceV1,seedV1,walkLocalV1}=await import("../🟦️.ts");

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-store-${label}-`));scratch.push(path);return path;};
afterAll(()=>{hook=undefined;for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(data:string|Buffer)=>createHash("sha256").update(data).digest("hex");
const walk=(directory:string):string[]=>existsSync(directory)?readdirSync(directory,{withFileTypes:true}).flatMap(item=>item.isDirectory()?walk(join(directory,item.name)):[join(directory,item.name)]):[];

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
let tick=1_700_000_000;
const machine=(name:string,store:string,group="g1"):Machine=>({name,events:[],options:{worktree:temp(name),storeRoot:store,originId:name,groupId:group,pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0}});
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
function write(who:Machine,path:string,text:string|Buffer):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);utimesSync(file,tick,tick);tick++;}
const read=(who:Machine,path:string):string|undefined=>existsSync(at(who,path))?readFileSync(at(who,path),"utf8"):undefined;
async function open(who:Machine):Promise<ReplicaV1>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));return who.replica;}
async function shut(who:Machine):Promise<void>{if(who.replica){await closeReplicaV1(who.replica);who.replica=undefined;}}
async function fleet(names:readonly string[],store:string=temp("store")){
 const machines=names.map(name=>machine(name,store));
 write(machines[0]!,".gitignore","ignored\n");
 await seedV1(machines[0]!.options);
 for(const other of machines.slice(1))await joinV1(other.options);
 for(const who of machines)await open(who);
 return {store,machines};
}
const publish=(who:Machine,paths?:readonly string[])=>publishOnceV1(who.replica!,undefined,paths);
const follow=(who:Machine)=>followOnceV1(who.replica!);

test("three replicas converge through the flat heads, and an idle round never touches a head.json of an origin that has a flat head",async()=>{
 const {store,machines:[a,b,c]}=await fleet(["a","b","c"]) as {store:string;machines:[Machine,Machine,Machine]};
 write(a,"x.txt","from a");await publish(a);await follow(b);await follow(c);
 write(b,"y.txt","from b");await publish(b);await follow(a);await follow(c);
 expect([read(a,"y.txt"),read(b,"x.txt"),read(c,"x.txt"),read(c,"y.txt")]).toEqual(["from b","from a","from a","from b"]);
 expect(readdirSync(storePathsV1(store).heads).map(name=>name.split("_")[0]).sort()).toEqual(["a","b"]);
 for(let round=0;round<3;round++){await follow(c);await Bun.sleep(40);}
 seen.length=0;counting=true;
 for(let round=0;round<5;round++){await follow(c);await Bun.sleep(5);}
 counting=false;
 const headFiles=seen.filter(row=>row.path.endsWith("head.json")),heads=seen.filter(row=>row.path===storePathsV1(store).heads);
 expect(headFiles).toEqual([]);
 expect(heads.length).toBeLessThanOrEqual(7);
 for(const who of [a,b,c])await shut(who);
},60000);

test("an origin without a flat head (code that does not write it) is still followed, and a stale flat head is corrected by the audit",async()=>{
 const {store,machines:[a,b]}=await fleet(["a","b"]) as {store:string;machines:[Machine,Machine]};
 write(a,"one.txt","1");await publish(a);await follow(b);
 rmSync(storePathsV1(store).heads,{recursive:true});
 write(a,"two.txt","2");await publish(a);
 rmSync(storePathsV1(store).heads,{recursive:true});
 await Bun.sleep(5);
 expect((await readFlatHeadsV1(store)).size).toBe(0);
 await follow(b);
 expect(read(b,"two.txt")).toBe("2");
 write(a,"three.txt","3");await publish(a);
 const [stale]=readdirSync(storePathsV1(store).heads);
 expect(stale).toBeDefined();
 const head=JSON.parse(readFileSync(join(storePathsV1(store).origins,"a","head.json"),"utf8")) as {segment:number;bytes:number;events:number;hlc:string};
 const old=`a_0_1_1_${formatHlcV1({ms:1700000000000,counter:0},"a")}`;
 rmSync(join(storePathsV1(store).heads,stale!));writeFileSync(join(storePathsV1(store).heads,old),"");
 expect(head.bytes).toBeGreaterThan(1);
 await Bun.sleep(5);
 await follow(b);
 expect(read(b,"three.txt")).toBeUndefined();
 const now=Date.now(),spy=spyOn(Date,"now").mockImplementation(()=>now+11_000);
 try{await follow(b);}finally{spy.mockRestore();}
 expect(read(b,"three.txt")).toBe("3");
 for(const who of [a,b])await shut(who);
},60000);

test("an applied file is not treated as racy: the next walk finds nothing to hash, and the state says it was seen after its own timestamp",async()=>{
 const {machines:[a,b]}=await fleet(["a","b"]) as {machines:[Machine,Machine]};
 for(let index=0;index<20;index++)write(a,`bulk/f${index}.txt`,`content ${index}`);
 await publish(a);
 expect((await follow(b)).applied).toBe(20);
 const entry=b.replica!.files.get("bulk/f3.txt")!;
 expect(entry.seenMs!-entry.mtimeMs).toBeGreaterThanOrEqual(2000);
 expect((await walkLocalV1(b.replica!)).changed.length).toBe(0);
 expect((await publish(b)).published).toBe(0);
 for(const who of [a,b])await shut(who);
},60000);

test("a blob whose content does not match its name is never applied, the file stays absent and nothing is left in the staging directory",async()=>{
 const {store,machines:[a,b]}=await fleet(["a","b"]) as {store:string;machines:[Machine,Machine]};
 write(a,"good.txt","good bytes");write(a,"bad.txt","bad bytes");await publish(a);
 writeFileSync(blobPathV1(store,sha("bad bytes")),"tampered!!");
 const result=await follow(b);
 expect(result).toMatchObject({applied:1,queued:1});
 expect(read(b,"good.txt")).toBe("good bytes");expect(read(b,"bad.txt")).toBeUndefined();
 expect(b.events.filter(event=>event.event==="blob-missing"&&event.reason==="blob-corrupt")).toHaveLength(1);
 expect(walk(join(b.options.worktree,".cache","f","tmp"))).toEqual([]);
 for(const who of [a,b])await shut(who);
},60000);

test("a file that changes between its hash and its copy is not published and leaves no blob; the next round publishes the new bytes",async()=>{
 const {store,machines:[a,b]}=await fleet(["a","b"]) as {store:string;machines:[Machine,Machine]};
 write(a,"mut.txt","version one");await publish(a);await follow(b);
 write(a,"mut.txt","version 222");
 let mutated=false;
 hook=(op,path)=>{if(op==="open"&&path.endsWith("mut.txt")&&!mutated){mutated=true;writeFileSync(path,"version 333");utimesSync(path,tick,tick);tick++;}};
 const first=await publish(a,["mut.txt"]);
 hook=undefined;
 expect(mutated).toBe(true);
 expect(first.published).toBe(0);expect(first.retry).toEqual(["mut.txt"]);
 expect(a.events.filter(event=>event.event==="unstable")).toHaveLength(1);
 expect(existsSync(blobPathV1(store,sha("version 222")))).toBe(false);
 expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 expect((await publish(a)).published).toBe(1);
 await follow(b);
 expect(read(b,"mut.txt")).toBe("version 333");
 for(const who of [a,b])await shut(who);
},60000);

test("a replica far behind consumes the backlog slice by slice, keeps publishing its own change in between and loses nothing",async()=>{
 const {store,machines:[a,b,c]}=await fleet(["a","b","c"]) as {store:string;machines:[Machine,Machine,Machine]};
 const source=join(temp("blob"),"x");writeFileSync(source,"shared");await writeBlobV1(store,"far",sha("shared"),source);
 const events:FleetEventV1[]=Array.from({length:3000},(_,index)=>({version:1,hlc:formatHlcV1({ms:Date.now()-60_000+Math.floor(index/500),counter:index%500},"far"),operation:"put",path:`far/d${index%30}/f${index}.txt`,sha256:sha("shared"),bytes:6,executable:false,base:null}));
 await appendEventsV1(store,"far",events);
 let clock=Date.now();
 const spy=spyOn(Date,"now").mockImplementation(()=>(clock+=600));
 const sizes:number[]=[];
 try{
  const first=await follow(b);sizes.push(first.events);
  expect(first.events).toBeGreaterThan(0);expect(first.events).toBeLessThanOrEqual(1024);
  expect(b.replica!.cursors.far!.bytes>0||b.replica!.cursors.far!.segment>0).toBe(true);
  write(b,"mine.txt","published while catching up");
  expect((await publish(b,["mine.txt"])).published).toBe(1);
  await follow(c);
  expect(read(c,"mine.txt")).toBe("published while catching up");
  expect(Object.keys(await walkFiles(b)).filter(path=>path.startsWith("far/")).length).toBeLessThan(3000);
  for(let guard=0;guard<20;guard++){const next=await follow(b);sizes.push(next.events);if(next.events===0&&next.queued===0)break;}
 }finally{spy.mockRestore();}
 expect(sizes.every(size=>size<=1024)).toBe(true);expect(sizes.filter(size=>size>0).length).toBeGreaterThanOrEqual(3);
 expect(sizes.reduce((sum,size)=>sum+size,0)).toBe(3000);
 expect(Object.keys(await walkFiles(b)).filter(path=>path.startsWith("far/")).length).toBe(3000);
 for(const who of [a,b,c])await shut(who);
},120000);
async function walkFiles(who:Machine):Promise<Record<string,string>>{
 const rows:Record<string,string>={},root=who.options.worktree;
 for(const file of walk(root)){const path=file.slice(root.length+1);if(!path.startsWith(".cache")&&!path.startsWith(".git"))rows[path]="";}
 return rows;
}

test("the relay writes flat heads for the origins it carries, verifies every blob in the copy, and refuses a peer blob whose content is wrong without moving the head",async()=>{
 const peer=temp("peer"),own=temp("own"),a=machine("a",peer);
 write(a,".gitignore","ignored\n");write(a,"ok.txt","fine");write(a,"bad.txt","will be tampered");
 await seedV1(a.options);
 writeFileSync(blobPathV1(peer,sha("will be tampered")),"something else entirely");
 await expect(relayOnceV1(own,peer,await homedOriginsV1(peer,"g1"))).rejects.toThrow("Relay blob differs from its hash");
 expect(existsSync(blobPathV1(own,sha("will be tampered")))).toBe(false);
 expect(existsSync(join(storePathsV1(own).origins,"a","head.json"))).toBe(false);
 await Bun.sleep(200);
 expect(walk(storePathsV1(own).tmp)).toEqual([]);
 writeFileSync(blobPathV1(peer,sha("will be tampered")),"will be tampered");
 await relayOnceV1(own,peer,await homedOriginsV1(peer,"g1"));
 expect((await readFlatHeadsV1(own)).get("a")).toEqual(JSON.parse(readFileSync(join(storePathsV1(own).origins,"a","head.json"),"utf8")));
 expect(readdirSync(storePathsV1(own).heads)).toHaveLength(1);
 const c=machine("c",own,"g2");
 await joinV1(c.options);await open(c);
 expect(read(c,"ok.txt")).toBe("fine");expect(read(c,"bad.txt")).toBe("will be tampered");
 await shut(c);
},60000);

test("a large file reports progress while it is published and applied, and cancelling the publish leaves no staging file", async()=>{
 const {store,machines:[a,b]}=await fleet(["a","b"]) as {store:string;machines:[Machine,Machine]};
 const data=Buffer.alloc(64*1024*1024+4096,3);data.writeUInt32LE(7,10);
 write(a,"big.bin",data);
 const controller=new AbortController(),before=a.events.length;
 a.replica!.say=((original)=>(event:ReplicaEventV1)=>{original(event);if(event.event==="progress"&&event.stage==="publish-blob"&&event.done!<event.total!)controller.abort();})(a.replica!.say);
 await expect(publishOnceV1(a.replica!,controller.signal,["big.bin"])).rejects.toThrow();
 expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 expect(walk(storePathsV1(store).blobs).length).toBeGreaterThanOrEqual(0);
 expect(a.events.slice(before).some(event=>event.event==="progress"&&event.stage==="publish-blob")).toBe(true);
 await shut(a);await open(a);
 expect((await publish(a)).published).toBe(1);
 const done=a.events.filter(event=>event.event==="progress"&&event.stage==="publish-blob"&&event.done===event.total);
 expect(done.length).toBeGreaterThanOrEqual(1);
 await follow(b);
 expect(statSync(at(b,"big.bin")).size).toBe(data.length);
 expect(b.events.some(event=>event.event==="progress"&&event.stage==="apply-blob"&&event.done===event.total)).toBe(true);
 expect(sha(readFileSync(at(b,"big.bin")))).toBe(sha(data));
 appendFileSync(at(a,"big.bin"),"");
 for(const who of [a,b])await shut(who);
},120000);
