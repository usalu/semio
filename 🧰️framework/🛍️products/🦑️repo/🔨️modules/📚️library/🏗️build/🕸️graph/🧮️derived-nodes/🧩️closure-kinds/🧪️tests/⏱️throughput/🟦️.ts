/** ⏱️ Throughput law of the closure settler: thousands of closures over shared parts settle within a bound per closure, cold (every closure walked, parts and manifest written) and warm (a new settler over the stored manifest and a journal that names a few files: nothing walked, almost nothing checked), in-process and with the per-closure work of a real walk. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{CLOSURE_SETTLE_V1,createClosureSettlerV1}from"../../🟨️.mjs";
import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";

const COUNT=3000,names=Array.from({length:COUNT},(_,at)=>"q"+String(at).padStart(5,"0"));
const shared=Array.from({length:60},(_,part)=>Array.from({length:40},(_,row)=>({kind:"lstat",path:"/ws/shared/"+part+"/f"+row+".ts",fact:"file"})));

test("3 000 closures settle within a bound per closure, cold and warm",async()=>{
 const store=mkdtempSync(join(tmpdir(),"throughput-")),derived:string[]=[];
 try{
  const settle=async(dirty?:{paths:string[]})=>{
   const settler=createClosureSettlerV1({store:createNodeStoreV1({root:store}),limits:{...CLOSURE_SETTLE_V1,deriveConcurrency:4},farm:false});
   const began=performance.now(),settled=await settler.settle({pseudoRows:new Map(names.map(name=>[name,{kind:"derived",path:name,fact:"old"}])),derive:async(row:{path:string})=>{derived.push(row.path);const at=Number(row.path.slice(1));return{fact:"old",parts:[shared[at%60]!,shared[(at*7+3)%60]!,[{kind:"lstat",path:"/ws/own/"+row.path,fact:"file"}]]};},hold:async()=>true,...dirty?{dirty}:{}} as never);
   await settler.markIndexed({held:settled.held});
   return{settled,milliseconds:performance.now()-began};
  };
  const cold=await settle();
  expect(derived.length).toBe(COUNT);expect(cold.settled.failed).toBe(0);
  expect(cold.milliseconds/COUNT,"cold milliseconds per closure").toBeLessThan(25);
  derived.length=0;
  const warm=await settle({paths:["/ws/shared/3/f1.ts","/ws/elsewhere/x.ts"]});
  expect(derived,"nothing is walked while the rows hold").toEqual([]);
  expect(warm.settled.failed).toBe(0);expect(warm.settled.held.size+warm.settled.kept.size).toBe(COUNT);
  expect(warm.milliseconds/COUNT,"warm milliseconds per closure").toBeLessThan(15);
  derived.length=0;
  const again=await settle({paths:[]});
  expect(again.milliseconds/COUNT,"idle milliseconds per closure").toBeLessThan(15);
 }finally{rmSync(store,{recursive:true,force:true});}
},180000);
