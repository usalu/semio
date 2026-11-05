import{test,expect}from"bun:test";
import Ajv from"ajv";
import graphlib from"graphlib";
import{FloodRegionCursor,type FloodSource,type FloodGrant}from"../🟦️.ts";
import fixture from"./🔣️.json";
import schema from"./schema.json";
import protocolSchema from"../🧬️schema/🔣️.json";

const full:FloodGrant={maximumItems:1,maximumCopyBytes:1_048_576,maximumCapacityBytes:1_048_576,maximumReleaseBytes:1_048_576,maximumDepth:1};
function advance(cursor:FloodRegionCursor,source:FloodSource,work:number,cancelled=false){
 const quote=Number.isSafeInteger(work)&&work>0&&work<=65536?cursor.demands(work):undefined;
 const outcome=cursor.advance(source,work,full,cancelled);
 if(!outcome.result.ok)throw Error(outcome.result.error.message);
 const value=outcome.result.value;expect(value.work).toBe(outcome.work);expect(outcome.work).toBeLessThanOrEqual(work);
 if(quote){expect(value.progress.copiedBytes).toBeLessThanOrEqual(quote.copyBytes);expect(value.progress.retainedCapacityBytes).toBeLessThanOrEqual(quote.capacityBytes);}
 return value;
}
function close(cursor:FloodRegionCursor,release:number){
 let releasedBytes=0,done=cursor.closed;
 do{
  const quote=cursor.closeDemands(full.maximumCopyBytes),outcome=cursor.close({...full,maximumReleaseBytes:release-releasedBytes});
  expect(outcome.result.ok).toBe(true);if(!outcome.result.ok)throw Error(outcome.result.error.message);
  const step=outcome.result.value;expect(step.progress.releasedBytes).toBeLessThanOrEqual(quote.releaseBytes);expect(step.progress.copiedBytes).toBeLessThanOrEqual(quote.copyBytes);
  releasedBytes+=step.progress.releasedBytes;done=step.done;if(outcome.work===0)break;
 }while(!done&&releasedBytes<release);
 return{releasedBytes,done};
}

test("neutral bounded flood matches independent Graphlib reachability without publishing prefixes",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.cases){
  const source=new Uint8Array(row.rows.join("").split("").flatMap(symbol=>row.palette[symbol as keyof typeof row.palette]!));const seed="seed"in row?row.seed!:[Math.round(row.seedUv![0]*(row.width-1)),Math.round((1-row.seedUv![1])*(row.height-1))];const start=seed[1]*row.width+seed[0],target=source.slice(start*4,start*4+4),graph=new graphlib.Graph({directed:false});
  for(let i=0;i<source.length/4;i++)if(target.every((value,c)=>source[i*4+c]===value))graph.setNode(String(i));
  for(let y=0;y<row.height;y++)for(let x=0;x<row.width;x++){const i=y*row.width+x;for(const n of[x+1<row.width?i+1:-1,y+1<row.height?i+row.width:-1])if(graph.hasNode(String(i))&&graph.hasNode(String(n)))graph.setEdge(String(i),String(n));}
  const distance=graphlib.alg.dijkstra(graph,String(start),()=>1,node=>graph.nodeEdges(node)??[]),oracle=Object.keys(distance).filter(key=>Number.isFinite(distance[key]!.distance)).map(Number).sort((a,b)=>a-b);expect(oracle).toEqual(row.expected.reached);
  const cursor=new FloodRegionCursor(row.width,row.height,seed[0],seed[1]);let work=0,turns=0;
  while(!cursor.done){expect(()=>cursor.result()).toThrow();const step=advance(cursor,source,fixture.maximumWork);expect(step.work).toBeLessThanOrEqual(fixture.maximumWork);work+=step.work;expect(++turns).toBeLessThan(1000);}
  expect(Array.from(cursor.result()).flatMap((amount,index)=>amount?[index]:[])).toEqual(oracle);expect(work).toBeGreaterThanOrEqual(row.width*row.height);expect(source).toEqual(new Uint8Array(row.rows.join("").split("").flatMap(symbol=>row.palette[symbol as keyof typeof row.palette]!)));
  cursor.beginClose();let released=0;while(!cursor.closed)released+=close(cursor,1_048_576).releasedBytes;expect(released).toBe(cursor.workspaceBytes);expect(()=>cursor.result()).toThrow();
 }
});
test("large flood charges pixel work and cancellation retains buffers until bounded close",()=>{
 const row=fixture.large,source=new Uint8Array(row.width*row.height*4).fill(255),cursor=new FloodRegionCursor(row.width,row.height,0,0);let turns=0,work=0;
 while(!cursor.done){const step=advance(cursor,source,row.budget);work+=step.work;turns++;expect(step.work).toBeLessThanOrEqual(row.budget);}
 expect(turns).toBeGreaterThanOrEqual(row.minimumProgressTurns);expect(work).toBeGreaterThanOrEqual(row.width*row.height);expect(Array.from(cursor.result()).every(value=>value===255)).toBe(true);cursor.beginClose();while(!cursor.closed)close(cursor,1_048_576);
 const cancelled=new FloodRegionCursor(row.width,row.height,0,0);while(advance(cancelled,source,row.budget).completed<=row.width*row.height){}expect(()=>advance(cancelled,source,row.budget,true)).toThrow(/cancel/iu);expect(()=>cancelled.result()).toThrow();expect(cancelled.closed).toBe(false);cancelled.beginClose();expect(close(cancelled,0).releasedBytes).toBe(0);let released=0;while(!cancelled.closed)released+=close(cancelled,1_048_576).releasedBytes;expect(released).toBe(cancelled.workspaceBytes);
});

test("uniform source mutation refuses without releasing original workspace",()=>{
 const source:[number,number,number,number]=[7,8,9,10],cursor=new FloodRegionCursor(2,2,0,0);
 const retained=advance(cursor,source,1).progress.retainedCapacityBytes;source[0]=99;
 expect(()=>advance(cursor,source,1)).toThrow(/source changed/iu);expect(()=>cursor.result()).toThrow();expect(cursor.closed).toBe(false);
 cursor.beginClose();expect(close(cursor,0)).toEqual({releasedBytes:0,done:false});expect(close(cursor,retained)).toEqual({releasedBytes:retained,done:true});
});
test("completed flood cancellation revokes publication and done until paid close",()=>{
 const source=new Uint8Array([7,8,9,10]),cursor=new FloodRegionCursor(1,1,0,0);
 while(!cursor.done)advance(cursor,source,1);
 expect(Array.from(cursor.result())).toEqual([255]);expect(()=>advance(cursor,source,1,true)).toThrow(/cancel/iu);expect(cursor.done).toBe(false);expect(()=>cursor.result()).toThrow();
 cursor.beginClose();expect(close(cursor,0)).toEqual({releasedBytes:0,done:false});expect(close(cursor,cursor.workspaceBytes)).toEqual({releasedBytes:cursor.workspaceBytes,done:true});
});

test("uniform and borrowed RGBA sources produce identical bounded regions",()=>{
 const rgba=new Uint8Array(3*2*4);for(let pixel=0;pixel<6;pixel++)rgba.set([12,34,56,78],pixel*4);
 const borrowed=new FloodRegionCursor(3,2,1,0),uniform=new FloodRegionCursor(3,2,1,0);
 while(!borrowed.done){const left=advance(borrowed,rgba,1),right=advance(uniform,[12,34,56,78],1);expect(right).toEqual(left);expect(left.work).toBeLessThanOrEqual(1);}
 expect(Array.from(uniform.result())).toEqual(Array.from(borrowed.result()));expect(Array.from(uniform.result())).toEqual([255,255,255,255,255,255]);
 for(const cursor of[borrowed,uniform]){cursor.beginClose();let released=0;while(!cursor.closed)released+=close(cursor,36).releasedBytes;expect(released).toBe(cursor.workspaceBytes);}
});

test("denied work before and after progress leaves the next funded turn unchanged",()=>{
 const source=new Uint8Array(4*4*4).fill(255),control=new FloodRegionCursor(4,4,0,0),subject=new FloodRegionCursor(4,4,0,0);
 for(let turn=0;!control.done;turn++){
  if(turn===0||turn===5||turn===20){for(const budget of[0,-1,65537,0.5,Number.NaN])expect(()=>advance(subject,source,budget)).toThrow(/work budget/iu);expect(()=>subject.result()).toThrow();}
  const expected=advance(control,source,1),actual=advance(subject,source,1);expect(actual).toEqual(expected);expect(actual.work).toBeLessThanOrEqual(1);
 }
 expect(Array.from(subject.result())).toEqual(Array.from(control.result()));
 for(const cursor of[control,subject]){cursor.beginClose();expect(close(cursor,0)).toEqual({releasedBytes:0,done:false});expect(close(cursor,15)).toEqual({releasedBytes:0,done:false});expect(close(cursor,cursor.workspaceBytes)).toEqual({releasedBytes:cursor.workspaceBytes,done:true});}
});
test("stale source after pixel progress retains unpublished workspace through denied release",()=>{
 const source=new Uint8Array(4*4*4).fill(255),subject=new FloodRegionCursor(4,4,0,0),control=new FloodRegionCursor(4,4,0,0);let step;
 do{step=advance(subject,source,1);expect(step).toEqual(advance(control,source,1));}while(step.completed<=16);
 expect(()=>subject.result()).toThrow();expect(()=>advance(subject,source.slice(),1)).toThrow(/source changed/iu);expect(subject.closed).toBe(false);
 expect(advance(subject,source,1)).toEqual(advance(control,source,1));
 for(const cursor of[subject,control]){cursor.beginClose();expect(close(cursor,0)).toEqual({releasedBytes:0,done:false});expect(close(cursor,15)).toEqual({releasedBytes:0,done:false});expect(close(cursor,cursor.workspaceBytes)).toEqual({releasedBytes:cursor.workspaceBytes,done:true});expect(()=>cursor.result()).toThrow();}
});

test("physical borrowed range identity and funded denial use public outcomes",()=>{
 const bytes=new Uint8Array(16).fill(255),cursor=new FloodRegionCursor(2,2,0,0);
 const full={maximumItems:1,maximumCopyBytes:1_048_576,maximumCapacityBytes:1_048_576,maximumReleaseBytes:1_048_576,maximumDepth:1};
 const quote=cursor.demands(1);
 for(const key of["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumDepth"]as const){
  const denied={...full,[key]:key==="maximumCopyBytes"?quote.copyBytes-1:key==="maximumCapacityBytes"?quote.capacityBytes-1:0};
  const turn=cursor.advance(bytes,1,denied);expect(turn.work).toBe(0);expect(turn.result.ok).toBe(true);
 }
 expect(cursor.advance(bytes,1,full).work).toBe(1);
 expect(cursor.advance(new Uint8Array(bytes.buffer,bytes.byteOffset,bytes.byteLength),1,full).result.ok).toBe(true);
 const refused=cursor.advance(bytes.slice(),1,full);expect(refused.work).toBe(0);expect(refused.result.ok).toBe(false);
 cursor.beginClose();const closeQuote=cursor.closeDemands(1_048_576);
 const denied=cursor.close({...full,maximumReleaseBytes:0});expect(denied.work).toBe(0);
 let released=0;while(!cursor.closed){const turn=cursor.close(full);expect(turn.result.ok).toBe(true);if(turn.result.ok)released+=turn.result.value.progress.releasedBytes;}
 expect(released).toBe(16);expect(closeQuote.releaseBytes).toBeGreaterThan(0);
});

test("every quoted resource denial is atomic across allocation initialization flood and close",()=>{
 const source=new Uint8Array(4*4*4).fill(255),subject=new FloodRegionCursor(4,4,0,0),control=new FloodRegionCursor(4,4,0,0),validate=new Ajv({strict:true}).compile(protocolSchema);
 const zero={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};let turns=0;
 while(!subject.done){
  const quote=subject.demands(1);
  const grants=[{...full,maximumItems:0},{...full,maximumCopyBytes:quote.copyBytes-1},{...full,maximumDepth:0}];
  if(quote.capacityBytes)grants.push({...full,maximumCapacityBytes:quote.capacityBytes-1});
  for(const grant of grants){const outcome=subject.advance(source,1,grant);expect(validate({version:1,grant,quote,outcome})).toBe(true);expect(outcome.work).toBe(0);expect(outcome.result.ok).toBe(true);if(outcome.result.ok)expect(outcome.result.value.progress).toEqual(zero);expect(subject.demands(1)).toEqual(quote);}
  const outcome=subject.advance(source,1,full);expect(validate({version:1,grant:full,quote,outcome})).toBe(true);expect(outcome).toEqual(control.advance(source,1,full));expect(++turns).toBeLessThan(100);
 }
 expect(Array.from(subject.result())).toEqual(Array.from(control.result()));
 subject.beginClose();control.beginClose();let released=0;
 while(!subject.closed){
  const quote=subject.closeDemands(full.maximumCopyBytes);
  for(const grant of[{...full,maximumItems:0},{...full,maximumCopyBytes:quote.copyBytes-1},{...full,maximumReleaseBytes:0},{...full,maximumReleaseBytes:quote.releaseBytes-1},{...full,maximumDepth:0}]){
   const outcome=subject.close(grant);expect(validate({version:1,grant,quote,outcome})).toBe(true);expect(outcome.work).toBe(0);expect(outcome.result.ok).toBe(true);if(outcome.result.ok)expect(outcome.result.value.progress).toEqual(zero);expect(subject.closeDemands(full.maximumCopyBytes)).toEqual(quote);
  }
  const outcome=subject.close(full);expect(validate({version:1,grant:full,quote,outcome})).toBe(true);expect(outcome).toEqual(control.close(full));if(outcome.result.ok)released+=outcome.result.value.progress.releasedBytes;
 }
 expect(released).toBe(subject.workspaceBytes);
});
test("borrowed range rejects different offset and detached extent while retaining close ownership",()=>{
 const backing=new ArrayBuffer(20),source=new Uint8Array(backing,0,16).fill(255),cursor=new FloodRegionCursor(2,2,0,0);
 expect(cursor.advance(source,1,full).result.ok).toBe(true);
 expect(cursor.advance(new Uint8Array(backing,0,16),1,full).result.ok).toBe(true);
 const shifted=cursor.advance(new Uint8Array(backing,4,16),1,full);expect(shifted.work).toBe(0);expect(shifted.result.ok).toBe(false);if(!shifted.result.ok)expect(shifted.result.error.kind).toBe("invalidValue");
 structuredClone(backing,{transfer:[backing]});const detached=cursor.advance(source,1,full);expect(detached.work).toBe(0);expect(detached.result.ok).toBe(false);if(!detached.result.ok)expect(detached.result.error.kind).toBe("invalidValue");
 cursor.beginClose();expect(close(cursor,0)).toEqual({releasedBytes:0,done:false});expect(close(cursor,16)).toEqual({releasedBytes:16,done:true});
});
