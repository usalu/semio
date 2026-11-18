/** 🫀️ Contract: a claim of the fleet is renewed only while its holder makes progress, so a machine that holds the claim but does nothing is taken over at once when the lease lapses, and a waiter names the holder it waits for as a step of the read
 * @see ../../🟦️.ts */
import{expect,test}from"bun:test";
import{selectedGenerationV1}from"../../🟦️.ts";
import{makeFleet,makeMachine,makeRecords,sha,ownerOfV1,type FleetV1,type MachineV1}from"../🌍️world/🟦️.ts";

const scope=sha("scope-of-the-selection-and-the-registry"),platform="darwin-arm64";
const run=(fleet:FleetV1,machine:MachineV1,extra:Partial<Parameters<typeof selectedGenerationV1>[0]>={})=>selectedGenerationV1({client:fleet.client(machine.name),scope,platform,host:machine.host,owner:machine.name,ownerOf:ownerOfV1,holds:machine.holds,install:machine.install,read:machine.read,claimTtlMilliseconds:150,claimHeartbeatMilliseconds:30,waitSliceMilliseconds:40,waitMaximumMilliseconds:5000,...extra});
const world=()=>{const records=makeRecords(),fleet=makeFleet(),machines=["a","b"].map(name=>makeMachine({name,count:8,records}));return{fleet,machines:machines as [MachineV1,MachineV1]};};

test("a holder that makes no progress stops renewing and the waiter takes over before the holder ends",async()=>{
 const{fleet,machines:[a,b]}=world(),steps:string[]=[];
 const slow=async(differing:readonly string[]|undefined)=>{await Bun.sleep(900);return a.read(differing);};
 const started=Date.now();
 const [first,second]=await Promise.all([run(fleet,a,{read:slow,alive:()=>false}),(async()=>{await Bun.sleep(40);return run(fleet,b,{step:(id,body,detail)=>{steps.push(id+":"+JSON.stringify(detail??{}));return body();}});})()]);
 expect(second.outcome).toBe("taken-over");expect(first.outcome).toBe("read");
 expect(steps.some(line=>line.startsWith("exchange-wait:")&&line.includes("\"who\":\"a\""))).toBe(true);
 expect(Date.now()-started).toBeLessThan(5000);
});

test("a holder that makes progress keeps its claim and the waiter takes its head",async()=>{
 const{fleet,machines:[a,b]}=world();
 const slow=async(differing:readonly string[]|undefined)=>{await Bun.sleep(500);return a.read(differing);};
 const [first,second]=await Promise.all([run(fleet,a,{read:slow,alive:()=>true}),(async()=>{await Bun.sleep(40);return run(fleet,b);})()]);
 expect(first.outcome).toBe("read");expect(second.outcome).toBe("waited");
});

test("every await of the exchange is a named step: look, wait, install and publish",async()=>{
 const{fleet,machines:[a,b]}=world(),steps:string[]=[],step=(id:string,body:()=>Promise<unknown>)=>{steps.push(id);return body();};
 await run(fleet,a,{step:step as never});await run(fleet,b,{step:step as never});
 expect(steps).toContain("exchange-look");expect(steps).toContain("exchange-publish");expect(steps).toContain("exchange-install");
});
