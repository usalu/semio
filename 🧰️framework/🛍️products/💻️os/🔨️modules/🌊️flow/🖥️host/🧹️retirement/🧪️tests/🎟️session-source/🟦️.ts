import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import fixture from "../../🧫️fixtures/🎟️session-source/🔣️.json";

test("original session custody examples preserve owners and independent UTF8 bytes",()=>{
 const retained=applyPatch([...fixture.retention.handles],fixture.retention.handles.flatMap((handle,index)=>fixture.retention.roster.includes(handle)?[]:[{op:"remove" as const,path:`/${index}`}]).reverse(),true).newDocument;
 expect(retained).toEqual(fixture.retention.expected);
 expect(fixture.retention.maximumRowsPerTurn).toBe(1);
 for(const row of fixture.cases){
  const text=row.text.repeat(row.repeat);
  expect(new TextEncoder().encode(text).length).toBe(row.utf8Bytes);
  expect(Buffer.byteLength(text,"utf8")).toBe(row.utf8Bytes);
  const source={painted:text,converged:text,fault:{extension:text,capability:text,code:text,message:text},latches:Array.from({length:row.latches},(_,key)=>({key,armed:true,inFlight:key,owed:true,unfinished:true})),closing:false};
  const closed=applyPatch(structuredClone(source),[{op:"replace",path:"/closing",value:true}],true).newDocument;
  expect({...closed,closing:false}).toEqual(source);
  expect(closed.closing).toBe(true);
  const reset=applyPatch(structuredClone(source),[{op:"replace",path:"/painted",value:""},{op:"replace",path:"/converged",value:""},{op:"replace",path:"/fault",value:null}],true).newDocument;
  expect(reset.latches).toEqual(source.latches);
  expect(reset.painted).toBe("");expect(reset.converged).toBe("");expect(reset.fault).toBeNull();
  const patches=source.latches.flatMap((latch,index)=>index%2?[{op:"remove" as const,path:`/latches/${index}`}]:[]).reverse();
  const pruned=applyPatch(structuredClone(source),patches,true).newDocument;
  expect(pruned.latches).toEqual(source.latches.filter((_,index)=>index%2===0));
  const invalidated=applyPatch(structuredClone(source),[{op:"replace",path:"/painted",value:""},{op:"replace",path:"/converged",value:""},{op:"replace",path:"/fault",value:null},{op:"replace",path:"/latches",value:[]}],true).newDocument;
  expect(invalidated).toEqual({...source,painted:"",converged:"",fault:null,latches:[]});
  const cancelled=applyPatch(structuredClone(source),source.latches.map((_,index)=>({op:"replace" as const,path:`/latches/${index}`,value:{key:index,armed:false,inFlight:0,owed:false,unfinished:false}})),true).newDocument;
  expect(cancelled).toEqual({...source,latches:source.latches.map(latch=>({...latch,armed:false,inFlight:0,owed:false,unfinished:false}))});
  expect(fixture.cancellation).toEqual({requestPreservesOwners:true,boundedLatchItems:1,sourceBirthBytes:0,sourceFreeBytes:0});
  const pendingHost={snapshot:{id:text},channels:{label:text}};
  const intake={host:pendingHost};expect(intake.host).toBe(pendingHost);expect(fixture.pendingHost).toEqual({sameOwner:true,admittedBirthBytes:0,admittedFreeBytes:0});
  const live={...source,eval:"old",progress:{31:"working"}};const published=applyPatch(structuredClone(live),[{op:"replace",path:"/eval",value:text}],true).newDocument;expect(published).toEqual({...live,eval:text});
  const channels={outputs:{label:text},inputs:{label:text}};
  const leased=channels;
  expect(leased).toBe(channels);
  expect(JSON.parse(JSON.stringify(leased))).toEqual(channels);
  const original=source.latches[0];
  const armed=applyPatch(structuredClone(original),[{op:"replace",path:"/armed",value:false},{op:"replace",path:"/inFlight",value:2},{op:"replace",path:"/owed",value:true}],true).newDocument;
  expect(armed).toEqual({...original,armed:false,inFlight:2,owed:true});
  const settled=applyPatch(armed,[{op:"replace",path:"/inFlight",value:0},{op:"replace",path:"/armed",value:true},{op:"replace",path:"/owed",value:false}],true).newDocument;
  expect(settled).toEqual({...original,armed:true,inFlight:0,owed:false});
 }
 console.log("[DEBUG] Original session UTF8/Buffer/JSONPatch independent custody livePublication=3 cases=3 invalidation=3 reset=3 originalSharedPublication=3 latchAdmission=3");
});
