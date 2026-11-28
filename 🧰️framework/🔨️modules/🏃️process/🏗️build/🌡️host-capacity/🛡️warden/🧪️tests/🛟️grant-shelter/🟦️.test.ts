import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {decideHostWardenV1,HOST_WARDEN_POLICY_V1,type HostWardenHostV1,type HostWardenSubjectV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";

const policy=HOST_WARDEN_POLICY_V1,subjectOf=(megabytes:number,grant:number|null,extra:Record<string,unknown>={})=>({...fixture.subject,megabytes,...grant===null?{}:{grantMegabytes:grant},...extra}) as HostWardenSubjectV1;

test("the cap of a claimed subject is the host cap share (its owner judges it), the static cap holds for the others",()=>{
 const db=new Database(":memory:");
 try{
  const oracle=db.query("select ?1 > case when ?3 is null then min(?2, cast(?4 * ?5 / 10000 as integer)) else cast(?4 * ?5 / 10000 as integer) end as over");
  for(const item of fixture.cap){
   const expected=oracle.get(item.megabytes,fixture.subject.capMegabytes,item.grant,fixture.total,policy.capMaximumBasisPoints) as {over:number};
   const decision=decideHostWardenV1(policy,fixture.calm as HostWardenHostV1,[subjectOf(item.megabytes,item.grant)],{level:"calm",calmTicks:policy.calmTicks});
   expect(Boolean(expected.over),item.name).toBe(item.cancel);
   expect(decision.actions.some(action=>action.reason==="cap"),item.name).toBe(item.cancel);
  }
 }finally{db.close();}
});

test("an acute swap alarm with room above the guard line never cancels a worker within its grant",()=>{
 for(const item of fixture.acute.cases){
  const host={...fixture.acute.host,availableMegabytes:item.available} as HostWardenHostV1,paused=subjectOf(item.megabytes,item.grant,{suspended:true,suspendedSeconds:policy.pauseDeadlineSeconds+policy.suspendMaximumSeconds});
  const decision=decideHostWardenV1(policy,host,[paused],{level:"danger",calmTicks:0});
  expect(decision.actions.some(action=>action.action==="cancel"),item.name).toBe(item.cancel);
 }
});
