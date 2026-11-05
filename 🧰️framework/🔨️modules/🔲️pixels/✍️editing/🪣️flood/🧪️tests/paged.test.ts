
import{test,expect}from"bun:test";
import Ajv from"ajv";
import graphlib from"graphlib";
import{FloodRegionCursor}from"../🟦️.ts";
import fixture from"./paged.json";
import schema from"./paged.schema.json";
const zero={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
function finish(cursor:FloodRegionCursor,capacity:number,bound:number){
 cursor.beginClose();let released=0,turns=0;
 while(!cursor.closed){
  const quote=cursor.closeDemands(fixture.grant.maximumCopyBytes);
  expect(quote.releaseBytes).toBeLessThanOrEqual(fixture.grant.maximumReleaseBytes);
  expect(quote.copyBytes).toBeLessThanOrEqual(fixture.grant.maximumCopyBytes);
  const denied=cursor.close({...fixture.grant,maximumReleaseBytes:Math.max(0,quote.releaseBytes-1)});
  expect(denied.work).toBe(0);expect(denied.result.ok).toBe(true);
  if(denied.result.ok)expect(denied.result.value.progress).toEqual(zero);
  expect(cursor.closeDemands(fixture.grant.maximumCopyBytes)).toEqual(quote);
  const turn=cursor.close(fixture.grant);expect(turn.result.ok).toBe(true);
  if(turn.result.ok){released+=turn.result.value.progress.releasedBytes;if(capacity>0){expect(turn.work).toBe(1);expect(turn.result.value.progress.releasedBytes).toBeGreaterThan(0);}}
  expect(++turns).toBeLessThanOrEqual(bound);
 }
 expect(released).toBe(capacity);
}
test("paged non-square connectivity matches Graphlib and held result views revoke",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.cases){
  const count=row.width*row.height,pixels=new Uint8Array(count*4),graph=new graphlib.Graph({directed:false});
  for(let y=0;y<row.height;y++)for(let x=0;x<row.width;x++){
   const i=y*row.width+x,selected=x%row.whiteColumnStride===0||y===row.connectorRow;
   pixels.set(selected?row.target:row.background,i*4);if(selected)graph.setNode(String(i));
  }
  for(let y=0;y<row.height;y++)for(let x=0;x<row.width;x++){
   const i=y*row.width+x;for(const n of[x+1<row.width?i+1:-1,y+1<row.height?i+row.width:-1])if(graph.hasNode(String(i))&&graph.hasNode(String(n)))graph.setEdge(String(i),String(n));
  }
  const distance=graphlib.alg.dijkstra(graph,String(row.seed[1]!*row.width+row.seed[0]!),()=>1,node=>graph.nodeEdges(node)??[]);
  const oracle=Object.keys(distance).filter(key=>Number.isFinite(distance[key]!.distance)).map(Number).sort((a,b)=>a-b);
  expect(oracle.length).toBe(row.expectedReached);
  const cursor=new FloodRegionCursor(row.width,row.height,row.seed[0]!,row.seed[1]!);let capacity=0,turns=0;
  while(!cursor.done){
   const quote=cursor.demands(fixture.maximumWork),turn=cursor.advance(pixels,fixture.maximumWork,fixture.grant);expect(turn.result.ok).toBe(true);
   if(!turn.result.ok)throw Error(turn.result.error.message);
   expect(turn.work).toBeLessThanOrEqual(fixture.maximumWork);
   expect(turn.result.value.progress.retainedCapacityBytes).toBeLessThanOrEqual(quote.capacityBytes);
   expect(turn.result.value.progress.copiedBytes).toBeLessThanOrEqual(quote.copyBytes);
   capacity+=turn.result.value.progress.retainedCapacityBytes;expect(++turns).toBeLessThan(200);
  }
  expect(capacity).toBe(row.expectedCapacityBytes);expect(cursor.workspaceBytes).toBe(capacity);
  const held=cursor.result(),iterator=held[Symbol.iterator]();expect(held.length).toBe(count);
  expect(Array.from(held).flatMap((value,index)=>value?[index]:[])).toEqual(oracle);
  expect(iterator.next().done).toBe(false);
  const cancelled=cursor.advance(pixels,fixture.maximumWork,fixture.grant,true);expect(cancelled.result.ok).toBe(false);
  expect(()=>held.at(0)).toThrow();expect(()=>iterator.next()).toThrow();
  finish(cursor,capacity,3*row.expectedPages+3);
 }
});
test("every paged allocation prefix cancels and retires exact directory and leaf receipts",()=>{
 const row=fixture.cases[0]!,count=row.width*row.height,pages=Math.ceil(count/fixture.pagePixels),allocationActions=3+3*pages;
 for(let prefix=0;prefix<=allocationActions;prefix++){
  const cursor=new FloodRegionCursor(row.width,row.height,0,0);let capacity=0;
  for(let action=0;action<prefix;action++){
   const quote=cursor.demands(1);
   for(const denied of[{...fixture.grant,maximumItems:0},{...fixture.grant,maximumCapacityBytes:Math.max(0,quote.capacityBytes-1)},{...fixture.grant,maximumCopyBytes:Math.max(0,quote.copyBytes-1)},{...fixture.grant,maximumDepth:0}]){
    const turn=cursor.advance([255,255,255,255],1,denied);expect(turn.work).toBe(0);expect(turn.result.ok).toBe(true);if(turn.result.ok)expect(turn.result.value.progress).toEqual(zero);expect(cursor.demands(1)).toEqual(quote);
   }
   const turn=cursor.advance([255,255,255,255],1,fixture.grant);expect(turn.work).toBe(1);expect(turn.result.ok).toBe(true);
   if(turn.result.ok)capacity+=turn.result.value.progress.retainedCapacityBytes;
  }
  const cancel=cursor.advance([255,255,255,255],1,fixture.grant,true);expect(cancel.result.ok).toBe(false);
  expect(()=>cursor.result()).toThrow();finish(cursor,capacity,allocationActions+1);
 }
});
test("maximum accepted extent keeps directories within declared ownership bounds",()=>{
 const cursor=new FloodRegionCursor(16384,1024,0,0);let capacity=0;
 for(let action=0;action<3;action++){
  const quote=cursor.demands(1);expect(quote.capacityBytes).toBe(fixture.maximumDirectoryBytes);
  const turn=cursor.advance([1,2,3,4],1,fixture.grant);expect(turn.result.ok).toBe(true);
  if(turn.result.ok)capacity+=turn.result.value.progress.retainedCapacityBytes;
 }
 expect(capacity).toBe(3*fixture.maximumDirectoryBytes);finish(cursor,capacity,4);
});

test("each leaf allocation failure preserves every previously funded backing for close",()=>{
 const row=fixture.cases[0]!,pages=row.expectedPages;
 for(let leaf=0;leaf<pages*3;leaf++){
  const cursor=new FloodRegionCursor(row.width,row.height,0,0);let capacity=0;
  for(let action=0;action<3+leaf;action++){
   const turn=cursor.advance([1,2,3,4],1,fixture.grant);expect(turn.result.ok).toBe(true);if(turn.result.ok)capacity+=turn.result.value.progress.retainedCapacityBytes;
  }
  const quote=cursor.demands(1),original8=globalThis.Uint8Array,original32=globalThis.Uint32Array;
  let refused;
  try{
   if(leaf<pages*2)globalThis.Uint8Array=class extends original8{constructor(length:number){super(0);throw Error("Test leaf allocation refusal");}} as typeof Uint8Array;
   else globalThis.Uint32Array=class extends original32{constructor(length:number){super(0);throw Error("Test leaf allocation refusal");}} as typeof Uint32Array;
   refused=cursor.advance([1,2,3,4],1,fixture.grant);
  }finally{globalThis.Uint8Array=original8;globalThis.Uint32Array=original32;}
  expect(refused.work).toBe(1);expect(refused.result.ok).toBe(false);
  if(!refused.result.ok){expect(refused.result.error.kind).toBe("allocationFailed");expect(refused.result.error.progress.retainedCapacityBytes).toBe(0);}
  expect(cursor.demands(1)).toEqual(quote);expect(()=>cursor.result()).toThrow();
  finish(cursor,capacity,3+3*pages);
 }
});
test("held completed mask revokes on begin-close without cancellation",()=>{
 const cursor=new FloodRegionCursor(1,1,0,0);let capacity=0;
 while(!cursor.done){
  const turn=cursor.advance([1,2,3,4],1,fixture.grant);expect(turn.result.ok).toBe(true);if(turn.result.ok)capacity+=turn.result.value.progress.retainedCapacityBytes;
 }
 const held=cursor.result(),iterator=held[Symbol.iterator]();expect(held.at(0)).toBe(255);expect(iterator.next().value).toBe(255);
 for(const index of[-1,1,0.5,Number.NaN])expect(()=>held.at(index)).toThrow();
 cursor.beginClose();expect(()=>held.at(0)).toThrow();expect(()=>held.length).toThrow();expect(()=>iterator.next()).toThrow();
 finish(cursor,capacity,6);expect(()=>held.at(0)).toThrow();
});
