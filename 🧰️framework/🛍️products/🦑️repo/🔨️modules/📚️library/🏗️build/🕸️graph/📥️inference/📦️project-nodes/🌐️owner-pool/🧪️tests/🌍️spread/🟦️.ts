import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolRendezvousV1,pullOwnerPoolV1}from"../../🟦️.ts";
import type{ProjectClaimV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";

const fast={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
type Unit={name:string;megabytes?:number};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const wave=(count:number,open:number,heavy:number):{units:Unit[];table:Map<string,"held"|"stored">}=>{
 const units=Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(5,"0")}`})) as Unit[],table=new Map<string,"held"|"stored">(),step=Math.floor(count/open);
 const openAt=new Set<number>(Array.from({length:open},(_all,index)=>index*step+3));
 units.forEach((unit,index)=>{if(!openAt.has(index))table.set(unit.name,"stored");});
 [...openAt].slice(0,heavy).forEach(index=>{units[index]={name:units[index]!.name,megabytes:3000};});
 return{units,table};
};
const claimOf=(table:Map<string,"held"|"stored">,counter:{asked:number})=>async(unit:Unit):Promise<ProjectClaimV1>=>{
 counter.asked++;await pause(2);
 const state=table.get(unit.name);
 if(state==="stored")return{state:"stored"};if(state==="held")return{state:"held",by:"x"};
 table.set(unit.name,"held");
 return{state:"granted",digest:sha(unit.name),why:()=>"",code:()=>"",deterministic:()=>false,publish:async()=>{table.set(unit.name,"stored");return true;},abandon:async()=>{table.delete(unit.name);}};
};

/** 🌍️ Where the pullers of one wave start: apart, whatever the order of the list, and over the whole wave while most owners are stored. */
test("the rendezvous score of an owner differs per puller and is a pure function of both",()=>{
 expect(ownerPoolRendezvousV1("m1:1","a")).toBe(ownerPoolRendezvousV1("m1:1","a"));
 expect(ownerPoolRendezvousV1("m1:1","a")).not.toBe(ownerPoolRendezvousV1("m2:1","a"));
 expect(ownerPoolRendezvousV1("m1:1","a")).not.toBe(ownerPoolRendezvousV1("m1:1","b"));
 const score=ownerPoolRendezvousV1("m1:1","a");expect(score>=0&&score<1).toBe(true);
 const first=new Set(Array.from({length:8},(_all,index)=>{const names=Array.from({length:8},(_n,at)=>`heavy-${at}`);return names.map(name=>({name,score:ownerPoolRendezvousV1(`machine-${index}:1`,name)})).sort((a,b)=>a.score-b.score)[0]!.name;}));
 expect(first.size).toBeGreaterThanOrEqual(4);
});

test("eight pullers on a wave of which 3 percent is open: after the first claim round at least eight distinct owners run and nobody waits on a peer",async()=>{
 const {units,table}=wave(1000,32,6),counter={asked:0},running=new Set<string>(),seen=new Set<string>(),perPuller=new Map<number,number>(),hold=new Set<number>(),snapshots:number[]=[];
 const fleet=Promise.all(Array.from({length:8},(_all,index)=>pullOwnerPoolV1<Unit>({units:()=>units,puller:`machine-${index}:1`,policy:fast,slots:()=>1,claim:claimOf(table,counter),busy:count=>{perPuller.set(index,count);if(count>0)hold.add(index);},run:async unit=>{running.add(unit.name);seen.add(unit.name);await pause(300);running.delete(unit.name);return true;}})));
 await pause(220);snapshots.push(running.size);
 expect(running.size).toBeGreaterThanOrEqual(8);
 expect(hold.size).toBe(8);
 const stats=await fleet;
 expect(stats.reduce((sum,row)=>sum+row.published,0)).toBe(32);
 expect(Math.max(...[...seen].map(name=>[...seen].filter(other=>other===name).length))).toBe(1);
},30000);

test("a puller does not end its pull after a few stored owners: one puller finds nearly every open owner of a wave of which 2 percent is open",async()=>{
 const {units,table}=wave(2000,40,0),counter={asked:0};
 const stats=await pullOwnerPoolV1<Unit>({units:()=>units,puller:"solo:1",policy:fast,slots:()=>1,claim:claimOf(table,counter),run:async()=>true});
 expect(stats.published).toBeGreaterThanOrEqual(30);
 expect(counter.asked).toBeLessThan(2000);
},30000);

test("heavy owners that do not fit the grant of a machine are left to a machine that holds them, and the others take them in an order of their own",async()=>{
 const {units,table}=wave(200,10,8),counter={asked:0},took:string[][]=[[],[]];
 const small=pullOwnerPoolV1<Unit>({units:()=>units,fits:unit=>(unit.megabytes??0)<=2048,puller:"small:1",policy:fast,slots:()=>1,claim:claimOf(table,counter),run:async unit=>{took[0]!.push(unit.name);return true;}});
 await small;
 expect(took[0]!.some(name=>units.find(unit=>unit.name===name)?.megabytes===3000)).toBe(false);
 const orders=await Promise.all(["a:1","b:1","c:1","d:1","e:1","f:1"].map(async puller=>{const own=new Map(table),claimed:string[]=[];await pullOwnerPoolV1<Unit>({units:()=>units,puller,policy:fast,slots:()=>1,stop:()=>claimed.length>=1,claim:async unit=>{own.delete(unit.name);return claimOf(new Map(own),{asked:0})(unit);},run:async unit=>{claimed.push(unit.name);return true;}});return claimed[0]!;}));
 expect(new Set(orders).size).toBeGreaterThanOrEqual(3);
},30000);
