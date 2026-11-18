/** 🫀️ Contract: a registered dependency store hold is a lease with a heartbeat: a live pid whose start the operating system could not tell and whose registration is older than the heartbeat bound is the dead owner of a reused pid and never blocks the store (g1-m05 waited on pid 15372 for over an hour), while a holder with a known matching start stays live and is reported as silent. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{observeDependencyStoreHoldersV1,dependencyStoreHolderPresentV1,dependencyStoreWaitKeyV1,dependencyStoreWaitLineV1,DEPENDENCY_STORE_WAIT_V1,type DependencyStoreRegistryPortsV1}from"../../🟦️.ts";

const registered=1_000_000_000_000,now=registered+10_000_000,stale=DEPENDENCY_STORE_WAIT_V1.heartbeatStaleMilliseconds,entry=(pid:number,updatedAtMs:number)=>({path:"/registry/"+pid+".json",entry:{version:1,pid,startedAtMs:registered,updatedAtMs,command:"d082 show projects",directory:"/d",holds:[{resource:"repository:dependencies-store",mode:"exclusive",sinceMs:registered+10}]}}) as never;
const ports=(rows:readonly[number,number][],started:(pid:number)=>number|undefined,removed:string[]):DependencyStoreRegistryPortsV1=>({entries:()=>rows.map(([pid,updated])=>entry(pid,updated)),alive:()=>true,osStartedAtMs:started,now:()=>now,ancestors:()=>[],remove:path=>{removed.push(path);},pid:1,startedAtMs:registered+999999});

test("a silent registration with an unknown start is gone and removed; fresh or known starts stay live",()=>{
 const removed:string[]=[],fresh=now-stale,silent=now-stale-1;
 const seen=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",ports([[15372,silent],[7,fresh],[9,silent]],pid=>pid===9?registered:undefined,removed));
 expect(seen.gone.map(row=>row.pid)).toEqual([15372]);expect(removed).toEqual(["/registry/15372.json"]);
 expect(seen.live.map(row=>row.pid).sort((a,b)=>a-b)).toEqual([7,9]);
 expect(seen.live.find(row=>row.pid===9)!.silentMs).toBe(stale+1);expect(seen.live.find(row=>row.pid===7)!.silentMs).toBe(stale);
});

test("an ancestor with an unknown start is never judged by its heartbeat",()=>{
 const removed:string[]=[],base=ports([[15372,now-10*stale]],()=>undefined,removed),seen=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",{...base,ancestors:()=>[15372],startedAtMs:registered+1});
 expect(seen.gone).toEqual([]);expect(removed).toEqual([]);expect(seen.live.map(row=>[row.pid,row.ancestor])).toEqual([[15372,true]]);
});

test("the presence rule is exact around the heartbeat bound and the start tolerance",()=>{
 const tolerance=DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;
 expect(dependencyStoreHolderPresentV1(false,registered,registered,0)).toBe(false);
 expect(dependencyStoreHolderPresentV1(true,registered,undefined,stale)).toBe(true);expect(dependencyStoreHolderPresentV1(true,registered,undefined,stale+1)).toBe(false);
 expect(dependencyStoreHolderPresentV1(true,registered,registered,10*stale)).toBe(true);
 expect(dependencyStoreHolderPresentV1(true,registered,registered+tolerance,0)).toBe(true);expect(dependencyStoreHolderPresentV1(true,registered,registered+tolerance+1,0)).toBe(false);
});

test("a wait line names a silent holder differently and its values fill exactly the placeholders of both languages",()=>{
 const holder=(silentMs:number)=>({pid:1,command:"c",sinceMs:0,silentMs,mode:"exclusive" as const,ancestor:false});
 expect(dependencyStoreWaitKeyV1()).toBe("dependency-store.waiting");expect(dependencyStoreWaitKeyV1(holder(stale))).toBe("dependency-store.waiting-for-holder");expect(dependencyStoreWaitKeyV1(holder(stale+1))).toBe("dependency-store.waiting-for-silent-holder");
 const texts=JSON.parse(readFileSync(new URL("../../../../../../../../../📖️texts/🔣️.json",import.meta.url),"utf8")).messages,holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/g)].map(match=>match[1]).sort();
 for(const line of[dependencyStoreWaitLineV1(5,60),dependencyStoreWaitLineV1(5,60,holder(0)),dependencyStoreWaitLineV1(5,60,holder(stale+1)),dependencyStoreWaitLineV1(5,60,{...holder(0),silentMs:undefined})]){
  const row=texts[line.key];expect(row,line.key).toBeDefined();expect(holes(row.en),line.key).toEqual(holes(row.de));expect(Object.keys(line.values).sort(),line.key).toEqual(holes(row.en));
 }
 expect(dependencyStoreWaitLineV1(5,60,holder(stale+1)).values.silent).toBe(Math.round((stale+1)/1000));
});
