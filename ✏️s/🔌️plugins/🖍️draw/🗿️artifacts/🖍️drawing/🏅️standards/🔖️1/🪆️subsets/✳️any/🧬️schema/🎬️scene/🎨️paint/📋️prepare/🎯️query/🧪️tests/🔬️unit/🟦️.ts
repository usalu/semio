/** 🖱️ Complete-cache picks agree across grants and independent rendered SVG paint. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {readFileSync} from "node:fs";
import {builtinFontLocations} from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/🟦️.ts";
import {Box2,Matrix3,Vector2} from "three";
import rows from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import vectorRows from "../../../../../📋️prepare/🧫️fixtures/🎬️vector/🔣️.json";
import {DocumentVectorJob} from "../../../../../📋️prepare/🟦️.ts";
import type {DrawingArtifact} from "../../../../../../🟦️.ts";
import {ScenePaintJob,PreparedSceneCloseJob,type PreparedScene} from "../../../🟦️.ts";
import {PreparedScenePickJob,preparedSelectionBounds,type PreparedSceneIdentity,type PreparedScenePick} from "../../🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
const identity:PreparedSceneIdentity={source:{instance:710034,base:"9007199254740993",generation:"18446744073709551615",revision:Array(32).fill(3)},build:"9007199254740993",flatness:.001};
const lift=(v:any):any=>typeof v==="number"?binary64(v):Array.isArray(v)?v.map(lift):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,v])=>[k,lift(v)])):v;
function scene(name:string):PreparedScene{const row=vectorRows.find(r=>r.name===name)!,doc={...row.document,layers:lift(row.document.layers)}as DrawingArtifact,vector=new DocumentVectorJob(doc,row.limits,row.algorithms);while(!vector.advance(4096).done){if(vector.needsFontSources())vector.admitFontSources(builtinFontLocations().map(source=>({family:source.family,id:source.id,bytes:new Uint8Array(readFileSync(source.url))})));}const moved=vector.intoRetirement();while(!moved.job.advance(4096).done){}const paint=new ScenePaintJob(moved.output!,.001,{maxNodes:1024,maxSegments:65536,maxPoints:262144,maxContours:65536,maxWork:1e9});while(!paint.advance(4096).done){}const ready=paint.intoRetirement();while(!ready.job.advance(4096).done){}return ready.output!;}
function close(scene:PreparedScene):void{const job=new PreparedSceneCloseJob(scene);while(!job.advance(4096).done){}}
test("prepared scene picking uses actual resolved paint in front-to-back order with immutable cache",async()=>{
 const ajv=new Ajv({strict:true});for(const row of rows)expect(ajv.compile(schema)(row.query)).toBe(true);let svgSamples=0;
 for(const row of rows)for(const grant of [1,7,4096]){
  const ready=scene(row.source),before=structuredClone(ready),job=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256);let work=0;
  expect(()=>job.result(identity)).toThrow(/incomplete/);
  for(let n=0;n<2000000;n++){const p=job.advance(ready,identity,grant);expect(ajv.compile(schema.$defs.progress)(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}
  expect(job.result(identity).map(index=>ready.plan.nodes[index]!.id),`${row.source} ${JSON.stringify(row.query)}`).toEqual(row.expected);expect(ready).toEqual(before);
  const fixture=vectorRows.find(r=>r.name===row.source)!,oracle=fixture.oracle as any;
  if(row.query.kind==="point"&&oracle){const paths=oracle.paths??[{d:oracle.d,matrix:oracle.matrix}],svg=`<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">${paths.map((p:any)=>`<path d="${p.d}" transform="matrix(${p.matrix.join(" ")})" fill="black"/>`).join("")}</svg>`,pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();expect(pixels[(Math.floor(row.query.point[1]!)*16+Math.floor(row.query.point[0]!))*4+3]!>127).toBe(row.expected.length>0);svgSamples++;}
  close(ready);
 }
 process.stderr.write(`[DEBUG] Complete scene picks matched ${rows.length*3} neutral queries and ${svgSamples} independent actual Boolean/PNG trace SVG samples with immutable caches\n`);
});
test("crossing rectangle selection agrees with independent rendered SVG paint",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);for(const row of rows)expect(validate(row.query)).toBe(true);let samples=0;
 for(const row of rows){if(row.query.kind!=="rectangle"||!row.query.crossing)continue;
  const q=row.query,fixture=vectorRows.find(r=>r.name===row.source)!,oracle=fixture.oracle as any;
  expect(oracle).toBeDefined();const paths=oracle.paths??[{d:oracle.d,matrix:oracle.matrix}],svg=`<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 16 16">${paths.map((p:any)=>`<path d="${p.d}" transform="matrix(${p.matrix.join(" ")})" fill="black"/>`).join("")}</svg>`,pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  let painted=false;for(let y=Math.max(0,Math.ceil(Math.min(q.start[1],q.end[1])*4));y<Math.min(64,Math.floor(Math.max(q.start[1],q.end[1])*4));y++)for(let x=Math.max(0,Math.ceil(Math.min(q.start[0],q.end[0])*4));x<Math.min(64,Math.floor(Math.max(q.start[0],q.end[0])*4));x++){painted ||=pixels[(y*64+x)*4+3]!>127;samples++;}
  expect(painted).toBe(row.expected.length>0);const ready=scene(row.source),job=new PreparedScenePickJob(identity,q as PreparedScenePick,256);try{while(!job.advance(ready,identity,1).done){}expect(job.result(identity).map(index=>ready.plan.nodes[index]!.id),`${row.source} ${JSON.stringify(row.query)}`).toEqual(row.expected);}finally{close(ready);}
 }
 expect(samples).toBeGreaterThan(0);process.stderr.write(`[DEBUG] Crossing rectangle selection matched ${samples} independent SVG paint samples\n`);
});
test("lasso containment agrees with independent SVG masks for actual affine paint",async()=>{
 let comparisons=0;
 for(const row of rows){if(row.query.kind!=="lasso")continue;
  const fixture=vectorRows.find(r=>r.name===row.source)!,oracle=fixture.oracle as any,expected=fixture.expected as any;
  const paths: {id:string;d:string;matrix:number[]}[]=(row as any).oracle??(oracle?(oracle.paths??[{d:oracle.d,matrix:oracle.matrix}]).map((p:any)=>({...p,id:"result"})):(expected?.nodes??[]).filter((n:any)=>n.visible&&n.opacity>0&&n.lockedAncestors===0&&n.content.kind!=="group").map((n:any)=>({id:n.id,matrix:n.transform,d:n.content.kind==="path"?n.content.segments.map((s:any)=>s.kind==="close"?"Z":`${s.kind==="move"?"M":"L"}${s.to.join(" ")}`).join(" "):`M0 0H${n.content.width}V${n.content.height}H0Z`})));
  const polygon=row.query.points.map(p=>p.join(" ")).join(" "),points=row.query.points,x=Math.min(...points.map(p=>p[0]!))-1,y=Math.min(...points.map(p=>p[1]!))-1,w=Math.max(...points.map(p=>p[0]!))-x+1,h=Math.max(...points.map(p=>p[1]!))-y+1;
  const render=(body:string)=>sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="160" height="160" viewBox="${x} ${y} ${w} ${h}">${body}</svg>`)).ensureAlpha().raw().toBuffer();
  const ids:string[]=[];
  for(const p of paths){const shape=`<path d="${p.d}" transform="matrix(${p.matrix.join(" ")})" fill="black"/>`,outside=await render(`<defs><mask id="outside" maskUnits="userSpaceOnUse" x="${x}" y="${y}" width="${w}" height="${h}"><rect x="${x}" y="${y}" width="${w}" height="${h}" fill="white"/><polygon points="${polygon}" fill="black" fill-rule="evenodd"/></mask></defs><g mask="url(#outside)">${shape}</g>`),all=await render(shape);let painted=0,excluded=0;for(let at=3;at<all.length;at+=4){if(all[at]!>127)painted++;if(outside[at]!>127)excluded++;}if(painted>0&&excluded===0)ids.unshift(p.id);comparisons++;}
  expect(ids).toEqual(row.expected);
  const ready=scene(row.source),job=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256);try{while(!job.advance(ready,identity,1).done){}expect(job.result(identity).map(i=>ready.plan.nodes[i]!.id),row.source).toEqual(ids);}finally{close(ready);}
 }
 expect(comparisons).toBeGreaterThan(0);process.stderr.write(`[DEBUG] Painted lasso containment matched ${comparisons} independent SVG scene/mask comparisons\n`);
});
test("scene pick pending output, authority, precision, cancellation and capacity refuse without cache edits",()=>{
 const ready=scene("shared asset geometry"),before=structuredClone(ready),query={kind:"rectangle",start:[-1,-1],end:[3,2],crossing:false}as const;
 const job=new PreparedScenePickJob(identity,query,1);expect(()=>{while(!job.advance(ready,identity,7).done){}}).toThrow(/capacity/);expect(()=>job.result(identity)).toThrow(/capacity/);expect(ready).toEqual(before);
 for(const grant of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>new PreparedScenePickJob(identity,query,256).advance(ready,identity,grant)).toThrow(/grant/);
 const cancelled=new PreparedScenePickJob(identity,query,256);cancelled.advance(ready,identity,1);cancelled.cancel();expect(()=>cancelled.advance(ready,identity,1)).toThrow(/cancel/);expect(()=>cancelled.result(identity)).toThrow(/cancel/);
 const stale=new PreparedScenePickJob(identity,query,256);stale.advance(ready,identity,1);expect(()=>stale.advance(ready,{...identity,build:"9007199254740992"},1)).toThrow(/authority/);expect(()=>stale.result(identity)).toThrow(/authority/);
 expect(()=>new PreparedScenePickJob(identity,{kind:"point",point:[0,0],tolerance:0,requiredFlatness:.0005},256)).toThrow(/precision/);
 expect(ready).toEqual(before);close(ready);process.stderr.write("[DEBUG] Complete-cache scene pick cancellation, precision, stale authority and bounded result capacity refused without geometry mutation\n");
});
test("crossing rectangle boundary work refuses cancellation and changed cache authority privately",()=>{
 const row=rows.find(r=>r.query.kind==="rectangle"&&r.query.crossing&&!r.expected.length)!,ready=scene(row.source),before=structuredClone(ready);
 try{for(const action of ["cancel","build","precision"]){const job=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256);while(job.advance(ready,identity,1).phase!=="paint"){}expect(job.advance(ready,identity,20).done).toBe(false);expect(()=>job.result(identity)).toThrow(/incomplete/);if(action==="cancel")job.cancel();const live=action==="build"?{...identity,build:"9007199254740992"}:identity;if(action==="precision")ready.flatness=.002;expect(()=>job.advance(ready,live,1)).toThrow(action==="cancel"?/cancel/:action==="build"?/authority/:/precision/);ready.flatness=.001;expect(()=>job.result(identity)).toThrow();expect(ready).toEqual(before);}}finally{close(ready);}
 process.stderr.write("[DEBUG] Borrowed crossing boundary work keeps cancelled and stale partial hits private\n");
});
test("lasso segment partitions stay private across cancellation and changed cache authority",()=>{
 const row=rows.find(r=>r.source==="group full affine"&&r.query.kind==="lasso"&&r.expected.length)!,ready=scene(row.source),before=structuredClone(ready);
 try{for(const action of ["cancel","build","precision"]){const job=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256);let work=0;for(let n=0;n<1000;n++){const p=job.advance(ready,identity,1);work=p.work;if(p.phase==="paint")break;}expect(job.advance(ready,identity,20).done).toBe(false);expect(()=>job.result(identity)).toThrow(/incomplete/);if(action==="cancel")job.cancel();const live=action==="build"?{...identity,build:"9007199254740992"}:identity;if(action==="precision")ready.geometry[1]!.paint.kind==="path"&&(ready.geometry[1]!.paint.regions.flatness=.002);expect(()=>job.advance(ready,live,1)).toThrow(action==="cancel"?/cancel/:action==="build"?/authority/:/precision/);if(ready.geometry[1]!.paint.kind==="path")ready.geometry[1]!.paint.regions.flatness=.001;expect(()=>job.result(identity)).toThrow();expect(ready).toEqual(before);expect(work).toBeGreaterThan(0);}}
 finally{close(ready);}
 process.stderr.write("[DEBUG] Borrowed lasso segment work rejects cancellation and cache changes without partial selection or geometry copies\n");
});
test("cached selection bounds use the same unlocked selected group prefix as pointer handles",()=>{
 const ready=scene("group full affine"),matrix=new Matrix3().set(2,1,22,0,3,34,0,0,1),oracle=new Box2().setFromPoints([[0,0],[2,0],[2,1],[0,1]].map(([x,y])=>new Vector2(x,y).applyMatrix3(matrix)));expect([oracle.min.x,oracle.min.y,oracle.max.x,oracle.max.y]).toEqual([22,34,27,37]);expect(preparedSelectionBounds(ready,["group"])).toEqual([22,34,27,37]);expect(preparedSelectionBounds(ready,["child"])).toEqual([22,34,27,37]);
 ready.plan.nodes[1]!.lockedAncestors=2;expect(preparedSelectionBounds(ready,["group"])).toEqual([22,34,27,37]);expect(preparedSelectionBounds(ready,["child"])).toBeNull();close(ready);
 const hidden=scene("hidden ancestor retains child");expect(preparedSelectionBounds(hidden,["group"])).toBeNull();close(hidden);
 process.stderr.write("[DEBUG] Cached group/leaf handle bounds share actual paint geometry and unlocked prefix authority\n");
});
test("late lasso interior work refuses cancellation and changed authority with no published hits",()=>{
 const row=rows.find(r=>"name" in r&&r.name==="twice traced island cancels")!,ready=scene(row.source),before=structuredClone(ready),complete=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256);let work=0;
 try{while(true){const progress=complete.advance(ready,identity,1);work=progress.work;if(progress.done)break;}expect(complete.result(identity).map(i=>ready.plan.nodes[i]!.id)).toEqual(["child"]);expect(work).toBeGreaterThan(32);
  for(const action of ["cancel","build","precision"]){const job=new PreparedScenePickJob(identity,row.query as PreparedScenePick,256),progress=job.advance(ready,identity,work-16);expect(progress.phase).toBe("paint");expect(progress.done).toBe(false);expect(()=>job.result(identity)).toThrow(/incomplete/);if(action==="cancel")job.cancel();const live=action==="build"?{...identity,build:"9007199254740992"}:identity;const paint=ready.geometry[1]!.paint;if(action==="precision"&&paint.kind==="path")paint.regions.flatness=.002;expect(()=>job.advance(ready,live,1)).toThrow(action==="cancel"?/cancel/:action==="build"?/authority/:/precision/);if(paint.kind==="path")paint.regions.flatness=.001;expect(()=>job.result(identity)).toThrow();expect(ready).toEqual(before);}
 }finally{close(ready);}
 process.stderr.write(`[DEBUG] Late lasso interior work refused cancellation, build and painted-entry precision after ${work-16} grants without publishing partial hits\n`);
});
