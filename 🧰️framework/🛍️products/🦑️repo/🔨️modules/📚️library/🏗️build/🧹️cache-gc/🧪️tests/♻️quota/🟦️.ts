import{expect,test}from"bun:test";
import{cacheLedgerV1}from"../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import{reclaimGraphOptionalV1,type GraphReclaimPortsV1}from"../../🚰️reclaim/🟦️.ts";

const GIB=1024**3,MIB=1024**2,ledger=cacheLedgerV1(),target=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!.targetAvailableBytes;
type Lru=Awaited<ReturnType<NonNullable<GraphReclaimPortsV1["lru"]>>>;
const over:Lru={usedBytes:GIB,maximumBytes:GIB,targetBytes:512*MIB,remainingBytes:200*MIB,retiredGroups:2,retiredFiles:7,retiredBytes:824*MIB,busy:1,protected:1,cancelled:false};
/** 🧪️ Reclaim ports that log every call; the optional cache reports `lru` and the image always has `free` bytes. */
function fake(free:number,log:string[],lru:Lru):GraphReclaimPortsV1{
 return{
  exclusive:async resource=>{log.push("lease:"+resource);return{release:()=>{log.push("release");}};},
  open:async()=>{log.push("open");return{close:async()=>{log.push("close");}};},
  closures:async()=>{log.push("closures");return{candidates:0,removed:0,bytes:0,busy:0,unvouched:0,cancelled:false};},
  free:async()=>{log.push("free");return free;},
  selected:async()=>{log.push("selected");},
  optional:async()=>{log.push("optional");},
  lru:async(_image,dryRun)=>{log.push("lru:"+dryRun);return lru;}
 };
}

test("the collector enforces the optional cache's own limit even with ample free space: a dry run reports the over-limit area, a run retires the least recently used groups first and completes",async()=>{
 const planned:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"en",fake(target+GIB,planned,over))).toBe("The optional graph cache holds 1.00 GiB against its limit of 1.00 GiB, above the low-water mark of 512.00 MiB; would retire 2 least recently used groups (824.00 MiB), leaving 200.00 MiB; 1 groups busy, 1 in use\nThe graph image has 3.00 GiB free, at least the target of 2.00 GiB; nothing to reclaim");
 expect(planned).toEqual(["lease:repository-graph-optional-collector:/w","open","lru:true","closures","free","close","release"]);
 const done:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"de",fake(target+GIB,done,over))).toBe("2 am längsten unbenutzte optionale Graph-Gruppen entfernt (7 Dateien, 824.00 MiB); 200.00 MiB bleiben bei einer Niedrigwassermarke von 512.00 MiB (Grenze 1.00 GiB); 1 Gruppen belegt, 1 in Gebrauch\nDas Graph-Image hat 3.00 GiB frei, mindestens das Ziel von 2.00 GiB; nichts zurückzugewinnen");
 expect(done.indexOf("lru:false")).toBeLessThan(done.indexOf("closures"));
 const within:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"en",fake(target+GIB,within,{...over,usedBytes:300*MIB,remainingBytes:300*MIB,retiredGroups:0,retiredFiles:0,retiredBytes:0,busy:0,protected:0}))).toStartWith("The optional graph cache holds 300.00 MiB against its limit of 1.00 GiB; within the low-water mark of 512.00 MiB\n");
 const cancelled:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake(target+GIB,cancelled,{...over,cancelled:true}))).toEndWith("\n"+(await import("../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts")).cacheTextV1("en","error.cancelled"));
 expect(cancelled).not.toContain("closures");expect(cancelled.slice(-2)).toEqual(["close","release"]);
});
