/** 🧪️ Neutral filled regions, independent polygon clipping and SVG raster oracles. */
import {expect,test} from "vitest";
import Ajv from "ajv";
import pc from "polygon-clipping";
import sharp from "sharp";
import rows from "../🧫️fixtures/🔣️.json";
import endpointRows from "../🧫️fixtures/🔗️endpoints/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {BooleanJob,booleanRegions,type BooleanInput,type BooleanProgress} from "../🟦️.ts";
import {pathSegmentsToSvgD,type PathSegment,type Vec2} from "../../🟦️.ts";
const ajv=new Ajv({strict:true}),valid=ajv.compile(schema),progressValid=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),resultValid=ajv.compile({$ref:schema.$id+"#/definitions/result"});
function finish(input:BooleanInput,grant:number,observer?:(p:BooleanProgress)=>void):PathSegment[]{
 const job=new BooleanJob(input);expect(()=>job.result()).toThrow(/incomplete/);let work=0;
 for(let at=0;at<input.maxWork;at++){const p=job.advance(grant);expect(progressValid(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;observer?.(p);if(p.done){const result=job.result();expect(resultValid(result)).toBe(true);return result;}expect(()=>job.result()).toThrow(/incomplete/);}throw Error("Boolean did not terminate");
}
function contours(segments:PathSegment[]):Vec2[][]{const rings:Vec2[][]=[];for(const s of segments)if(s.kind==="move")rings.push([s.to]);else if(s.kind==="line")rings.at(-1)!.push(s.to);return rings;}
const cross=(a:Vec2,b:Vec2,c:Vec2)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
function area(ring:readonly Vec2[]):number{let sum=0;for(let at=1;at<ring.length-1;at++)sum+=cross(ring[0]!,ring[at]!,ring[at+1]!);return sum/2;}
function filled(rings:readonly (readonly Vec2[])[],p:Vec2,rule="nonzero"):boolean{let winding=0;for(const ring of rings)for(let at=0;at<ring.length;at++){const a=ring[at]!,b=ring[(at+1)%ring.length]!,c=cross(a,b,p);if(a[1]<=p[1]&&b[1]>p[1]&&c>0)winding++;else if(a[1]>p[1]&&b[1]<=p[1]&&c<0)winding--;}return rule==="evenodd"?winding%2!==0:winding!==0;}
const apply=(operation:string,a:boolean,b:boolean)=>operation==="union"?a||b:operation==="difference"?a&&!b:operation==="intersection"?a&&b:a!==b;
test("translated stroke junctions preserve endpoint incidence and independent SVG coverage",async()=>{
 for(const row of endpointRows){const input=row.input as unknown as BooleanInput;let previous:PathSegment[]|undefined;
  for(const grant of [1,7,4096]){const output=finish(input,grant),rings=contours(output);expect(rings).toHaveLength(1);expect(rings.reduce((sum,c)=>sum+area(c),0)).toBeGreaterThan(0);if(previous)expect(output).toEqual(previous);previous=output;const points=input.operands[0]!.contours.flat(),origin:[number,number]=[Math.min(...points.map(p=>p[0])),Math.min(...points.map(p=>p[1]))],svg=(c:readonly(readonly Vec2[])[])=>'<svg xmlns="http://www.w3.org/2000/svg" width="128" height="256" viewBox="-0.1 -0.1 1.1 2.3"><path d="'+c.map(r=>'M'+r.map(p=>[p[0]-origin[0],p[1]-origin[1]].join(" ")).join(" L")+" Z").join(" ")+'" fill="black"/></svg>';
   const actual=await sharp(Buffer.from(svg(rings))).ensureAlpha().raw().toBuffer(),reference=await sharp(Buffer.from(svg(input.operands[0]!.contours))).ensureAlpha().raw().toBuffer();let maximum=0,different=0;for(let at=3;at<actual.length;at+=4){const delta=Math.abs(actual[at]!-reference[at]!);maximum=Math.max(maximum,delta);if(delta)different++;if(reference[at]===0||reference[at]===255)expect(delta).toBeLessThanOrEqual(1);}console.log("[DEBUG] Endpoint stroke SVG",row.name,"maximum_alpha_delta",maximum,"changed_pixels",different);expect(maximum).toBeLessThanOrEqual(row.raster.maxAlphaDelta);expect(different).toBeLessThanOrEqual(row.raster.maxChangedPixels);
   console.log("[DEBUG] Endpoint-preserving translated stroke union",row.name,"grant",grant,"independent_SVG_within_bounds",true);
  }
 }
});
for(const row of rows)test(row.name,()=>{
 expect(valid(row.input)).toBe(true);const input=row.input as unknown as BooleanInput,before=structuredClone(input);let previous:PathSegment[]|null=null;
 for(const grant of [1,7,4096]){const result=finish(input,grant,p=>{if(p.done&&"maxPairs" in row.expected){expect(p.pairs).toBeLessThanOrEqual(row.expected.maxPairs as number);expect(p.work).toBeLessThanOrEqual(row.expected.maxWork as number);}}),rings=contours(result);if("segments" in row.expected)expect(result).toEqual(row.expected.segments);expect(rings).toHaveLength(row.expected.contours);const actual=rings.reduce((sum,ring)=>sum+area(ring),0);expect(Math.abs(actual-row.expected.area)).toBeLessThanOrEqual(Math.max(1e-18,row.input.epsilon*row.input.epsilon*16));if(previous)expect(result).toEqual(previous);previous=result;
  if(row.name!=="translated mixed axis preserves geometry"&&row.name!=="tiny separate regions")for(let y=-2;y<17;y++)for(let x=-2;x<23;x++){const p:Vec2=[x+.317,y+.173],values=input.operands.map(operand=>filled(operand.contours,p,operand.fillRule)),expected=values.slice(1).reduce((a,b)=>apply(input.operation,a,b),values[0]!);expect(filled(rings,p)).toBe(expected);}
 }expect(input).toEqual(before);
});
test("rectangular operations match independent polygon clipping and SVG pixels",async()=>{
 const input=rows[0]!.input as unknown as BooleanInput;
 const closed=(ring:readonly Vec2[])=>[...ring,ring[0]!];
 for(const operation of ["union","difference","intersection","xor"] as const){const rings=contours(finish({...input,operation},4096)),a=input.operands[0]!.contours.map(closed),b=input.operands[1]!.contours.map(closed),reference=pc[operation](a,b);
  const oracleArea=reference.reduce((sum,polygon)=>sum+polygon.reduce((sum,ring)=>sum+area(ring as Vec2[]),0),0);expect(rings.reduce((sum,ring)=>sum+area(ring),0)).toBe(oracleArea);
  const actualSvg='<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"><path d="'+pathSegmentsToSvgD(finish({...input,operation},4096))+'" fill="red"/></svg>',oracleSvg='<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"><path d="'+reference.flatMap(p=>p.map(r=>'M'+r.map(q=>q.join(" ")).join(" L")+" Z")).join(" ")+'" fill="red"/></svg>';
  expect(await sharp(Buffer.from(actualSvg)).ensureAlpha().raw().toBuffer()).toEqual(await sharp(Buffer.from(oracleSvg)).ensureAlpha().raw().toBuffer());
 }
});
test("all authored fill rules independently agree with SVG engine pixels",async()=>{
 for(const row of rows){const input=row.input as unknown as BooleanInput,segments=finish(input,4096),points=input.operands.flatMap(o=>o.contours.flat()),x=Math.min(...points.map(p=>p[0]),0),y=Math.min(...points.map(p=>p[1]),0),maxX=Math.max(...points.map(p=>p[0]),0),maxY=Math.max(...points.map(p=>p[1]),0);
  const originX=row.name==="translated mixed axis preserves geometry"?Math.min(...points.map(p=>p[0])):x,originY=y,w=maxX-originX||1,h=maxY-originY||1,margin=Math.max(w,h)*.05;
  const svg=(d:string,rule:string)=>'<svg xmlns="http://www.w3.org/2000/svg" width="72" height="64" viewBox="'+[-margin,-margin,w+margin*2,h+margin*2].join(" ")+'"><path d="'+d+'" fill="red" fill-rule="'+rule+'"/></svg>';
  const operandPixels=await Promise.all(input.operands.map(async o=>{const d=o.contours.filter(r=>r.length).map(r=>'M'+r.map(p=>[p[0]-originX,p[1]-originY].join(" ")).join(" L")+" Z").join(" ");return sharp(Buffer.from(svg(d,o.fillRule))).ensureAlpha().raw().toBuffer();}));
  const local=segments.map(s=>s.kind==="move"||s.kind==="line"?{...s,to:[s.to[0]-originX,s.to[1]-originY]}:s),actual=await sharp(Buffer.from(svg(pathSegmentsToSvgD(local as PathSegment[]),"nonzero"))).ensureAlpha().raw().toBuffer();let checked=0;
  for(let at=0;at<72*64;at++){const alpha=operandPixels.map(p=>p[at*4+3]!);if(alpha.some(n=>n!==0&&n!==255))continue;const expected=alpha.slice(1).reduce((a,b)=>apply(input.operation,a,b===255),alpha[0]===255);expect(Math.abs(actual[at*4+3]!-(expected?255:0))).toBeLessThanOrEqual(1);checked++;}expect(checked).toBeGreaterThan(72*64/2);
 }console.log("[DEBUG] Every authored boolean fill-rule fixture matched independent Sharp SVG pixels");
});
test("exact caps admit the same complete output",()=>{
 const input=rows[0]!.input as unknown as BooleanInput;let last!:BooleanProgress;const expected=finish(input,1,p=>last=p);expect(finish({...input,maxEdges:last.vertices,maxParameters:last.parameters,maxAtomicEdges:last.atomicEdges,maxSegments:last.segments,maxWork:last.work},7)).toEqual(expected);
});
test("invalid input, precision and cap failures remain sticky and private",()=>{
 const input=rows[0]!.input as unknown as BooleanInput;
 for(const patch of [{operation:"bogus"},{operands:[]},{operands:Array(1025).fill(input.operands[0])},{epsilon:0},{epsilon:Infinity},{maxEdges:0},{maxParameters:0},{maxAtomicEdges:0},{maxSegments:0},{maxWork:0}])expect(()=>new BooleanJob({...input,...patch} as BooleanInput)).toThrow();
 for(const patch of [{operands:[{contours:[[[NaN,0]]],fillRule:"nonzero"}]},{operands:[{contours:[],fillRule:"bogus"}]},{operands:[{contours:[[[1e12+1,0]]],fillRule:"nonzero"}]},{maxEdges:3},{maxParameters:2},{maxAtomicEdges:1},{maxSegments:3},{maxWork:1},{...rows[25]!.input,epsilon:1e-12}]){const job=new BooleanJob({...input,...patch} as BooleanInput);expect(()=>{while(!job.advance(4096).done){}}).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();}
 for(const grant of [0,-1,NaN,Infinity,1.5])expect(()=>new BooleanJob(input).advance(grant)).toThrow();
});
test("all public phases cancel and published paths survive cancellation",()=>{
 const input=rows[0]!.input as unknown as BooleanInput,seen=new Set<string>();let work=0;
 finish(input,1,p=>{if(!seen.has(p.phase)){const job=new BooleanJob(input);for(let at=0;at<p.work;at++)job.advance(1);job.cancel();expect(()=>job.result()).toThrow(/cancelled/);expect(()=>job.advance(1)).toThrow(/cancelled/);seen.add(p.phase);}work=p.work;});
 expect(seen).toEqual(new Set(["preparing","indexing","intersections","splitting","classifying","contours","compacting","emitting","complete"]));expect(work).toBeLessThan(input.maxWork);
 const job=new BooleanJob(input);while(!job.advance(4096).done){}const result=job.result(),copy=structuredClone(result);job.cancel();expect(result).toEqual(copy);
});
test("async operations yield and reject observer aborts before publication",async()=>{
 const input=rows[0]!.input as unknown as BooleanInput,events:string[]=[];const timer=setTimeout(()=>events.push("timer"),0);
 expect(await booleanRegions(input,{workBudget:1,onProgress:p=>events.push(p.phase)})).toEqual(finish(input,4096));clearTimeout(timer);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 for(const when of ["preparing","complete"]){const controller=new AbortController();await expect(booleanRegions(input,{workBudget:1,signal:controller.signal,onProgress:p=>{if(p.phase===when)controller.abort();}})).rejects.toThrow(/cancelled/);}
});

/** 🧹️ Neutral private arrangements retire without consuming caller geometry. */
import retirementRows from "../🧫️fixtures/🧹️retirement/🔣️.json";
const retirementValid=ajv.compile({$ref:schema.$id+"#/definitions/retirementProgress"});
test("boolean retirement hands off original operands and completed paths while draining private arrangements",()=>{
 for(const row of retirementRows.cases)for(const grant of [1,7,4096]){
  const fixture=rows.find(r=>r.name===row.source)!;expect(fixture).toBeDefined();const source=structuredClone(fixture.input) as unknown as BooleanInput;
  if("maxParameters" in row)source.maxParameters=row.maxParameters!;const before=structuredClone(source),job=new BooleanJob(source),state=job as any;
  if(row.phase==="failure")expect(()=>job.advance(4096)).toThrow(/parameter/);else if(row.phase!=="fresh"){
   if(row.phase!=="cancelled"){let reached=false;for(let at=0;at<100000;at++){const p=job.advance(1);if(p.phase===row.phase){reached=true;break;}}expect(reached).toBe(true);}
   for(let at=0;at<row.offset;at++)job.advance(1);
  }
  if(row.phase==="cancelled")job.cancel();const published=row.phase==="complete"?job.result():null;
  const inventory=JSON.parse(JSON.stringify({edges:state.source.map((e:any)=>e.parameters.size),grid:state.grid.size,atomic:state.atomicIds.size,outgoing:state.outgoing.size,raw:state.raw.length,positions:state.positions.size,rings:state.rings.length}));
  const expected=retirementRows.flatSlots+inventory.edges.length*retirementRows.sourceEdgeSteps+inventory.edges.reduce((sum:number,n:number)=>sum+n,0)+inventory.grid+inventory.atomic+inventory.outgoing+inventory.raw+inventory.positions+inventory.rings;
  const transferred=job.intoRetirement();expect(transferred.operands).toEqual(before.operands);expect(transferred.operands).toBe(source.operands);
  expect(transferred.output).toBe(published);expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();
  for(const n of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>transferred.job.advance(n)).toThrow(/grant/i);
  let work=0;for(let at=0;at<=expected;at++){const beforeEdges=state.source.length,beforeRings=state.rings.length,beforeRaw=state.raw.length,p=transferred.job.advance(grant);expect(beforeEdges-state.source.length).toBeLessThanOrEqual(grant);expect(beforeRings-state.rings.length).toBeLessThanOrEqual(grant);expect(beforeRaw-state.raw.length).toBeLessThanOrEqual(grant);expect(retirementValid(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}
  expect(work).toBe(expected);expect(transferred.job.terminalIsEmpty()).toBe(true);expect(transferred.job.advance(1)).toEqual({phase:"complete",work,done:true});
  for(const field of ["rules","source","tree","level","nextLevel","query","nodes","atomic","leftWinding","rightWinding","boundary","raw","ring","compactPoints","rings","output"])expect(state[field]).toHaveLength(0);
  for(const field of ["grid","atomicIds","outgoing","positions","indexHeap","splitHeap","ringHeap"])expect(state[field].size).toBe(0);
  expect(state.splitValues).toBe(null);expect(state.splitRing).toBe(null);expect(state.emitting).toBe(null);expect(state.input.operands).toHaveLength(0);expect(source).toEqual(before);
  if(published){expect(resultValid(published)).toBe(true);expect(contours(published).reduce((sum,r)=>sum+area(r),0)).toBe(fixture.expected.area);}
  console.log(`[DEBUG] Boolean retirement ${row.source} ${row.phase}: ${expected} structural steps at grant ${grant}`);
 }
});

import admissionRows from "../../🧹️retire/🧫️fixtures/📥️admission/🔣️.json";
import admissionSchema from "../../🧹️retire/🧬️schema/📥️admission/🔣️.json";
test("retained Boolean admission preserves refused input until close",()=>{expect(new Ajv({strict:true}).compile(admissionSchema)(admissionRows)).toBe(true);for(const row of admissionRows.cases){if(row.kind!=="boolean")continue;const input:BooleanInput={...rows[0]!.input as BooleanInput,maxEdges:row.maxEdges!},operands=input.operands;expect(valid(input)).toBe(false);expect(()=>new BooleanJob(input)).toThrow(/contract/);const job=BooleanJob.admit(input);expect((job as unknown as {input:BooleanInput}).input.operands).toBe(operands);expect(()=>job.advance(1)).toThrow(/contract/);expect(()=>job.result()).toThrow(/contract/);const retired=job.intoRetirement();expect(retired.operands).toBe(operands);expect(retired.output).toBe(null);while(!retired.job.advance(1).done){}expect(retired.job.terminalIsEmpty()).toBe(true);console.log("[DEBUG] Refused Boolean admission retained actual operands until explicit close");}});
