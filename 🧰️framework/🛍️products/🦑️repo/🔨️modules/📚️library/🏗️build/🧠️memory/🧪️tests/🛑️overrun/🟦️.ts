import{expect,test}from"bun:test";
import{GraphOwnerTooHeavyErrorV1,createOwnerScopeV1,creditOwnerSharedV1,ownerTooHeavyOfV1,runInOwnerScopeV1,throwIfOwnerExceededV1}from"../../🎯️owner-scope/🟨️.mjs";
import{createGraphParentMemoryGovernorV1,createMemoryGovernorV1,createMemoryPortsV1}from"../../🟨️.mjs";

/** 🛑️ Contract: one project that grows past what the machine can give one project is stopped by the governor that measures it, before the host runs short of memory; it is confirmed by a collection first (garbage is no growth), released through its own scope, recorded under its own name and never taught to its kind; the light owners beside it are not touched. */
const MB=1048576,wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Row={key:string;kind:string;takenBytes:number;seconds:number;stopped:boolean};

function harness(limitMegabytes:number){
 const state={bytes:0,collections:0,garbage:0},rows:Row[]=[],stops:{key:string;takenBytes:number;limitBytes:number}[]=[],written:Record<string,number[]>[]=[],lines:string[]=[];
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>10**12,measure:()=>state.bytes,collect:()=>{state.collections++;state.bytes-=state.garbage;state.garbage=0;},ownerLimitBytes:limitMegabytes*MB,costs:{record:row=>rows.push(row)},onStop:stop=>stops.push(stop),peaks:{read:()=>({}),write:peaks=>written.push(peaks)},report:line=>lines.push(line),environment:{LANG:"en_US.UTF-8"},policy:{...createMemoryGovernorPolicy(),stopCollectMinimumMilliseconds:0}});
 return{state,rows,stops,written,lines,governor};
}
function createMemoryGovernorPolicy(){return{...JSON.parse(JSON.stringify(require("../../🧬️schema/🔣️.json").definitions.Policy.const))};}

test("a synthetic inference that grows past the limit is stopped at its next yield, long before the host would run short; the stop is recorded, worded and not learned into the kind",async()=>{
 const {state,rows,stops,written,lines,governor}=harness(1000);let reached=0,error:unknown;
 try{
  await governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<500;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;reached=state.bytes;await wait(10);}}),{kind:"project",key:"@semio-tech/s-dev-composition"});
 }catch(reason){error=reason;}finally{governor.dispose();}
 expect(error).toBeInstanceOf(GraphOwnerTooHeavyErrorV1);
 const detail=ownerTooHeavyOfV1(error)!;expect(detail.name).toBe("@semio-tech/s-dev-composition");expect(detail.limitBytes).toBe(1000*MB);expect(detail.takenBytes).toBeGreaterThan(1000*MB);
 expect(reached).toBeLessThan(5000*MB);expect(state.collections).toBeGreaterThanOrEqual(1);
 expect(stops).toHaveLength(1);expect(rows).toHaveLength(1);expect(rows[0]).toMatchObject({key:"@semio-tech/s-dev-composition",kind:"project",stopped:true});expect(rows[0]!.takenBytes).toBeGreaterThanOrEqual(1000*MB);
 expect(written).toEqual([]);expect(governor.stats().kinds).toEqual({});expect(governor.stats().stops).toBe(1);
 expect(lines.filter(line=>line.includes("marked too heavy here"))).toHaveLength(1);
});

test("growth that a collection frees is garbage and not a reason to stop; a light owner beside the heavy one completes and is learned",async()=>{
 const {state,rows,stops,written,governor}=harness(1000);
 try{
  const light=governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<10;step++){throwIfOwnerExceededV1();await wait(5);}return"light";}),{kind:"project",key:"@semio-tech/light"});
  const garbage=governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<30;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;state.garbage+=100*MB;await wait(5);}return"garbage";}),{kind:"language:cargo",key:"@semio-tech/garbage"});
  expect(await Promise.all([light,garbage])).toEqual(["light","garbage"]);
 }finally{governor.dispose();}
 expect(stops).toEqual([]);expect(rows.map(row=>[row.key,row.stopped])).toEqual(expect.arrayContaining([["@semio-tech/light",false],["@semio-tech/garbage",false]]));
 expect(Object.keys(written.at(-1)!).sort()).toEqual(["language:cargo","project"]);
});

test("memory an owner builds for the owners after it is credited and never stops that owner, while its own growth still does",async()=>{
 const {state,rows,stops,governor}=harness(1000);
 try{
  await governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<35;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;creditOwnerSharedV1(100*MB);await wait(5);}}),{kind:"project",key:"@semio-tech/warmup"});
  expect(stops).toEqual([]);expect(rows[0]).toMatchObject({key:"@semio-tech/warmup",stopped:false,takenBytes:0});
  let error:unknown;
  try{await governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<50;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;creditOwnerSharedV1(10*MB);await wait(10);}}),{kind:"project",key:"@semio-tech/own"});}catch(reason){error=reason;}
  expect(error).toBeInstanceOf(GraphOwnerTooHeavyErrorV1);
 }finally{governor.dispose();}
});

test("without a limit nothing is enforced, and a limit the owner stays under never trips",async()=>{
 const free=harness(1);const unlimited=createMemoryGovernorV1({workers:1,headroom:()=>10**12,measure:()=>free.state.bytes,environment:{LANG:"en_US.UTF-8"}});
 try{await unlimited.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<10;step++){throwIfOwnerExceededV1();free.state.bytes+=500*MB;await wait(5);}}),{kind:"project",key:"x"});}finally{unlimited.dispose();free.governor.dispose();}
 expect(unlimited.stats().stops).toBe(0);
 const under=harness(2000);
 try{await under.governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<10;step++){throwIfOwnerExceededV1();under.state.bytes+=100*MB;await wait(5);}}),{kind:"project",key:"y"});}finally{under.governor.dispose();}
 expect(under.stops).toEqual([]);expect(under.rows[0]).toMatchObject({key:"y",stopped:false});
});

test("with several owners running the one whose attributed share passes the limit is stopped and the other goes on",async()=>{
 const {state,stops,governor}=harness(400);let heavyStopped=false,lightDone=false;
 try{
  const heavy=governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{try{for(let step=0;step<200;step++){throwIfOwnerExceededV1();state.bytes+=50*MB;await wait(10);}}catch(error){heavyStopped=ownerTooHeavyOfV1(error)!==undefined;throw error;}}),{kind:"project",key:"heavy"}).catch(()=>undefined);
  const light=governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<40;step++){throwIfOwnerExceededV1();await wait(10);}lightDone=true;}),{kind:"project",key:"light"});
  await Promise.all([heavy,light]);
 }finally{governor.dispose();}
 expect(heavyStopped).toBe(true);expect(lightDone).toBe(true);expect(stops.map(stop=>stop.key)).toEqual(["heavy"]);
});

test("the scope is silent outside an owner, the error is found through causes, and a stop is exceeded once",()=>{
 expect(()=>throwIfOwnerExceededV1()).not.toThrow();
 const scope=createOwnerScopeV1("p");scope.exceed({takenBytes:5,limitBytes:4});scope.exceed({takenBytes:9,limitBytes:4});
 expect(scope.exceeded).toEqual({takenBytes:5,limitBytes:4});
 let thrown:unknown;try{runInOwnerScopeV1(scope,()=>throwIfOwnerExceededV1());}catch(error){thrown=error;}
 expect(ownerTooHeavyOfV1(new Error("tag",{cause:thrown}))).toEqual({name:"p",takenBytes:5,limitBytes:4});
 expect(ownerTooHeavyOfV1(new Error("other"))).toBeUndefined();expect(ownerTooHeavyOfV1(undefined)).toBeUndefined();
});

test("real memory: a project that allocates real buffers past a 192 MB limit is stopped by the counters of this process (JavaScriptCore counts a buffer twice, in the heap and as external memory), long before it reaches 1 GB",async()=>{
 const ports=createMemoryPortsV1(),kept:ArrayBuffer[]=[],lines:string[]=[];let allocated=0,error:unknown;
 const governor=createGraphParentMemoryGovernorV1({workers:1,ports,budgetBytes:4096*MB,baselineBytes:ports.sample(),ownerLimitBytes:192*MB,environment:{LANG:"en_US.UTF-8"},report:line=>lines.push(line),headroom:()=>10**12});
 try{
  await governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<64;step++){throwIfOwnerExceededV1();const buffer=new ArrayBuffer(32*MB);new Uint8Array(buffer).fill(1);kept.push(buffer);allocated+=32*MB;await wait(60);}}),{kind:"project",key:"real"});
 }catch(reason){error=reason;}finally{governor.dispose();kept.length=0;}
 const detail=ownerTooHeavyOfV1(error);expect(detail?.name).toBe("real");expect(detail!.takenBytes).toBeGreaterThan(192*MB);expect(allocated).toBeLessThan(1024*MB);expect(allocated).toBeGreaterThanOrEqual(32*MB);
 expect(lines.some(line=>line.includes("marked too heavy here"))).toBe(true);
},60000);

test("an owner that runs alone with its own limit is held to that limit and not to the one of the governor: 1500 MB passes a 4000 MB solo limit that would have stopped it at 1000 MB, and 4500 MB does not",async()=>{
 const grow=async(megabytes:number,limitBytes:number|undefined)=>{
  const {state,governor,stops}=harness(1000);let error:unknown;
  try{await governor.run(async({scope})=>runInOwnerScopeV1(scope,async()=>{for(let step=0;step<megabytes/100;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;await wait(10);}throwIfOwnerExceededV1();}),{kind:"project",key:"solo",...limitBytes===undefined?{}:{limitBytes}});}catch(reason){error=reason;}finally{governor.dispose();}
  return{error,stops};
 };
 expect(ownerTooHeavyOfV1((await grow(1500,undefined)).error)?.limitBytes).toBe(1000*MB);
 expect((await grow(1500,4000*MB)).error).toBeUndefined();
 const over=await grow(4500,4000*MB);expect(ownerTooHeavyOfV1(over.error)?.limitBytes).toBe(4000*MB);expect(over.stops).toHaveLength(1);
});
