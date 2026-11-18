import{expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"../../../../../🧪️tests/🕸️graph/🧲️owner-claims/🔣️.json";
import{createRowLogV1}from"../../../🗃️row-log/🟨️.mjs";
import{rowKeyV1}from"../../../🧮️fact-rows/🟨️.mjs";
import{shapeComputeResultV1}from"../../🟨️.mjs";

type Row={kind:string;path:string;fact:unknown};
const keys=(rows:Iterable<Row>)=>[...new Set([...rows].map(row=>rowKeyV1(row)))];
const listed=(rows:{rows():Iterable<Row>}|readonly Row[])=>Array.isArray(rows)?rows as Row[]:[...(rows as {rows():Iterable<Row>}).rows()];

test("a part whose rows are a row log list claims exactly what the independent model and the array path claim over sixty worlds",()=>{
 const directory=mkdtempSync(join(tmpdir(),"streamed-shape-"));
 try{
  fixture.worlds.forEach((world,index)=>{
   const log=createRowLogV1<Row>({path:join(directory,"w"+index+".rows")}),rows=world.rows as Row[];
   const part=(partRows:unknown)=>({owners:world.owners.map(owner=>({name:owner.name,root:owner.root,...("nativeRoot"in owner?{nativeRoot:(owner as{nativeRoot:string}).nativeRoot}:{}),node:{},nodeRows:log.append(owner.nodeRows.map(at=>rows[at]!))})),edgeOnly:world.edgeOnly,edges:[],rows:partRows,env:{digest:"e".repeat(64),rows:[]}});
   const streamed=shapeComputeResultV1({workspaceRoot:world.workspaceRoot,part:part(log.append(world.partRows.map(at=>rows[at]!))) as never,stubs:world.stubs as never});
   const expected=world.expect as{owners:Record<string,number[]>;environment:number[]};
   for(const owner of streamed.owners)expect(keys(listed(owner.rows as never)),"world "+index+" owner "+owner.name).toEqual(keys(expected.owners[owner.name]!.map(at=>rows[at]!)));
   expect(keys(listed(streamed.env.rows as never)),"world "+index+" environment").toEqual(keys(expected.environment.map(at=>rows[at]!)));
   log.close();
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("shaping a row log keeps a few bytes per row, flat in the rows per owner",()=>{
 const directory=mkdtempSync(join(tmpdir(),"streamed-shape-law-"));
 const measure=(owners:number,perOwner:number)=>{
  const log=createRowLogV1<Row>({path:join(directory,"law-"+owners+"-"+perOwner+".rows")}),root="/ws";
  const lists=Array.from({length:owners},(_unused,owner)=>log.append(Array.from({length:perOwner},(_row,at)=>({kind:"file",path:root+"/pkg/p"+owner+"/src/f"+at+".ts",fact:{size:at,sha256:"0".repeat(60)+String(owner%1000).padStart(4,"0")}}))));
  const builder=log.builder();for(const list of lists)builder.addList(list as never);
  const outside=log.append(Array.from({length:50},(_row,at)=>({kind:"file",path:"/elsewhere/f"+at,fact:{size:at}})));builder.addList(outside as never);
  const all=builder.finish();
  const part={owners:lists.map((nodeRows,owner)=>({name:"p"+owner,root:"pkg/p"+owner,node:{},nodeRows})),edgeOnly:[],edges:[],rows:all,env:{digest:"e".repeat(64),rows:[]}};
  log.read(Uint32Array.of(0));(globalThis as{Bun?:{gc(force:boolean):void}}).Bun?.gc(true);const before=process.memoryUsage();
  const shaped=shapeComputeResultV1({workspaceRoot:root,part:part as never,stubs:{}});
  (globalThis as{Bun?:{gc(force:boolean):void}}).Bun?.gc(true);const after=process.memoryUsage();
  const retained=Math.max(0,after.heapUsed+after.arrayBuffers-before.heapUsed-before.arrayBuffers),envCount=(shaped.env.rows as {length:number}).length;
  log.close();return{retained,rows:owners*perOwner,envCount};
 };
 try{
  const small=measure(300,40),large=measure(300,160);
  expect(small.envCount).toBe(50);expect(large.envCount).toBe(50);
  expect(large.retained/large.rows,"bytes retained per row").toBeLessThan(24);
  expect(large.retained).toBeLessThan(Math.max(small.retained*4,4*1048576));
 }finally{rmSync(directory,{recursive:true,force:true});}
});
