import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";
import fixture from "./📊️retained.json";
import buildPolicy from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";

test("a durable graph or explicitly exact reservation survives the ramp and prevents overlapping cargo",()=>{
 const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),host=fixture.host as HostSampleV1,db=new Database(":memory:");
 const grants={graph:fixture.graph,quiet:fixture.quiet,exact:fixture.exact,forecast:{...fixture.exact,retainReservation:undefined}};
 try{
  const sql=db.query("select max(0,(case when ?1 or ?5 > 0 then ?2 else ?3 end)-?3) as unrealized, case when ?4 - max(0,(case when ?1 or ?5 > 0 then ?2 else ?3 end)-?3) < ?6 then 'wait' else 'admit' end as verdict");
  for(const item of fixture.cases){
   const grant=grants[item.grant as keyof typeof grants],retains=item.grant==="graph"||item.grant==="exact";
   const available=host.availableMegabytes-Math.floor(host.totalMegabytes*policy.memory.floorBasisPoints/10000);
   const result=evaluateHostCapacityV1(policy,host,item.grant==="quiet"?[grant]:[grant,fixture.quiet] as readonly HostGrantV1[],[fixture.cargo as HostWaiterV1],fixture.nowMs);
   expect(sql.get(Number(retains),grant.reservedMegabytes,grant.observedMegabytes,available,policy.memory.rampMilliseconds-(fixture.nowMs-grant.admittedAtMs),fixture.cargo.request.fixedMegabytes),item.name).toEqual({unrealized:item.unrealized,verdict:item.verdict});
   expect<unknown>({unrealized:result.unrealizedMegabytes,verdict:result.decisions[0]?.verdict},item.name).toEqual({unrealized:item.unrealized,verdict:item.verdict});
  }
 }finally{db.close();}
});
