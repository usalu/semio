import {test,expect} from "bun:test";
import Ajv2020 from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import floatSchema from "./🧫️fixtures/🟰️float-schema.json";
import floatFixture from "./🧫️fixtures/🟰️float.json";
import workSchema from "./🧫️fixtures/⛽️work-schema.json";
import workFixture from "./🧫️fixtures/⛽️work.json";
import {RetainedSliceEquality} from "../🟦️.ts";
const byte={byteWidth:1,work:1,inlineEqual:(left:number,right:number)=>left===right};

test("portable work vectors agree with independent SQLite byte comparisons",()=>{
 expect(new Ajv2020({strict:true}).compile(workSchema)(workFixture)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of workFixture){
  const left=Uint8Array.from(row.left),right=Uint8Array.from(row.right);
  const equal=db.query<{equal:number},[Uint8Array,Uint8Array]>("SELECT ?1 = ?2 AS equal").get(left,right)!.equal!==0;
  const first=db.query<{first:number|null},[Uint8Array,Uint8Array]>("WITH RECURSIVE i(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM i WHERE n < length(?1)) SELECT min(n) AS first FROM i WHERE substr(?1,n,1) <> substr(?2,n,1)").get(left,right)!.first;
  const immediate=row.maximumWork>0&&(left.length!==right.length||left.length===0);
  const compared=left.length===right.length&&left.length>0&&row.maximumWork>1?Math.min(row.maximumWork-1,Math.floor(row.payloadBodyBytes/2),first??left.length):0;
  const work=immediate?1:compared>0?1+compared:0;
  const ready=immediate||(compared>0&&(first!==null&&first<=compared||compared===left.length));
  expect({work,ready,equal:ready?equal:null}).toEqual({work:row.work,ready:row.ready,equal:row.equal});
  const cursor=new RetainedSliceEquality(byte),body=cursor.fixedCopyBytes+row.payloadBodyBytes,demand=cursor.demand(left.length,right.length,body,row.maximumWork),grant={maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};
  const receipt=cursor.advance(left,right,body,row.maximumWork,grant);
  expect({work:receipt.work,ready:cursor.ready,equal:cursor.equal??null}).toEqual({work:row.work,ready:row.ready,equal:row.equal});
 }}finally{db.close();}
});

test("floating equality follows the independent SQLite numeric result",()=>{
 expect(new Ajv2020({strict:true}).compile(floatSchema)(floatFixture)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of floatFixture){
  const read=(word:string)=>{const bytes=new ArrayBuffer(row.width/8),view=new DataView(bytes);if(row.width===32){view.setUint32(0,Number.parseInt(word,16));return view.getFloat32(0)}view.setBigUint64(0,BigInt("0x"+word));return view.getFloat64(0)};
  const left=read(row.left),right=read(row.right);
  expect(db.query<{equal:number},[number,number]>("SELECT coalesce(?1 = ?2, 0) AS equal").get(left,right)!.equal).toBe(Number(row.expected));
  const element={byteWidth:row.width/8,work:1,inlineEqual:(a:number,b:number)=>a===b},cursor=new RetainedSliceEquality(element),body=cursor.fixedCopyBytes+row.width/4,demand=cursor.demand(1,1,body,2);
  cursor.advance([left],[right],body,2,{maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1});expect(cursor.equal).toBe(row.expected);
 }}finally{db.close();}
});

test("bounded slice equality agrees with independent SQLite blob identity",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){expect(db.query<{equal:number},[Uint8Array,Uint8Array]>("SELECT ?1=?2 AS equal").get(Uint8Array.from(row.left),Uint8Array.from(row.right))!.equal).toBe(Number(row.expected));const owner=new RetainedSliceEquality(byte);let turns=0;
  while(!owner.ready){const body=owner.fixedCopyBytes+2,demand=owner.demand(row.left.length,row.right.length,body,4096),grant={maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};expect(owner.advance(row.left,row.right,body,4096,{...grant,maximumItems:0}).copiedBytes).toBe(0);expect(owner.advance(row.left,row.right,body,4096,{...grant,maximumCopyBytes:owner.fixedCopyBytes-1}).copiedBytes).toBe(0);expect(owner.advance(row.left,row.right,body,4096,grant).copiedBytes).toBeLessThanOrEqual(demand);expect(++turns).toBeLessThan(row.left.length+4);}
  expect(owner.equal).toBe(row.expected);
 }}finally{db.close();}
});
test("wide original equality buffers remain borrowed across yielding and cancellation",()=>{
 const left=new Uint8Array(fixture.wide.length),right=new Uint8Array(fixture.wide.length);right[right.length-1]=1;const original=[left.buffer,right.buffer];
 for(const cut of [...fixture.wide.cancelCuts,null]){const owner=new RetainedSliceEquality(byte);let steps=0;while(!owner.ready&&(cut===null||steps<cut)){const body=owner.fixedCopyBytes+fixture.wide.pageBytes*2,demand=owner.demand(left.length,right.length,body,4096),p=owner.advance(left,right,body,4096,{maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1});if(p.copiedBytes>demand)throw Error("unfunded original comparison");if(++steps>left.length+1)throw Error("slice comparison stalled");}if(cut===null)expect(owner.equal).toBe(false);expect([left.buffer,right.buffer]).toEqual(original);}
});

test("malformed scalar funding refuses before original equality work",()=>{
 const owner=new RetainedSliceEquality(byte),left=[1],right=[1],grant={maximumItems:1,maximumCopyBytes:owner.fixedCopyBytes+2,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};
 for(const amount of [NaN,Infinity,-1,0.5]){expect(()=>owner.advance(left,right,grant.maximumCopyBytes,4096,{...grant,maximumCopyBytes:amount})).toThrow();expect(owner.ready).toBe(false);expect(()=>owner.demand(1,1,amount,4096)).toThrow();expect(()=>owner.demand(1,1,grant.maximumCopyBytes,amount)).toThrow();}
 expect(()=>owner.advance(left,right,grant.maximumCopyBytes,4096,{...grant,maximumCapacityBytes:NaN})).toThrow();expect(()=>owner.advance(left,right,grant.maximumCopyBytes,4096,{...grant,maximumReleaseBytes:NaN})).toThrow();expect(owner.ready).toBe(false);
 expect(owner.advance(left,right,grant.maximumCopyBytes,4096,grant).copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(owner.equal).toBe(true);
});
