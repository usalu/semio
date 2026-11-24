/** 🚪️ Tests for the upfront storage admission of a native build. */
import { expect, test } from "bun:test";
import { admitNativeBuildUpfrontV1 } from "../../🟦️.ts";
import { renderRepoErrorV1 } from "../../../../../🏗️build/🗣️language/🚨️error/🟦️.ts";

const GIB=1024**3,upfront={upfrontPlanShareBasisPoints:5000,upfrontSlackCreditBasisPoints:5000};
test("upfront admission: a build that cannot fit is refused in both languages before any pass, a roomy host admits, and a shortfall is first offered to the cache collector",async()=>{
 const batches=[{bytes:20*GIB},{bytes:10*GIB}],base={partition:"/System/Volumes/Data",batches,reserveBytes:2*GIB,slack:()=>4*GIB,policy:upfront};
 await admitNativeBuildUpfrontV1({...base,free:()=>100*GIB});
 const refused=await admitNativeBuildUpfrontV1({...base,free:()=>11*GIB}).catch(error=>error);
 expect(refused.code).toBe("refused");expect(renderRepoErrorV1(refused,"de")).toContain("Nicht genug Speicherplatz für diesen Build: Partition /System/Volumes/Data hat noch 11 GiB frei, der Build braucht etwa 18 GiB neuen Platz");expect(renderRepoErrorV1(refused,"en")).toContain("so 9 GiB are missing. Nothing was started");
 const asked:number[]=[],lines:string[]=[];let free=11*GIB;
 await admitNativeBuildUpfrontV1({...base,free:()=>free,log:line=>lines.push(line),reclaim:async request=>{asked.push(request.needBytes);free=40*GIB;}});
 expect(asked).toEqual([9*GIB]);expect(lines[0]).toContain("9 GiB short");
 const still=await admitNativeBuildUpfrontV1({...base,free:()=>11*GIB,reclaim:async()=>undefined}).catch(error=>error);
 expect(still.code).toBe("refused");
 let cancelled=false;
 await expect(admitNativeBuildUpfrontV1({...base,free:()=>11*GIB,reclaim:async()=>undefined,check:()=>{cancelled=true;throw Error("cancelled");}})).rejects.toThrow("cancelled");expect(cancelled).toBe(true);
});
