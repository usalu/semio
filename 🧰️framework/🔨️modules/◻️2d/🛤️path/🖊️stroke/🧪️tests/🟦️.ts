/** 🧮️ Shared stroke alpha vectors and independent SVG paint. */
import {expect,test,vi} from "vitest";
import Ajv from "ajv";
import sharp from "sharp";
import {createCanvas} from "@napi-rs/canvas";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {StrokeOutlineJob,prepareStroke,type StrokeOutlineInput} from "../🟦️.ts";
import {CoverageJob} from "../../../../🔲️pixels/🖊️coverage/🟦️.ts";
import retirementRows from "../🧫️fixtures/🧹️retirement/🔣️.json";
test("stroke retirement transfers completed paint and retains interrupted private owners until granted cleanup",async()=>{
 const validProgress=new Ajv({strict:true}).compile(schema.definitions.retirementProgress);
 for(const row of retirementRows)for(const grant of [1,7,4096]){
  const base=fixture.find(sample=>sample.name===row.source)!,source=input(base);if(row.repeat)source.contours=Array.from({length:row.repeat},()=>structuredClone(source.contours[0]!));if(row.stop==="failure")source.contours[0]!.points[0]![0]=1000000001;const before=structuredClone(source),job=new StrokeOutlineJob(source),state=job as any;
  if(row.stop==="preparing")job.advance(1);else if(row.stop!=="fresh"){let reached=false;for(let at=0;at<200000;at++){try{job.advance(1);}catch(error){if(row.stop!=="failure")throw error;reached=true;break;}const stop=row.stop==="cancelled"?"queue":row.stop;if(stop==="queue"?state.queue.length>0:stop==="round"?state.round!==null:state.stage===stop){reached=true;break;}}expect(reached,row.name).toBe(true);}
  const external=row.publishedBeforeCancel?job.result():null,externalCopy=external?structuredClone(external):null;if(row.stop==="cancelled"||external||row.cancelBeforeTransfer)job.cancel();const published=row.output?job.result():null;expect(10+state.sources.length+state.runs.length+(published||external?0:state.polygons.length)+state.queue.length,row.name).toBe(row.work.typescript);const moved=job.intoRetirement();expect(moved.output!==null,row.name).toBe(row.output);if(published)expect(moved.output).toBe(published);expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();
  for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>moved.job.advance(invalid)).toThrow(/grant/i);
  let work=0;for(let at=0;at<=row.work.typescript;at++){const count=state.sources.length+state.runs.length+state.polygons.length+state.queue.length,progress=moved.job.advance(grant);expect(validProgress(progress)).toBe(true);expect(progress.work-work).toBeGreaterThan(0);expect(progress.work-work).toBeLessThanOrEqual(grant);expect(count-state.sources.length-state.runs.length-state.polygons.length-state.queue.length).toBeLessThanOrEqual(grant);work=progress.work;if(progress.done)break;}expect(work,row.name).toBe(row.work.typescript);expect(moved.job.terminalIsEmpty()).toBe(true);expect(moved.job.advance(1)).toEqual({phase:"complete",work,done:true});
  for(const key of ["sources","prepared","runs","seams","polygons","queue","pattern"])expect(state[key],`${row.name}: ${key}`).toHaveLength(0);expect(state.round).toBeNull();expect(state.input.contours).toHaveLength(0);expect(state.input.style.dash).toHaveLength(0);expect(source).toEqual(before);
  if(external)expect(external).toEqual(externalCopy);const paint=moved.output??external;if(paint){const coverage=new CoverageJob({width:base.extent[0]!,height:base.extent[1]!,contours:paint,transform:source.transform,rule:"nonzero"});while(!coverage.advance(4096).done){}const pixels=[...coverage.result().coverage];expect(pixels).toEqual(base.expected);const oracle=await reference(source,base.extent);pixels.forEach((pixel,at)=>expect(Math.abs(pixel-oracle[at]!)).toBeLessThanOrEqual(2));const retained=structuredClone(paint),sourcePoints=source.contours.flatMap(contour=>contour.points);for(const polygon of paint)for(const point of polygon)expect(sourcePoints.includes(point)).toBe(false);source.contours[0]!.points[0]![0]+=100;expect(paint).toEqual(retained);source.contours[0]!.points[0]![0]=before.contours[0]!.points[0]![0];}
 }
 process.stderr.write(`[DEBUG] ${retirementRows.length} actual stroke interruption states transferred complete masks against independent SVG and drained private structural owners under grants 1/7/4096 without mutating the borrowed input\n`);
});
test("async stroke wrapper retires unpublished completed candidates on final abort and callback failure",async()=>{
 const source=input(fixture.find(row=>row.name==="round cap")!),original=StrokeOutlineJob.prototype.intoRetirement;
 for(const failure of ["abort","callback"]){const controller=new AbortController(),error=new Error("injected stroke callback failure"),observed:{owner:ReturnType<StrokeOutlineJob["intoRetirement"]>;privatePolygons:number;work:number;maximumGrant:number}[]=[];
  const spy=vi.spyOn(StrokeOutlineJob.prototype,"intoRetirement").mockImplementation(function(this:StrokeOutlineJob){const privatePolygons=(this as any).polygons.length,owner=original.call(this),record={owner,privatePolygons,work:0,maximumGrant:0};observed.push(record);const advance=owner.job.advance.bind(owner.job);owner.job.advance=grant=>{const progress=advance(grant);record.work=progress.work;record.maximumGrant=Math.max(record.maximumGrant,grant);return progress;};return owner;});
  try{const result=prepareStroke(source,{signal:controller.signal,workBudget:1,onProgress:progress=>{if(progress.done){if(failure==="abort")controller.abort();else throw error;}}});if(failure==="abort")await expect(result).rejects.toThrow(/cancelled/i);else await expect(result).rejects.toBe(error);}finally{spy.mockRestore();}
  expect(observed).toHaveLength(1);const record=observed[0]!;expect(record.privatePolygons).toBeGreaterThan(0);expect(record.owner.output).toBeNull();expect(record.owner.job.terminalIsEmpty()).toBe(true);expect(record.work).toBe(15);expect(record.maximumGrant).toBe(1);process.stderr.write(`[DEBUG] Actual async stroke ${failure} after complete geometry retired ${record.privatePolygons} unpublished polygons through ${record.work} one-unit cleanup steps with no output handoff\n`);
 }
});
import type {Vec2} from "../../../🟦️.ts";
const validate=new Ajv({strict:true}).compile(schema);
function input(row:typeof fixture[number]):StrokeOutlineInput {
 const points=row.points.map((point):Vec2=>{const [x,y]=point;if(point.length!==2||typeof x!=="number"||typeof y!=="number")throw Error("Invalid stroke fixture point");return [x,y];});
 return {contours:[{points,closed:row.closed}],transform:[1,0,0,1,0,0],tolerance:.0001,style:{width:row.width,cap:row.cap as StrokeOutlineInput["style"]["cap"],join:row.join as StrokeOutlineInput["style"]["join"],miterLimit:row.miterLimit,dash:row.dash,dashOffset:row.dashOffset}};
}
function complete(source:StrokeOutlineInput,budget:number):Vec2[][] {
 const job=new StrokeOutlineJob(source);let work=0;
 for(let at=0;at<200000;at++){const p=job.advance(budget);expect(p.work-work).toBeLessThanOrEqual(budget);work=p.work;if(p.done)return job.result();}
 throw Error("Stroke preparation did not terminate");
}
function mask(source:StrokeOutlineInput,extent:readonly number[],budget:number):Uint8Array {
 const contours=complete(source,budget),coverage=new CoverageJob({width:extent[0]!,height:extent[1]!,contours,transform:source.transform,rule:"nonzero"});
 while(!coverage.advance(4096).done){}return coverage.result().coverage;
}
async function reference(source:StrokeOutlineInput,extent:readonly number[]):Promise<number[]> {
 const scale=128,w=extent[0]!,h=extent[1]!,s=source.style;
 const d=source.contours.filter(c=>c.points.length>1||c.closed).map(c=>c.points.length?`M${c.points.map(p=>p.join(",")).join("L")}${c.closed?"Z":""}`:"").join("");
 const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="${w*scale}" height="${h*scale}" viewBox="0 0 ${w} ${h}"><path d="${d}" transform="matrix(${source.transform.join(" ")})" fill="none" stroke="red" stroke-width="${s.width}" stroke-linecap="${s.cap}" stroke-linejoin="${s.join}" stroke-miterlimit="${s.miterLimit}" stroke-dasharray="${s.dash.length?s.dash.join(" "):"none"}" stroke-dashoffset="${s.dashOffset}"/></svg>`;
 const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer(),out:number[]=[];
 for(let at=0;at<w*h;at++){const x=at%w,y=Math.floor(at/w);let sum=0;for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)sum+=pixels[((y*scale+dy)*w*scale+x*scale+dx)*4+3]!;out.push(Math.round(sum/(scale*scale)));}
 return out;
}
for(const row of fixture)test(row.name,async()=>{
 const source=input(row),before=structuredClone(source);expect(validate(source)).toBe(true);
 for(const budget of [1,7,4096])expect([...mask(source,row.extent,budget)]).toEqual(row.expected);
 expect(source).toEqual(before);const oracle=await reference(source,row.extent);
 for(let at=0;at<row.expected.length;at++)expect(Math.abs(row.expected[at]!-oracle[at]!)).toBeLessThanOrEqual(2);
});
test("crossings, short joins, reversals and dash degeneracies match SVG under affine transforms",async()=>{
 const paths:StrokeOutlineInput["contours"][]=[
  [{points:[[1,2],[5,2],[1,4],[5,4]],closed:false}],
  [{points:[[1,2],[1.1,2],[1.1,2.1]],closed:false}],
  [{points:[[1,2],[5,2],[1,2]],closed:false}],
  [{points:[[1,1],[5,1],[5,5],[1,5]],closed:true}],
  [{points:[[3,3],[3,3]],closed:false}],
  [{points:[[3,3],[3,3]],closed:true}],
  [{points:[[3,3]],closed:false}],
  [{points:[[3,3]],closed:true}],
 ];
 for(const cap of ["butt","round","square"] as const)for(const join of ["miter","round","bevel"] as const)for(let at=0;at<paths.length;at++)for(const dash of [[],[1,0],[0,1],[1,2,3],[0,0,1,1],[.3,0,.4,.2]]) {
  const source:StrokeOutlineInput={contours:paths[at]!,transform:at%2?[.8,.2,-.3,1,1,0]:[-1,0,.2,1,6,0],tolerance:.0001,style:{width:.8,cap,join,miterLimit:4,dash,dashOffset:.35}},actual=mask(source,[6,6],4096),oracle=await reference(source,[6,6]);
  const delta=Math.max(...actual.map((value,p)=>Math.abs(value-oracle[p]!)));if(delta>2)throw Error(JSON.stringify({cap,join,at,dash,actual:[...actual],oracle,delta}));expect(delta).toBeLessThanOrEqual(2);
 }
},30000);
test("invalid contracts and cancelled preparation cannot expose partial geometry",()=>{
 const source=input(fixture[4]!);
 for(const steps of [0,1,5,20]){const job=new StrokeOutlineJob(source);expect(()=>job.result()).toThrow();for(let at=0;at<steps;at++)job.advance(1);job.cancel();expect(()=>job.advance(1)).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);}
 for(const patch of [{width:-1},{cap:"triangle"},{join:"arcs"},{miterLimit:0},{dash:[-1,2]},{dashOffset:NaN}])expect(()=>new StrokeOutlineJob({...source,style:{...source.style,...patch}} as StrokeOutlineInput)).toThrow();
 for(const grant of [0,-1,NaN,Infinity,1.5])expect(()=>new StrokeOutlineJob(source).advance(grant)).toThrow();
 const bad=new StrokeOutlineJob({...source,contours:[{points:[[NaN,0],[1,0]],closed:false}]});expect(()=>bad.advance(100)).toThrow();expect(()=>bad.result()).toThrow();
});
test("large crossing strokes fit the coverage point budget without a primitive-count cutoff",async()=>{
 const source=input(fixture[2]!);source.contours=[{points:Array.from({length:9001},(_,at)=>[at%2,1] as Vec2),closed:false}];
 expect([...mask(source,[1,2],4096)]).toEqual([255,255]);expect(await reference(source,[1,2])).toEqual([255,255]);
});
test("dash expansion has a geometry budget and never publishes an incomplete candidate",()=>{
 const source=input(fixture[2]!);source.style.dash=[.000001,.000001];const job=new StrokeOutlineJob(source);
 expect(()=>{while(!job.advance(4096).done){}}).toThrow(/budget/);expect(()=>job.result()).toThrow();
});
test("a moveto alone paints no square cap, matching the SVG rule and independent native canvas",()=>{
 const source=input(fixture[11]!);source.style.cap="square";expect([...mask(source,[2,2],4096)]).toEqual([0,0,0,0]);
 const canvas=createCanvas(2,2),ctx=canvas.getContext("2d");ctx.lineCap="square";ctx.lineWidth=2;ctx.moveTo(1,1);ctx.stroke();
 expect([...ctx.getImageData(0,0,2,2).data].filter((_,at)=>at%4===3)).toEqual([0,0,0,0]);
});
test("async stroke preparation yields, responds to cancellation and preserves published geometry",async()=>{
 const source=input(fixture[4]!),before=structuredClone(source),events:string[]=[];
 const timer=setTimeout(()=>events.push("timer"),0),polygons=await prepareStroke(source,{workBudget:7,onProgress:p=>events.push(p.phase)});clearTimeout(timer);
 expect(polygons.length).toBeGreaterThan(0);expect(events).toContain("outlining");expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(prepareStroke(source,{signal:controller.signal,workBudget:1,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);
 let called=false;await expect(prepareStroke(source,{signal:controller.signal,onProgress:()=>{called=true;}})).rejects.toThrow(/cancelled/);expect(called).toBe(false);
 const job=new StrokeOutlineJob(source);while(!job.advance(4096).done){}const published=job.result(),copy=structuredClone(published);job.cancel();expect(published).toEqual(copy);expect(source).toEqual(before);
});
