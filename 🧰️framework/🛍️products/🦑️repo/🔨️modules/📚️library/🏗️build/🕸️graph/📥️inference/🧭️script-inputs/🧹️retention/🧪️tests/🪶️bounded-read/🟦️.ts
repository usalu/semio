/** 🪶️ Contract: collecting a closure store reads its entries one small batch at a time and keeps of each only what the retention rule needs (the identity of its recipe, the layers and the value it names), never its observation rows; a store whose entries hold hundreds of megabytes of rows is collected inside a bounded heap, and the result is the one the rule gives. The collection that kept every parsed entry took 5.9 GB resident on a store of 41 429 entries (3.8 GB of rows) and was the memory that ended a 16 GB worker.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{INFERENCE_LIMITS_V1}from"../../../../🧬️schema/🟨️.mjs";
import{createClosureStoreV1}from"../../../💾️entries/🟨️.mjs";
import{collectClosureStoreV1}from"../../🟨️.mjs";

const MB=1048576,ENTRIES=80,ROWS=12000;
const rows=(entry:number)=>Array.from({length:ROWS},(_unused,at)=>({kind:"file",path:`/work/entry-${entry}/file-${at}.ts`,fact:{size:at,mtimeMs:1700000000000+at,ctimeMs:1700000000000+at,ino:at,sha256:(at.toString(16)+entry.toString(16)).padStart(64,"a")}}));

test("a store with 80 entries of 2 MB of rows is collected with a heap that grows by far less than the rows, and nothing live is removed",async()=>{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"closure-bounded-"));
 try{
  const limits={...INFERENCE_LIMITS_V1,closureStoreGraceMilliseconds:3600000,closureStoreCollectIntervalMilliseconds:0},store=createClosureStoreV1({directory,limits});
  for(let entry=0;entry<ENTRIES;entry++)await store.save("id-"+entry,{recipe:{fn:"target-sources",projectRoot:"p"+entry,target:{options:{command:"c",cwd:"."}}},value:["{workspaceRoot}/f"+entry],rows:rows(entry),layers:[]});
  await store.settled();
  Bun.gc(true);const base=process.memoryUsage().heapUsed+process.memoryUsage().external;let peak=0;
  const watch=setInterval(()=>{const used=process.memoryUsage().heapUsed+process.memoryUsage().external;if(used>peak)peak=used;},2);
  const report=await collectClosureStoreV1({directory,graceMilliseconds:3600000,now:Date.now()+10*60000});
  clearInterval(watch);
  expect(report.kept).toEqual({entries:ENTRIES,layers:0,values:ENTRIES});expect(report.removed).toEqual({entries:0,layers:0,values:0});
  expect(Math.round((peak-base)/MB)).toBeLessThan(110);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);
