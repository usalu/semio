/** 📸️ Contract: the slice heap diagnostic is opt-in (directory named), fires once above its threshold, and writes a small summary that names the classes and the retainers holding most bytes.
 * @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{HEAP_SNAPSHOT_POLICY_V1,heapSnapshotControlV1,heapSnapshotDecisionV1,summarizeHeapSnapshotV1,writeHeapSummaryV1}from"../🟨️.mjs";

const MiB=1048576,policy=HEAP_SNAPSHOT_POLICY_V1;
const synthetic={
 nodeClassNames:["<root>","Memo","Row","Array"],edgeNames:["rows","owners"],edgeTypes:["Internal","Property","Index"],
 nodes:[1,0,0,0, 2,100,1,0, 3,40,2,0, 4,40,2,0, 5,16,3,0],
 edges:[1,2,1,0, 2,5,1,0, 5,3,2,0, 5,4,2,1],
};

test("the diagnostic is off without a directory, fires once above the threshold and not before",()=>{
 const environment={[policy.directoryEnvironment]:"/x",[policy.thresholdEnvironment]:"3000"};
 expect(heapSnapshotDecisionV1({environment:{},heapBytes:9e9})).toBeUndefined();
 expect(heapSnapshotDecisionV1({environment,heapBytes:2999*MiB})).toBeUndefined();
 expect(heapSnapshotDecisionV1({environment,heapBytes:3001*MiB,taken:true})).toBeUndefined();
 expect(heapSnapshotDecisionV1({environment,heapBytes:3001*MiB})).toEqual({directory:"/x",thresholdMegabytes:3000,heapMegabytes:3001});
 expect(heapSnapshotDecisionV1({environment:{[policy.directoryEnvironment]:"/x",[policy.thresholdEnvironment]:"1"},heapBytes:(policy.defaultThresholdMegabytes+1)*MiB})?.thresholdMegabytes).toBe(policy.defaultThresholdMegabytes);
 expect(heapSnapshotDecisionV1({environment:{[policy.directoryEnvironment]:"/x"},heapBytes:Number.NaN})).toBeUndefined();
});

test("the summary names the classes and the edge that retains most bytes",()=>{
 const summary=summarizeHeapSnapshotV1(synthetic);
 expect(summary.nodes).toBe(5);expect(summary.totalBytes).toBe(196);
 expect(summary.classes[0]).toEqual({name:"Memo",count:1,bytes:100});
 expect(summary.classes.find(row=>row.name==="Row")).toEqual({name:"Row",count:2,bytes:80});
 expect(summary.retainers[0]).toEqual({retainer:"<root>.rows -> Memo",count:1,bytes:100});
 expect(summary.retainerPaths[0]).toEqual({path:"Memo rows <- <root>",count:1,bytes:100});
 expect(summary.largestObjects[0]).toEqual({bytes:100,path:"Memo rows <- <root>"});
 expect(summary.retainerPaths.find(row=>row.path.startsWith("Row "))?.path).toBe("Row [] <- Array rows <- Memo rows <- <root>");
});

test("the summary is written once, as a small file next to nothing else, from the runtime's own snapshot",()=>{
 const directory=mkdtempSync(join(tmpdir(),"heap-summary-"));
 try{
  const keep:{label:string;rows:number[]}[]=Array.from({length:2000},(_,index)=>({label:"owner-"+index,rows:[index]}));
  const path=writeHeapSummaryV1({directory,pid:7,now:1,extra:{heapMegabytes:1}});
  expect(readdirSync(directory)).toEqual([policy.filePrefix+"7-1.json"]);
  const written=JSON.parse(readFileSync(path,"utf8"));
  expect(written.pid).toBe(7);expect(written.heapMegabytes).toBe(1);expect(written.classes.length).toBeLessThanOrEqual(policy.topRows);expect(written.retainers.length).toBeGreaterThan(0);
  expect(written.classes.some((row:{name:string})=>row.name==="Array"||row.name==="Object")).toBe(true);
  expect(keep.length).toBe(2000);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a synthetic scope held in a growing memo keeps its heap inside a budget once the rows are shared",()=>{
 const owners=400,rowsPerOwner=300,distinct=2000,pool=Array.from({length:distinct},(_,index)=>({kind:"file",path:"/w/f"+index,fact:{sha256:"a".repeat(64)}}));
 Bun.gc(true);const before=process.memoryUsage().heapUsed;
 const shared=Array.from({length:owners},(_,owner)=>({facts:{owner},rows:Array.from({length:rowsPerOwner},(_,at)=>pool[(owner*7+at)%distinct])}));
 Bun.gc(true);const grown=(process.memoryUsage().heapUsed-before)/MiB;
 expect(shared.length).toBe(owners);expect(grown).toBeLessThan(40);
});

test("a service without the caller's environment is steered by the control file, the environment wins where both exist",()=>{
 const directory=mkdtempSync(join(tmpdir(),"heap-control-"));
 try{
  expect(heapSnapshotControlV1({directory})).toBeUndefined();
  writeFileSync(join(directory,policy.controlFile),JSON.stringify({directory:"/from-file",thresholdMegabytes:2500}));
  const control=heapSnapshotControlV1({directory});
  expect(control).toEqual({directory:"/from-file",thresholdMegabytes:2500});
  expect(heapSnapshotDecisionV1({environment:{},heapBytes:2501*MiB,control})).toEqual({directory:"/from-file",thresholdMegabytes:2500,heapMegabytes:2501});
  expect(heapSnapshotDecisionV1({environment:{},heapBytes:2499*MiB,control})).toBeUndefined();
  expect(heapSnapshotDecisionV1({environment:{[policy.directoryEnvironment]:"/env",[policy.thresholdEnvironment]:"100"},heapBytes:101*MiB,control})?.directory).toBe("/env");
  writeFileSync(join(directory,policy.controlFile),"not json");
  expect(heapSnapshotControlV1({directory})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
