import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClaimTableV1}from"../../../../🌐️remote-cache/🎟️claims/🟦️.ts";
import{createRemoteCacheHandlerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{RemoteCacheStoreV1}from"../../../../🌐️remote-cache/💾️store/🟦️.ts";
import{graphFleetClientV1,type GraphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{createProjectFleetPortV1,type ProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";

/** 🏭️ Law: nothing of the project graph is inferred twice in a fleet. Two machines (two roots, two stores) share one in-process fleet cache whose claim table runs on a clock the test moves: a machine that wants an owner another machine is inferring waits for its upload and takes it, never infers it; a claimant that dies is replaced after its lease lapsed; a machine holding some owners takes the others from the fleet; a fleet that does not answer ends in a local inference within the declared timeouts, never in a hang.
 * @see ../../🟦️.ts */
const scratch:string[]=[],closers:(()=>unknown)[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-machines-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const close of closers)await close();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const environment={SEMIO_REMOTE_CACHE_ENDPOINT:"http://fleet.test",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};

async function fleet(){
 const skew={milliseconds:0},table=createClaimTableV1({now:()=>Date.now()+skew.milliseconds}),store=await RemoteCacheStoreV1.open({directory:temp("cache"),maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeMilliseconds:0,graceMilliseconds:150});
 const handler=createRemoteCacheHandlerV1({store,claims:table,tokens:{read:"r",write:"w"},defaultInstance:"main",validation:"off"});
 closers.push(()=>table.close(),()=>store.close());
 const fetcher=((input:unknown,init?:RequestInit)=>handler(new Request(input as string,init))) as unknown as typeof fetch;
 const machine=(label:string,wrap:(client:GraphFleetClientV1)=>GraphFleetClientV1=client=>client,viaFetch:typeof fetch=fetcher)=>{
  const root=temp(label),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:viaFetch}),port=createProjectFleetPortV1({root,environment,client:wrap(client),owner:`${label}:1`})!;
  return{root,port};
 };
 return{skew,table,machine};
}
const keyOf=(name:string)=>JSON.stringify(["rev",null,[["authority"],"libs/"+name,{name}]]);
const portable=(root:string,name:string)=>({digest:sha(name),authority:[],observations:[{kind:"file",path:`${root}/libs/${name}/src/a.ts`,fact:{sha256:sha("x"+name),size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image:JSON.stringify({name,root:"libs/"+name})});
type Seat={record?:unknown;imported:unknown[]};
const infoOf=(root:string,name:string,seat:Seat):ProjectRecordInfoV1=>({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:keyOf(name),exportRecord:()=>seat.record,async import(record){seat.imported.push(record);return true;}});
const seatOf=(record?:unknown):Seat=>({record,imported:[]});
const settle=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const stalled=():typeof fetch=>((_input:unknown,init?:RequestInit)=>new Promise<Response>((_resolve,reject)=>init?.signal?.addEventListener("abort",()=>reject(Object.assign(new Error("timed out"),{name:"TimeoutError"}))))) as unknown as typeof fetch;

test("machine B wants the owner machine A is inferring: it waits for A's upload and takes it, it never infers, and the record names B's paths only",async()=>{
 const{machine}=await fleet(),a=machine("a"),b=machine("b"),seatA=seatOf(),seatB=seatOf();
 seatA.record=portable(a.root,"alpha");
 const gateA=await a.port.gate(infoOf(a.root,"alpha",seatA));
 expect(gateA.seeded).toBe(false);
 const waiting=b.port.gate(infoOf(b.root,"alpha",seatB));
 await settle(300);
 gateA.publish();await a.port.flush();
 const gateB=await waiting;
 expect(gateB.seeded).toBe(true);
 expect(b.port.stats()).toMatchObject({seeded:1,waited:1,computed:0});
 expect(a.port.stats()).toMatchObject({published:1,computed:1});
 const text=JSON.stringify(seatB.imported);
 expect(text).toContain(b.root);expect(text).not.toContain(a.root);
},60000);

test("the claim of a pool child of machine A is the claim every worker honours: B waits for the pool's upload and never infers the owner",async()=>{
 const{machine}=await fleet(),a=machine("pool"),b=machine("worker"),seatA=seatOf(),seatB=seatOf();
 seatA.record=portable(a.root,"beta");
 const claimed=await a.port.claim(infoOf(a.root,"beta",seatA));
 expect(claimed.state).toBe("granted");
 const waiting=b.port.gate(infoOf(b.root,"beta",seatB));
 await settle(300);
 if(claimed.state==="granted")expect(await claimed.publish()).toBe(true);
 expect((await waiting).seeded).toBe(true);
 expect(b.port.stats().computed).toBe(0);
 const again=await a.port.claim(infoOf(a.root,"beta",seatA));
 expect(again.state).toBe("stored");
},60000);

test("a claimant that dies is replaced: when its lease lapsed machine B takes the claim over, infers the owner once and publishes it for machine C",async()=>{
 const{machine,skew}=await fleet(),dead=machine("dead",client=>({...client,renew:async()=>"lost" as never,release:async()=>undefined})),b=machine("takeover"),c=machine("late"),seatB=seatOf(portable("/b","gamma")),seatC=seatOf();
 const gateDead=await dead.port.gate(infoOf(dead.root,"gamma",seatOf()));
 expect(gateDead.seeded).toBe(false);
 const waiting=b.port.gate(infoOf(b.root,"gamma",seatB));
 await settle(400);
 skew.milliseconds=31*60_000;
 const gateB=await waiting;
 expect(gateB.seeded).toBe(false);
 expect(b.port.stats()).toMatchObject({computed:1,seeded:0});
 seatB.record=portable(b.root,"gamma");
 gateB.publish();await b.port.flush();
 expect(b.port.stats().published).toBe(1);
 skew.milliseconds=0;
 expect((await c.port.gate(infoOf(c.root,"gamma",seatC))).seeded).toBe(true);
 expect(c.port.stats().computed).toBe(0);
},60000);

test("a machine that holds some owners takes the rest of the slice from the fleet: five owners, one local, four published by another machine, none inferred",async()=>{
 const{machine}=await fleet(),a=machine("producer"),b=machine("slice"),names=["n1","n2","n3","n4","n5"],seats=new Map(names.map(name=>[name,seatOf()]));
 for(const name of names){const seat=seatOf(portable(a.root,name)),gate=await a.port.gate(infoOf(a.root,name,seat));gate.publish();}
 await a.port.flush();
 expect(a.port.stats().published).toBe(5);
 const local=await b.port.gate(infoOf(b.root,"n1",seats.get("n1")!));
 expect(local.seeded).toBe(true);
 for(const name of names.slice(1))expect((await b.port.gate(infoOf(b.root,name,seats.get(name)!))).seeded,name).toBe(true);
 expect(b.port.stats()).toMatchObject({seeded:5,computed:0});
 for(const seat of seats.values())expect(JSON.stringify(seat.imported)).not.toContain(a.root);
},60000);

test("a fleet that never answers ends in a local inference of both machines within the breaker and timeout bounds, and the owners after the first skip the open breaker at once",async()=>{
 const{machine}=await fleet(),a=machine("down-a",undefined,stalled()),b=machine("down-b",undefined,stalled()),began=Date.now();
 const gates=await Promise.all([a.port.gate(infoOf(a.root,"delta",seatOf())),b.port.gate(infoOf(b.root,"delta",seatOf()))]);
 expect(gates.map(gate=>gate.seeded)).toEqual([false,false]);
 expect(gates.map(gate=>gate.deferred)).toEqual([undefined,undefined]);
 expect(Date.now()-began).toBeLessThan(45000);
 expect([a.port.stats().computed,b.port.stats().computed]).toEqual([1,1]);
 for(const gate of gates)await gate.abandon();
 const next=Date.now(),second=await a.port.gate(infoOf(a.root,"epsilon",seatOf()));
 expect(second.seeded).toBe(false);expect(Date.now()-next,"while the breaker is open an owner skips the fleet at once").toBeLessThan(2000);
 await second.abandon();
 const dead=machine("refused",undefined,(async()=>{throw new TypeError("connect ECONNREFUSED");}) as unknown as typeof fetch),quick=Date.now();
 expect((await dead.port.gate(infoOf(dead.root,"delta",seatOf()))).seeded).toBe(false);
 expect(Date.now()-quick).toBeLessThan(45000);
},150000);

test("a record that cannot be shared is counted with its reason, and a shareable one of the same machine is published once and adopted by another machine",async()=>{
 const{machine}=await fleet(),a=machine("a"),b=machine("b"),broken=seatOf({digest:sha("gamma"),authority:[],observations:[],image:"not json"}),good=seatOf(),taken=seatOf();
 const first=await a.port.claim(infoOf(a.root,"gamma",broken));
 expect(first.state).toBe("granted");
 if(first.state==="granted")expect(await first.publish()).toBe(false);
 expect(a.port.stats().refused).toBe(1);expect(a.port.stats().refusals).toEqual({"result is no JSON":1});
 good.record=portable(a.root,"delta");
 const second=await a.port.claim(infoOf(a.root,"delta",good));
 if(second.state==="granted")expect(await second.publish()).toBe(true);
 await a.port.flush();
 expect((await b.port.gate(infoOf(b.root,"delta",taken))).seeded).toBe(true);
 expect(b.port.stats()).toMatchObject({seeded:1,computed:0});expect(b.port.stats().refusals).toEqual({});
},60000);
