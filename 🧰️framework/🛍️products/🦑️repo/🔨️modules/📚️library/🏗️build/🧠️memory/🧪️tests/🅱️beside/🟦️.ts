/** 🅱️ Contract: what the memory governor reserves for an owner is what the child that inferred it really took (never the growth of the parent or of the tree), and an owner that has waited the wait bound runs beside parked owners as soon as it fits the memory that is really free, not only when it would run alone.
 * @see ../../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{GRAPH_MEMORY_POLICY_V1,createMemoryGovernorV1}from"../../🟨️.mjs";

const MB=1048576,fast={...GRAPH_MEMORY_POLICY_V1,admitMinimumMilliseconds:0,pollMilliseconds:1,waitBoundMilliseconds:300,progressMilliseconds:60000};
const within=async<T>(promise:Promise<T>,milliseconds:number)=>Promise.race([promise,new Promise<"late">(done=>setTimeout(()=>done("late"),milliseconds))]);

test("eight parked owners that reserve more than the free memory do not hold back an owner that fits it: after the wait bound it runs beside them",async()=>{
 const governor=createMemoryGovernorV1({workers:10,headroom:()=>15500*MB,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({"language:cargo":[1900*MB,1900*MB,1900*MB],project:[500*MB,500*MB,500*MB]}),write:()=>undefined}});
 const release:(()=>void)[]=[],parked=Array.from({length:8},(_all,index)=>governor.run(()=>new Promise<void>(done=>release.push(done)),{kind:"language:cargo",key:"parked-"+index}));
 await Bun.sleep(100);
 const reports:string[]=[];
 const small=governor.run(async()=>"ran",{kind:"project",key:"small"});
 expect(await within(small,4000),"the small owner ran beside the parked ones").toBe("ran");
 void reports;
 for(const done of release)done();
 await Promise.all(parked);governor.dispose();
});

test("an owner larger than the one owner is granted keeps waiting beside parked owners even when the memory is free (the rule never admits what the grant forbids)",async()=>{
 const governor=createMemoryGovernorV1({workers:10,headroom:()=>9000*MB,estimateBytes:256*MB,policy:fast,peaks:{read:()=>({"root:language:cargo":[3000*MB,3000*MB,3000*MB]}),write:()=>undefined}});
 const release:(()=>void)[]=[],parked=[0,1,2].map(index=>governor.run(()=>new Promise<void>(done=>release.push(done)),{kind:"root:language:cargo",key:"parked-"+index}));
 await Bun.sleep(50);
 const big=governor.run(async()=>"ran",{kind:"root:language:cargo",key:"big"}),answer=await within(big,1500);
 expect(answer,"3000 MB are above the 2048 MB one owner is granted").toBe("late");
 for(const done of release)done();
 await Promise.all(parked);expect(await within(big,3000)).toBe("ran");governor.dispose();
});

test("the cost and the learned peak of an owner are what its child measured, not what the parent grew by meanwhile",async()=>{
 let live=0;const rows:{key:string;takenBytes:number}[]=[],written:Record<string,number[]>[]=[];
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>40000*MB-live,estimateBytes:256*MB,policy:fast,measured:key=>key==="farmed"?300*MB:undefined,costs:{record:row=>rows.push({key:row.key,takenBytes:row.takenBytes})},peaks:{read:()=>({}),write:value=>{written.push(structuredClone(value) as Record<string,number[]>);}}});
 await governor.run(async()=>{live+=3000*MB;await Bun.sleep(40);live-=3000*MB;},{kind:"language:cargo",key:"farmed"});
 expect(rows).toEqual([{key:"farmed",takenBytes:300*MB}]);
 expect(governor.stats().kinds["language:cargo"]).toBe(300);
 await governor.run(async()=>{live+=700*MB;await Bun.sleep(40);live-=700*MB;},{kind:"language:rust",key:"inline"});
 expect(rows[1]!.takenBytes).toBeGreaterThanOrEqual(650*MB);expect(rows[1]!.takenBytes).toBeLessThanOrEqual(750*MB);
 governor.dispose();
});
