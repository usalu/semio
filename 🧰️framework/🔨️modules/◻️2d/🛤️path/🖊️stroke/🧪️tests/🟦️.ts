/** 🧮️ Shared stroke alpha vectors and independent SVG paint. */
import {expect,spyOn,test} from "bun:test";
import Ajv from "ajv";
import {reference} from "./🔮️oracle/🟦️.ts";
import {createCanvas} from "@napi-rs/canvas";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {StrokeOutlineJob,type StrokeContours,type StrokeOutlineInput,type StrokeOutlineProgress} from "../🟦️.ts";
import {CoverageJob} from "@semio-tech/framework-pixels-ts/coverage";
import {Bank} from "../../📏️flatten/🧪️tests/🎟️work/🟦️ts/funded.test.ts";
import {RetainedWorkLedger} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import retirementRows from "./🧫️fixtures/🧹️retirement/🔣️.json";
import type {Vec2} from "../../../📐️drawing-contract/🟦️.ts";
const validate=new Ajv({strict:true}).compile(schema),maximumTurns=1048576,ample=()=>new RetainedWorkLedger({maximumItems:1000,maximumCopyBytes:2**30,maximumCapacityBytes:2**30,maximumReleaseBytes:2**30,maximumDepth:64});
type MutableStrokeInput=Omit<StrokeOutlineInput,"contours">&{contours:{points:[number,number][];closed:boolean}[]};
const rich=()=>Object.assign(new Bank(),{items:2**26,cpu:2**26});
const banks=new WeakMap<StrokeOutlineJob,Bank>(),size=(values:{length:number}|null|undefined)=>values?.length??0;
function bank(job:StrokeOutlineJob):Bank{let value=banks.get(job);if(value===undefined){value=new Bank();banks.set(job,value);}return value;}
function advance(job:StrokeOutlineJob,quantum:number):StrokeOutlineProgress{return bank(job).turn(job,work=>job.advance(quantum,work),quantum);}
function drain(job:StrokeOutlineJob):ReturnType<StrokeOutlineJob["intoRetirement"]>{const wallet=bank(job),moved=job.intoRetirement();wallet.close(moved.job);return moved;}
function discard(job:StrokeOutlineJob):void{const wallet=bank(job),moved=drain(job);if(moved.output!==null)wallet.close(moved.output.intoRetirement());wallet.conserved();}
function project(paint:StrokeContours):Vec2[][]{return Array.from({length:paint.length},(_,at)=>{const row=paint.at(at);if(row===undefined)throw Error("Stroke contour absent");return Array.from({length:row.length},(_,point):Vec2=>{const value=row.at(point);if(value===undefined)throw Error("Stroke point absent");return[value[0],value[1]];});});}
function input(row:typeof fixture[number]):MutableStrokeInput {
 const points=row.points.map((point):[number,number]=>{const [x,y]=point;if(point.length!==2||typeof x!=="number"||typeof y!=="number")throw Error("Invalid stroke fixture point");return [x,y];});
 return {contours:[{points,closed:row.closed}],transform:[1,0,0,1,0,0],tolerance:.0001,style:{width:row.width,cap:row.cap as StrokeOutlineInput["style"]["cap"],join:row.join as StrokeOutlineInput["style"]["join"],miterLimit:row.miterLimit,dash:row.dash,dashOffset:row.dashOffset}};
}
function complete(job:StrokeOutlineJob,budget:number):void {
 let work=0;
 for(let at=0;at<maximumTurns;at++){const p=advance(job,budget);expect(p.work-work).toBeLessThanOrEqual(budget);work=p.work;if(p.done)return;}
 throw Error("Stroke preparation did not terminate");
}
function cover(paint:StrokeContours|Vec2[][],source:StrokeOutlineInput,extent:readonly number[],budget:number):number[] {
 const coverage=new CoverageJob({width:extent[0]!,height:extent[1]!,contours:paint,transform:source.transform,rule:"nonzero"}),wallet=rich();
 try{for(let at=0;at<maximumTurns*64;at++)if(wallet.turn(coverage,work=>coverage.advance(budget,work),budget).done)break;return Array.from(coverage.result().borrow());}
 finally{const moved=coverage.intoRetirement();wallet.close(moved.job);
 if(moved.output!==null)wallet.close(moved.output.intoRetirement());}
}
function mask(source:StrokeOutlineInput,extent:readonly number[],budget:number):number[] {
 const job=new StrokeOutlineJob(source);
 try{complete(job,budget);return cover(job.result(),source,extent,budget);}finally{discard(job);}
}
async function prepare(source:StrokeOutlineInput,policy:{quantum:number;signal?:AbortSignal;onProgress?:(progress:StrokeOutlineProgress)=>void}):Promise<Vec2[][]> {
 const abort=()=>{if(policy.signal?.aborted)throw new DOMException("Stroke preparation cancelled","AbortError");};abort();const job=new StrokeOutlineJob(source);
 try{for(;;){abort();const progress=advance(job,policy.quantum);policy.onProgress?.(progress);abort();if(progress.done)return project(job.result());await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}finally{discard(job);}
}
test("stroke retirement transfers completed paint and retains interrupted private owners until granted cleanup",async()=>{
 for(const row of retirementRows)for(const grant of [1,7,4096]){
  const base=fixture.find(sample=>sample.name===row.source)!,source=input(base);if(row.repeat)source.contours=Array.from({length:row.repeat},()=>structuredClone(source.contours[0]!));if(row.stop==="failure")source.contours[0]!.points[0]![0]=1000000001;const before=structuredClone(source),job=new StrokeOutlineJob(source),state=job as any,wallet=bank(job);
  if(row.stop==="preparing")advance(job,1);else if(row.stop!=="fresh"){let reached=false;for(let at=0;at<maximumTurns;at++){let progress:StrokeOutlineProgress;try{progress=advance(job,1);}catch(error){if(row.stop!=="failure")throw error;reached=true;break;}const stop=row.stop==="cancelled"?"queue":row.stop;if(stop==="queue"?size(state.queue)>0:stop==="round"?state.round!==null:stop==="done"?progress.done:state.stage===stop){reached=true;break;}}expect(reached,row.name).toBe(true);}
  const external=row.publishedBeforeCancel?job.result():null,externalCopy=external?project(external):null;if(row.stop==="cancelled"||external||row.cancelBeforeTransfer)job.cancel();const published=row.output?job.result():null;
  const moved=job.intoRetirement();expect(moved.output!==null,row.name).toBe(row.output);if(published)expect(moved.output).toBe(published);expect(()=>advance(job,1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();
  for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>moved.job.advance(invalid,ample())).toThrow(/grant/i);
  const paint=moved.output===null?null:project(moved.output);
  let work=0;for(let at=0;at<maximumTurns;at++){const count=size(state.sources)+size(state.runs)+size(state.polygons)+size(state.queue),progress=wallet.turn(moved.job,ledger=>moved.job.advance(grant,ledger),grant);if(!progress.done)expect(progress.work-work).toBeGreaterThan(0);expect(progress.work-work).toBeLessThanOrEqual(grant);expect(count-size(state.sources)-size(state.runs)-size(state.polygons)-size(state.queue)).toBeLessThanOrEqual(grant);work=progress.work;if(progress.done)break;}
  expect(work,row.name).toBe(row.work.typescript);  expect(moved.job.terminalIsEmpty()).toBe(true);expect(moved.job.advance(1,ample())).toEqual({work,done:true});
  for(const key of ["sources","prepared","runs","seams","polygons","queue","pattern"])expect(size(state[key]),`${row.name}: ${key}`).toBe(0);expect(state.round).toBeNull();expect(size(state.input?.contours)).toBe(0);expect(size(state.input?.style.dash)).toBe(0);expect(source).toEqual(before);
  if(external)expect(()=>external.length).toThrow(/transferred/i);
  const shown=paint??externalCopy;if(shown){const pixels=cover(shown,source,base.extent,4096);expect(pixels).toEqual(base.expected);const oracle=await reference(source,base.extent);pixels.forEach((pixel,at)=>expect(Math.abs(pixel-oracle[at]!)).toBeLessThanOrEqual(2));const retained=structuredClone(shown),sourcePoints=source.contours.flatMap(contour=>contour.points);for(const polygon of shown)for(const point of polygon)expect(sourcePoints.some(candidate=>Object.is(candidate,point))).toBe(false);source.contours[0]!.points[0]![0]+=100;expect(shown).toEqual(retained);source.contours[0]!.points[0]![0]=before.contours[0]!.points[0]![0];}
  if(moved.output!==null)wallet.close(moved.output.intoRetirement());wallet.conserved();
 }
});
test("async stroke wrapper retires unpublished completed candidates on final abort and callback failure",async()=>{
 const source=input(fixture.find(row=>row.name==="round cap")!),original=StrokeOutlineJob.prototype.intoRetirement;
 for(const failure of ["abort","callback"]){const controller=new AbortController(),error=new Error("injected stroke callback failure"),observed:{owner:ReturnType<StrokeOutlineJob["intoRetirement"]>;privatePolygons:number;work:number;maximumGrant:number}[]=[];
  const spy=spyOn(StrokeOutlineJob.prototype,"intoRetirement").mockImplementation(function(this:StrokeOutlineJob){const privatePolygons=size((this as any).polygons)+size((this as any).output),owner=original.call(this),record={owner,privatePolygons,work:0,maximumGrant:0};observed.push(record);const advanceOwner=owner.job.advance.bind(owner.job);owner.job.advance=(grant,ledger)=>{const progress=advanceOwner(grant,ledger);record.work=progress.work;record.maximumGrant=Math.max(record.maximumGrant,grant);return progress;};return owner;});
  try{const result=prepare(source,{quantum:1,signal:controller.signal,onProgress:progress=>{if(progress.done){if(failure==="abort")controller.abort();else throw error;}}});if(failure==="abort")await expect(result).rejects.toThrow(/cancelled/i);else await expect(result).rejects.toBe(error);}finally{spy.mockRestore();}
  expect(observed).toHaveLength(1);const record=observed[0]!;expect(record.privatePolygons).toBeGreaterThan(0);expect(record.owner.output).toBeNull();expect(record.owner.job.terminalIsEmpty()).toBe(true);expect(record.work).toBeGreaterThan(0);expect(record.maximumGrant).toBe(1);
 }
},120000);
for(const row of fixture)test(row.name,async()=>{
 const source=input(row),before=structuredClone(source);expect(validate(source)).toBe(true);
 for(const budget of [1,7,4096])expect(mask(source,row.extent,budget)).toEqual(row.expected);
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
},120000);
test("invalid contracts and cancelled preparation cannot expose partial geometry",()=>{
 const source=input(fixture[4]!);
 for(const steps of [0,1,5,20]){const job=new StrokeOutlineJob(source);try{expect(()=>job.result()).toThrow();for(let at=0;at<steps;at++)advance(job,1);job.cancel();expect(()=>advance(job,1)).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);}finally{discard(job);}}
 for(const patch of [{width:-1},{cap:"triangle"},{join:"arcs"},{miterLimit:0},{dashOffset:NaN}])expect(()=>new StrokeOutlineJob({...source,style:{...source.style,...patch}} as StrokeOutlineInput)).toThrow();
 const negative=new StrokeOutlineJob({...source,style:{...source.style,dash:[-1,2]}});try{expect(()=>complete(negative,100)).toThrow();}finally{discard(negative);}
 for(const grant of [0,-1,NaN,Infinity,1.5])expect(()=>new StrokeOutlineJob(source).advance(grant,ample())).toThrow();
 const bad=new StrokeOutlineJob({...source,contours:[{points:[[NaN,0],[1,0]],closed:false}]});try{expect(()=>complete(bad,100)).toThrow();expect(()=>bad.result()).toThrow();}finally{discard(bad);}
});
test("large crossing strokes fit the coverage point budget without a primitive-count cutoff",async()=>{
 const source=input(fixture[2]!);source.contours=[{points:Array.from({length:9001},(_,at):[number,number]=>[at%2,1]),closed:false}];
 expect(mask(source,[1,2],4096)).toEqual([255,255]);expect(await reference(source,[1,2])).toEqual([255,255]);
},120000);
test("dash expansion has a geometry budget and never publishes an incomplete candidate",()=>{
 const source=input(fixture[2]!);source.style.dash=[.000001,.000001];const job=new StrokeOutlineJob(source);
 try{expect(()=>complete(job,4096)).toThrow(/budget/);expect(()=>job.result()).toThrow();}finally{discard(job);}
},120000);
test("a moveto alone paints no square cap, matching the SVG rule and independent native canvas",()=>{
 const source=input(fixture[11]!);source.style.cap="square";expect(mask(source,[2,2],4096)).toEqual([0,0,0,0]);
 const canvas=createCanvas(2,2),ctx=canvas.getContext("2d");ctx.lineCap="square";ctx.lineWidth=2;ctx.moveTo(1,1);ctx.stroke();
 expect([...ctx.getImageData(0,0,2,2).data].filter((_,at)=>at%4===3)).toEqual([0,0,0,0]);
});
test("caller-funded stroke preparation yields, responds to cancellation and preserves published geometry",async()=>{
 const source=input(fixture[4]!),before=structuredClone(source),events:string[]=[];
 const timer=setTimeout(()=>events.push("timer"),0),polygons=await prepare(source,{quantum:7,onProgress:p=>events.push(p.phase)});clearTimeout(timer);
 expect(polygons.length).toBeGreaterThan(0);expect(events).toContain("outlining");expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(prepare(source,{quantum:1,signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);
 let called=false;await expect(prepare(source,{quantum:1,signal:controller.signal,onProgress:()=>{called=true;}})).rejects.toThrow(/cancelled/);expect(called).toBe(false);
 const job=new StrokeOutlineJob(source);try{complete(job,4096);const published=job.result(),copy=project(published);job.cancel();expect(project(published)).toEqual(copy);expect(()=>job.result()).toThrow(/cancelled/);}finally{discard(job);}expect(source).toEqual(before);
});
