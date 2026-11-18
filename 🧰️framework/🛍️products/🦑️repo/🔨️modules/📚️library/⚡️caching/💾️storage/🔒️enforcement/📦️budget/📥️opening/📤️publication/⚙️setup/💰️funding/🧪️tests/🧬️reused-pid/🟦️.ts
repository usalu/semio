/** 🧬️ Contract: a registered dependency store hold whose pid is alive but whose process started after the registration is a reused pid; its entry is removed and it never blocks the store (g1-m05 waited up to six hours on pid 48352 of two days before), while a holder that started with its registration, an unknown start and this process stay live. */
import{expect,test}from"bun:test";
import{observeDependencyStoreHoldersV1,dependencyStorePidReusedV1,DEPENDENCY_STORE_WAIT_V1,type DependencyStoreRegistryPortsV1}from"../../🟦️.ts";

const registered=1_000_000_000_000,entry=(pid:number)=>({path:"/registry/"+pid+".json",entry:{version:1,pid,startedAtMs:registered,command:"3a31 show projects",directory:"/d",holds:[{resource:"repository:dependencies-store",mode:"shared",sinceMs:registered+10}]}}) as never;
const ports=(started:(pid:number)=>number|undefined,removed:string[]):DependencyStoreRegistryPortsV1=>({entries:()=>[entry(48352),entry(7)],alive:()=>true,osStartedAtMs:started,ancestors:()=>[],remove:path=>{removed.push(path);},pid:1,startedAtMs:registered+999999});

test("a reused pid is gone and removed; the true holder, an unknown start and the tolerance stay live",()=>{
 const removed:string[]=[],tolerance=DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;
 const seen=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",ports(pid=>pid===48352?registered+2*86_400_000:registered+tolerance,removed));
 expect(seen.gone.map(row=>row.pid)).toEqual([48352]);expect(removed).toEqual(["/registry/48352.json"]);expect(seen.live.map(row=>row.pid)).toEqual([7]);
 const unknown=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",ports(()=>undefined,[]));expect(unknown.live.map(row=>row.pid).sort((a,b)=>a-b)).toEqual([7,48352]);expect(unknown.gone).toEqual([]);
});

test("the rule is exact around the tolerance and ignores unknown starts",()=>{
 const tolerance=DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;
 expect(dependencyStorePidReusedV1(registered,registered+tolerance)).toBe(false);expect(dependencyStorePidReusedV1(registered,registered+tolerance+1)).toBe(true);expect(dependencyStorePidReusedV1(registered,registered-86_400_000)).toBe(false);expect(dependencyStorePidReusedV1(registered,undefined)).toBe(false);expect(dependencyStorePidReusedV1(undefined,registered)).toBe(false);
});
