import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {decideHostWardenV1,HOST_WARDEN_POLICY_V1,type HostWardenHostV1,type HostWardenSubjectV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";
import recovery from "./📊️recovery.json";
import resumeRoom from "./📊️resume-room.json";
import {Database} from "bun:sqlite";

test("resume consumes only the expected page-in room above the physical guard floor",()=>{
 const policy=HOST_WARDEN_POLICY_V1,db=new Database(":memory:");
 try{
  const oracle=db.query("select case when (?1 is not null or ?6 = 'calm') and max(coalesce(?1, ?7) + ?2, ?5) <= ?3 then 'resume' when ?4 and ?8 then 'cancel' when ?4 and ?3 >= ?5 then 'resume' end as action");
  const floor=Math.min(policy.guardMaximumMegabytes,Math.max(policy.guardMinimumMegabytes,Math.floor(resumeRoom.host.totalMegabytes*policy.guardBasisPoints/10000)));
  const recoveryFloor=Math.max(floor,Math.floor(resumeRoom.host.totalMegabytes*policy.resumeBasisPoints/10000));
  for(const item of resumeRoom.cases){
   const level=item.level as "calm"|"guard",host={...resumeRoom.host,availableMegabytes:item.available,...level==="calm"?{pressure:"normal",compressedMegabytes:0}:{} } as HostWardenHostV1;
   const canCancel="canCancel" in item?item.canCancel!:true;
   const subject={...resumeRoom.subject,canCancel,nonresidentMegabytes:item.cost,stoppedFootprintMegabytes:resumeRoom.subject.megabytes,suspendedSeconds:item.overdue?policy.suspendMaximumSeconds:resumeRoom.subject.suspendedSeconds};
   const decision=decideHostWardenV1(policy,host,[subject],{level,calmTicks:policy.calmTicks-1});
   expect(oracle.get(item.cost,floor,item.available,Number(item.overdue),recoveryFloor,level,subject.stoppedFootprintMegabytes,Number(canCancel)),item.name).toEqual({action:item.action});
   expect(decision.level,item.name).toBe(level);
   expect<unknown>(decision.actions.find(action=>action.id===subject.id)?.action??null,item.name).toBe(item.action);
  }
  const subjects=resumeRoom.overdueCosts.map((nonresidentMegabytes,index)=>({...resumeRoom.subject,pid:index+1,id:String(index),nonresidentMegabytes,suspendedSeconds:policy.suspendMaximumSeconds}));
  const rows=db.query("select sum(value) over (order by key) <= ?2 as fits from json_each(?1) order by key").all(JSON.stringify(resumeRoom.overdueCosts),resumeRoom.host.availableMegabytes-floor) as {fits:number}[];
  const result=decideHostWardenV1(policy,resumeRoom.host as HostWardenHostV1,subjects,{level:"guard",calmTicks:policy.calmTicks-1});
  expect(rows.map(row=>Boolean(row.fits))).toEqual(resumeRoom.expectedOverdueResumes);
  expect(subjects.map(subject=>result.actions.some(action=>action.action==="resume"&&action.id===subject.id))).toEqual(resumeRoom.expectedOverdueResumes);
  expect(result.actions).toHaveLength(1);
 }finally{db.close();}
});

test("warning recovery requires consecutive quiet samples before resuming",()=>{
 const db=new Database(":memory:");try{const rows=db.query("with recursive s(i,streak) as (select 0,0 union all select i+1,case when json_extract(?1,'$['||i||']') < ?2 then streak+1 else 0 end from s where i < json_array_length(?1)) select streak>=?3 as ready from s where i>0").all(JSON.stringify(recovery.swapRates),HOST_WARDEN_POLICY_V1.swapActiveMegabytesPerSecond,HOST_WARDEN_POLICY_V1.calmTicks) as {ready:number}[];expect(rows.map(row=>Boolean(row.ready))).toEqual(recovery.expectedResume);}finally{db.close();}
 let previous:{level:"calm"|"guard"|"danger";calmTicks:number}={level:"danger",calmTicks:0};
 const host={...fixture.warningHost,availableMegabytes:2832,compressedMegabytes:5000,intervalMilliseconds:10000} as HostWardenHostV1,subjects=[{...fixture.subjects[0]!,suspendedSeconds:0}] as HostWardenSubjectV1[];
 const seen=recovery.swapRates.map(rate=>{const result=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...host,swapOutMegabytesPerSecond:rate},subjects,previous);previous={level:result.level,calmTicks:result.calmTicks};return result.actions.some(action=>action.action==="resume");});
 expect(seen).toEqual(recovery.expectedResume);
 const overdue=[{...subjects[0]!,suspendedSeconds:HOST_WARDEN_POLICY_V1.suspendMaximumSeconds}],early=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...host,swapOutMegabytesPerSecond:0},overdue,{level:"danger",calmTicks:0}),ready=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...host,swapOutMegabytesPerSecond:0},overdue,{level:"guard",calmTicks:HOST_WARDEN_POLICY_V1.calmTicks-1});
 expect(early.actions.some(action=>action.action==="resume")).toBe(false);expect(ready.actions).toContainEqual({action:"resume",id:"paused",reason:"overdue"});
 const below=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...host,availableMegabytes:2000,swapOutMegabytesPerSecond:null},subjects,{level:"guard",calmTicks:2});expect(below.actions.some(action=>action.action==="resume")).toBe(false);
});

test("sustained compression and swap churn reclaim paused scoped work",()=>{
 const host=fixture.host as HostWardenHostV1,subjects=fixture.subjects as HostWardenSubjectV1[],previous={level:"danger" as const,calmTicks:0};
 const decide=(sample:HostWardenHostV1,items=subjects)=>decideHostWardenV1(HOST_WARDEN_POLICY_V1,sample,items,previous);
 const oracle=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",fileURLToPath(new URL("../🐍️.py",import.meta.url)),"--acute"]);
 expect(oracle.exitCode).toBe(0);
 expect(decide(host)).toEqual(JSON.parse(oracle.stdout.toString()));
 expect(decide(host).actions).toContainEqual({action:"cancel",id:"paused",reason:"danger"});
 expect(decide(host).actions.some(action=>action.id==="protected")).toBe(false);
 expect(decide({...host,swapOutMegabytesPerSecond:0}).actions.some(action=>action.action==="cancel")).toBe(false);
 expect(decide({...host,availableMegabytes:3500}).actions.some(action=>action.action==="cancel"),"paging with memory to spare is not a shortage").toBe(false);
 expect(decide({...host,pressure:"normal"}).actions.some(action=>action.action==="cancel"),"paging while the kernel is normal is not a shortage").toBe(false);
 expect(decide({...host,intervalMilliseconds:5000}).actions.some(action=>action.action==="cancel"),"swapping that is not sustained is not a shortage").toBe(false);
 expect(decide(host,[{...subjects[0]!,suspendedSeconds:0},subjects[1]!]).actions.some(action=>action.action==="cancel")).toBe(false);
 expect(decide(host,[{...subjects[0]!,suspended:false,suspendedSeconds:0}]).actions).toContainEqual({action:"suspend",id:"paused",reason:"danger"});
});

test("swapping that continues with the OS warning and little available memory releases an overdue build", () => {
 const host = fixture.warningHost as HostWardenHostV1, subjects = fixture.subjects as HostWardenSubjectV1[];
 const decide = (sample: HostWardenHostV1, level: "calm" | "guard", swapMilliseconds = 0) => decideHostWardenV1(HOST_WARDEN_POLICY_V1, sample, subjects, {level, calmTicks:0, swapMilliseconds});
 const first = decide(host, "calm");
 expect(first.level).toBe("guard");
 expect(first.actions.some(action => action.action === "cancel")).toBe(false);
 const continuing = decide(host, "guard", first.swapMilliseconds);
 const oracle = Bun.spawnSync([testPythonV1() ?? Bun.which("python3") ?? "python3", "-I", fileURLToPath(new URL("../🐍️.py", import.meta.url)), "--warning-churn"]);
 expect(oracle.exitCode).toBe(0);
 expect(continuing).toEqual(JSON.parse(oracle.stdout.toString()));
 expect(continuing.level).toBe("danger");
 expect(continuing.actions).toContainEqual({action:"cancel",id:"paused",reason:"danger"});
 expect(continuing.actions.some(action => action.id === "protected")).toBe(false);
 expect(decide({...host,swapOutMegabytesPerSecond:0}, "guard", first.swapMilliseconds).actions.some(action => action.action === "cancel")).toBe(false);
});
