import {test as register} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {ShapeUtils, Vector2} from "three";
import {triangulateRings} from "../🟦️.ts";
export const records: {name:string; passed:boolean; error?:string}[]=[];
export const checks: Promise<void>[]=[];
const test=(name:string,run:()=>void):void=>{checks.push(register(name,()=>{try{run();records.push({name,passed:true});}catch(error){records.push({name,passed:false,error:String(error)});throw error;}}));};
type Point=[number,number,number];
const fixtures=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")) as {cases:{name:string;rings:Point[][];area:number}[]};
const area=(p:Point[],t:number[][]):number=>t.reduce((sum,[a,b,c])=>sum+Math.abs((p[b][0]-p[a][0])*(p[c][1]-p[a][1])-(p[b][1]-p[a][1])*(p[c][0]-p[a][0]))/2,0);
const input=(rings:Point[][])=>{const p:Point[]=[],r:number[][]=[];for(const ring of rings){r.push(ring.map(point=>{p.push(point);return p.length-1;}));}return {p,r};};
for(const row of fixtures.cases)test("ring coverage: "+row.name,()=>{
  for(const reversed of [false,true])for(const scale of [1,1e-120,1e120]){
    const rings=row.rings.map(ring=>(reversed?[...ring].reverse():ring).map(p=>p.map(v=>v*scale) as Point)),{p,r}=input(rings);
    const triangles=triangulateRings(p,r,()=>{});
    const reference=ShapeUtils.triangulateShape(rings[0].map(p=>new Vector2(p[0]/scale,p[1]/scale)),rings.slice(1).map(r=>r.map(p=>new Vector2(p[0]/scale,p[1]/scale))));
    const normalized=p.map(p=>p.map(v=>v/scale) as Point);
    assert.ok(Math.abs(area(normalized,triangles)-row.area)<1e-8);
    const oraclePoints=rings.flat().map(p=>p.map(v=>v/scale) as Point);
    assert.ok(Math.abs(area(oraclePoints,reference)-row.area)<1e-8);
  }
});
test("rings reject outside, nested, touching, crossing and nonplanar holes",()=>{
  const outer:Point[]=[[0,0,0],[10,0,0],[10,10,0],[0,10,0]],hole:Point[]=[[2,2,0],[4,2,0],[4,4,0],[2,4,0]];
  for(const rings of [[outer,[[11,1,0],[12,1,0],[12,2,0]]],[outer,hole,[[2.5,2.5,0],[3,2.5,0],[3,3,0]]],[outer,[[0,2,0],[2,2,0],[2,4,0]]],[outer,[[2,2,0],[8,8,0],[2,8,0],[8,2,0]]],[outer,[[2,2,0],[4,2,0.1],[4,4,0]]]] as Point[][][]){const {p,r}=input(rings);assert.throws(()=>triangulateRings(p,r,()=>{}));}
});
test("ring triangulation preserves orientation and every control interruption",()=>{
  const {p,r}=input(fixtures.cases[0].rings);let work=0;const triangles=triangulateRings(p,r,()=>{work++;});
  for(const [a,b,c] of triangles)assert.ok((p[b][0]-p[a][0])*(p[c][1]-p[a][1])-(p[b][1]-p[a][1])*(p[c][0]-p[a][0])>0);
  for(let stop=1;stop<=work;stop++){let count=0;assert.throws(()=>triangulateRings(p,r,()=>{if(++count===stop)throw new Error("cancelled");}),/cancelled/);}
});

test("collinear and near-collinear boundaries retain each source segment",()=>{
  for(const row of fixtures.cases.slice(3)){
    const {p,r}=input(row.rings),triangles=triangulateRings(p,r,()=>{}),edges=new Map<string,number>(),key=(a:number,b:number)=>a<b?a+":"+b:b+":"+a;
    for(const triangle of triangles)for(let i=0;i<3;i++){const edge=key(triangle[i],triangle[(i+1)%3]);edges.set(edge,(edges.get(edge)??0)+1);}
    for(const ring of r){if(p[ring[0]].every((value,axis)=>value===p[ring[ring.length-1]][axis]))ring.pop();for(let i=0;i<ring.length;i++)assert.equal(edges.get(key(ring[i],ring[(i+1)%ring.length])),1,row.name);}
    assert.equal(edges.get(key(0,1)),1,row.name);assert.equal(edges.get(key(1,2)),1,row.name);assert.notEqual(edges.get(key(0,2)),1,row.name);
  }
});

test("oblique holes choose the visible occurrence of a repeated bridge endpoint",()=>{
  const row=JSON.parse(readFileSync(new URL("./📜️oblique.json",import.meta.url),"utf8")) as {rings:[number,number][][];area:number;triangles:number};
  for(const reverse of [false,true])for(const scale of [1,1e-90,1e90]){
    const source=row.rings.map((ring,i)=>reverse&&i%2===0?[...ring].reverse():ring),points=source.flat().map(([x,y])=>[x/Math.SQRT2-y/Math.sqrt(6),x/Math.SQRT2+y/Math.sqrt(6),2*y/Math.sqrt(6)].map(v=>v*scale) as Point);let offset=0;const rings=source.map(r=>r.map(()=>offset++));
    const triangles=triangulateRings(points,rings,()=>{});assert.equal(triangles.length,row.triangles);
    const flat=source.flat().map(([x,y])=>[x,y,0] as Point),oracle=ShapeUtils.triangulateShape(source[0].map(p=>new Vector2(...p)),source.slice(1).map(r=>r.map(p=>new Vector2(...p))));
    assert.ok(Math.abs(area(flat,triangles)-row.area)<1e-8);assert.ok(Math.abs(area(flat,oracle)-row.area)<1e-8);
  }
});
