/** 🧫️ Painted path pixels checked against neutral vectors and independent SVG. */
import {expect,test,spyOn} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {createCanvas,Path2D} from "@napi-rs/canvas";
import cases from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import strokeCases from "../../🧫️fixtures/🧹️stroke/🔣️.json";
import strokeSchema from "../../🧬️schema/🧹️stroke/🔣️.json";
import coverageCases from "../../🧫️fixtures/🧹️coverage/🔣️.json";
import coverageSchema from "../../🧬️schema/🧹️coverage/🔣️.json";
import flattenCases from "../../🧫️fixtures/🧹️flatten/🔣️.json";
import flattenSchema from "../../🧬️schema/🧹️flatten/🔣️.json";
import retirementCases from "../../🧫️fixtures/🧹️retirement/🔣️.json";
import retirementSchema from "../../🧬️schema/🧹️retirement/🔣️.json";
import {StrokeOutlineJob} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts";
import {PreparedFill} from "../../../../🎨️fill/🎨️sampling/🟦️.ts";
import {PathFlattenJob} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🟦️.ts";
import {CoverageJob} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🟦️.ts";
import {PathRasterJob,rasterizePath,type PathRasterInput} from "../../🟦️.ts";
const validate=new Ajv({strict:true}).compile(schema);
const validateProgress=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/progress"});
const validateRetirement=new Ajv({strict:true}).compile(schema.definitions.retirementProgress);
test("whole painted path retirement retains real children, private candidates and published pixels",async()=>{
 expect(new Ajv({strict:true}).compile(retirementSchema)(retirementCases)).toBe(true);let comparisons=0;
 for(const row of retirementCases)for(const grant of [1,7,4096]){const source=cases.find(c=>c.name===row.source)!,input=structuredClone(source.input) as PathRasterInput;if(row.mode==="failure")input.segments=[{kind:"move",to:[0,0]},{kind:"line",to:[1e10,0]}];const before=structuredClone(input),job=new PathRasterJob(input),state=job as any,records:{source:object;job:any;work:number}[]=[];
  const spies=[PathFlattenJob,StrokeOutlineJob,CoverageJob,PreparedFill].map(kind=>{const prototype=kind.prototype as any,original=prototype.intoRetirement;return spyOn(prototype,"intoRetirement").mockImplementation(function(this:object){const result=original.call(this),owner="job"in result?result.job:result,record={source:this,job:owner,work:0};records.push(record);const advance=owner.advance.bind(owner);owner.advance=(unit:number)=>{expect(unit).toBe(1);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return result;});});
  try{if(row.mode==="failure")expect(()=>job.advance(4096)).toThrow();else{let reached=false;for(let at=0;at<2000000;at++){if(state.phase===row.phase){reached=true;break;}job.advance(1);}expect(reached).toBe(true);if(row.steps)job.advance(row.steps);}
   const active=[state.flatten,state.outline,state.coverage,state.fill].filter(Boolean),closing=["flattenRetirement","outlineRetirement","coverageRetirement"].map(key=>[key,state[key]] as const),pixels=state.pixels,published=row.mode==="published"?job.result():null;
   if(row.mode==="cancelled"||published){job.cancel();if(!published)expect(state.pixels).toBe(pixels);for(const [key,owner]of closing)if(owner)expect(state[key]).toBe(owner);for(const child of active)expect([state.flatten,state.outline,state.coverage,state.fill]).toContain(child);}
   const retired=(job as any).intoRetirement();expect(retired.output===null).toBe(row.phase!=="complete"||row.mode!=="live");if(retired.output)expect(retired.output.pixels).toBe(pixels);for(const [key,owner]of closing)if(owner)expect(state[key]).toBe(owner);for(const child of active)expect(records.some(record=>record.source===child)).toBe(true);
   expect(()=>job.result()).toThrow(/cancelled/);expect(()=>job.advance(1)).toThrow(/cancelled/);expect(()=>(job as any).intoRetirement()).toThrow(/transferred/);job.cancel();for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>retired.job.advance(invalid)).toThrow(/grant/i);
   const inventory=()=>[state.flat,state.fillContours,state.strokeContours,state.strokePolygons].reduce((n:number,contours:any[])=>n+contours.length+contours.reduce((sum,c)=>sum+(Array.isArray(c)?c:c.points).length,0),0);let work=0,done=false;
   for(let at=0;at<2000000;at++){const owners=inventory(),p=retired.job.advance(grant);expect(validateRetirement(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(owners-inventory()).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}
   expect(done).toBe(true);expect(retired.job.terminalIsEmpty()).toBe(true);expect(retired.job.advance(1)).toEqual({phase:"complete",work,done:true});expect(inventory()).toBe(0);for(const key of ["flatten","outline","coverage","fill","flattenRetirement","outlineRetirement","coverageRetirement","fillRetirement","fillMask","strokeMask","style","strokeColor"])expect(state[key]).toBeNull();expect(state.pixels.length).toBe(0);expect(state.segments.length).toBe(0);expect(state.source.length).toBe(0);for(const record of records)expect(record.job.terminalIsEmpty()).toBe(true);expect(input).toEqual(before);
   const output=retired.output??published;if(output){expect([...output.pixels]).toEqual(source.expected);const reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();expect(delta(output.pixels,reference)).toBeLessThanOrEqual(2);comparisons++;}
   process.stderr.write(`[DEBUG] Whole painted path retirement ${row.name}: grant=${grant} structural_work=${work} children=${records.length} terminal_empty=true\n`);
  }finally{spies.forEach(spy=>spy.mockRestore());}
 }
 process.stderr.write(`[DEBUG] Whole path published ownership survived ${comparisons} independent SVG comparisons\n`);
});
test("async whole path drains real owners after final cancellation and callback failure",async()=>{
 const original=(PathRasterJob.prototype as any).intoRetirement;
 for(const mode of ["success","abort","callback"]){const input=structuredClone(cases[15]!.input) as PathRasterInput,before=structuredClone(input),controller=new AbortController();let adopted=0,closed:any=null,work=0;
  const spy=spyOn(PathRasterJob.prototype as any,"intoRetirement").mockImplementation(function(this:PathRasterJob){adopted++;const result=original.call(this);closed=result;const advance=result.job.advance.bind(result.job);result.job.advance=(grant:number)=>{expect(grant).toBe(1);const p=advance(grant);expect(p.work-work).toBe(1);work=p.work;return p;};return result;});
  try{const pending=rasterizePath(input,{signal:controller.signal,workBudget:1,onProgress:p=>{if(p.done){if(mode==="abort")controller.abort();if(mode==="callback")throw Error("whole path completion callback failed");}}});if(mode==="success"){const image=await pending;expect(image.pixels).toBe(closed.output.pixels);expect([...image.pixels]).toEqual(cases[15]!.expected);}else{await expect(pending).rejects.toThrow(mode==="abort"?/cancelled/:/completion callback failed/);expect(closed.output).toBeNull();}expect(adopted).toBe(1);expect(closed.job.terminalIsEmpty()).toBe(true);expect(work).toBeGreaterThanOrEqual(16);expect(input).toEqual(before);process.stderr.write(`[DEBUG] Actual async whole-path ${mode}: one adoption and ${work} one-unit cleanup steps\n`);
  }finally{spy.mockRestore();}
 }
});
test("painted paths consume real flattened contours before conversion",async()=>{
 expect(new Ajv({strict:true}).compile(flattenSchema)(flattenCases)).toBe(true);const original=PathFlattenJob.prototype.intoRetirement;let comparisons=0;
 for(const row of flattenCases)for(const grant of [1,7,4096]){const source=cases.find(c=>c.name===row.source)!,input=source.input as PathRasterInput,before=structuredClone(input),job=new PathRasterJob(input),state=job as any;let adopted=0,childWork=0,owner:ReturnType<PathFlattenJob["intoRetirement"]>|null=null;
  const spy=spyOn(PathFlattenJob.prototype,"intoRetirement").mockImplementation(function(this:PathFlattenJob){adopted++;const contours=this.result(),retired=original.call(this);owner=retired;expect(retired.output).toBe(contours);const advance=retired.job.advance.bind(retired.job);retired.job.advance=unit=>{expect(unit).toBe(1);expect(state.phase).toBe("flattenCleanup");expect(state.flat).toBe(contours);expect(state.flatten).toBeNull();expect(state.fillContours).toHaveLength(0);expect(state.strokeContours).toHaveLength(0);expect(state.coverage).toBeNull();const p=advance(unit);expect(p.work-childWork).toBe(1);childWork=p.work;return p;};return retired;});
  try{let done=false,work=0;for(let at=0;at<2000000;at++){if(state.phase==="contours")expect(owner!.job.terminalIsEmpty()).toBe(true);const p=job.advance(grant);expect(validateProgress(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}expect(done).toBe(true);}finally{spy.mockRestore();}
  expect(adopted).toBe(1);expect(childWork).toBe(row.work);expect(owner!.job.terminalIsEmpty()).toBe(true);expect(state.flattenRetirement).toBeNull();expect([...job.result().pixels]).toEqual(source.expected);expect(input).toEqual(before);const reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();expect(delta(job.result().pixels,reference)).toBeLessThanOrEqual(2);comparisons++;
 }
 process.stderr.write(`[DEBUG] Actual painted flatten handoff: ${comparisons} neutral/grant SVG comparisons; contour conversion waits for real child retirement\n`);
});
function complete(input:PathRasterInput,budget=4096) {
 const job=new PathRasterJob(input);let work=0;
 for(let step=0;step<2000000;step++) {
  const progress=job.advance(budget);expect(progress.work-work).toBeLessThanOrEqual(budget);work=progress.work;
  if(progress.done)return job.result();
 }
 throw Error("Painted path job did not terminate");
}
function pathD(segments:PathRasterInput["segments"]):string {
 return segments.map((s:any)=>s.kind==="close"?"Z":s.kind==="move"?`M${s.to}`:s.kind==="line"?`L${s.to}`:s.kind==="quad"?`Q${s.ctrl} ${s.to}`:s.kind==="cubic"?`C${s.ctrl1} ${s.ctrl2} ${s.to}`:`A${s.rx} ${s.ry} ${s.rotation} ${Number(s.largeArc)} ${Number(s.sweep)} ${s.to}`).join(" ");
}
function svg(input:PathRasterInput,scale=1):string {
 const fill=input.fill;let defs="",paint="none",alpha=1;
 const rgb=(color:readonly number[])=>`rgb(${color.slice(0,3).map(v=>v*255).join(",")})`;
 if(fill?.kind==="solid") {paint=rgb(fill.color);alpha=fill.color[3];}
 else if(fill) {
  const constant=fill.stops.length<=1||(fill.kind==="linearGradient"?fill.x1===fill.x2&&fill.y1===fill.y2:fill.r===0);
  if(constant) {const color=fill.stops.length?([...fill.stops].sort((a:any,b:any)=>a.offset-b.offset).at(-1)!.color):[0,0,0,0];paint=rgb(color);alpha=color[3]!;}
  else {
   const tag=fill.kind==="linearGradient"?"linearGradient":"radialGradient",geometry=fill.kind==="linearGradient"?`x1="${fill.x1}" y1="${fill.y1}" x2="${fill.x2}" y2="${fill.y2}"`:`cx="${fill.cx}" cy="${fill.cy}" r="${fill.r}"`;
   defs=`<defs><${tag} id="paint" gradientUnits="userSpaceOnUse" ${geometry}>${[...fill.stops].sort((a:any,b:any)=>a.offset-b.offset).map((s:any)=>`<stop offset="${s.offset}" stop-color="${rgb(s.color)}" stop-opacity="${s.color[3]}"/>`).join("")}</${tag}></defs>`;paint="url(#paint)";
  }
 }
 const stroke=input.stroke,m=[...input.transform];m[4]-=input.origin[0];m[5]-=input.origin[1];
 const style=stroke?`stroke="${rgb(stroke.color)}" stroke-opacity="${stroke.color[3]}" stroke-width="${stroke.width}" stroke-linecap="${stroke.cap}" stroke-linejoin="${stroke.join}" stroke-miterlimit="4" ${stroke.dash?.some((v:number)=>v>0)?`stroke-dasharray="${stroke.dash.join(" ")}"`:""}`:"";
 return `<svg xmlns="http://www.w3.org/2000/svg" width="${input.width*scale}" height="${input.height*scale}" viewBox="0 0 ${input.width} ${input.height}">${defs}<path d="${pathD(input.segments)}" transform="matrix(${m.join(" ")})" fill="${paint}" fill-opacity="${alpha}" fill-rule="${input.fillRule}" ${style}/></svg>`;
}
function delta(a:Uint8Array,b:Uint8Array):number {
 let max=0;
 for(let p=0;p<a.length;p+=4) {max=Math.max(max,Math.abs(a[p+3]!-b[p+3]!));for(let c=0;c<3;c++)max=Math.max(max,Math.abs(a[p+c]!*a[p+3]!/255-b[p+c]!*b[p+3]!/255));}
 return max;
}
for(const row of cases)test(row.name,async()=>{
 expect(validate(row.input)).toBe(true);const input=row.input as PathRasterInput,before=structuredClone(input);
 for(const budget of [1,7,4096])expect([...complete(input,budget).pixels]).toEqual(row.expected);
 expect(input).toEqual(before);
 let reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();
 if(row.name==="zero length round cap") {const raw=await sharp(Buffer.from(svg(input,128))).ensureAlpha().raw().toBuffer();let alpha=0;for(let at=3;at<raw.length;at+=4)alpha+=raw[at]!;reference=Buffer.from([255,0,0,Math.round(alpha/(128*128))]);}
 expect(delta(new Uint8Array(row.expected),reference)).toBeLessThanOrEqual(2);
});
test("painted path lifecycle is resumable, cancellable and refuses partial pixels",()=>{
 for(const steps of [0,1,10,30,80]) {
  const job=new PathRasterJob(cases[15]!.input as PathRasterInput);expect(()=>job.result()).toThrow();
  for(let at=0;at<steps;at++)job.advance(1);job.cancel();expect(()=>job.advance(1)).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);
 }
 const input=cases[0]!.input as PathRasterInput,before=structuredClone(input),job=new PathRasterJob(input);
 while(!job.advance(4096).done){}const pixels=job.result().pixels;job.cancel();expect([...pixels]).toEqual(cases[0]!.expected);expect(input).toEqual(before);
});
test("async painted paths yield, report phases and respond to abort",async()=>{
 const events:string[]=[];const timer=setTimeout(()=>events.push("timer"),0);
 const image=await rasterizePath(cases[15]!.input as PathRasterInput,{workBudget:7,onProgress:(p:any)=>events.push(p.phase)});
 clearTimeout(timer);expect([...image.pixels]).toEqual(cases[15]!.expected);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(rasterizePath(cases[0]!.input as PathRasterInput,{workBudget:1,signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);
 let reported=false;await expect(rasterizePath(cases[0]!.input as PathRasterInput,{signal:controller.signal,onProgress:()=>{reported=true;}})).rejects.toThrow(/cancelled/);expect(reported).toBe(false);
});
test("invalid extent, paint, geometry and work grants fail without publication",()=>{
 const input=cases[0]!.input as PathRasterInput;
 for(const patch of [{width:0},{width:16384,height:16384},{origin:[0,Infinity]},{fillRule:"inverse"},{fill:{kind:"solid",color:[2,0,0,1]}},{stroke:{color:[1,0,0,1],width:-1,cap:"butt",join:"miter"}},{transform:[1,0,0,1,Infinity,0]},{tolerance:0}])expect(()=>new PathRasterJob({...input,...patch} as PathRasterInput)).toThrow();
 for(const grant of [0,-1,1.5,NaN,Infinity])expect(()=>new PathRasterJob(input).advance(grant)).toThrow();
 const job=new PathRasterJob({...input,segments:[{kind:"move",to:[NaN,0]}]});expect(()=>job.advance(4096)).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
});

test("authored curves retain transformed stroke caps, joins and dash boundaries",async()=>{
 const paths:PathRasterInput["segments"][]=[
  [{kind:"move",to:[.5,1.5]},{kind:"cubic",ctrl1:[3,-2],ctrl2:[-2,4],to:[2.5,1.5]}],
  [{kind:"move",to:[.5,1.5]},{kind:"quad",ctrl:[4,-1],to:[2.5,2.5]}],
  ...[false,true].flatMap(largeArc=>[false,true].map(sweep=>[{kind:"move",to:[.5,1.5]},{kind:"arc",rx:2,ry:.75,rotation:35,largeArc,sweep,to:[2.5,1.5]},{kind:"close"}] as PathRasterInput["segments"]))
 ];let comparisons=0;
 for(const segments of paths)for(const cap of ["butt","round","square"] as const)for(const join of ["miter","round","bevel"] as const)for(const dash of [null,[.6,.4],[0,.75]]) {
  const input:PathRasterInput={width:4,height:4,origin:[0,0],segments,transform:comparisons%2?[1,.2,-.15,.8,.3,.25]:[-.8,.2,.3,1,3,.1],tolerance:.0001,fillRule:"nonzero",fill:null,stroke:{color:[1,0,0,.7],width:.5,cap,join,dash}};
  const actual=complete(input).pixels,scale=128;let raw:Uint8Array;
  raw=await sharp(Buffer.from(svg(input,scale))).ensureAlpha().raw().toBuffer();
  if(segments[1]?.kind==="arc") {
   const canvas=createCanvas(4*scale,4*scale),ctx=canvas.getContext("2d");ctx.scale(scale,scale);ctx.transform(...input.transform as [number,number,number,number,number,number]);ctx.strokeStyle="red";ctx.globalAlpha=.7;ctx.lineWidth=.5;ctx.lineCap=cap;ctx.lineJoin=join;ctx.miterLimit=4;ctx.setLineDash(dash??[]);ctx.stroke(new Path2D(pathD(segments)));raw=ctx.getImageData(0,0,canvas.width,canvas.height).data;
  }
  let max=0;
  for(let at=0;at<16;at++) {const x=at%4,y=Math.floor(at/4);let alpha=0;for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)alpha+=raw[((y*scale+dy)*4*scale+x*scale+dx)*4+3]!;max=Math.max(max,Math.abs(actual[at*4+3]!-Math.round(alpha/(scale*scale))));}
  const limit=segments[1]?.kind==="arc"?3:2;if(max>limit)throw Error(JSON.stringify({input,max,actual:[...actual]}));expect(max).toBeLessThanOrEqual(limit);comparisons++;
 }
 console.error(`[DEBUG] ${comparisons} authored curve/stroke Sharp and native-canvas comparisons completed`);
},120000);
test("local gradient paint follows both affine axes and canvas origin",async()=>{
 const input:PathRasterInput={...cases[5]!.input as PathRasterInput,width:24,height:24,origin:[-3,2],segments:[{kind:"move",to:[-100,-100]},{kind:"line",to:[100,-100]},{kind:"line",to:[100,100]},{kind:"line",to:[-100,100]},{kind:"close"}],transform:[1,.2,.4,.9,4,2],fill:{kind:"linearGradient",x1:-5,y1:3,x2:25,y2:17,stops:[{offset:0,color:[1,.2,.5,.6]},{offset:.4,color:[.2,.7,.4,.8]},{offset:1,color:[.8,.3,.1,1]}]}};
 const actual=complete(input).pixels,reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();expect(delta(actual,reference)).toBeLessThanOrEqual(2);
});

test("reversing quadratic retains the ideal segment area despite renderer cusp variation",async()=>{
 const input:PathRasterInput={...cases[0]!.input as PathRasterInput,width:4,height:4,fill:null,segments:[{kind:"move",to:[.5,1.5]},{kind:"quad",ctrl:[5,1.5],to:[.5,1.5]}],transform:[1,.2,-.15,.8,.3,.25],stroke:{color:[1,0,0,.7],width:.5,cap:"butt",join:"miter"}};
 const equivalent:PathRasterInput={...input,segments:[{kind:"move",to:[.5,1.5]},{kind:"line",to:[2.75,1.5]},{kind:"line",to:[.5,1.5]}]};
 expect([...complete(input).pixels]).toEqual([...complete(equivalent).pixels]);
 const scale=128,canvas=createCanvas(4*scale,4*scale),ctx=canvas.getContext("2d");ctx.scale(scale,scale);ctx.transform(...input.transform as [number,number,number,number,number,number]);ctx.strokeStyle="red";ctx.globalAlpha=.7;ctx.lineWidth=.5;ctx.lineCap="butt";ctx.lineJoin="miter";ctx.stroke(new Path2D(pathD(input.segments)));const reference=ctx.getImageData(0,0,canvas.width,canvas.height).data,actual=complete(input).pixels;let max=0;
 for(let at=0;at<16;at++) {let alpha=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)alpha+=reference[(((Math.floor(at/4)*scale+y)*4*scale+at%4*scale+x)*4)+3]!;max=Math.max(max,Math.abs(actual[at*4+3]!-Math.round(alpha/(scale*scale))));}
 expect(max).toBeLessThanOrEqual(8);console.error(`[DEBUG] Reversing quadratic equals exact doubled line area; native canvas cusp variation=${max}`);
});

test("painted paths retire the real stroke before downstream coverage",async()=>{
 expect(new Ajv({strict:true}).compile(strokeSchema)(strokeCases)).toBe(true);let comparisons=0;
 for(const row of strokeCases)for(const grant of [1,7,4096]) {
  const source=cases.find(c=>c.name===row.source)!;expect(source).toBeDefined();const input=source.input as PathRasterInput,before=structuredClone(input),job=new PathRasterJob(input),state=job as any;
  let work=0,cleanupWork=0,cleanupCalls=0,observed=false,completed=false,polygons:unknown=null,retired:any=null;
  for(let step=0;step<2000000;step++) {
   const closing=state.phase==="strokeCleanup",owner=state.outlineRetirement;
   if(closing) {
    observed=true;expect(state.outline).toBeNull();expect(state.coverage).toBeNull();expect(state.fillMask).toBeNull();expect(state.strokeMask).toBeNull();expect(()=>job.result()).toThrow(/incomplete/);if(!polygons)polygons=state.strokePolygons;expect(state.strokePolygons).toBe(polygons);
    if(!retired){retired=owner;const advance=owner.advance.bind(owner);owner.advance=(unit:number)=>{expect(unit).toBe(1);expect(state.coverage).toBeNull();expect(state.fillMask).toBeNull();expect(state.strokeMask).toBeNull();const p=advance(unit);expect(p.work-cleanupWork).toBe(1);cleanupWork=p.work;cleanupCalls++;return p;};}
    expect(owner).toBe(retired);
   }
   const budget=closing||retired?grant:1,p=job.advance(budget);expect(validateProgress(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(budget);work=p.work;
   if(closing){if(state.phase==="strokeCleanup")expect(state.outlineRetirement).toBe(owner);else{expect(owner.terminalIsEmpty()).toBe(true);expect(state.outlineRetirement).toBeNull();}}
   if(p.done){completed=true;break;}
  }
  expect(completed).toBe(true);expect(observed).toBe(row.work.typescript>0);expect(cleanupWork).toBe(row.work.typescript);expect(cleanupCalls).toBe(row.work.typescript);expect([...job.result().pixels]).toEqual(source.expected);expect(input).toEqual(before);
  let reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();
  if(source.name==="zero length round cap") {const raw=await sharp(Buffer.from(svg(input,128))).ensureAlpha().raw().toBuffer();let alpha=0;for(let at=3;at<raw.length;at+=4)alpha+=raw[at]!;reference=Buffer.from([255,0,0,Math.round(alpha/(128*128))]);}
  expect(delta(job.result().pixels,reference)).toBeLessThanOrEqual(2);comparisons++;
 }
 process.stderr.write(`[DEBUG] Actual painted-path stroke handoff: ${comparisons} neutral/grant SVG comparisons; downstream coverage waits for terminal child retirement\n`);
});

test("painted stroke cleanup refuses invalid grants and interrupted output",()=>{
 for(const row of strokeCases.filter(c=>c.work.typescript>0)) {
  const input=cases.find(c=>c.name===row.source)!.input as PathRasterInput,job=new PathRasterJob(input),state=job as any;
  for(let at=0;at<2000000&&state.phase!=="strokeCleanup";at++)job.advance(1);
  expect(state.phase).toBe("strokeCleanup");const owner=state.outlineRetirement,polygons=state.strokePolygons,work=state.work;
  for(const grant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>job.advance(grant)).toThrow(/grant/);
  expect(state.work).toBe(work);expect(state.outlineRetirement).toBe(owner);expect(state.strokePolygons).toBe(polygons);job.advance(1);job.cancel();expect(()=>job.advance(1)).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);
 }
});

test("painted paths consume actual coverage masks before downstream work",async()=>{
 expect(new Ajv({strict:true}).compile(coverageSchema)(coverageCases)).toBe(true);const original=CoverageJob.prototype.intoRetirement;let comparisons=0;
 for(const row of coverageCases)for(const grant of [1,7,4096]){
  const source=cases.find(c=>c.name===row.source)!,input=source.input as PathRasterInput,before=structuredClone(input),job=new PathRasterJob(input),state=job as any,records:{kind:string;owner:ReturnType<CoverageJob["intoRetirement"]>;work:number}[]=[];
  const spy=spyOn(CoverageJob.prototype,"intoRetirement").mockImplementation(function(this:CoverageJob){const kind=state.phase==="fillCoverage"?"fill":"stroke",mask=this.result(),owner=original.call(this);expect(owner.output).toBe(mask);const record={kind,owner,work:0};records.push(record);const advance=owner.job.advance.bind(owner.job);owner.job.advance=unit=>{expect(unit).toBe(1);expect(state.phase).toBe(`${kind}CoverageCleanup`);expect(state.coverage).toBeNull();expect(state[`${kind}Mask`]).toBe(mask);expect(state.pixels.every((v:number)=>v===0)).toBe(true);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return owner;});
  try{let done=false,work=0;for(let at=0;at<2000000;at++){const p=job.advance(grant);expect(validateProgress(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}expect(done).toBe(true);}finally{spy.mockRestore();}
  expect(records.map(r=>r.kind)).toEqual(row.coverage);for(const record of records){expect(record.work).toBeGreaterThanOrEqual(14);expect(record.owner.job.terminalIsEmpty()).toBe(true);expect(record.owner.output!.coverage.length).toBe(input.width*input.height);}expect(state.coverageRetirement).toBeNull();expect([...job.result().pixels]).toEqual(source.expected);expect(input).toEqual(before);
  const reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();expect(delta(job.result().pixels,reference)).toBeLessThanOrEqual(2);comparisons++;
 }
 process.stderr.write(`[DEBUG] Actual painted coverage handoff: ${comparisons} neutral/grant SVG comparisons; masks move once and painting waits for real private sweep retirement\n`);
});
