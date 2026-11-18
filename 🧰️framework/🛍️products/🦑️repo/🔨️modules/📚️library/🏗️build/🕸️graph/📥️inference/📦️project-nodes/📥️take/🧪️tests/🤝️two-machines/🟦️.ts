/** 🤝️ Contract: two machines (and a third that dies) pull the owners of one wave through the real claim table of the cache server on a virtual clock: every owner is inferred exactly once in the fleet, both machines work from the first moment (neither waits for the other), the claims are leases (renewed by a heartbeat, a dead claimer blocks its owners for no longer than the declared lease), and a stored owner is picked up by the peer without a second inference.
 * @see ../../../🌐️owner-pool/🟦️.ts */
import{expect,test}from"bun:test";
import{createClaimTableV1}from"../../../../../../🌐️remote-cache/🎟️claims/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,pullOwnerPoolV1}from"../../../🌐️owner-pool/🟦️.ts";
import schema from"../../../../../../🪪️graph-state/🧬️schema/🔣️.json"with{type:"json"};

const heartbeat=(schema as {definitions:{Policy:{const:{claimHeartbeatMilliseconds:number}}}}).definitions.Policy.const.claimHeartbeatMilliseconds;

type Unit={name:string;expectedMilliseconds:number};
const world=()=>{
 let clock=0;const timers:{at:number;wake():void}[]=[];
 const sleep=(milliseconds:number)=>new Promise<void>(wake=>{timers.push({at:clock+milliseconds,wake});});
 const settle=async()=>{for(let turn=0;turn<20;turn++)await new Promise(done=>setImmediate(done));};
 const drive=async(finished:()=>boolean)=>{
  for(let guard=0;guard<200000&&!finished();guard++){
   await settle();if(finished())break;
   if(timers.length===0)break;
   timers.sort((a,b)=>a.at-b.at);const next=timers.shift()!;clock=Math.max(clock,next.at);next.wake();
  }
 };
 return{now:()=>clock,sleep,drive};
};

const machine=(input:{name:string;table:ReturnType<typeof createClaimTableV1>;units:Unit[];stored:Set<string>;runs:Map<string,string[]>;firstRun:Map<string,number>;now:()=>number;sleep:(milliseconds:number)=>Promise<void>;ownerMilliseconds:number;dies?:{afterClaims:number}}):Promise<unknown>=>{
 let claimed=0,dead=false;const ttl=P.claimTtlMilliseconds;
 return pullOwnerPoolV1<Unit>({
  units:()=>input.units,puller:input.name,slots:()=>dead?0:2,stop:()=>dead,now:input.now,sleep:input.sleep,policy:{...P,sweepPollMilliseconds:500,heldRetryMilliseconds:2000,faultRetryMilliseconds:2000},
  claim:async unit=>{
   if(dead)return{state:"fault"} as never;
   const answer=input.table.claim("pool","ac",unit.name,{owner:input.name,ttlMilliseconds:ttl,expectedMilliseconds:unit.expectedMilliseconds},input.stored.has(unit.name));
   if(answer.state==="stored")return{state:"stored"} as never;
   if(answer.state!=="granted")return{state:"held",by:"peer"} as never;
   claimed++;
   if(input.dies!==undefined&&claimed>input.dies.afterClaims){dead=true;return{state:"fault"} as never;}
   let alive=true;
   void(async()=>{while(alive&&!dead){await input.sleep(heartbeat);if(alive&&!dead)input.table.renew("pool","ac",unit.name,{owner:input.name,ttlMilliseconds:ttl});}})();
   return{state:"granted",digest:unit.name,why:()=>"",code:()=>"",deterministic:()=>false,publish:async()=>{alive=false;input.stored.add(unit.name);input.table.stored("pool","ac",unit.name);return true;},abandon:async()=>{alive=false;input.table.release("pool","ac",unit.name,input.name);}} as never;
  },
  run:async unit=>{
   const at=input.now();if(!input.firstRun.has(input.name))input.firstRun.set(input.name,at);
   input.runs.set(unit.name,[...(input.runs.get(unit.name)??[]),input.name]);
   if(input.dies!==undefined&&claimed>input.dies.afterClaims)return false;
   await input.sleep(input.ownerMilliseconds);
   return true;
  },
 });
};

const setup=(count:number)=>{
 const w=world(),table=createClaimTableV1({now:w.now}),units:Unit[]=Array.from({length:count},(_,index)=>({name:"@o/"+String(index).padStart(3,"0"),expectedMilliseconds:20000})),stored=new Set<string>(),runs=new Map<string,string[]>(),firstRun=new Map<string,number>();
 return{w,table,units,stored,runs,firstRun};
};

test("two machines infer every owner of a wave exactly once and both work from the first second",async()=>{
 const {w,table,units,stored,runs,firstRun}=setup(40);
 const a=machine({name:"m-a",table,units,stored,runs,firstRun,now:w.now,sleep:w.sleep,ownerMilliseconds:20000}),b=machine({name:"m-b",table,units,stored,runs,firstRun,now:w.now,sleep:w.sleep,ownerMilliseconds:20000});
 let over=false;void Promise.all([a,b]).then(()=>{over=true;});
 await w.drive(()=>over);
 expect(over).toBe(true);
 expect(stored.size).toBe(40);
 expect([...runs.values()].every(who=>who.length===1)).toBe(true);
 const byMachine=new Map<string,number>();for(const who of runs.values())byMachine.set(who[0]!,(byMachine.get(who[0]!)??0)+1);
 expect(byMachine.get("m-a")!).toBeGreaterThan(8);
 expect(byMachine.get("m-b")!).toBeGreaterThan(8);
 expect(firstRun.get("m-a")!).toBeLessThanOrEqual(1000);
 expect(firstRun.get("m-b")!).toBeLessThanOrEqual(1000);
 expect(w.now()).toBeLessThan(40*20000/2*1.6);
});

test("a machine that dies with claims blocks its owners for no longer than the lease, the others infer them once",async()=>{
 const {w,table,units,stored,runs,firstRun}=setup(24);
 const dying=machine({name:"m-dead",table,units,stored,runs,firstRun,now:w.now,sleep:w.sleep,ownerMilliseconds:20000,dies:{afterClaims:3}}),a=machine({name:"m-a",table,units,stored,runs,firstRun,now:w.now,sleep:w.sleep,ownerMilliseconds:20000}),b=machine({name:"m-b",table,units,stored,runs,firstRun,now:w.now,sleep:w.sleep,ownerMilliseconds:20000});
 let over=false;void Promise.all([dying,a,b]).then(()=>{over=true;});
 await w.drive(()=>over);
 expect(over).toBe(true);
 expect(stored.size).toBe(24);
 const live=[...runs].filter(([,who])=>who.some(name=>name!=="m-dead"));
 expect(live.every(([,who])=>who.filter(name=>name!=="m-dead").length===1)).toBe(true);
 expect(P.claimTtlMilliseconds).toBeLessThanOrEqual(60000);
 expect(w.now()).toBeLessThan(P.claimTtlMilliseconds+24*20000/2+60000);
});

test("a claim is a lease: renewed by the heartbeat it outlives the lease time, left alone it lapses at the lease time",()=>{
 let clock=0;const table=createClaimTableV1({now:()=>clock}),ttl=P.claimTtlMilliseconds;
 expect(table.claim("pool","ac","x",{owner:"a",ttlMilliseconds:ttl,expectedMilliseconds:20000}).state).toBe("granted");
 for(let beat=0;beat<Math.ceil(ttl*1.5/heartbeat)+1;beat++){clock+=heartbeat;expect(table.renew("pool","ac","x",{owner:"a",ttlMilliseconds:ttl}).state).toBe("granted");}
 expect(clock).toBeGreaterThan(ttl);
 expect(table.claim("pool","ac","x",{owner:"b",ttlMilliseconds:ttl}).state).toBe("held");
 clock+=ttl+1;
 expect(table.claim("pool","ac","x",{owner:"b",ttlMilliseconds:ttl}).state).toBe("granted");
 expect(heartbeat*3).toBeLessThanOrEqual(ttl);
});
