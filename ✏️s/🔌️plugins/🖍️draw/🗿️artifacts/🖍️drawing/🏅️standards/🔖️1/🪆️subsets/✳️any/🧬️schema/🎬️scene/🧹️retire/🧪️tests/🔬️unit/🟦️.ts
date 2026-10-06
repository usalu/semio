/** 🧹️ Shared retirement counts and independent JSON ownership census. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import cases from "../../🧫️fixtures/🔣️.json";
import sources from "../../../📋️prepare/🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {DocumentSceneJob,type DocumentScenePlan} from "../../../📋️prepare/🟦️.ts";
import {ScenePlanCloseJob} from "../../🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
function lift(value:any):any{return typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([k,v])=>[k,lift(v)])):value;}
const validate=new Ajv({strict:true}).compile(schema);
const paint=(c:any):number=>(c.fill?1+(c.fill.stops?1:0):0)+(c.stroke?1+(c.stroke.dash?1:0):0);
const census=(p:any):number=>3+p.assets.length*4+p.nodes.reduce((total:number,n:any)=>total+6+n.groups.length*3+paint(n.content)+(n.content.segments?1:0)+(n.content.children?1+n.content.children.length:0)+(typeof n.content.content==="string"?1:0)+(n.content.operation?1:0)+(n.content.source?1:0)+(n.content.asset?1:0),0);
for(const row of cases)test("retirement grants release "+row.name,()=>{
 const source=row.source?sources.find(r=>r.name===row.source):null;
 const expected=source?JSON.parse(JSON.stringify(source.expected)):{assets:[],nodes:[]};expect(census(expected)).toBe(row.owners);
 for(const grant of [1,7,4096]){
  let plan:DocumentScenePlan={assets:[],nodes:[]};if(source){const job=new DocumentSceneJob({...source.document,layers:lift(source.document.layers)} as any,source.limits);while(!job.advance(4096).done){}plan=job.result();}
  const retained=plan.assets.map(asset=>({...asset}));const close=new ScenePlanCloseJob(plan);let work=0,owners=0;
  while(!close.terminalIsEmpty()){const p=close.advance(grant);expect(validate(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(p.owners).toBeGreaterThanOrEqual(owners);expect(p.owners-p.work).toBeLessThanOrEqual(0);work=p.work;owners=p.owners;}
  expect(owners).toBe(row.owners);expect(work).toBe(row.work);expect(close.advance(1)).toEqual({phase:"complete",owners,work,done:true});expect(retained).toEqual(expected.assets);
 }
 console.log("[DEBUG] Grant-bounded scene retirement reaches an empty shell: "+row.name);
});
test("retirement rejects invalid grants without consuming owners",()=>{const close=new ScenePlanCloseJob({assets:[],nodes:[]});for(const grant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>close.advance(grant)).toThrow("grant");expect(validate({phase:"complete",owners:0,work:0,done:false})).toBe(false);expect(validate({phase:"closing",owners:0,work:0,done:true})).toBe(false);expect(close.terminalIsEmpty()).toBe(false);expect(close.advance(3)).toEqual({phase:"complete",owners:3,work:3,done:true});});
