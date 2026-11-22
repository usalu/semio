import{expect,test}from"bun:test";
import{registerSliceReportSourceV1,sliceReadStepV1,describeSliceReadStepV1}from"../../🟨️.mjs";

/** 🧭️ Contract: the inventory step counts the seconds since it last made progress (an owner check, a verified memo row, an observation of this read) against its stall bound, and counts observations of this read, not of the process. */
test("inventory done is the quiet time, reset by every kind of progress, and observations are counted from the step start",async()=>{
 let interned=5000,checks=0,rows=0,admitted=0;
 const remove=registerSliceReportSourceV1("caches",()=>({interned}));
 try{
  await sliceReadStepV1("inventory",async()=>{
   const base=performance.now(),at=(seconds:number)=>describeSliceReadStepV1(base+seconds*1000)!;
   expect(at(0).phase).toContain("(0 distinct source observations)");
   expect(at(100).done).toBe(100);
   interned=5040;
   expect(at(100).phase).toContain("(40 distinct source observations)");expect(at(100).done).toBe(0);
   expect(at(250).done).toBe(150);
   checks=1;
   expect(at(250).done).toBe(0);expect(at(250).phase).toContain("1 completed owner checks");
   rows=64;
   expect(at(300).done).toBe(0);expect(at(300).phase).toContain("64 verified memo rows");
   admitted=949;
   expect(at(300).done).toBe(0);expect(at(300).phase).toContain("949 owners admitted)");expect(at(300).phase).toContain("949 zugelassene Projekte");
   expect(at(1500).done).toBe(1200);expect(at(1500).total).toBe(600);
  },{},()=>({version:1,active:{phase:"scoped-discovery",elapsedMilliseconds:0,completedOwners:checks,...(admitted?{admittedOwners:admitted}:{}),...(rows?{verifiedMemoRows:rows}:{})},completed:[]}));
 }finally{remove();}
});
