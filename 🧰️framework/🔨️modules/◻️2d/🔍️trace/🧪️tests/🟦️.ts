/** 🧪️ Neutral bitmap contours validated independently through polygon clipping and SVG pixels. */
import {expect,test} from "vitest";
import Ajv from "ajv";
import sharp from "sharp";
import polygonClipping from "polygon-clipping";
import schema from "../🧬️schema/🔣️.json";
import fixtures from "../🧫️fixtures/🔣️.json";
import {BitmapTraceJob,traceBitmap,type BitmapTraceInput,type BitmapTraceProgress} from "../🟦️.ts";
import {pathSegmentsToSvgD,type PathSegment,type Vec2} from "../../🟦️.ts";
const ajv=new Ajv({strict:true}),inputValid=ajv.compile(schema),outputValid=ajv.compile({$ref:schema.$id+"#/definitions/result"}),progressValid=ajv.compile({$ref:schema.$id+"#/definitions/progress"});
function complete(input:BitmapTraceInput,grant:number,observer?:(p:BitmapTraceProgress)=>void) {
 const job=new BitmapTraceJob(input);let work=0;expect(()=>job.result()).toThrow(/incomplete/);
 for(let at=0;at<input.maxWork;at++){const p=job.advance(grant);expect(progressValid(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;observer?.(p);if(p.done){const result=job.result();expect(outputValid(result)).toBe(true);return result;}expect(()=>job.result()).toThrow(/incomplete/);}
 throw Error("Trace did not terminate");
}
function rings(segments:PathSegment[]) {
 const result:Vec2[][]=[];for(const segment of segments){if(segment.kind==="move")result.push([segment.to]);else if(segment.kind==="line")result.at(-1)!.push(segment.to);}return result;
}
const area=(ring:readonly Vec2[])=>ring.reduce((sum,p,at)=>{const q=ring[(at+1)%ring.length]!;return sum+p[0]!*q[1]!-q[0]!*p[1]!;},0)/2;
function foreground(segments:PathSegment[],input:BitmapTraceInput) {
 const contours=rings(segments);return Array.from({length:input.width*input.height},(_,at)=>{const x=at%input.width+.5,y=Math.floor(at/input.width)+.5;let winding=0;
 for(const ring of contours)for(let p=0;p<ring.length;p++){const a=ring[p]!,b=ring[(p+1)%ring.length]!,cross=(b[0]!-a[0]!)*(y-a[1]!)-(b[1]!-a[1]!)*(x-a[0]!);if(a[1]!<=y&&b[1]!>y&&cross>0)winding++;else if(a[1]!>y&&b[1]!<=y&&cross<0)winding--;}return winding?1:0;});
}
for(const row of fixtures)test(row.name,async()=>{
 expect(inputValid(row.input)).toBe(true);const before=structuredClone(row.input);let actual:PathSegment[]=[];
 for(const grant of [1,7,4096]){const result=complete(row.input,grant);expect(foreground(result,row.input)).toEqual(row.expected.foreground);expect(rings(result)).toHaveLength(row.expected.contours);if(actual.length)expect(result).toEqual(actual);actual=result;}
 expect(row.input).toEqual(before);if("segments" in row.expected)expect(actual).toEqual(row.expected.segments);
 const cells=row.expected.foreground.flatMap((on,at)=>{const x=at%row.input.width,y=Math.floor(at/row.input.width);return on?[[[[x,y],[x+1,y],[x+1,y+1],[x,y+1],[x,y]]]]:[];});
 const oracle=cells.length?polygonClipping.union(...cells as Parameters<typeof polygonClipping.union>):[];
 expect(rings(actual).reduce((sum,ring)=>sum+Math.sign(area(ring)),0)).toBe(oracle.reduce((sum,polygon)=>sum+2-polygon.length,0));
 if(row.input.simplifyEpsilon===0)expect(rings(actual).reduce((sum,ring)=>sum+area(ring),0)).toBe(row.expected.foreground.reduce((a,b)=>a+b,0));
 const svg='<svg xmlns="http://www.w3.org/2000/svg" width="'+row.input.width+'" height="'+row.input.height+'"><path d="'+pathSegmentsToSvgD(actual)+'" fill="red"/></svg>';
 const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();if(row.input.simplifyEpsilon===0)expect(Array.from({length:row.input.width*row.input.height},(_,at)=>pixels[at*4+3]!)).toEqual(row.expected.foreground.map(value=>value*255));
});
test("invalid contracts and resource failures remain private and sticky",()=>{
 const input=fixtures[0]!.input;
 for(const patch of [{width:0},{height:8193},{width:1.5},{mask:[]},{mask:[255,255]},{threshold:-.1},{threshold:1.1},{threshold:NaN},{simplifyEpsilon:-1},{simplifyEpsilon:Infinity},{maxPixels:0},{maxEdges:0},{maxSegments:0},{maxWork:0},{maxPixels:16777217},{maxEdges:65537},{maxSegments:65537},{maxWork:1000000001}])expect(()=>new BitmapTraceJob({...input,...patch})).toThrow();
 for(const patch of [{mask:[-1]},{mask:[256]},{mask:[.5]},{mask:[NaN]},{maxEdges:3},{maxSegments:4},{maxWork:1}]){const job=new BitmapTraceJob({...input,...patch});expect(()=>{while(!job.advance(4096).done){}}).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();}
 expect(()=>new BitmapTraceJob({...input,width:2,height:1,mask:[255,255],maxPixels:1})).toThrow();
 for(const grant of [0,-1,1.5,NaN,Infinity])expect(()=>new BitmapTraceJob(input).advance(grant)).toThrow();
});
test("all 512 three by three masks retain independent polygon areas and exact pixels",()=>{
 let simplified=0;
 for(let bits=0;bits<512;bits++){const mask=Array.from({length:9},(_,at)=>(bits>>at)&1?255:0),input={...fixtures[0]!.input,width:3,height:3,mask};
  const cells=mask.flatMap((value,at)=>{const x=at%3,y=Math.floor(at/3);return value?[[[[x,y],[x+1,y],[x+1,y+1],[x,y+1],[x,y]]]]:[];}),oracle=cells.length?polygonClipping.union(...cells as Parameters<typeof polygonClipping.union>):[];
  for(const epsilon of [0,.55,8192]){const output=complete({...input,simplifyEpsilon:epsilon},4096,p=>{if(p.done&&p.simplified)simplified++;});expect(foreground(output,input)).toEqual(mask.map(value=>value?1:0));const contours=rings(output);expect(contours.reduce((sum,ring)=>sum+Math.sign(area(ring)),0)).toBe(oracle.reduce((sum,polygon)=>sum+2-polygon.length,0));if(epsilon===0)expect(contours.reduce((sum,ring)=>sum+area(ring),0)).toBe(mask.filter(Boolean).length);}
 }
 expect(simplified).toBeGreaterThan(0);
});
test("blank wide images complete in linear work at the exact grant limit",()=>{
 const input={...fixtures[0]!.input,width:8192,height:1,mask:new Uint8Array(8192),maxPixels:8192,maxEdges:1,maxSegments:1,maxWork:8196},job=new BitmapTraceJob(input),p=job.advance(8196);expect(p.done).toBe(true);expect(p.work).toBe(8196);expect(p.edges).toBe(0);expect(job.result()).toEqual([]);
});
test("owned byte buffers preserve neutral contours under all grants and cancellation",()=>{
 for(const row of fixtures){const expected=complete(row.input,4096);for(const grant of [1,7,4096]){const mask=Uint8Array.from(row.input.mask),before=mask.slice(),job=new BitmapTraceJob({...row.input,mask});expect(()=>job.result()).toThrow(/incomplete/);while(!job.advance(grant).done){}const published=job.result();expect(published).toEqual(expected);job.cancel();expect(published).toEqual(expected);expect(mask).toEqual(before);}}
 console.log("[DEBUG] Owned trace byte buffers preserve neutral contours and published ownership under every grant");
});
test("every phase cancels without publishing private paths or mutating input",()=>{
 const input=fixtures[13]!.input,before=structuredClone(input),seen=new Set<string>();
 for(let stop=0;stop<20000;stop++){const job=new BitmapTraceJob(input);let p:BitmapTraceProgress|undefined;for(let at=0;at<stop;at++){p=job.advance(1);if(p.done)break;}const phase=p?.phase??"scan";if(!seen.has(phase)){job.cancel();expect(()=>job.result()).toThrow(/cancelled/);expect(()=>job.advance(1)).toThrow(/cancelled/);seen.add(phase);}if(p?.done)break;}
 expect(seen).toEqual(new Set(["scan","contours","compact","simplify","topology","coverage","emit","complete"]));expect(input).toEqual(before);
 const job=new BitmapTraceJob(input);while(!job.advance(4096).done){}const published=job.result(),copy=structuredClone(published);job.cancel();expect(published).toEqual(copy);
});
test("async tracing yields and rejects aborts before final publication",async()=>{
 const input=fixtures[0]!.input,events:string[]=[];const timer=setTimeout(()=>events.push("timer"),0);
 const result=await traceBitmap(input,{workBudget:1,onProgress:p=>events.push(p.phase)});clearTimeout(timer);expect(foreground(result,input)).toEqual([1]);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 for(const when of ["scan","complete"]){const controller=new AbortController();await expect(traceBitmap(input,{workBudget:1,signal:controller.signal,onProgress:p=>{if(p.phase===when)controller.abort();}})).rejects.toThrow(/cancelled/);}
 const controller=new AbortController();controller.abort();let called=false;await expect(traceBitmap(input,{signal:controller.signal,onProgress:()=>{called=true;}})).rejects.toThrow(/cancelled/);expect(called).toBe(false);
});

/** 🧹️ Interrupts real trace states and compares cleanup to an independent JSON inventory. */
import retirementRows from "../🧫️fixtures/🧹️retirement/🔣️.json";
const retirementValid=ajv.compile({$ref:schema.$id+"#/definitions/retirementProgress"});
test("trace retirement transfers masks and drains every actual phase under exact grants",()=>{
 for(const row of retirementRows)for(const grant of [1,7,4096]){
  const source=fixtures.find(v=>v.name===row.source)!;const mask=Uint8Array.from(source.input.mask);const job=new BitmapTraceJob({...source.input,mask,...("maxEdges" in row?{maxEdges:row.maxEdges}: {})});
  if(row.phase==="cancelled")job.cancel();else if(row.phase==="failure"){expect(()=>job.advance(4096)).toThrow();expect(()=>job.result()).toThrow();}
  else if(row.phase!=="fresh"){let found=false;for(let at=0;at<source.input.maxWork;at++){const p=job.advance(1);if(p.phase===row.phase){found=true;break;}if(p.done)break;}expect(found).toBe(true);}
  const state=job as any,inventory=JSON.parse(JSON.stringify({outgoing:[...state.outgoing],raw:state.raw,base:state.base,candidate:state.candidate,kept:[...state.kept],changes:[...state.changes],positions:[...state.positions]}));
  const expected=13+Object.values(inventory).reduce((n:number,v:any)=>n+v.length,0);if("work" in row)expect(expected).toBe(row.work);
  const published=row.phase==="complete"?job.result():null,publishedBefore=published?JSON.parse(JSON.stringify(published)):null;const returned=job.intoRetirement();expect(returned.mask).toBe(row.phase==="cancelled"?null:mask);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);for(const invalidGrant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>returned.job.advance(invalidGrant)).toThrow(/grant/i);
  let work=0;for(let at=0;at<=expected;at++){const p=returned.job.advance(grant);expect(retirementValid(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}
  if(grant===1)console.log("[DEBUG] Trace retirement "+row.phase+" work="+work);expect(work).toBe(expected);expect(returned.job.terminalIsEmpty()).toBe(true);expect(returned.job.advance(1)).toEqual({phase:"complete",work,done:true});expect([...mask]).toEqual(source.input.mask);if(published)expect(published).toEqual(publishedBefore);
  expect(state.input.mask).toHaveLength(0);for(const key of ["edges","raw","base","candidate","flat","ring","ranges","output"])expect(state[key]).toHaveLength(0);for(const key of ["outgoing","kept","changes","positions"])expect(state[key].size).toBe(0);expect(state.split).toBe(null);
 }
 console.log("[DEBUG] Actual trace phases transfer genuine masks and drain JSON-counted geometry under grants 1, 7 and 4096");
});
