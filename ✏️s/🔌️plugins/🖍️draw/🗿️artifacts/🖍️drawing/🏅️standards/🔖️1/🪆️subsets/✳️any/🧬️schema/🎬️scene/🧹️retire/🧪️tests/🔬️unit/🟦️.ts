const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🧹️ Shared retirement counts and independent JSON ownership census. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import cases from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/🧹️retire/🔣️.json";
import sources from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {stock,sceneOwner} from "../../../🎨️paint/🧪️tests/🎟️funding/🟦️.ts";
import {ScenePlanCloseJob} from "../../🟦️.ts";
import {binary64} from "@semio-tech/framework/value/ieee754";
function lift(value:any):any{return typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([k,v])=>[k,lift(v)])):value;}
const validate=new Ajv({strict:true}).compile(schema);
const paint=(c:any):number=>(c.fill?1+(c.fill.stops?1:0):0)+(c.stroke?1+(c.stroke.dash?1:0):0);
const census=(p:any):number=>3+p.assets.length*3+p.nodes.reduce((total:number,n:any)=>total+6+n.groups.length*3+paint(n.content)+(n.content.segments?1:0)+(n.content.children?1+n.content.children.length:0)+(typeof n.content.content==="string"?1:0)+(n.content.operation?1:0)+(n.content.source?1:0)+(n.content.asset?1:0),0);
const emptyLimits={maxNodes:32,maxDepth:16,maxSegments:65536,maxReferences:256,maxSourceBytes:100000};
const ownerOf=(source:any)=>source?sceneOwner({...source.document,layers:lift(source.document.layers)},source.limits):sceneOwner({schema:"drawing.document",id:"empty",layers:[],assets:{}},emptyLimits);
for(const row of cases)test("retirement grants release "+row.name,()=>{
 const source=row.source?sources.find(r=>r.name===row.source):null;
 const expected=source?JSON.parse(JSON.stringify(source.expected)):{assets:[],nodes:[]};expect(census(expected)).toBe(row.owners);
 for(const grant of [1,7,4096]){
  const close=new ScenePlanCloseJob(ownerOf(source));let work=0,owners=0;
  for(let at=0;at<2000000&&!close.terminalIsEmpty();at++){const p=close.advance(grant,stock(grant));expect(validate(neutral(p))).toBe(true);expect(p.work).toBeGreaterThanOrEqual(work);expect(p.owners).toBeGreaterThanOrEqual(owners);work=p.work;owners=p.owners;}
  expect(close.terminalIsEmpty()).toBe(true);expect(owners).toBe(row.owners);expect(close.advance(1,stock(1))).toEqual({phase:"complete",owners,work,done:true});
 }
});
test("retirement rejects invalid grants without consuming owners",()=>{const close=new ScenePlanCloseJob(ownerOf(null));for(const grant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>close.advance(grant,stock(1))).toThrow(/grant/);expect(validate({phase:"complete",owners:0,work:0,done:false})).toBe(false);expect(validate({phase:"closing",owners:0,work:0,done:true})).toBe(false);expect(close.terminalIsEmpty()).toBe(false);let done=false;for(let at=0;at<100000&&!done;at++)done=close.advance(3,stock(3)).done;expect(done).toBe(true);expect(close.terminalIsEmpty()).toBe(true);});
