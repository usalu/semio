import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import globalFixture from"./🔣️.json";
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

test("an unscoped streamed graph keeps ordered environment rows without rereading captured paths",()=>{
 const directory=mkdtempSync(join(tmpdir(),"streamed-shape-global-")),log=createRowLogV1<Row>({path:join(directory,"rows")}),oracle=new Database(":memory:");
 try{
  const {positions,expected}=globalFixture,rows=Array.from({length:7},(_unused,at)=>({kind:"file",path:"/source/f"+at,fact:{size:at}})),take=(indices:readonly number[])=>indices.map(at=>rows[at]!);
  const all=log.append(take(positions.head)).concat(take(positions.tail)),base=log.append(take(positions.owner)),before=log.stats().reads;
  const selected=oracle.query(`WITH inputs AS (SELECT 0 AS band, CAST(key AS INT) AS slot, value AS id FROM json_each(?1)
    UNION ALL SELECT 1, CAST(key AS INT), value FROM json_each(?2) WHERE value NOT IN (SELECT value FROM json_each(?4))
    UNION ALL SELECT 2, CAST(key AS INT), value FROM json_each(?3) WHERE value NOT IN (SELECT value FROM json_each(?4))),
    ranked AS (SELECT id, band, slot, row_number() OVER (PARTITION BY id ORDER BY band, slot) AS first FROM inputs)
    SELECT id FROM ranked WHERE first=1 ORDER BY band, slot`).all(...[positions.explicit,positions.head,positions.tail,positions.owner].map(row=>JSON.stringify(row))) as {id:number}[];
  expect([...selected.map(row=>row.id),...positions.implementation,...positions.environmentImplementation]).toEqual(expected.environment);
  const shaped=shapeComputeResultV1({workspaceRoot:globalFixture.workspaceRoot,part:{owners:[{name:globalFixture.owner,root:globalFixture.ownerRoot,node:{},nodeRows:base}],edgeOnly:[],edges:[],rows:all,env:{digest:"e".repeat(64),rows:take(positions.explicit)}} as never,stubs:{},implementation:{all:take(positions.implementation),environment:take(positions.environmentImplementation)} as never});
  expect(log.stats().reads-before).toBe(expected.shapeReads);
  expect(listed(shaped.env.rows as never)).toEqual(take(expected.environment));
  expect(listed(shaped.owners[0]!.rows as never)).toEqual(take(expected.owner));
 }finally{oracle.close();log.close();rmSync(directory,{recursive:true,force:true});}
});
