import {afterAll,expect,test} from "bun:test";
import {appendFileSync,existsSync,readFileSync,readdirSync,realpathSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {digestTaskViewInputsV1,openTaskViewV1,taskViewInputsDigestV1} from "../../🟦️.ts";
import {TASK_VIEW_EXECUTOR_POLICY_V1,TASK_VIEW_EXECUTOR_TEXTS_V1,runOnTaskRootV1,type TaskRootNoticeV1} from "../🟦️.ts";
import {growTreeV1,removeV1,scratchV1} from "../../🧪️tests/🧰️world/🟦️.ts";

/** 🏃️ The task runner uses the view root for the class that asks for it, keeps outputs at their declared locations, pins again after a moved input and leaves the live path exactly as it was. */
const scratch:string[]=[];
const world=(tag:string)=>{const root=realpathSync(scratchV1(tag));scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const setup=(count=30)=>{const live=world("te-live"),directory=world("te-views"),target=world("te-target"),paths=growTreeV1(live,count);return{live,directory,target,paths};};

test("the policy and the texts are closed and bilingual",()=>{
 expect(TASK_VIEW_EXECUTOR_POLICY_V1).toEqual({version:1,maximumPins:4,maximumReruns:1});
 expect(Object.keys(TASK_VIEW_EXECUTOR_TEXTS_V1.en).sort()).toEqual(Object.keys(TASK_VIEW_EXECUTOR_TEXTS_V1.de).sort());
});

test("the live path is unchanged: the default class runs once on the workspace, no digest, no view, no environment",async()=>{
 const {live,directory,target,paths}=setup();let digests=0,opens=0;const seen:unknown[]=[];
 const result=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,environment:{},paths:async()=>paths,digest:async(...a)=>{digests++;return digestTaskViewInputsV1(...a);},open:async request=>{opens++;return openTaskViewV1(request);},run:async context=>{seen.push(context);return 7;}});
 expect(result).toMatchObject({value:7,mode:"live",root:live,hash:null,pins:0,reruns:0,fallback:"none"});
 expect([digests,opens]).toEqual([0,0]);
 expect(seen).toEqual([{mode:"live",root:live,hash:null,environment:{},view:null}]);
 const classed=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{SEMIO_TASK_ROOT:"live"},paths:async()=>paths,run:async context=>context.mode});
 expect(classed.value,"the environment turns a probe back to live without code").toBe("live");
 expect(readdirSync(directory),"nothing was pinned").toEqual([]);
});

test("the readiness probe runs on the view: its root is not the workspace, it holds the start bytes while the live tree is edited, and the result answers for the start hash",async()=>{
 const {live,directory,target,paths}=setup();const start=readFileSync(join(live,paths[3]!),"utf8");
 const result=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,run:async context=>{
  expect(context.mode).toBe("view");expect(context.root).not.toBe(live);expect(context.view).not.toBeNull();
  writeFileSync(join(live,paths[3]!),"// edited in the live tree\n");writeFileSync(join(live,"new-file.rs"),"x");
  const read=readFileSync(join(context.root,paths[3]!),"utf8");
  writeFileSync(join(context.environment.CARGO_TARGET_DIR!,"out.rlib"),"artifact");
  return{read,environment:context.environment,hasNew:existsSync(join(context.root,"new-file.rs"))};
 }});
 expect(result.mode).toBe("view");expect(result.value.read,"the view holds the start bytes").toBe(start);expect(result.value.hasNew).toBe(false);
 const digests=await digestTaskViewInputsV1(live,paths.map(path=>path===paths[3]?path:path));
 expect(result.hash).toMatch(/^[0-9a-f]{64}$/u);expect(result.hash,"the hash is of the start state, not of the edited live tree").not.toBe(taskViewInputsDigestV1(digests));
 expect(result.value.environment).toEqual({CARGO_TARGET_DIR:target,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:result.root});
 expect(readFileSync(join(target,"out.rlib"),"utf8"),"the output is at its declared location").toBe("artifact");
 expect(existsSync(join(result.root,"out.rlib"))).toBe(false);expect(result).toMatchObject({pins:1,reruns:0,fallback:"none"});
});

test("an output directory inside the view is refused before the task runs, in both languages",async()=>{
 const {live,directory,target,paths}=setup();
 const probe=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,run:async context=>context.root});
 let ran=false;
 const error=await runOnTaskRootV1({workspace:live,directory,targetDirectory:join(probe.value,"target"),taskClass:"readiness-probe",environment:{},paths:async()=>paths,run:async()=>{ran=true;return 1;}}).catch(failure=>failure as Error&{code:string;en:string;de:string});
 expect(ran).toBe(false);expect((error as {code:string}).code).toBe("task-view-output-inside");expect((error as {en:string}).en).toContain("declared locations");expect((error as {de:string}).de).toContain("erklärten Orte");
 expect(readdirSync(join(directory,".use")),"the lease of the refused run is released").toEqual([]);
});

test("a moved input pins again before the task starts, the new bytes win, and the notice names the first file",async()=>{
 const {live,directory,target,paths}=setup();const notices:TaskRootNoticeV1[]=[];let digests=0;
 const result=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,notify:notice=>notices.push(notice),
  digest:async(source,list)=>{const rows=await digestTaskViewInputsV1(source,list);if(++digests===1)appendFileSync(join(source,paths[5]!),"// moved between hashing and pinning\n");return rows;},
  run:async context=>readFileSync(join(context.root,paths[5]!),"utf8")});
 expect(result).toMatchObject({mode:"view",pins:2,reruns:0,fallback:"none"});expect(result.value).toContain("moved between hashing and pinning");
 expect(notices.map(notice=>notice.kind)).toEqual(["repinned"]);expect(notices[0]!.en).toContain(paths[5]!);expect(notices[0]!.de).toContain("erneut gehasht");
 expect(result.hash).toBe(taskViewInputsDigestV1(await digestTaskViewInputsV1(live,paths)));
});

test("inputs that never settle run live after the bounded pins, with a notice and no failure",async()=>{
 const {live,directory,target,paths}=setup();const notices:TaskRootNoticeV1[]=[];let digests=0;
 const result=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,notify:notice=>notices.push(notice),
  digest:async(source,list)=>{const rows=await digestTaskViewInputsV1(source,list);digests++;appendFileSync(join(source,paths[1]!),"// again\n");return rows;},run:async context=>context.mode});
 expect(result).toMatchObject({value:"live",mode:"live",pins:TASK_VIEW_EXECUTOR_POLICY_V1.maximumPins,fallback:"pins-exhausted"});expect(digests).toBe(TASK_VIEW_EXECUTOR_POLICY_V1.maximumPins);
 expect(notices.at(-1)!.kind).toBe("pins-exhausted");expect(notices.filter(notice=>notice.kind==="repinned")).toHaveLength(TASK_VIEW_EXECUTOR_POLICY_V1.maximumPins);
});

test("a view that cannot be made runs live with the refusal in both languages",async()=>{
 const {live,directory,target,paths}=setup();const notices:TaskRootNoticeV1[]=[];
 const result=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,notify:notice=>notices.push(notice),open:async()=>({kind:"refused",code:"path-too-long",en:"too long",de:"zu lang"}),run:async context=>context.mode});
 expect(result).toMatchObject({value:"live",fallback:"refused",pins:1});expect(notices).toHaveLength(1);expect(notices[0]!.en).toContain("too long");expect(notices[0]!.de).toContain("zu lang");
});

test("a hard-link view edited in place runs again on a new view; a view that keeps changing runs live",async()=>{
 const {live,directory,target,paths}=setup();const notices:TaskRootNoticeV1[]=[];let runs=0;
 const links=(request:Parameters<typeof openTaskViewV1>[0])=>openTaskViewV1({...request,strategies:["hardlink"]});
 const once=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,open:links,notify:notice=>notices.push(notice),run:async context=>{if(++runs===1)writeFileSync(join(live,paths[2]!),"// in place\n");return readFileSync(join(context.root,paths[2]!),"utf8");}});
 expect(once).toMatchObject({mode:"view",reruns:1,fallback:"none",value:"// in place\n"});expect(runs).toBe(2);expect(notices.map(notice=>notice.kind)).toEqual(["rerun"]);
 let always=0;notices.length=0;
 const never=await runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,open:links,notify:notice=>notices.push(notice),run:async context=>{writeFileSync(join(live,paths[4]!),"// in place "+(++always)+"\n");return context.mode;}});
 expect(never).toMatchObject({value:"live",fallback:"reruns-exhausted"});expect(notices.at(-1)!.kind).toBe("reruns-exhausted");
});

test("a failing task releases its lease and the error leaves unchanged",async()=>{
 const {live,directory,target,paths}=setup();
 await expect(runOnTaskRootV1({workspace:live,directory,targetDirectory:target,taskClass:"readiness-probe",environment:{},paths:async()=>paths,run:async()=>{throw new Error("boom");}})).rejects.toThrow("boom");
 expect(readdirSync(join(directory,".use")),"no lease is left").toEqual([]);
});
