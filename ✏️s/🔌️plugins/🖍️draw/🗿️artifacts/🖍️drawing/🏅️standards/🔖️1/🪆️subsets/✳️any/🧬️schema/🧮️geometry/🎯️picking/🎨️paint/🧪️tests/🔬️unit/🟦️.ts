/** 🎨️ Painted query grants match independent SVG cap, dash, join and affine regions. */
import {expect,test,spyOn} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import sharp from "sharp";
import cases from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import phases from "../../🧫️fixtures/🧹️retirement/🔣️.json";
import resolvedRows from "../../🧫️fixtures/🎬️resolved/🔣️.json";
import pointSchema from "../../../../📍️point/🧬️schema/🔣️.json";
import vectorRows from "../../../../../🎬️scene/📋️prepare/🧫️fixtures/🎬️vector/🔣️.json";
import {DocumentVectorJob} from "../../../../../🎬️scene/📋️prepare/🟦️.ts";
import type {DrawingArtifact} from "../../../../../🟦️.ts";
import {ScenePlanCloseJob} from "../../../../../🎬️scene/🧹️retire/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {PaintedPathHitJob,type PaintedPathQuery} from "../../🟦️.ts";
import type {PathGeometrySegment} from "../../../../../🟦️.ts";
const progressValid=new Ajv({strict:true}).compile(schema.$defs.PaintedPathQueryProgress),hitValid=new Ajv({strict:true}).compile(schema.$defs.PaintedPathQueryHit);
import {PathFlattenJob} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🟦️.ts";
import {StrokeOutlineJob} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts";
test("painted query uses real stroke regions under every grant",async()=>{
 let comparisons=0;
 for(const row of cases)for(const grant of [1,7,4096]){
  const before=structuredClone(row),job=new PaintedPathHitJob(row.query as PaintedPathQuery);let work=0,done=false,admitted=0;
  expect(()=>job.result()).toThrow(/incomplete/);
  for(let at=0;at<2000000;at++){const p=job.advance(grant,index=>{admitted++;return row.segments[index] as PathGeometrySegment|undefined;});expect(progressValid(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}
  expect(done).toBe(true);expect(admitted).toBe(row.segments.length+1);expect(job.result().contains).toBe(row.expected);expect(hitValid(job.result())).toBe(true);if("bounds"in row)expect(job.result().bounds).toEqual(row.bounds);expect(row).toEqual(before);
  const output=job.result(),retired=job.intoRetirement();expect(retired.output).toBe(output);expect(retired.output!.contains).toBe(row.expected);while(!retired.job.advance(grant).done){}expect(retired.job.terminalIsEmpty()).toBe(true);
  if(row.query.tolerance===0){const d=row.segments.map((s:any)=>s.kind==="close"?"Z":s.kind==="move"?`M${s.to}`:s.kind==="line"?`L${s.to}`:s.kind==="quad"?`Q${s.ctrl} ${s.to}`:s.kind==="cubic"?`C${s.ctrl1} ${s.ctrl2} ${s.to}`:`A${s.rx} ${s.ry} ${s.rotation} ${Number(s.largeArc)} ${Number(s.sweep)} ${s.to}`).join(" "),q=row.query,s=q.stroke,svg=`<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100"><path d="${d}" transform="matrix(${q.transform.join(" ")})" fill="${q.fill?"black":"none"}" fill-rule="${q.fillRule}" ${s?`stroke="black" stroke-width="${s.width}" stroke-linecap="${s.cap}" stroke-linejoin="${s.join}" stroke-miterlimit="4" stroke-dasharray="${s.dash.join(" ")}"`:""}/></svg>`;
   const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer(),alpha=pixels[(Math.floor(q.point[1]!)*200+Math.floor(q.point[0]!))*4+3]!;expect(alpha>127).toBe(row.expected);comparisons++;
  }
  process.stderr.write(`[DEBUG] Painted query ${row.name} grant=${grant} work=${work} contains=${row.expected} terminal_empty=true\n`);
 }
 process.stderr.write(`[DEBUG] Painted queries matched ${comparisons} independent SVG samples\n`);
});
test("painted query cancellation retires real borrowed children and owned contours",()=>{
 const row=cases[6]!,before=structuredClone(row);expect(phases.every(phase=>schema.$defs.PaintedPathQueryProgress.properties.phase.enum.includes(phase))).toBe(true);
 for(const phase of phases)for(const grant of [1,7,4096]){
  const job=new PaintedPathHitJob(row.query as PaintedPathQuery),state=job as any,children:{job:any;work:number}[]=[];
  const spies=[PathFlattenJob,StrokeOutlineJob].map(kind=>{const prototype=kind.prototype as any,original=prototype.intoRetirement;return spyOn(prototype,"intoRetirement").mockImplementation(function(this:object){const moved=original.call(this),record={job:moved.job,work:0};children.push(record);const advance=record.job.advance.bind(record.job);record.job.advance=(unit:number)=>{expect(unit).toBe(1);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return moved;});});
  try{
   let reached=false;for(let at=0;at<2000000;at++){if(state.phase===phase){reached=true;break;}job.advance(1,index=>row.segments[index] as PathGeometrySegment|undefined);}expect(reached).toBe(true);
   const flat=state.flat,contours=state.contours,polygons=state.polygons,active=[state.flatten,state.outline].filter(Boolean);job.cancel();expect(state.flat).toBe(flat);expect(state.contours).toBe(contours);expect(state.polygons).toBe(polygons);expect([state.flatten,state.outline].filter(Boolean)).toEqual(active);expect(()=>job.result()).toThrow(/cancelled/);expect(()=>job.advance(1,()=>undefined)).toThrow(/cancelled/);
   const moved=job.intoRetirement();expect(moved.output).toBeNull();expect(()=>job.intoRetirement()).toThrow(/transferred/);for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>moved.job.advance(invalid)).toThrow(/grant/);
   let work=0,done=false;for(let at=0;at<2000000;at++){const p=moved.job.advance(grant);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}
   expect(done).toBe(true);expect(moved.job.terminalIsEmpty()).toBe(true);expect(moved.job.advance(1)).toEqual({phase:"complete",work,done:true});for(const child of children)expect(child.job.terminalIsEmpty()).toBe(true);for(const key of ["flat","contours","polygons","segments"])expect(state[key]).toHaveLength(0);for(const key of ["flatten","outline","flattenRetirement","outlineRetirement","style"])expect(state[key]).toBeNull();expect(state.query.stroke).toBeNull();expect(row).toEqual(before);
   process.stderr.write(`[DEBUG] Painted query cancellation phase=${phase} grant=${grant} children=${children.length} actual_retirement_work=${work} terminal_empty=true\n`);
  }finally{spies.forEach(spy=>spy.mockRestore());}
 }
});
test("painted query refuses unsafe grants before reading a borrowed source",()=>{
 const job=new PaintedPathHitJob(cases[0]!.query as PaintedPathQuery);let reads=0;
 for(const grant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>job.advance(grant,()=>{reads++;return undefined;})).toThrow(/grant/);expect(reads).toBe(0);
 const moved=job.intoRetirement();while(!moved.job.advance(1).done){}expect(moved.job.terminalIsEmpty()).toBe(true);
});
test("failed painted query retains its genuine child until cancellation retires it",()=>{
 const query=cases[0]!.query as PaintedPathQuery,source:PathGeometrySegment[]=[{kind:"move",to:[1e10,0]},{kind:"line",to:[10,10]}],job=new PaintedPathHitJob(query),state=job as any;
 expect(()=>job.advance(4096,index=>source[index])).toThrow(/point/);const child=state.flatten;expect(child).not.toBeNull();expect(()=>job.result()).toThrow(/point/);expect(()=>job.advance(1,()=>undefined)).toThrow(/point/);job.cancel();expect(state.flatten).toBe(child);
 const moved=job.intoRetirement();expect(moved.output).toBeNull();let work=0;while(!moved.job.terminalIsEmpty()){const p=moved.job.advance(1);expect(p.work-work).toBe(1);work=p.work;}expect(state.flatten).toBeNull();expect(state.flattenRetirement).toBeNull();expect(state.segments).toHaveLength(0);process.stderr.write(`[DEBUG] Failed painted query retired its real flatten child in ${work} structural units\n`);
});
test("painted queries borrow actual resolved Boolean and encoded trace geometry",async()=>{
 expect(resolvedRows.every(row=>new Ajv2020({strict:true}).compile(pointSchema)(row.point))).toBe(true);let comparisons=0;
 const lift=(value:any):any=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,value])=>[key,lift(value)])):value;
 for(const row of resolvedRows){const fixture=vectorRows.find(source=>source.name===row.source)!,document={...fixture.document,layers:lift(fixture.document.layers)} as DrawingArtifact,producer=new DocumentVectorJob(document,fixture.limits,fixture.algorithms);while(!producer.advance(4096).done){}const moved=producer.intoRetirement();while(!moved.job.advance(4096).done){}const plan=moved.output!,node=plan.nodes.find(node=>node.id==="result")!;expect(node.content.kind).toBe("path");if(node.content.kind!=="path")throw Error("resolved leaf required");const source=node.content.segments,before=structuredClone(source);
  for(const grant of [1,7,4096]){const query=PaintedPathHitJob.fromPrepared(node,row.point as [number,number],0,.001);while(!query.advance(grant,index=>source[index]).done){}expect(query.result().contains).toBe(row.expected);const retired=query.intoRetirement();while(!retired.job.advance(grant).done){}expect(retired.job.terminalIsEmpty()).toBe(true);expect(node.content.segments).toBe(source);expect(source).toEqual(before);
   const oracle=fixture.oracle as any,paths=oracle.paths??[{d:oracle.d,matrix:oracle.matrix}],svg=`<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">${paths.map((path:any)=>`<path d="${path.d}" transform="matrix(${path.matrix.join(" ")})" fill="black"/>`).join("")}</svg>`,pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();expect(pixels[(Math.floor(row.point[1]!)*16+Math.floor(row.point[0]!))*4+3]!>127).toBe(row.expected);comparisons++;
  }const close=new ScenePlanCloseJob(plan);while(!close.advance(4096).done){}
 }
 process.stderr.write(`[DEBUG] Actual resolved Boolean/PNG trace queries matched ${comparisons} independent authored SVG pixels while borrowing immutable completed geometry\n`);
});
