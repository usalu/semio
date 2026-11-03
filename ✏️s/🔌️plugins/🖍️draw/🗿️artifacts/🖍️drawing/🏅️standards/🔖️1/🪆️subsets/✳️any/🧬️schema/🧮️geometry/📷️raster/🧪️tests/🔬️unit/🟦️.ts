/** 🧫️ Painted path pixels checked against neutral vectors and independent SVG. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {createCanvas,Path2D} from "@napi-rs/canvas";
import cases from "../../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import strokeCases from "../../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️stroke/🔣️.json";
import coverageCases from "../../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️coverage/🔣️.json";
import flattenCases from "../../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️flatten/🔣️.json";
import retirementCases from "../../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️retirement/🔣️.json";
import {PathRasterJob,type PathRasterInput} from "../../🟦️.ts";
import {stock,drain,drive,controllerFixture} from "../../../../🎬️scene/🎨️paint/🧪️tests/🎟️funding/🟦️.ts";
import {driveDrawingSceneWork} from "../../../../🎬️scene/🎟️work/🟦️.ts";
import {admitted} from "../../../../../🧪️tests/🧫️fixtures/🟦️.ts";
const validate=new Ajv({strict:true}).compile<PathRasterInput>(schema);
const rasterInput=(value:unknown):PathRasterInput=>admitted(validate,value,"path raster input");
const validateProgress=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/progress"});
const validateRetirement=new Ajv({strict:true}).compile(schema.definitions.retirementProgress);
function complete(input:PathRasterInput,budget=4096) {
 const job=new PathRasterJob(input);drive(job,budget);const image=job.result(),pixels=new Uint8Array(image.pixels);const moved=job.intoRetirement();drain(moved.job,budget);return{width:image.width,height:image.height,pixels};
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
 expect(validate(row.input)).toBe(true);const input=rasterInput(row.input),before=structuredClone(input);
 for(const budget of [1,7,4096])expect([...complete(input,budget).pixels]).toEqual(row.expected);
 expect(input).toEqual(before);
 let reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();
 if(row.name==="zero length round cap") {const raw=await sharp(Buffer.from(svg(input,128))).ensureAlpha().raw().toBuffer();let alpha=0;for(let at=3;at<raw.length;at+=4)alpha+=raw[at]!;reference=Buffer.from([255,0,0,Math.round(alpha/(128*128))]);}
 expect(delta(new Uint8Array(row.expected),reference)).toBeLessThanOrEqual(2);
});
test("painted path lifecycle is resumable, cancellable and refuses partial pixels",()=>{
 for(const steps of [0,1,10,30,80]) {
  const job=new PathRasterJob(rasterInput(cases[15]!.input));expect(()=>job.result()).toThrow();
  for(let at=0;at<steps;at++)job.advance(1,stock(1));job.cancel();expect(()=>job.advance(1,stock(1))).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);const moved=job.intoRetirement();expect(moved.output).toBeNull();drain(moved.job);
 }
 const input=rasterInput(cases[0]!.input),before=structuredClone(input),job=new PathRasterJob(input);
 drive(job);const pixels=new Uint8Array(job.result().pixels);job.cancel();expect([...pixels]).toEqual(cases[0]!.expected);expect(input).toEqual(before);drain(job.intoRetirement().job);
});
test("every painted path phase retires its private owners and published pixels",()=>{
 for(const row of retirementCases)for(const grant of [1,7,4096]){const source=cases.find(c=>c.name===row.source)!,input=rasterInput(structuredClone(source.input));if(row.mode==="failure")input.segments=[{kind:"move",to:[NaN,0]},...input.segments] as never;
  const job=new PathRasterJob(input);
  if(row.mode==="failure")expect(()=>drive(job,4096)).toThrow();else{let reached=false;for(let at=0;at<2000000;at++){if((job as any).phase===row.phase){reached=true;break;}job.advance(1,stock(1));}expect(reached,row.phase).toBe(true);for(let at=0;at<row.steps;at++)job.advance(1,stock(1));}
  const published=row.phase==="complete"&&row.mode!=="cancelled"?new Uint8Array(job.result().pixels):null;if(row.mode==="cancelled"||row.mode==="published")job.cancel();
  const moved=job.intoRetirement();expect(moved.output===null).toBe(row.phase!=="complete"||row.mode!=="live");if(moved.output&&published)expect([...moved.output.pixels]).toEqual([...published]);
  expect(()=>job.result()).toThrow(/cancelled/);expect(()=>job.advance(1,stock(1))).toThrow(/cancelled/);expect(()=>job.intoRetirement()).toThrow(/transferred/);
  for(const invalid of [0,-1,.5,NaN,Infinity])expect(()=>moved.job.advance(invalid,stock(1))).toThrow(/grant/);
  drain(moved.job,grant);expect(moved.job.terminalIsEmpty()).toBe(true);if(moved.output)expect([...moved.output.pixels]).toEqual(source.expected);
 }
});
test("async painted paths yield under a caller controller and respond to abort",async()=>{
 const rasterize=(value:PathRasterInput,quantum:number,signal?:AbortSignal,onProgress?:(p:any)=>void)=>{const{control}=controllerFixture(value,quantum,signal);return driveDrawingSceneWork(control.work,control.operation,control.source,new PathRasterJob(value),onProgress);};
 const events:string[]=[];const timer=setTimeout(()=>events.push("timer"),0);
 const image=await rasterize(rasterInput(cases[15]!.input),7,undefined,(p:any)=>events.push(p.phase));
 clearTimeout(timer);expect([...image.pixels]).toEqual(cases[15]!.expected);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(rasterize(rasterInput(cases[0]!.input),1,controller.signal,()=>controller.abort())).rejects.toThrow(/cancel|abort/i);
});
test("invalid extent, paint, geometry and work grants fail without publication",()=>{
 const input=rasterInput(cases[0]!.input);
 for(const patch of [{width:0},{width:16384,height:16384},{origin:[0,Infinity]},{fillRule:"inverse"},{fill:{kind:"solid",color:[2,0,0,1]}},{stroke:{color:[1,0,0,1],width:-1,cap:"butt",join:"miter"}},{transform:[1,0,0,1,Infinity,0]},{tolerance:0}]){let job:PathRasterJob|undefined;expect(()=>{job=new PathRasterJob({...input,...patch} as PathRasterInput);drive(job);}).toThrow();if(job)drain(job.intoRetirement().job);}
 for(const grant of [0,-1,1.5,NaN,Infinity])expect(()=>new PathRasterJob(input).advance(grant,stock(1))).toThrow();
 const job=new PathRasterJob({...input,segments:[{kind:"move",to:[NaN,0]}]});expect(()=>drive(job)).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1,stock(1))).toThrow();drain(job.intoRetirement().job);
});

test("authored curves retain transformed stroke caps, joins and dash boundaries",async()=>{
 const paths:PathRasterInput["segments"][]=[
  [{kind:"move",to:[.5,1.5]},{kind:"cubic",ctrl1:[3,-2],ctrl2:[-2,4],to:[2.5,1.5]}],
  [{kind:"move",to:[.5,1.5]},{kind:"quad",ctrl:[4,-1],to:[2.5,2.5]}],
  ...[false,true].flatMap(largeArc=>[false,true].map(sweep=>[{kind:"move",to:[.5,1.5]},{kind:"arc",rx:2,ry:.75,rotation:35,largeArc,sweep,to:[2.5,1.5]},{kind:"close"}] as PathRasterInput["segments"]))
 ];let comparisons=0;
 for(const segments of paths)for(const cap of ["butt","round","square"] as const)for(const join of ["miter","round","bevel"] as const)for(const dash of [null,[.6,.4],[0,.75]]) {
  const input:PathRasterInput={width:4,height:4,origin:[0,0],segments,transform:comparisons%2?[1,.2,-.15,.8,.3,.25]:[-.8,.2,.3,1,3,.1],tolerance:.0001,fillRule:"nonzero",fill:null,stroke:{color:[1,0,0,.7],width:.5,cap,join,dash}};
  const actual=complete(input).pixels,scale=128;let raw:Uint8Array|Uint8ClampedArray;
  raw=await sharp(Buffer.from(svg(input,scale))).ensureAlpha().raw().toBuffer();
  if(segments[1]?.kind==="arc") {
   const canvas=createCanvas(4*scale,4*scale),ctx=canvas.getContext("2d");ctx.scale(scale,scale);ctx.transform(...input.transform as [number,number,number,number,number,number]);ctx.strokeStyle="red";ctx.globalAlpha=.7;ctx.lineWidth=.5;ctx.lineCap=cap;ctx.lineJoin=join;ctx.miterLimit=4;ctx.setLineDash(dash??[]);ctx.stroke(new Path2D(pathD(segments)));raw=ctx.getImageData(0,0,canvas.width,canvas.height).data;
  }
  let max=0;
  for(let at=0;at<16;at++) {const x=at%4,y=Math.floor(at/4);let alpha=0;for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)alpha+=raw[((y*scale+dy)*4*scale+x*scale+dx)*4+3]!;max=Math.max(max,Math.abs(actual[at*4+3]!-Math.round(alpha/(scale*scale))));}
  const limit=segments[1]?.kind==="arc"?3:2;if(max>limit)throw Error(JSON.stringify({input,max,actual:[...actual]}));expect(max).toBeLessThanOrEqual(limit);comparisons++;
 }
},120000);
test("local gradient paint follows both affine axes and canvas origin",async()=>{
 const input:PathRasterInput={...rasterInput(cases[5]!.input),width:24,height:24,origin:[-3,2],segments:[{kind:"move",to:[-100,-100]},{kind:"line",to:[100,-100]},{kind:"line",to:[100,100]},{kind:"line",to:[-100,100]},{kind:"close"}],transform:[1,.2,.4,.9,4,2],fill:{kind:"linearGradient",x1:-5,y1:3,x2:25,y2:17,stops:[{offset:0,color:[1,.2,.5,.6]},{offset:.4,color:[.2,.7,.4,.8]},{offset:1,color:[.8,.3,.1,1]}]}};
 const actual=complete(input).pixels,reference=await sharp(Buffer.from(svg(input))).ensureAlpha().raw().toBuffer();expect(delta(actual,reference)).toBeLessThanOrEqual(2);
});

test("reversing quadratic retains the ideal segment area despite renderer cusp variation",async()=>{
 const input:PathRasterInput={...rasterInput(cases[0]!.input),width:4,height:4,fill:null,segments:[{kind:"move",to:[.5,1.5]},{kind:"quad",ctrl:[5,1.5],to:[.5,1.5]}],transform:[1,.2,-.15,.8,.3,.25],stroke:{color:[1,0,0,.7],width:.5,cap:"butt",join:"miter"}};
 const equivalent:PathRasterInput={...input,segments:[{kind:"move",to:[.5,1.5]},{kind:"line",to:[2.75,1.5]},{kind:"line",to:[.5,1.5]}]};
 expect([...complete(input).pixels]).toEqual([...complete(equivalent).pixels]);
 const scale=128,canvas=createCanvas(4*scale,4*scale),ctx=canvas.getContext("2d");ctx.scale(scale,scale);ctx.transform(...input.transform as [number,number,number,number,number,number]);ctx.strokeStyle="red";ctx.globalAlpha=.7;ctx.lineWidth=.5;ctx.lineCap="butt";ctx.lineJoin="miter";ctx.stroke(new Path2D(pathD(input.segments)));const reference=ctx.getImageData(0,0,canvas.width,canvas.height).data,actual=complete(input).pixels;let max=0;
 for(let at=0;at<16;at++) {let alpha=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)alpha+=reference[(((Math.floor(at/4)*scale+y)*4*scale+at%4*scale+x)*4)+3]!;max=Math.max(max,Math.abs(actual[at*4+3]!-Math.round(alpha/(scale*scale))));}
 expect(max).toBeLessThanOrEqual(8);
});

test("painted stroke cleanup refuses invalid grants and interrupted output",()=>{
 const job=new PathRasterJob(rasterInput(cases[15]!.input));for(let at=0;at<40;at++)job.advance(1,stock(1));const moved=job.intoRetirement();for(const grant of [0,-1,.5,NaN,Infinity])expect(()=>moved.job.advance(grant,stock(1))).toThrow(/grant/);drain(moved.job);expect(moved.output).toBeNull();
});
