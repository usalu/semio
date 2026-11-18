import{expect,test}from"bun:test";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolIdleCoresV1,ownerPoolTargetV1,type OwnerPoolHostV1}from"../../🟦️.ts";

const host=(extra:Partial<OwnerPoolHostV1>={}):OwnerPoolHostV1=>({cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",load:null,swapOutMegabytesPerSecond:null,foreground:{cpu:0,unrealizedMegabytes:0,waiters:0},...extra});
const times=(user:number,idle:number)=>({times:{user,nice:0,sys:0,idle,irq:0}});

/** 🎚️ The lanes of the pool follow the free memory and the idle cores: other processes and the machine's own farm are no reason to run one child. */
test("the idle cores between two cpu samples are the idle share of the time that passed times the cores",()=>{
 expect(ownerPoolIdleCoresV1([times(0,0),times(0,0)],[times(100,100),times(50,150)])).toBeCloseTo(1.25,10);
 expect(ownerPoolIdleCoresV1([times(0,0)],[times(0,0)])).toBeNull();
 expect(ownerPoolIdleCoresV1([times(0,0)],[times(1,1),times(1,1)])).toBeNull();
 expect(ownerPoolIdleCoresV1([],[])).toBeNull();
});

test("a loud machine with free memory keeps a quarter of its cores, measured idle cores add children, and the machine's own waiting work still wins",()=>{
 const quarter=Math.ceil(10*P.minimumShareBasisPoints/10000);
 for(const load of [18,42.7,200])expect(ownerPoolTargetV1({host:host({load}),children:1,childMegabytes:1800}).children).toBeGreaterThanOrEqual(quarter);
 expect(ownerPoolTargetV1({host:host({load:42.7,idleCores:4}),children:3,childMegabytes:800}).children).toBe(7);
 expect(ownerPoolTargetV1({host:host({load:42.7,idleCores:0}),children:5,childMegabytes:800}).children).toBe(5);
 expect(ownerPoolTargetV1({host:host({foreground:{cpu:10,unrealizedMegabytes:0,waiters:0}}),children:1,childMegabytes:800}).children).toBe(quarter);
 expect(ownerPoolTargetV1({host:host({foreground:{cpu:10,unrealizedMegabytes:0,waiters:3}}),children:1,childMegabytes:800}).children).toBe(1);
 expect(ownerPoolTargetV1({host:host({availableMegabytes:5000,load:99}),children:1,childMegabytes:1800}).children).toBe(1);
});
