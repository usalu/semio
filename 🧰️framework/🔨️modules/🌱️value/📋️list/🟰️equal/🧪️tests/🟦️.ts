import {test,expect} from "bun:test";
import Ajv2020 from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {RetainedSliceEquality} from "../🟦️.ts";
const byte={byteWidth:1,inlineEqual:(left:number,right:number)=>left===right};

test("bounded slice equality agrees with independent SQLite blob identity",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){expect(db.query<{equal:number},[Uint8Array,Uint8Array]>("SELECT ?1=?2 AS equal").get(Uint8Array.from(row.left),Uint8Array.from(row.right))!.equal).toBe(Number(row.expected));const owner=new RetainedSliceEquality(byte);let turns=0;
  while(!owner.ready){const demand=owner.demand(row.left.length,row.right.length,owner.fixedCopyBytes+2),grant={maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};expect(owner.advance(row.left,row.right,{...grant,maximumItems:0}).copiedBytes).toBe(0);expect(owner.advance(row.left,row.right,{...grant,maximumCopyBytes:owner.fixedCopyBytes-1}).copiedBytes).toBe(0);expect(owner.advance(row.left,row.right,grant).copiedBytes).toBeLessThanOrEqual(demand);expect(++turns).toBeLessThan(row.left.length+4);}
  expect(owner.equal).toBe(row.expected);
 }}finally{db.close();}
});
test("wide original equality buffers remain borrowed across yielding and cancellation",()=>{
 const left=new Uint8Array(fixture.wide.length),right=new Uint8Array(fixture.wide.length);right[right.length-1]=1;const original=[left.buffer,right.buffer];
 for(const cut of [...fixture.wide.cancelCuts,null]){const owner=new RetainedSliceEquality(byte);let steps=0;while(!owner.ready&&(cut===null||steps<cut)){const demand=owner.demand(left.length,right.length,owner.fixedCopyBytes+fixture.wide.pageBytes*2),p=owner.advance(left,right,{maximumItems:1,maximumCopyBytes:demand,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1});if(p.copiedBytes>demand)throw Error("unfunded original comparison");if(++steps>left.length+1)throw Error("slice comparison stalled");}if(cut===null)expect(owner.equal).toBe(false);expect([left.buffer,right.buffer]).toEqual(original);}
});

test("malformed scalar funding refuses before original equality work",()=>{
 const owner=new RetainedSliceEquality(byte),left=[1],right=[1],grant={maximumItems:1,maximumCopyBytes:owner.fixedCopyBytes+2,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};
 for(const amount of [NaN,Infinity,-1,0.5]){expect(()=>owner.advance(left,right,{...grant,maximumCopyBytes:amount})).toThrow();expect(owner.ready).toBe(false);expect(()=>owner.demand(1,1,amount)).toThrow();}
 expect(()=>owner.advance(left,right,{...grant,maximumCapacityBytes:NaN})).toThrow();expect(()=>owner.advance(left,right,{...grant,maximumReleaseBytes:NaN})).toThrow();expect(owner.ready).toBe(false);
 expect(owner.advance(left,right,grant).copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(owner.equal).toBe(true);
});
