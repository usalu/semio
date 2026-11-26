import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{profileHeapOnceV1,summariseHeapSnapshotV1}from"../🟨️.mjs";

const holder=new Map<string,{name:string;rows:string[]}>();

test("the summary names the property that holds a large collection of a real snapshot",()=>{
 for(let at=0;at<20000;at++)holder.set("key-"+at+"-"+Math.random(),{name:"owner-"+at+Math.random(),rows:["row-a-"+at+Math.random(),"row-b-"+at+Math.random()]});
 (globalThis as unknown as{hold:unknown}).hold=holder;
 const summary=summariseHeapSnapshotV1(globalThis.Bun.generateHeapSnapshot());
 const top=summary.retainers.find(row=>row.path.includes("hold:Map"));
 expect(top).toBeDefined();
 expect(top!.bytes).toBeGreaterThan(1_000_000);
});

test("one snapshot per process, only past the minimum and with a directory, written atomically",()=>{
 const directory=mkdtempSync(join(tmpdir(),"heap-profile-")),tiny={nodes:[0,0,0,0],edges:[],nodeClassNames:["<root>"],edgeNames:[],edgeTypes:[]};
 try{
  expect(profileHeapOnceV1({directory,managedMegabytes:10,generate:()=>tiny,minimum:100})).toBeUndefined();
  expect(profileHeapOnceV1({directory:undefined,managedMegabytes:500,generate:()=>tiny,minimum:100})).toBeUndefined();
  expect(profileHeapOnceV1({directory,managedMegabytes:500,generate:()=>tiny,minimum:100,name:"x.json"})?.nodes).toBe(1);
  expect(JSON.parse(readFileSync(join(directory,"x.json"),"utf8")).managedMegabytes).toBe(500);
  expect(existsSync(join(directory,"x.json.tmp"))).toBe(false);
  expect(profileHeapOnceV1({directory,managedMegabytes:900,generate:()=>tiny,minimum:100})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
