/** 🛟️ Contract: a graph read that is stopped loses no finished owner and repeats no finished owner. Real processes infer synthetic owners through the real project record path of a private repository; the parent stops a process after k owners (SIGINT, as the stall watchdog does, or SIGKILL), or inside owner k+1, and the next process infers exactly the unfinished ones; an edit of one input makes exactly that owner and its dependent infer again, a restamp of identical bytes none, a half-written or damaged record exactly its own owner; the identity of a record names the owner key and the record stamp and nothing of the process, session or machine (equal to an independent Python recomputation).
 * @see ../../📋️present/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{readFileSync,readdirSync,rmSync,truncateSync,utimesSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{hostname}from"node:os";
import{spawnSync}from"node:child_process";
import{brotliDecompressSync}from"node:zlib";
import{privateGraphWorkspaceV1}from"../../../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts";
import observationSchema from"../../../../../👁️observation/🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const child=join(import.meta.dir,"🧒️child","🟦️.ts"),oracle=join(import.meta.dir,"🐍️oracle.py"),stamp=(observationSchema as any).$defs.record.const.stamp as string,sources:string[]=[];
afterAll(()=>{for(const source of sources)rmSync(source,{recursive:true,force:true});});
type Run={lines:string[];computed:number[];done:number[];root:string;code:number|null};

async function read(config:Record<string,unknown>,stop?:{when:RegExp;signal:NodeJS.Signals}):Promise<Run>{
 const process_=Bun.spawn([process.execPath,"test",child,"--timeout","600000"],{cwd:process.cwd(),env:{...process.env,R60_CHILD:JSON.stringify(config),HOSTNAME:"machine-"+Math.random().toString(16).slice(2)},stdout:"pipe",stderr:"pipe"}),lines:string[]=[],errors:string[]=[];
 let buffer="",signalled=false;
 const reader=(async()=>{for await(const chunk of process_.stdout as unknown as AsyncIterable<Uint8Array>){buffer+=Buffer.from(chunk).toString("utf8");let at:number;while((at=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,at);buffer=buffer.slice(at+1);if(!line.startsWith("R60 "))continue;lines.push(line.slice(4));if(stop&&!signalled&&stop.when.test(line.slice(4))){signalled=true;process_.kill(stop.signal);}}}})();
 const errs=(async()=>{for await(const chunk of process_.stderr as unknown as AsyncIterable<Uint8Array>)errors.push(Buffer.from(chunk).toString("utf8"));})();
 const timer=setTimeout(()=>process_.kill("SIGKILL"),240000);await process_.exited;clearTimeout(timer);await reader;await errs;if(process.env.R60_DEBUG)process.stderr.write(errors.join("").split("\n").filter(line=>!line.startsWith("[script]")).join("\n").slice(-3000));
 const numbers=(prefix:string)=>lines.filter(line=>line.startsWith(prefix+" ")).map(line=>Number(line.split(" ")[1]));
 return{lines,computed:numbers("COMPUTE"),done:numbers("DONE"),root:(lines.find(line=>line.startsWith("ROOT "))??"ROOT ").slice(5),code:process_.exitCode};
}
async function scenario(name:string){
 const ws=await privateGraphWorkspaceV1(),source=await ws.scratch("r60-"+name+"-");sources.push(source);
 for(let index=0;index<fixture.owners;index++)writeFileSync(join(source,"in"+index+".txt"),"input "+index);
 await Bun.sleep(2300);
 const config=(extra:Record<string,unknown>={})=>({source,owners:fixture.owners,stopAfter:-1,stallIn:-1,shared:fixture.shared,session:"session-"+Math.random().toString(16).slice(2),key:fixture.key,...extra});
 return{source,config};
}
const names=(root:string)=>readdirSync(join(root,"np")).filter(name=>name.endsWith(".json"));
const header=(path:string)=>{const text=brotliDecompressSync(readFileSync(path)).toString("utf8");return JSON.parse(text.slice(0,text.indexOf("\n"))) as {digest:string};};

test("SIGINT after k finished owners: the next read infers exactly the n-k unfinished ones and a third read none",async()=>{
 const{config}=await scenario("int"),k=3,first=await read(config({stopAfter:k}),{when:new RegExp("^READY "+k+"$"),signal:"SIGINT"});
 expect(first.done).toEqual([0,1,2]);expect(first.lines.some(line=>line.startsWith("FINISHED"))).toBe(false);expect(names(first.root).length).toBe(k);
 const second=await read(config());expect(second.computed).toEqual([3,4,5,6,7]);expect(second.computed.length).toBe(fixture.owners-k);expect(second.lines.at(-1)).toBe("FINISHED 5");
 const third=await read(config());expect(third.computed).toEqual([]);expect(third.lines.at(-1)).toBe("FINISHED 0");
},600000);

test("SIGKILL inside owner k+1: the owners finished before it are durable and taken, the one in flight and the rest are inferred",async()=>{
 const{config}=await scenario("kill"),k=3,first=await read(config({stallIn:k}),{when:new RegExp("^STALLED "+k+"$"),signal:"SIGKILL"});
 expect(first.done).toEqual([0,1,2]);expect(names(first.root).length).toBe(k);
 const second=await read(config());expect(second.computed).toEqual([3,4,5,6,7]);
 const third=await read(config());expect(third.computed).toEqual([]);
},600000);

test("the key names the owner and the record stamp only: records written by different processes under different sessions and host names carry the digest of an independent Python recomputation, and no process or session text",async()=>{
 const{config}=await scenario("key"),first=await read(config({stopAfter:4,session:"first-session-token"}),{when:/^READY 4$/,signal:"SIGKILL"}),second=await read(config({session:"second-session-token"}));
 expect(second.computed).toEqual([4,5,6,7]);
 const keys=Array.from({length:fixture.owners},(_,index)=>["r60",index,fixture.key]),file=join(await (await privateGraphWorkspaceV1()).scratch("r60-oracle-"),"keys.json");sources.push(file);
 writeFileSync(file,JSON.stringify({stamp,keys}));
 const run=spawnSync("python3",["-I",oracle,file],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);const want=JSON.parse(run.stdout) as string[];
 const{ownerRecordKeyV1}=await import("../../📋️present/🟨️.mjs") as any;
 for(let index=0;index<fixture.owners;index++){const path=join(second.root,"np",ownerRecordKeyV1("owner-"+index)+".json");expect(header(path).digest,"owner "+index).toBe(want[index]);const text=brotliDecompressSync(readFileSync(path)).toString("utf8");for(const token of["first-session-token","second-session-token",hostname(),"machine-"])expect(text.includes(token),token).toBe(false);}
 expect(first.root).toBe(second.root);
 writeFileSync(file,JSON.stringify({stamp,keys:fixture.keyCases}));const cases=spawnSync("python3",["-I",oracle,file],{encoding:"utf8"});expect(cases.status,cases.stderr).toBe(0);expect(JSON.parse(cases.stdout).length).toBe(fixture.keyCases.length);
},600000);

test("an edit of one input infers that owner and its dependent only, a restamp of identical bytes none, and the new records are taken by the next read",async()=>{
 const{source,config}=await scenario("edit"),first=await read(config());expect(first.computed).toEqual([0,1,2,3,4,5,6,7]);
 const stamped=join(source,"in5.txt"),bytes=readFileSync(stamped);writeFileSync(stamped,bytes);utimesSync(stamped,new Date(),new Date());
 const restamp=await read(config());expect(restamp.computed).toEqual([]);
 writeFileSync(join(source,"in"+fixture.shared+".txt"),"input 3 edited");await Bun.sleep(2300);
 const edited=await read(config());expect(edited.computed).toEqual(fixture.edited);
 const again=await read(config());expect(again.computed).toEqual([]);
},600000);

test("a half-written, a damaged and a leftover temporary record are never trusted and never block: exactly the owners whose record is bad are inferred",async()=>{
 const{config}=await scenario("half"),first=await read(config()),{ownerRecordKeyV1}=await import("../../📋️present/🟨️.mjs") as any,path=(index:number)=>join(first.root,"np",ownerRecordKeyV1("owner-"+index)+".json");
 truncateSync(path(5),Math.floor(readFileSync(path(5)).length/2));writeFileSync(path(6),Buffer.from("not a record at all"));writeFileSync(path(2)+".99.99.tmp",readFileSync(path(2)).subarray(0,10));
 const repaired=await read(config());expect(repaired.computed).toEqual([5,6]);
 const again=await read(config());expect(again.computed).toEqual([]);
},600000);
