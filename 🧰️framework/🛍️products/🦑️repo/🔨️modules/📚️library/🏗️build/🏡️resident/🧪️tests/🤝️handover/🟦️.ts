/** 🤝️ Contract: a read that hands over to current code reaches the client as the refusal `handover` (both texts), and the plan client asks the service again a bounded number of times, saying so in both languages, before it reports the refusal.
 * @see ../../🏛️service/🧯️failure/🟨️.mjs */
import {expect,test} from "bun:test";
import table from "../../🧬️schema/🔣️.json";
import {failureRefusalV1} from "../../🏛️service/🧯️failure/🟨️.mjs";
import {PlanRefusedErrorV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {requestInvocationPlanV1,stalledHandoversAfterV1} from "../../../🧭️invocation/🛰️plan/🟦️.ts";
import rows from "./🔣️.json";
import {CUSTODY_POLICY_V1} from "../../../🕸️graph/🕰️generations/🔐️custody/🟨️.mjs";

const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["host-graph-handover"]?node:Object.values(node).map(find).find(Boolean)):undefined;
const handover=()=>Object.assign(new Error("graph: this read runs code aaaaaaaaaaaa; it hands over after 40 of 87 owners\nGraph: dieses Lesen übergibt nach 40 von 87 Eigentümern"),{code:"SEMIO_STALE_IMPLEMENTATION"});

test("the hand-over of a read is the refusal handover with its first line, also when it is the cause of the failure, and both texts exist",()=>{
 const direct=failureRefusalV1(handover(),{log:()=>undefined,logPath:"/log",root:"/r"}),wrapped=failureRefusalV1(Object.assign(new Error("outer"),{cause:handover()}),{log:()=>undefined,logPath:"/log",root:"/r"});
 for(const refusal of[direct,wrapped]){expect(refusal).toMatchObject({code:"handover",key:"host-graph-handover",retry:"later"});expect((refusal.fields as any).message).toContain("hands over after 40 of 87 owners");expect((refusal.fields as any).message).not.toContain("\n");}
 const texts=find(table);for(const language of["en","de"])expect(texts[language]["host-graph-handover"]).toContain("{message}");
 expect(table.definitions.RefusalCode.enum).toContain("handover");
 expect(failureRefusalV1(new Error("boom"),{log:()=>undefined,logPath:"/log",root:"/r"}).code).toBe("internal");
});

test("the plan client asks again after a hand-over, bounded, and tells the user in both languages",async()=>{
 const written:string[]=[],environment={SEMIO_BUILD_TOOL:"nx"},request={arguments:["run","app:build"],tool:"nx",targets:["app:build"]},frame={t:"refused",code:"handover",retry:"later",message:{en:"hands over",de:"übergibt"}};
 let asked=0;
 const outcome=await requestInvocationPlanV1({root:process.cwd(),request,environment,agent:"agent-1",write:line=>written.push(line),ports:{requestPlanV1:async()=>{asked++;throw new PlanRefusedErrorV1(frame as never);},spawnHostServiceV1:()=>undefined,probeConsentV1:async()=>({state:"unprotected",folder:"Documents"})}}).then(()=>undefined,(error:unknown)=>error);
 expect(asked).toBe(1+CUSTODY_POLICY_V1.maximumStalledHandovers);expect(outcome).toBeDefined();
 expect(written.filter(line=>line.includes("handed over to current code")&&line.includes("an aktuellen Code übergeben")).length).toBe(CUSTODY_POLICY_V1.maximumStalledHandovers);
 let again=0;
 const second=await requestInvocationPlanV1({root:process.cwd(),request,environment,agent:"agent-1",write:()=>undefined,ports:{requestPlanV1:async()=>{if(again++===0)throw new PlanRefusedErrorV1(frame as never);throw new Error("second ask reached");},spawnHostServiceV1:()=>undefined,probeConsentV1:async()=>({state:"unprotected",folder:"Documents"})}}).then(()=>undefined,(error:unknown)=>error);
 expect(again).toBe(2);expect(String((second as Error).message)).toContain("second ask reached");
});

test("the count of hand-overs without progress follows the table",()=>{
 expect(rows.maximum).toBe(CUSTODY_POLICY_V1.maximumStalledHandovers);
 for(const row of rows.cases)expect(stalledHandoversAfterV1({stalled:row.stalled,progressed:row.progressed,maximum:rows.maximum}),row.name).toBe(row.next??undefined);
});

test("a read that keeps progressing is asked again after every hand-over, however many, and ends with its answer instead of a failure",async()=>{
 const written:string[]=[],environment={SEMIO_BUILD_TOOL:"nx"},request={arguments:["run","app:build"],tool:"nx",targets:["app:build"]},frame={t:"refused",code:"handover",retry:"later",message:{en:"hands over",de:"übergibt"}};
 const slices=CUSTODY_POLICY_V1.maximumStalledHandovers*4;let asked=0;
 const outcome=await requestInvocationPlanV1({root:process.cwd(),request,environment,agent:"agent-1",write:line=>written.push(line),ports:{requestPlanV1:async(options:any)=>{
  if(asked++<slices){options.onProgress({phase:"slice",done:1,total:0,observed:true,message:{en:"reading",de:"lese"}});throw new PlanRefusedErrorV1(frame as never);}
  return{reply:{},runtime:{directory:"/unused",tag:"t"},plan:{tasks:[]}};
 },spawnHostServiceV1:()=>undefined,probeConsentV1:async()=>({state:"unprotected",folder:"Documents"})}}).then(()=>undefined,(error:unknown)=>error);
 expect(asked).toBe(slices+1);expect(String((outcome as Error).message)).toContain("answered without a graph slice");
 expect(written.filter(line=>line.includes("handed over to current code")).length).toBe(slices);
 let idle=0;
 const stalled=await requestInvocationPlanV1({root:process.cwd(),request,environment,agent:"agent-1",write:()=>undefined,ports:{requestPlanV1:async(options:any)=>{
  options.onProgress({phase:"slice",done:idle,total:0,message:{en:"waiting",de:"warte"}});idle++;throw new PlanRefusedErrorV1(frame as never);
 },spawnHostServiceV1:()=>undefined,probeConsentV1:async()=>({state:"unprotected",folder:"Documents"})}}).then(()=>undefined,(error:unknown)=>error);
 expect(idle).toBe(1+CUSTODY_POLICY_V1.maximumStalledHandovers);expect(stalled).toBeDefined();
});
