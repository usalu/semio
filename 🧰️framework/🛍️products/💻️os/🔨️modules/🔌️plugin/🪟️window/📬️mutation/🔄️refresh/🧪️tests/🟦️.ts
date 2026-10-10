/** 🧪️ SQLite independently records the actual original retained across yielded authority refresh. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv2020 from"ajv/dist/2020.js";
import{transferWindowRefresh,windowRefreshDemand,type WindowRefreshGrant,type WindowRefreshQueue,type WindowRefreshProgress}from"../🟦️.ts";
import law from"../🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("window refresh retains original address and displaced snapshot across every denied currency",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(law)).toBe(true);
 for(const copy of law.copyGrants){for(const rejected of[false,true]){
  const db=new Database(":memory:");db.run("CREATE TABLE custody(slot TEXT PRIMARY KEY,owner TEXT,address TEXT)");db.run("INSERT INTO custody VALUES('current',?,?)",[law.before,law.windowId]);db.run("INSERT INTO custody VALUES('pending',?,'')",[law.after]);
  const before={windowId:law.windowId,payload:{text:law.before}},after={windowId:"",payload:{text:law.after}},current={value:before},pending:{value:typeof before|undefined}={value:after};
  const retained:typeof before[]=[];let reserved=false,refuse=rejected;
  const progress=(capacity:number):WindowRefreshProgress=>({copiedItems:1,copiedBytes:0,retainedCapacityBytes:capacity,releasedBytes:0});
  const queue:WindowRefreshQueue<typeof before>={get length(){return retained.length;},hasReservedSlot:()=>reserved,reserveCapacity:()=>law.queueCapacity,frameCapacity:()=>law.frameCapacity,reserveStep(){reserved=true;return progress(law.queueCapacity);},admitOwned(value){if(refuse){refuse=false;return{error:new Error("original queue refused"),original:value};}retained.push(value);reserved=false;db.run("UPDATE custody SET slot='retired',address='' WHERE slot='current'");db.run("UPDATE custody SET slot='current',address=? WHERE slot='pending'",[law.windowId]);return progress(law.frameCapacity);}};
  let ready=false;
  for(let turn=0;turn<law.maximumTurns&&!ready;turn++){
   const demand=windowRefreshDemand(pending.value,queue),grant={maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:demand.capacityBytes,maximumReleaseBytes:demand.releaseBytes,maximumDepth:demand.depth};
   const denials:WindowRefreshGrant[]=[{...grant,maximumItems:0},{...grant,maximumDepth:demand.depth-1}];if(demand.capacityBytes>0)denials.push({...grant,maximumCapacityBytes:demand.capacityBytes-1});
   for(const denied of denials){expect(transferWindowRefresh(current,pending,queue,denied).progress.copiedItems).toBe(0);expect(current.value).toBe(before);expect(pending.value).toBe(after);expect(retained).toHaveLength(0);}
   try{ready=transferWindowRefresh(current,pending,queue,grant).kind==="ready";}catch{expect(current.value).toBe(before);expect(pending.value).toBe(after);expect(before.windowId).toBe(law.windowId);expect(after.windowId).toBe("");}
  }
  expect(ready).toBe(true);expect(current.value).toBe(after);expect(current.value.windowId).toBe(law.windowId);expect(retained[0]).toBe(before);expect(before.windowId).toBe("");expect(db.query("SELECT owner,address FROM custody WHERE slot='current'").get()).toEqual({owner:law.after,address:law.windowId});expect(db.query("SELECT owner,address FROM custody WHERE slot='retired'").get()).toEqual({owner:law.before,address:""});
  db.close();console.log(`[DEBUG] Window refresh grant=${copy} reject=${rejected} original retained address preserved SQL matched`);
 }}
});
